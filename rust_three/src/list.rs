use std::fs::File;
use std::io::Write;
use std::ops::Index;
use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use rayon::prelude::{IntoParallelIterator,ParallelIterator};
use rayon::iter::ParallelBridge;

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

    fn add(self, other: u8) -> Self {
        let result = (self as u8 + other) % (TILE_LAST_KIND as u8 + 1);
        Tile::from_u8(result).unwrap()
    }
}

impl std::ops::AddAssign<u8> for Tile {
    fn add_assign(&mut self, other: u8) {
        *self = self.clone() + other;
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
    hai: Vec<Tile>,
}

impl HaiSet {
    fn new() -> HaiSet {
        let hai: Vec<Tile> = Vec::with_capacity(HAINUM);

        HaiSet {
            hai,
        }
    }

    fn push(&mut self, value: Tile) {
        let size = self.hai.len();
        match size {
            HAINUM => panic!("Exceeded array length"),
            _ => self.hai.push(value),
        }
    }

    fn into_vec_u8(self) -> Vec<u8> {
        self.hai.into_iter()
                .map(|x| x as u8 + b'A')
                .collect()
    }

    fn iter<'a>(&'a self) -> HaiSetIterator<impl Iterator<Item = Tile> + 'a> {
        HaiSetIterator {
            haisetiter: self.hai.iter().cloned()
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


impl Index<usize> for HaiSet {
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
        const N_LAST: usize = HAINUM % NR_PER_TILE - 1;
        let mut set = HaiSet::new();
        for _ in 0..NR_PER_TILE {set.push(Tile::Red);}
        for _ in 0..NR_PER_TILE {set.push(Tile::Green);}
        for _ in 0..N_LAST {set.push(Tile::White);}
        set.push(Tile::Green);
        HaiSetGen {
            set
        }
    }

    fn is_last(&self) -> bool {
        lazy_static! {
            // FIXME: still some annoying const here
            static ref LAST_PATTERN: HaiSet = {
                let mut set = HaiSet::new();
                for _ in 0..HAINUM%NR_PER_TILE {set.push(Tile::Dot4);}
                for _ in 0..NR_PER_TILE {set.push(Tile::Dot5);}
                for _ in 0..NR_PER_TILE {set.push(TILE_LAST_KIND);}
                set
            };
        }
        (*LAST_PATTERN).hai == self.set.hai
    }
}

impl Iterator for HaiSetGen {
    type Item = HaiSet;

