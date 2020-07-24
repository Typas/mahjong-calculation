use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
// use rayon::{
//     prelude::{IntoParallelIterator,ParallelIterator},
//     iter::ParallelBridge,
//     slice::ParallelSliceMut,
// };

const HAINUM: usize = 11;
const NR_PER_TILE: usize = 4;
const TILE_LAST_KIND: Tile = Tile::Dot6;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
enum Tile {
    Red,
    Green,
    White,
    Bamboo1,
    Bamboo2,
    Bamboo3,
    Bamboo4,
    Bamboo5,
    Bamboo6,
    Character1,
    Character2,
    Character3,
    Character4,
    Character5,
    Character6,
    Dot1,
    Dot2,
    Dot3,
    Dot4,
    Dot5,
    Dot6,
}

impl std::ops::Add<u8> for Tile {
    type Output = Self;

    fn add(self, rhs: u8) -> Self {
        const NR_TILE_KIND: u8 = TILE_LAST_KIND as u8 + 1;
        let result = (self as u8 + rhs) % NR_TILE_KIND;
        Tile::from_u8(result).unwrap()
    }
}

impl std::ops::AddAssign<u8> for Tile {
    fn add_assign(&mut self, rhs: u8) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::Sub<u8> for Tile {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        const NR_TILE_KIND: i16 = TILE_LAST_KIND as i16 + 1;
        let mut result = self as i16 + rhs as i16;
        while result < 0 {
            result += NR_TILE_KIND;
        }

        Tile::from_u8(result as u8).unwrap()
    }
}

impl std::ops::SubAssign<u8> for Tile {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.clone() - rhs;
    }
}

struct MahjongStatistics {
    pub combination_count: u64,
    pub valid_count: u64,
    pub pattern_count: u64,
    pub valid_pattern_count: u64,
}

impl MahjongStatistics {
    fn new() -> MahjongStatistics {
        MahjongStatistics {
            combination_count: 0,
            valid_count: 0,
            pattern_count: 0,
            valid_pattern_count: 0,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct HaiSet {
    hai: [Tile; HAINUM],
}

impl HaiSet {
    fn from_array(arr: [Tile; HAINUM]) -> HaiSet {
        HaiSet { hai: arr }
    }

    fn into_vec_u8(self) -> Vec<u8> {
        self.hai.iter().map(|x| x.clone() as u8 + b'A').collect()
    }

    fn iter<'a>(&'a self) -> HaiSetIterator<impl Iterator<Item = Tile> + 'a> {
        HaiSetIterator {
            haisetiter: self.hai.iter().cloned(),
        }
    }

    fn len(&self) -> usize {
        self.hai.len()
    }
}

struct HaiSetIterator<I> {
    haisetiter: I,
}

impl<'a, I> Iterator for HaiSetIterator<I>
where
    I: std::iter::Iterator<Item = Tile>,
{
    type Item = Tile;
    fn next(&mut self) -> Option<Tile> {
        self.haisetiter.next()
    }
}

impl std::ops::Index<usize> for HaiSet {
    type Output = Tile;

    fn index(&self, i: usize) -> &Self::Output {
        &self.hai[i]
    }
}

struct HaiSetGen {
    set: HaiSet,
}

impl HaiSetGen {
    fn new() -> HaiSetGen {
        // FIXME: generalize the parameters
        HaiSetGen {
            set: HaiSet::from_array([
                Tile::Red,
                Tile::Red,
                Tile::Red,
                Tile::Red,
                Tile::Green,
                Tile::Green,
                Tile::Green,
                Tile::Green,
                Tile::White,
                Tile::White,
                Tile::Green,
            ]),
        }
    }
}

impl Iterator for HaiSetGen {
    type Item = HaiSet;

