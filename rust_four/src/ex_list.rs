use std::io::Write;
use std::{collections::HashSet, fs::File};

use arrayvec::ArrayVec;
use itertools::Itertools;

use ex_set::{Meld, MeldKind, Pair, Set, SetBuilder};
use tile::{Tile, TILEVARIANT};

mod ex_set;
mod tile;

const HAINUM: usize = 14;
const SETNUM: usize = HAINUM / 3;
const SETKINDVARIANT: usize = 2_usize.pow(SETNUM as u32);
const TILESELECTNUM: usize = TILEVARIANT * 4 - 7 * 3;

fn main() {
    let filename = "patterns_rust_four_extend.dat";
    let pairs = get_pairs();

    let kinds: ArrayVec<ArrayVec<MeldKind, SETNUM>, SETKINDVARIANT> =
        std::iter::repeat(MeldKind::ConcealedChow)
            .take(SETNUM)
            .chain(std::iter::repeat(MeldKind::ConcealedPung).take(SETNUM))
            .permutations(SETNUM)
            .unique()
            .map(|v| (&v as &[_]).try_into().unwrap())
            .collect();

    let meld_heads: ArrayVec<_, TILESELECTNUM> = pairs
        .clone()
        .into_iter()
        .take_while(|x| x.is_honor())
        .chain(
            pairs
                .clone()
                .into_iter()
                .skip_while(|x| x.is_honor())
                .map(|x| std::iter::repeat(x).take(4))
                .flatten(),
        )
        .collect();

    // general sets
    let possible_general_sets: Vec<ArrayVec<Tile, HAINUM>> = pairs
        .clone()
        .into_iter()
        .map(|p| {
            let p_pos = meld_heads.iter().position(|&h| h == p).unwrap();
            let heads: Vec<ArrayVec<Tile, SETNUM>> = meld_heads
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(i, h)| remove_impossible_head(i, h, p_pos, p))
                .tuple_combinations::<(_, _, _, _)>()
                .unique()
                .map(|(m1, m2, m3, m4)| ArrayVec::from([m1, m2, m3, m4]))
                .collect();

            let sb = SetBuilder::new().add_pair(Pair::new(p, true));
            kinds
                .clone()
                .into_iter()
                .map(|ks| {
                    heads
                        .clone()
                        .into_iter()
                        .filter_map(|hs| generate_set(sb.clone(), &hs, &ks))
                        .filter(|hai| is_valid_hai(hai))
                        .collect::<Vec<ArrayVec<Tile, HAINUM>>>()
                })
                .flatten()
                .collect::<Vec<ArrayVec<Tile, HAINUM>>>()
        })
        .flatten()
        .collect();

    let mut general_sets: Vec<ArrayVec<u8, HAINUM>> = possible_general_sets
        .into_iter()
        .map(|s| s.into_iter().map(|t| t.to_char() as u8).collect())
        .collect();

    general_sets.sort_unstable();
    general_sets.dedup();
    println!("general sets: {}", general_sets.len());
    drop(kinds);
    drop(meld_heads);

    let mut file = File::create(filename).expect("not able to open file");
    {
        let tmp: Vec<u8> = general_sets.into_iter().flatten().collect();
        assert_eq!(tmp.len() % HAINUM, 0);
        file.write_all(&tmp)
            .expect("cannot write all into the file");
    }

    // create seven pairs without same chows
    const SEVENPAIRNUM: usize = 2 * TILEVARIANT;
    let seven_pair_heads: ArrayVec<_, SEVENPAIRNUM> = pairs
        .clone()
        .into_iter()
        .map(|x| std::iter::repeat(x).take(2))
        .flatten()
        .collect();
    assert_eq!(seven_pair_heads.len(), SEVENPAIRNUM);

    let seven_pairs_no_general_sets: HashSet<ArrayVec<Tile, HAINUM>> = seven_pair_heads
        .into_iter()
        .combinations(7)
        .map(|s| s.into_iter().collect::<ArrayVec<Tile, HAINUM>>())
        .collect();

    let seven_pairs_no_general_sets: Vec<ArrayVec<u8, HAINUM>> = seven_pairs_no_general_sets
        .into_iter()
        .sorted_unstable()
        .filter(|s| is_seven_pair_not_general_set(&s))
        .map(|s| {
            s.into_iter()
                .map(|t| t.to_char() as u8)
                .map(|x| std::iter::repeat(x).take(2))
                .flatten()
                .collect::<ArrayVec<u8, HAINUM>>()
        })
        .collect();

    assert_eq!(seven_pairs_no_general_sets[0].len(), HAINUM);
    println!(
        "seven pair sets (non-general): {}",
        seven_pairs_no_general_sets.len()
    );

    {
        let tmp: Vec<u8> = seven_pairs_no_general_sets.into_iter().flatten().collect();
        assert_eq!(tmp.len() % HAINUM, 0);
        file.write_all(&tmp)
            .expect("cannot write all into the file");
    }

    // create thirteen orphans
    let thirteen_orphans: ArrayVec<_, 13> = pairs
        .into_iter()
        .filter(|x| x.is_terminal() || x.is_honor())
        .collect();
    assert_eq!(thirteen_orphans.len(), 13);
    let thirteen_orphans_sets: Vec<ArrayVec<u8, HAINUM>> = thirteen_orphans
        .clone()
        .into_iter()
        .map(|x| {
            std::iter::once(x)
                .chain(thirteen_orphans.clone().into_iter())
                .sorted()
                .collect::<ArrayVec<Tile, HAINUM>>()
        })
        .map(|s| s.into_iter().map(|t| t.to_char() as u8).collect())
        .collect();
    assert_eq!(thirteen_orphans_sets[0].len(), HAINUM);
    println!("thirteen orphan sets: {}", thirteen_orphans_sets.len());

    {
        let tmp: Vec<u8> = thirteen_orphans_sets.into_iter().flatten().collect();
        assert_eq!(tmp.len() % HAINUM, 0);
        file.write_all(&tmp)
            .expect("cannot write all into the file");
    }
}

