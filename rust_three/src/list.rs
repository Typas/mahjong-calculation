use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::sync::{Mutex};

const HAINUM: usize = 11;

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

impl Tile {
    fn next(&mut self) {
        let next = match self {
            Tile::Dot6 => 0,
            _ => self.clone() as u8 + 1,
        };
        *self = Tile::from_u8(next).unwrap();
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

struct HaiSetGen {
    set: Mutex<HaiSet>,
}

impl HaiSetGen {
    fn new() -> HaiSetGen {
        let mut set = HaiSet::new();
        for _ in 0..4 {set.push(Tile::Red);}
        for _ in 0..4 {set.push(Tile::Green);}
        for _ in 0..2 {set.push(Tile::White);}
        set.push(Tile::Green);
        HaiSetGen {
            set: Mutex::new(set)
        }
    }

    fn get(&self) -> Option<HaiSet> {
        if self.is_last() {
            return None;
        }
        // iterate to next
        match self.set.lock().unwrap().hai.last()? {
            Tile::Dot6 => {
                let mut n = self.set.lock().unwrap().hai.iter()
                    .rposition(|x| x != &Tile::Dot6)?;
                while n < HAINUM
                    && self.set.lock().unwrap().hai[n] != Tile::Dot6
                {
                    self.set.lock().unwrap().hai[n].next();
                    let t = self.set.lock().unwrap().hai[n].clone();
                    for c in self.set.lock().unwrap().hai
                                     .iter_mut()
                                     .skip(n) {
                                         *c = t.clone();
                                     }
                    n += 4;
                }
            },
            _ => self.set.lock().unwrap().hai[HAINUM-1].next(),
        }

        Some(self.set.lock().unwrap().clone())
    }

    fn is_last(&self) -> bool {
        lazy_static! {
            static ref LAST_PATTERN: HaiSet = {
                let mut set = HaiSet::new();
                for _ in 0..3 {set.push(Tile::Dot4);}
                for _ in 0..4 {set.push(Tile::Dot5);}
                for _ in 0..4 {set.push(Tile::Dot6);}
                set
            };
        }
        (*LAST_PATTERN).hai == self.set.lock().unwrap().hai
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
    let hsg = HaiSetGen::new();
    const NR_WORKER: u8 = 100;
    for _ in 0..=NR_WORKER {
        while let Some(hs) = hsg.get() {
            let tile_counts: Vec<u8> = vec![1];

        }
    }

    records
}

fn is_valid(hai: &HaiSet) -> bool
{
    false
}

fn check_hai(hai: &HaiSet, n: i32) -> bool
{
    false
}