    fn next(&mut self) -> Option<HaiSet> {
        // iterate to next
        match self.set.hai.last()? {
            &TILE_LAST_KIND => {
                // FIXME: performance bottle neck
                // find the `pivot` of new pattern
                let len = self.set.hai.len();
                let mut i = len;
                let pivot = loop {
                    i -= 1;
                    let r = len - i - 1;
                    let target = self.set.hai[i].clone() + (r / NR_PER_TILE) as u8;
                    match (target, i) {
                        (TILE_LAST_KIND, 0) => return None,
                        (TILE_LAST_KIND, _) => (),
                        _ => break i,
                    }
                };

                let hai_pivot = self.set.hai[pivot].clone() + 1;
                for i in pivot..len {
                    self.set.hai[i] = hai_pivot.clone() + ((i - pivot) / NR_PER_TILE) as u8;
                }
            }
            _ => {
                self.set.hai[HAINUM - 1] += 1;
            }
        }

        Some(self.set.clone())
    }
}

#[derive(Debug)]
struct HaiTriplet<'a> {
    s: [&'a Tile; 3],
}

impl<'a> HaiTriplet<'a> {
    fn new(first: &'a Tile, second: &'a Tile, third: &'a Tile) -> HaiTriplet<'a> {
        HaiTriplet {
            s: [first, second, third],
        }
    }

    fn is_chow(&self) -> bool {
        let zero_next = self.s[0].clone() + 1;
        let one_next = self.s[1].clone() + 1;
        if &zero_next == self.s[1] && &one_next == self.s[2] {
            // FIXME: find a better match pattern approach
            match self.s[0] {
                Tile::Bamboo1
                | Tile::Bamboo2
                | Tile::Bamboo3
                | Tile::Bamboo4
                | Tile::Character1
                | Tile::Character2
                | Tile::Character3
                | Tile::Character4
                | Tile::Dot1
                | Tile::Dot2
                | Tile::Dot3
                | Tile::Dot4 => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_pung(&self) -> bool {
        self.s[0] == self.s[1] && self.s[1] == self.s[2]
    }
}

fn main() -> std::io::Result<()> {
    let t = Instant::now();
    let mut mj_stat = MahjongStatistics::new();
    let record = hailoop(&mut mj_stat)
        .into_iter()
        .map(|x| x.into_vec_u8())
        .flatten()
        .collect::<Vec<_>>();
    println!("time used: {} secs", t.elapsed().as_secs_f64());

    println!("total combination: {}", mj_stat.combination_count);
    println!("total agari combination: {}", mj_stat.valid_count);
    println!("total pattern: {}", mj_stat.pattern_count);
    println!("total agari pattern: {}", mj_stat.valid_pattern_count);

    let filename = "patterns_general_three.dat";
    let mut file = File::create(filename)?;
    file.write(&record)?;
    println!("time used: {} secs", t.elapsed().as_secs_f64());
    Ok(())
}

fn hailoop(mj_stat: &mut MahjongStatistics) -> Vec<HaiSet> {
    let hsg = HaiSetGen::new();
    // main problem: how to record "total combination count"?
    // this should be done in the iteration,
    // since it's impossible to store 100G+ in the memory

    let records: Vec<HaiSet>
    // let mut records: Vec<HaiSet>
        = hsg.into_iter()
             .map(|x| {
                 mj_stat.pattern_count += 1;
                 mj_stat.combination_count += combination(&x).unwrap();

                 x
             })
             // .par_bridge()
             // .into_par_iter()
             .filter(|hs| is_valid(hs))
             .collect();

    // records.par_sort();

    records.iter().for_each(|hs| {
        mj_stat.valid_count += combination(hs).unwrap();
        mj_stat.valid_pattern_count += 1;
    });

    records
}

fn combination(hai: &HaiSet) -> Option<u64> {
    let mut ts = [&hai[0]; HAINUM];
    let mut ns = [0; HAINUM];
    let mut is = 0;
    ns[0] = 1;

    for i in 1..HAINUM {
        if &hai[i] == ts[is] {
            ns[is] += 1;
        } else {
            is += 1;
            ts[is] = &hai[i];
            ns[is] = 1;
        }
    }

    let mut product = 1;
    for i in 0..ts.len() {
        product *= binom(NR_PER_TILE as u32, ns[i]) as u64;
    }


    Some(product)
}

#[inline(always)]
fn binom(n: u32, k: u32) -> u32 {
    let mut res = 1;

    let k = if k > (n - k) { n - k } else { k };

    (0..k).for_each(|i| res = res * (n - i) / (i + 1));

    res
}

fn is_valid(hai: &HaiSet) -> bool {
    let sum: i32 = hai.iter().map(|x| x.clone() as i32).sum();
    assert_eq!(hai.len(), HAINUM);

    // first step: check at least one pair exist
    let mut has_pair = [false; HAINUM - 1];
    (0..HAINUM - 1).for_each(|i| has_pair[i] = hai[i] == hai[i + 1]);
    if has_pair.iter().filter(|&&x| x).count() == 0 {
        return false;
    }

    // second step: check sum despite pair is a multiple of 3
    let mut possible_pairs = [false; HAINUM - 1];
    (0..HAINUM - 1).for_each(|i| {
        if has_pair[i] {
            possible_pairs[i] = ((sum - 2 * (hai[i].clone() as i32)) % 3) == 0;
        }
    });
    if possible_pairs.iter().filter(|&&x| x).count() == 0 {
        return false;
    }

    // dedup possible pairs
    (1..HAINUM - 1).rev().for_each(|i| {
        if possible_pairs[i] && possible_pairs[i - 1] {
            possible_pairs[i] = false;
        }
    });

    // final step: check for triples
    let mut valid_pairs = [false; HAINUM - 1];
    for i in 0..HAINUM - 1 {
        if possible_pairs[i] {
            let mut cphai = [&Tile::Red; HAINUM - 2];
            (0..HAINUM)
                .into_iter()
                .filter(|j| (j != &i) && (*j != i + 1))
                .enumerate()
                .for_each(|(k, j)| cphai[k] = &hai[j]);
            valid_pairs[i] = check_hai(&cphai);
        }
    }

    match valid_pairs.iter().filter(|&&x| x).count() {
        0 => false,
        _ => true,
    }
}

fn check_hai(hai: &[&Tile; HAINUM - 2]) -> bool {
    let hai_len = hai.len();
    const CHECK_LEN: usize = HAINUM - 2;
    assert_eq!(
        hai_len, CHECK_LEN,
        "not correct sub hai length: {}, should be {}",
        hai_len, CHECK_LEN
    );

    if hai_len != CHECK_LEN {
        return true;
    }

    let mut used_index = [false; CHECK_LEN];
    let mut i = 0;
    while i < CHECK_LEN {
        if used_index[i] {
            i += 1;
            continue;
        }

        let triplet = HaiTriplet::new(&hai[i], &hai[i + 1], &hai[i + 2]);
        if (triplet.is_chow() || triplet.is_pung()) && !(used_index[i + 1]) && !(used_index[i + 2])
        {
            // FIXME: find a way to assign in one line
            used_index[i] = true;
            used_index[i + 1] = true;
            used_index[i + 2] = true;
            i += 3;
            continue;
        }

        let next_index = match used_index
            .iter()
            .enumerate()
            .skip(i + 1)
            .find(|&(n, c)| !c && &hai[n] != &hai[i])
            .map(|(i, _)| i)
        {
            Some(n) => n,
            None => return false,
        };
        let last_index = match used_index
            .iter()
            .enumerate()
            .skip(next_index + 1)
            .find(|&(n, c)| !c && &hai[n] != &hai[next_index])
            .map(|(i, _)| i)
        {
            Some(n) => n,
            None => return false,
        };

        let sparse_triplet = HaiTriplet::new(&hai[i], &hai[next_index], &hai[last_index]);
        if sparse_triplet.is_chow() {
            // FIXME: find a way to assign in one line
            used_index[i] = true;
            used_index[next_index] = true;
            used_index[last_index] = true;
            i += 1;
            continue;
        } else {
            return false;
        }
    }

    match used_index.iter().filter(|&&x| x == false).count() {
        0 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_hai() {
        let pattern = [
            &Tile::Dot4,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot6,
            &Tile::Dot6,
            &Tile::Dot6,
            &Tile::Dot6,
        ];
        assert_eq!(check_hai(&pattern), true);

        let pattern = [
            &Tile::Dot3,
            &Tile::Dot3,
            &Tile::Dot4,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot5,
            &Tile::Dot6,
            &Tile::Dot6,
        ];
        assert_eq!(check_hai(&pattern), false);

        let pattern = [
            &Tile::Character4,
            &Tile::Character4,
            &Tile::Character5,
            &Tile::Character6,
            &Tile::Dot2,
            &Tile::Dot4,
            &Tile::Dot5,
            &Tile::Dot6,
            &Tile::Dot6,
        ];
        assert_eq!(check_hai(&pattern), false);
        let pattern = [
            &Tile::Character4,
            &Tile::Character4,
            &Tile::Character5,
            &Tile::Character6,
            &Tile::Dot2,
            &Tile::Dot3,
            &Tile::Dot3,
            &Tile::Dot4,
            &Tile::Dot5,
        ];
        assert_eq!(check_hai(&pattern), false);
    }

    #[test]
    fn test_combination() {
        let pattern = HaiSet {
            hai: [
                Tile::Dot4,
                Tile::Dot4,
                Tile::Dot4,
                Tile::Dot5,
                Tile::Dot5,
                Tile::Dot5,
                Tile::Dot5,
                Tile::Dot6,
                Tile::Dot6,
                Tile::Dot6,
                Tile::Dot6,
            ],
        };
        assert_eq!(combination(&pattern).unwrap(), 4);
    }
}
