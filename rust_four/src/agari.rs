use arrayvec::ArrayVec;
use itertools::Itertools;
use set::{HandList, Set, SetBuilder, HAINUM};
use std::{collections::HashMap, fs::File, io::Read};

use crate::{
    hand::Hand,
    set::{Meld, MeldKind},
    tile::{Tile, TILEVARIANT},
};
mod hand;
mod set;
mod tile;

fn main() -> std::io::Result<()> {
    let mut hands: HashMap<HandList, u64> = HashMap::new();
    let mut reader = File::open("patterns_general_four.dat")?;
    let mut raw_hai = [0u8; HAINUM];
    while let Ok(_) = reader.read_exact(&mut raw_hai) {
        let sets = allsets(&raw_hai);
        let combinations = comb(&raw_hai);
        for s in sets.into_iter() {
            let list = s.hands();
            if hands.contains_key(&list) {
                if let Some(v) = hands.get_mut(&list) {
                    *v += combinations;
                }
            } else {
                hands.insert(list, combinations);
            }
        }
    }

    // 役種、組合數、總分
    let mut result: Vec<(Hand, u64, u64)> = vec![(Hand::try_from(0).unwrap(), 0, 0)];
    result
        .iter_mut()
        .enumerate()
        .for_each(|(i, (h, _, _))| *h = Hand::try_from(i).unwrap());

    for (handlist, combination) in hands.into_iter() {
        let score = handlist.score() as u64;
        handlist
            .into_iter()
            .enumerate()
            .filter(|(_, h)| *h)
            .for_each(|(i, _)| {
                result[i].1 += combination;
                result[i].2 += combination * score;
            });
    }

    // 役牌特殊處理
    result[4].1 += result[1].1;
    result[4].2 += result[1].2;
    result[4].1 += result[2].1;
    result[4].2 += result[2].2;
    result[4].1 += result[3].1;
    result[4].2 += result[3].2;

    println!("役種\t組合數\t平均分數");
    for (hand, combination, score) in result.into_iter() {
        println!(
            "{}\t{}\t{}",
            hand.name(),
            combination,
            score as f64 / combination as f64
        );
    }

    Ok(())
}

fn allsets(raw: &[u8; HAINUM]) -> Vec<Set> {
    let mut sorted_raw = raw.clone();
    sorted_raw.sort();
    let tiles: ArrayVec<Tile, HAINUM> = sorted_raw
        .into_iter()
        .map(|x| Tile::try_from(x as char).unwrap())
        .collect();

    let tmp4 = get_pairs(&tiles);
    let mut tmp3 = Vec::new();
    for (sb, remains) in tmp4.into_iter() {
        let mut tmp = get_first_melds(&remains, sb);
        tmp3.append(&mut tmp);
    }
    let mut tmp2 = Vec::new();
    for (sb, remains) in tmp3.into_iter() {
        let mut tmp = get_second_melds(&remains, sb);
        tmp2.append(&mut tmp);
    }
    let mut tmp1 = Vec::new();
    for (sb, remains) in tmp2.into_iter() {
        let mut tmp = get_third_melds(&remains, sb);
        tmp1.append(&mut tmp);
    }
    let mut result = Vec::new();
    for (sb, remains) in tmp1.into_iter() {
        let tmp = get_last_melds(&remains, sb);
        if let Some(tmp) = tmp {
            result.push(tmp);
        }
    }

    result.into_iter().unique().collect()
}

fn comb(raw: &[u8; HAINUM]) -> u64 {
    let mut counts = [0u64; TILEVARIANT];

    raw.iter()
        .map(|r| Tile::try_from(*r as char).unwrap())
        .for_each(|t| counts[t as usize] += 1);

    counts
        .into_iter()
        .map(|c| match c {
            0 | 4 => 1,
            1 | 3 => 4,
            2 => 6,
            _ => unreachable!(),
        })
        .product()
}

fn is_chow(meld: &[Tile]) -> bool {
    match meld.len() {
        3 => meld[0].is_ascending(meld[1]) && meld[1].is_ascending(meld[2]),
        _ => false,
    }
}

fn is_pung(meld: &[Tile]) -> bool {
    match meld.len() {
        3 => meld.windows(2).all(|w| w[0] == w[1]),
        _ => false,
    }
}

fn remove_subset<T>(main: &Vec<T>, subset: &Vec<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut sub = subset.iter();
    let mut current = sub.next();
    let mut result = Vec::new();
    for m in main {
        match current {
            Some(c) if m == c => current = sub.next(),
            _ => result.push(m.clone()),
        }
    }

    result
}

const FOURSET: usize = 4 * 3;
const THREESET: usize = 3 * 3;
const TWOSET: usize = 2 * 3;
const ONESET: usize = 1 * 3;