fn get_pairs() -> ArrayVec<Tile, TILEVARIANT> {
    ArrayVec::from([
        Tile::Red,
        Tile::Green,
        Tile::White,
        Tile::East,
        Tile::South,
        Tile::West,
        Tile::North,
        Tile::B1,
        Tile::B2,
        Tile::B3,
        Tile::B4,
        Tile::B5,
        Tile::B6,
        Tile::B7,
        Tile::B8,
        Tile::B9,
        Tile::C1,
        Tile::C2,
        Tile::C3,
        Tile::C4,
        Tile::C5,
        Tile::C6,
        Tile::C7,
        Tile::C8,
        Tile::C9,
        Tile::D1,
        Tile::D2,
        Tile::D3,
        Tile::D4,
        Tile::D5,
        Tile::D6,
        Tile::D7,
        Tile::D8,
        Tile::D9,
    ])
}

fn remove_impossible_head(i: usize, h: Tile, p_pos: usize, p: Tile) -> Option<Tile> {
    match p.is_honor() {
        true => {
            if i == p_pos {
                None
            } else {
                Some(h)
            }
        }
        false => {
            if i == p_pos || i == p_pos + 1 {
                None
            } else {
                Some(h)
            }
        }
    }
}

fn generate_set(
    sb: SetBuilder,
    hs: &ArrayVec<Tile, SETNUM>,
    ks: &ArrayVec<MeldKind, SETNUM>,
) -> Option<ArrayVec<Tile, HAINUM>> {
    sb.add_meld(Meld::new(hs[0], ks[0]))
        .expect("failed to add 1st meld")
        .add_meld(Meld::new(hs[1], ks[1]))
        .expect("failed to add 2nd meld")
        .add_meld(Meld::new(hs[2], ks[2]))
        .expect("failed to add 3rd meld")
        .add_meld(Meld::new(hs[3], ks[3]))
        .expect("failed to add 4th meld")
        .build()
        .expect("cannot build set")
        .to_arrayvec()
}

