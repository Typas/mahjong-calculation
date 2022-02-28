use std::fs::File;
use std::io::Write;

use arrayvec::ArrayVec;
use itertools::Itertools;

use ex_set::{Meld, MeldKind, Pair, Set, SetBuilder, HAINUM, SETNUM};
use tile::{Tile, TILEVARIANT};
mod ex_set;
mod tile;

const SETKINDVARIANT: usize = 2_usize.pow(SETNUM as u32);
const TILESELECTNUM: usize = TILEVARIANT * 4 - 5 * 3;

fn main() {
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
        .into_iter()
        .map(|p| {
            let p_pos = meld_heads.iter().position(|&h| h == p).unwrap();
            let heads: Vec<ArrayVec<Tile, SETNUM>> = meld_heads
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(i, h)| remove_impossible_head(i, h, p_pos, p))
                .tuple_combinations::<(_, _, _)>()
                .unique()
                .map(|(m1, m2, m3)| ArrayVec::from([m1, m2, m3]))
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

    general_sets.sort();
    general_sets.dedup();

    let tmp: Vec<u8> = general_sets.into_iter().flatten().collect();
    let filename = "patterns_rust_three_extended.dat";
    let mut file = File::create(filename).expect("not able to open file");
    file.write_all(&tmp)
        .expect("cannot write all into the file");
}

fn get_pairs() -> ArrayVec<Tile, TILEVARIANT> {
    ArrayVec::from([
        Tile::Sun,
        Tile::Moon,
        Tile::Red,
        Tile::Green,
        Tile::White,
        Tile::B1,
        Tile::B2,
        Tile::B3,
        Tile::B4,
        Tile::B5,
        Tile::B6,
        Tile::C1,
        Tile::C2,
        Tile::C3,
        Tile::C4,
        Tile::C5,
        Tile::C6,
        Tile::D1,
        Tile::D2,
        Tile::D3,
        Tile::D4,
        Tile::D5,
        Tile::D6,
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
        .build()
        .expect("cannot build set")
        .to_arrayvec()
}

fn is_valid_hai(hai: &ArrayVec<Tile, HAINUM>) -> bool {
    let mut counters = [0; TILEVARIANT];

    hai.iter().for_each(|h| counters[*h as usize] += 1);

    !counters.into_iter().any(|c| c > 4)
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
                Tile::Sun
                | Tile::Moon
                | Tile::Red
                | Tile::Green
                | Tile::White
                | Tile::B5
                | Tile::B6
                | Tile::C5
                | Tile::C6
                | Tile::D5
                | Tile::D6 => Err("Not valid chow")?,
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

        tmp.push(*self.pair as u8);
        tmp.push(*self.pair as u8);

        for m in self.melds.clone().into_iter() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ok() {
        assert_eq!(2 + 2, 4);
    }
}