    fn next(&mut self) -> Option<HaiSet> {
        if self.is_last() {
            return None;
        }
        // iterate to next
        match self.set.hai.last()? {
            &TILE_LAST_KIND => {
                // FIXME: performance bottle neck
                let mut n = self.set.hai.iter()
                                        .rposition(|x| x != &TILE_LAST_KIND)?;
                if n <= 5 {
                    eprintln!("{:?}", self.set.hai);
                }
                while n < HAINUM
                    && self.set.hai[n] != TILE_LAST_KIND
                {
                    if n <= 5 {
                        eprintln!("n = {}, hai[n] = {:?}", n, self.set.hai[n]);
                    }
                    self.set.hai[n] += 1;
                    let t = self.set.hai[n].clone();
                    for c in self.set.hai
                                     .iter_mut()
                                     .skip(n) {
                                         *c = t.clone();
                                     }

                    n += NR_PER_TILE;
                }
            },
            _ => {self.set.hai[HAINUM-1] += 1;},
        }

        Some(self.set.clone())
    }
}

struct HaiTriplet<'a>{
    s: [&'a Tile; 3]
}

impl<'a> HaiTriplet<'a> {
    fn new(first: &'a Tile,
           second: &'a Tile,
           third: &'a Tile)
           -> HaiTriplet<'a>
    {
        HaiTriplet{
            s: [first, second, third,]
        }
    }

    fn is_chow(&self) -> bool {
        let zero_next = self.s[0].clone() + 1;
        let one_next = self.s[1].clone() + 1;
        if &zero_next == self.s[1]
            && &one_next == self.s[2]
        {
            // FIXME: find a better match pattern approach
            match self.s[0] {
                Tile::Bamboo1 | Tile::Bamboo2 | Tile::Bamboo3 | Tile::Bamboo4
                    | Tile::Character1 | Tile::Character2 | Tile::Character3 | Tile::Character4
                    | Tile::Dot1 | Tile::Dot2 | Tile::Dot3 | Tile::Dot4
                    => true,
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
    let mut mj_stat = MahjongStatistics::new();
    let record = hailoop(&mut mj_stat).into_iter()
                                      .map(|x| x.into_vec_u8())
                                      .flatten()
                                      .collect::<Vec<_>>();

    println!("total combination: {}", mj_stat.combination_count);
    println!("total agari combination: {}", mj_stat.valid_count);
    println!("total pattern: {}", mj_stat.pattern_count);
    println!("total agari pattern: {}", mj_stat.valid_pattern_count);

    let filename = "patterns_general_three.dat";
    let mut file = File::create(filename)?;
    file.write(&record)?;
    Ok(())
}

fn hailoop(mj_stat: &mut MahjongStatistics) -> Vec<HaiSet>
{
    let hsg = HaiSetGen::new();
    // main problem: how to record "total combination count"?
    // this should be done in the iteration,
    // since it's impossible to store 100G+ in the memory
    let records: Vec<HaiSet> = hsg.into_iter()
    // .par_bridge()
    // .into_par_iter()
                                  .filter(|hs| is_valid(hs))
                                  .map(|x| {
                                      eprintln!("done {:?}", x);
                                      x
                                  })
                                  .collect();

    for hs in records.iter() {
        mj_stat.valid_count += combination(hs).unwrap();
        mj_stat.valid_pattern_count += 1;
    }

    records
}

fn combination(hai: &HaiSet) -> Option<u64> {
    let mut ts = Vec::new();
    let mut ns = Vec::new();
    ts.push(&hai[0]);
    ns.push(1);

    for i in 1..HAINUM {
        if &&hai[i] == ts.last()? {
            let last = ns.last_mut()?;
            *last += 1;
        } else {
            ts.push(&hai[i]);
            ns.push(1);
        }
    }

    let mut product = 1;
    for i in 1..ts.len() {
        product *= binom(NR_PER_TILE as u64, ns[i]);
    }

    Some(product)
}

fn binom(n: u64, k: u64) -> u64 {
    let mut res = 1;

    for i in 0..k {
        res = res * (n-i) / (i+1);
    }

    res
}

fn is_valid(hai: &HaiSet) -> bool
{
    let sum: i32 = hai.iter().map(|x| x as i32).sum();

    assert_eq!(hai.len(), HAINUM);

    // first step: check at least one pair exist
    let mut has_pair: Vec<bool> = Vec::with_capacity(HAINUM-1);
    for i in 0..HAINUM-1 {
        has_pair.push(hai[i] == hai[i+1]);
    }
    if has_pair.iter().filter(|&&x| x).count() == 0 {
        return false;
    }

    // second step: check sum despite pair is a multiple of 3
    let mut possible_pairs: Vec<bool> = Vec::with_capacity(HAINUM-1);
    for i in 0..HAINUM-1 {
        if has_pair[i] {
            possible_pairs.push(((sum - 2 * (hai[i].clone() as i32)) % 3) == 0);
        } else {
            possible_pairs.push(false);
        }
    }
    if possible_pairs.iter().filter(|&&x| x).count() == 0 {
        return false;
    }

    // final step: check for triples
    let mut valid_pairs: Vec<bool> = Vec::with_capacity(HAINUM-1);
    for i in 0..HAINUM-1 {
        if possible_pairs[i] {
            let cphai: Vec<Tile> = hai.iter()
                                      .enumerate()
                                      .filter(|&(j, _)| (j != i) && (j != i+1))
                                      .map(|(_, e)| e)
                                      .collect();
            valid_pairs.push(check_hai(&cphai));
        } else {
            valid_pairs.push(false);
        }
    }

    match valid_pairs.iter().filter(|&&x| x).count() {
        0 => false,
        _ => true,
    }
}

fn check_hai(hai: &Vec<Tile>) -> bool
{
    assert_eq!(hai.len() % 3, 0, "not correct sub hai length: {}, should be multiple of 3", hai.len());
    let hai_len = hai.len();

    if hai_len == 0 {
        return true;
    }

    let mut used_index: Vec<bool> = Vec::with_capacity(hai_len);
    for _ in 0..hai_len {
        used_index.push(false);
    }
    let mut i = 0;
    while i < hai_len-2 {
        if used_index[i] {
            i += 1;
            continue;
        }

        let triplet = HaiTriplet::new(&hai[i], &hai[i+1], &hai[i+2]);
        if triplet.is_chow() || triplet.is_pung() && !used_index[i+1] && !used_index[i+2] {
            // FIXME: find a way to assign in one line
            used_index[i] = true;
            used_index[i+1] = true;
            used_index[i+2] = true;
            i += 3;
            continue;
        }

        let next_index = match used_index.iter()
                                         .skip(i)
                                         .enumerate()
                                         .find(|&(_, c)| !c)
                                         .map(|(i, _)| i) {
                                             Some(n) => n,
                                             None => return false,
                                         };
        let last_index = match used_index.iter()
                                         .skip(next_index)
                                         .enumerate()
                                         .find(|&(_, c)| !c)
                                         .map(|(i, _)| i) {
                                             Some(n) => n,
                                             None => return false,
                                         };

        let sparse_triplet = HaiTriplet::new(&hai[i], &hai[next_index], &hai[last_index]);
        if sparse_triplet.is_chow() {
            // FIXME: find a way to assign in one line
            used_index[i] = true;
            used_index[i+1] = true;
            used_index[i+2] = true;
            continue;
        } else {
            return false;
        }
    }

    true
}