fn is_valid_hai(hai: &ArrayVec<Tile, HAINUM>) -> bool {
    let mut counters = [0; TILEVARIANT];

    hai.iter().for_each(|h| counters[*h as usize] += 1);

    let testcase = [71, 71, 71, 72, 72, 72, 84, 84, 84, 85, 86, 97, 97, 97];
    if let true = hai.iter().zip(testcase.iter()).all(|(h, t)| *h as u8 == *t) {
        eprintln!(
            "testcase exists, validity: {}",
            !counters.iter().any(|c| *c > 4)
        );
    }

    !counters.into_iter().any(|c| c > 4)
}

fn is_seven_pair_not_general_set(s: &[Tile]) -> bool {
    const CHOW_INDEXES: [([usize; 3], [usize; 3]); 14] = [
        ([0, 1, 2], [3, 4, 5]),
        ([0, 1, 2], [3, 4, 6]),
        ([0, 1, 2], [3, 5, 6]),
        ([0, 1, 2], [4, 5, 6]),
        ([0, 1, 3], [4, 5, 6]),
        ([0, 2, 3], [4, 5, 6]),
        ([1, 2, 3], [4, 5, 6]),
        ([0, 2, 4], [1, 3, 5]),
        ([0, 2, 4], [1, 3, 6]),
        ([0, 2, 5], [1, 3, 6]),
        ([0, 2, 5], [1, 4, 6]),
        ([0, 3, 5], [1, 4, 6]),
        ([0, 3, 5], [2, 4, 6]),
        ([1, 3, 5], [2, 4, 6]),
    ];
    match s.len() {
        7 => {
            CHOW_INDEXES
                .into_iter()
                .map(|ixs| {
                    let (chow1, chow2) = (
                        [s[ixs.0[0]], s[ixs.0[1]], s[ixs.0[2]]],
                        [s[ixs.1[0]], s[ixs.1[1]], s[ixs.1[2]]],
                    );
                    (is_chow(&chow1), is_chow(&chow2))
                })
                .filter(|b| match b {
                    (true, true) => true,
                    _ => false,
                })
                .count()
                == 0
        }
        _ => false,
    }
}

fn is_chow(tiles: &[Tile]) -> bool {
    match tiles.len() {
        3 => tiles[0].is_ascending(tiles[1]) && tiles[1].is_ascending(tiles[2]),
        _ => false,
    }
}

impl Tile {
    fn to_char(&self) -> char {
        (*self as u8 + b'A') as char
    }
}

impl Meld {
    fn tryinto_arrayvec(self) -> Result<ArrayVec<u8, 3>, Box<dyn std::error::Error>> {
        match self.kind() {
            MeldKind::ConcealedChow => match self.head() {
                Tile::Red
                | Tile::Green
                | Tile::White
                | Tile::East
                | Tile::South
                | Tile::West
                | Tile::North
                | Tile::B8
                | Tile::B9
                | Tile::C8
                | Tile::C9
                | Tile::D8
                | Tile::D9 => Err("Not valid chow")?,
                _ => Ok(ArrayVec::from([
                    self.head() as u8,
                    self.head() as u8 + 1,
                    self.head() as u8 + 2,
                ])),
            },
            MeldKind::ConcealedPung => Ok(ArrayVec::from([
                self.head() as u8,
                self.head() as u8,
                self.head() as u8,
            ])),
            _ => unreachable!(),
        }
    }
}

impl Set {
    fn to_arrayvec(&self) -> Option<ArrayVec<Tile, HAINUM>> {
        let mut tmp = ArrayVec::<u8, HAINUM>::new_const();

        tmp.push(*self.pair() as u8);
        tmp.push(*self.pair() as u8);

        for m in self.melds().clone().into_iter() {
            match m.tryinto_arrayvec() {
                Ok(a) => {
                    tmp.push(a[0]);
                    tmp.push(a[1]);
                    tmp.push(a[2]);
                }
                Err(_) => None?,
            }
        }

        assert!(tmp.is_full());

        tmp.sort();

        Some(
            tmp.into_iter()
                .map(|c| Tile::try_from((c + b'A') as char).unwrap())
                .collect(),
        )
    }
}
