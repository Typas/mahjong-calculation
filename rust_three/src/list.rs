use std::fs::File;
use std::io::Write;
use std::ops::Index;
use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use rayon::iter::ParallelBridge;

const HAINUM: usize = 11;
const NR_PER_TILE: usize = 4;
const TILE_LAST_KIND: Tile = Tile::Dot6;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
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

impl Iterator for Tile {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        let next = match self {
            &mut TILE_LAST_KIND => 0,
            _ => self.clone() as u8 + 1,
        };
        *self = Tile::from_u8(next).unwrap();
        Some(self.clone())
    }
}

struct MahjongStatistics {
    combination_count: u64,
    valid_count: u64,
    pattern_count: u64,
    valid_pattern_count: u64,
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

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct HaiSet {
    hai: Vec<Tile>,
}

impl HaiSet {
    fn new() -> HaiSet {
        let mut hai = Vec::new();
        hai.reserve_exact(HAINUM);

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
            // still some annoying const here
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
                let mut n = self.set.hai.iter()
                    .rposition(|x| x != &TILE_LAST_KIND)?;
                while n < HAINUM
                    && self.set.hai[n] != TILE_LAST_KIND
                {
                    self.set.hai[n].next();
                    let t = self.set.hai[n].clone();
                    for c in self.set.hai
                                     .iter_mut()
                                     .skip(n) {
                                         *c = t.clone();
                                     }

     n += 4;
                }
            },
            _ => {self.set.hai[HAINUM-1].next();},
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

    fn chow(&self) -> bool {
        let mut zero_next = self.s[0].clone();
        zero_next.next();
        let mut one_next = self.s[1].clone();
        one_next.next();
        if &zero_next == self.s[1]
            && &one_next == self.s[2]
        {
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

    fn pung(&self) -> bool {
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
    let mut records: Vec<HaiSet> = Vec::new();
    let mut hsg = HaiSetGen::new();
    const NR_WORKER: u8 = 100;
    let hss = hsg.into_iter().par_bridge();

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
    false
}

fn check_hai(hai: &HaiSet, n: i32) -> bool
{
    false
}