fn get_pairs(set: &[Tile]) -> Vec<(SetBuilder, ArrayVec<Tile, FOURSET>)> {
    assert_eq!(set.len(), HAINUM);
    let pair_loc = set
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .map(|(i, _)| i)
        .collect();
    let dup_pair_loc = set
        .windows(3)
        .enumerate()
        .filter(|(_, w)| w[0] == w[1] && w[1] == w[2])
        .map(|(i, _)| i)
        .collect();
    let pair_indexes: Vec<usize> = remove_subset(&pair_loc, &dup_pair_loc);

    let mut result = Vec::new();

    for pi in pair_indexes.into_iter() {
        let mut sb = SetBuilder::new(Tile::East);
        sb.add_pair(set[pi]);
        let remains: ArrayVec<Tile, FOURSET> = set
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != pi && *i != pi + 1)
            .map(|(_, t)| *t)
            .collect();

        result.push((sb, remains));
    }

    result
}

fn get_first_melds(set: &[Tile], sb: SetBuilder) -> Vec<(SetBuilder, ArrayVec<Tile, THREESET>)> {
    assert_eq!(set.len(), FOURSET);
    let mut result = Vec::new();
    for m in set.to_owned().into_iter().combinations(ONESET).unique() {
        let chow = is_chow(&m);
        let pung = is_pung(&m);
        if chow || pung {
            let new_meld = match (chow, pung) {
                (true, false) => Meld::new(m[0], MeldKind::ConcealedChow),
                (false, true) => Meld::new(m[0], MeldKind::ConcealedPung),
                _ => unreachable!(),
            };
            let mut new_sb = sb.clone();
            new_sb.add_meld(new_meld).unwrap();
            let mut meld_pos: ArrayVec<usize, ONESET> = m
                .iter()
                .map(|m| set.iter().position(|t| t == m).unwrap())
                .collect();
            if meld_pos[0] == meld_pos[1] {
                meld_pos[1] = meld_pos[0] + 1;
                meld_pos[2] = meld_pos[0] + 2;
            }
            assert_eq!(meld_pos.len(), ONESET);
            let remains: ArrayVec<Tile, THREESET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !meld_pos.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), THREESET);

            result.push((new_sb, remains));
        }
    }

    result
}

fn get_second_melds(set: &[Tile], sb: SetBuilder) -> Vec<(SetBuilder, ArrayVec<Tile, TWOSET>)> {
    assert_eq!(set.len(), THREESET);
    let mut result = Vec::new();
    for m in set.to_owned().into_iter().combinations(ONESET).unique() {
        let chow = is_chow(&m);
        let pung = is_pung(&m);
        if chow || pung {
            let new_meld = match (chow, pung) {
                (true, false) => Meld::new(m[0], MeldKind::ConcealedChow),
                (false, true) => Meld::new(m[0], MeldKind::ConcealedPung),
                _ => unreachable!(),
            };
            let mut new_sb = sb.clone();
            new_sb.add_meld(new_meld).unwrap();
            let mut meld_pos: ArrayVec<usize, ONESET> = m
                .iter()
                .map(|m| set.iter().position(|t| t == m).unwrap())
                .collect();
            if meld_pos[0] == meld_pos[1] {
                meld_pos[1] = meld_pos[0] + 1;
                meld_pos[2] = meld_pos[0] + 2;
            }
            assert_eq!(meld_pos.len(), ONESET);
            let remains: ArrayVec<Tile, TWOSET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !meld_pos.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), TWOSET);

            result.push((new_sb, remains));
        }
    }

    result
}

fn get_third_melds(set: &[Tile], sb: SetBuilder) -> Vec<(SetBuilder, ArrayVec<Tile, ONESET>)> {
    assert_eq!(set.len(), TWOSET);
    let mut result = Vec::new();
    for m in set.to_owned().into_iter().combinations(ONESET).unique() {
        let chow = is_chow(&m);
        let pung = is_pung(&m);
        if chow || pung {
            let new_meld = match (chow, pung) {
                (true, false) => Meld::new(m[0], MeldKind::ConcealedChow),
                (false, true) => Meld::new(m[0], MeldKind::ConcealedPung),
                _ => unreachable!(),
            };
            let mut new_sb = sb.clone();
            new_sb.add_meld(new_meld).unwrap();
            let mut meld_pos: ArrayVec<usize, ONESET> = m
                .iter()
                .map(|m| set.iter().position(|t| t == m).unwrap())
                .collect();
            if meld_pos[0] == meld_pos[1] {
                meld_pos[1] = meld_pos[0] + 1;
                meld_pos[2] = meld_pos[0] + 2;
            }
            assert_eq!(meld_pos.len(), ONESET);
            let remains: ArrayVec<Tile, ONESET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !meld_pos.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), ONESET);

            result.push((new_sb, remains));
        }
    }

    result
}

fn get_last_melds(set: &[Tile], mut sb: SetBuilder) -> Option<Set> {
    assert_eq!(set.len(), ONESET);
    let chow = is_chow(set);
    let pung = is_pung(set);
    match chow || pung {
        true => {
            let new_meld = match (chow, pung) {
                (true, false) => Meld::new(set[0], MeldKind::ConcealedChow),
                (false, true) => Meld::new(set[0], MeldKind::ConcealedPung),
                _ => unreachable!(),
            };
            sb.add_meld(new_meld).unwrap();
            sb.build().ok()
        }
        false => None,
    }
}
