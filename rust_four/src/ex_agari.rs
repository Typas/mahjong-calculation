use arrayvec::ArrayVec;
use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::Read, time::Instant};

use crate::{
    ex_hand::{Hand, HANDVARIANT},
    ex_handchecker::{HandChecker, HandList},
    ex_set::{Meld, MeldKind, Pair, SetBuilder, HAINUM},
    tile::{Tile, TILEVARIANT},
};

mod ex_hand;
mod ex_handchecker;
mod ex_set;
mod tile;

fn main() -> std::io::Result<()> {
    let mut reader = File::open("patterns_rust_four_extend.dat")?;
    let fsize = reader.metadata()?.len() as usize;
    let mut buffer = Vec::with_capacity(fsize);
    let time_read = Instant::now();
    let size_read = reader.read_to_end(&mut buffer)?;
    assert_eq!(size_read, fsize);
    let raw_hai_sets: Vec<ArrayVec<u8, HAINUM>> = buffer
        .into_iter()
        .chunks(HAINUM)
        .into_iter()
        .map(|c| c.collect())
        .collect();
    assert_eq!(raw_hai_sets.len(), fsize / HAINUM);
    println!(
        "time read and chunks in {:.2} s",
        time_read.elapsed().as_secs_f32()
    );

    println!("produce hand patterns");
    let start = Instant::now();
    let mut hands: HashMap<HandList, (u64, u64)> = HashMap::new();
    raw_hai_sets
        .into_iter()
        .enumerate()
        .for_each(|(_, raw_hai)| {
            let mut sets = allsets(&raw_hai);
            sets.dedup();
            let combinations = comb(&raw_hai);

            sets.into_iter().for_each(|s| {
                let list = s.hands();
                let h = &mut hands;
                if h.contains_key(&list) {
                    if let Some(v) = h.get_mut(&list) {
                        v.0 += 1;
                        v.1 += combinations;
                    }
                } else {
                    h.insert(list, (1, combinations));
                }
            });
        });

    println!(
        "time produce patterns in {:.2} s",
        start.elapsed().as_secs_f32()
    );

    println!("calculate combinations and total scores");
    // 役種、和牌形、組合數、總分
    let mut result: Vec<(Hand, u64, u64, u64)> =
        vec![(Hand::try_from(0).unwrap(), 0, 0, 0); HANDVARIANT];
    result
        .iter_mut()
        .enumerate()
        .for_each(|(i, (h, _, _, _))| *h = Hand::try_from(i).unwrap());

    hands
        .into_iter()
        .for_each(|(handlist, (pattern, combination))| {
            let score = handlist.score() as u64;
            handlist
                .into_iter()
                .enumerate()
                .filter(|(_, h)| *h)
                .for_each(|(i, _)| {
                    result[i].1 += pattern;
                    result[i].2 += combination;
                    result[i].3 += combination * score;
                });
        });

    // 役牌特殊處理
    result[4].1 += result[1].1;
    result[4].2 += result[1].2;
    result[4].3 += result[1].3;
    result[4].1 += result[2].1;
    result[4].2 += result[2].2;
    result[4].3 += result[2].3;
    result[4].1 += result[3].1;
    result[4].2 += result[3].2;
    result[4].3 += result[3].3;

    println!("end of process");
    println!("");

    println!("{:4}{:8}{:13}{:4}", "役種", "和牌形", "組合數", "平均分數");
    result
        .into_iter()
        .enumerate()
        .filter_map(|(i, r)| match i {
            1 | 2 | 3 => None,
            _ => Some(r),
        })
        .for_each(|(hand, pattern, combination, score)| {
            println!(
                "{:<4}{:>8}{:>16} {:>.*}",
                hand.name(),
                pattern,
                combination,
                5,
                (score as f64 / combination as f64).to_string()
            );
        });

    Ok(())
}

// main performance problem
fn allsets(raw: &[u8]) -> Vec<HandChecker> {
    assert_eq!(raw.len(), HAINUM);
    let mut sorted_raw = raw.to_vec();
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
        if let Some(mut tmp) = tmp {
            tmp.sort();
            result.push(tmp);
        }
    }

    result
}

fn comb(raw: &[u8]) -> u64 {
    assert_eq!(raw.len(), HAINUM);
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
        let sb = SetBuilder::new().add_pair(Pair::new(set[pi], true));
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

    // assume set is sorted
    // pung is always continuous (0, 1, 2)
    // chow is possible in various way, get only (0, x, y) where x + y min
    // FIXME: prove only "first" chow is needed
    let pung_set = [set[0], set[1], set[2]];
    let chow_indexes = [
        [0, 1, 2],
        [0, 1, 3],
        [0, 1, 4],
        [0, 1, 5],
        [0, 2, 4],
        [0, 2, 5],
        [0, 2, 6],
        [0, 3, 6],
        [0, 3, 7],
        [0, 4, 8],
    ];

    if is_pung(&pung_set) {
        let new_meld = Meld::new(pung_set[0], MeldKind::ConcealedPung);
        let new_sb = sb.clone().add_meld(new_meld).unwrap();
        let meld_pos = [0, 1, 2];
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

    for idx in chow_indexes.into_iter() {
        let chow_set = [set[idx[0]], set[idx[1]], set[idx[2]]];
        if is_chow(&chow_set) {
            let new_meld = Meld::new(chow_set[0], MeldKind::ConcealedChow);
            let new_sb = sb.clone().add_meld(new_meld).unwrap();
            let remains: ArrayVec<Tile, THREESET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !idx.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), THREESET);

            result.push((new_sb, remains));
            break;
        }
    }

    result
}

fn get_second_melds(set: &[Tile], sb: SetBuilder) -> Vec<(SetBuilder, ArrayVec<Tile, TWOSET>)> {
    assert_eq!(set.len(), THREESET);
    let mut result = Vec::new();
    // TODO: rewrite without combinations and unique
    // assume set is sorted
    // pung is always continuous (0, 1, 2)
    // chow is possible in various way, get only (0, x, y) where x + y min
    // FIXME: prove only "first" chow is needed
    let pung_set = [set[0], set[1], set[2]];
    let chow_indexes = [
        [0, 1, 2],
        [0, 1, 3],
        [0, 1, 4],
        [0, 1, 5],
        [0, 2, 4],
        [0, 2, 5],
        [0, 2, 6],
        [0, 3, 6],
        [0, 3, 7],
        [0, 4, 8],
    ];

    if is_pung(&pung_set) {
        let new_meld = Meld::new(pung_set[0], MeldKind::ConcealedPung);
        let new_sb = sb.clone().add_meld(new_meld).unwrap();
        let meld_pos = [0, 1, 2];
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

    for idx in chow_indexes.into_iter() {
        let chow_set = [set[idx[0]], set[idx[1]], set[idx[2]]];
        if is_chow(&chow_set) {
            let new_meld = Meld::new(chow_set[0], MeldKind::ConcealedChow);
            let new_sb = sb.clone().add_meld(new_meld).unwrap();
            let remains: ArrayVec<Tile, TWOSET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !idx.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), TWOSET);

            result.push((new_sb, remains));
            break;
        }
    }

    result
}

fn get_third_melds(set: &[Tile], sb: SetBuilder) -> Vec<(SetBuilder, ArrayVec<Tile, ONESET>)> {
    assert_eq!(set.len(), TWOSET);
    let mut result = Vec::new();
    // TODO: rewrite without combinations and unique
    // assume set is sorted
    // pung is always continuous (0, 1, 2)
    // chow is possible in various way, get only (0, x, y) where x + y min
    // FIXME: prove only "first" chow is needed
    let pung_set = [set[0], set[1], set[2]];
    let chow_indexes = [
        [0, 1, 2],
        [0, 1, 3],
        [0, 1, 4],
        [0, 1, 5],
        [0, 2, 4],
        [0, 2, 5],
    ];

    if is_pung(&pung_set) {
        let new_meld = Meld::new(pung_set[0], MeldKind::ConcealedPung);
        let new_sb = sb.clone().add_meld(new_meld).unwrap();
        let meld_pos = [0, 1, 2];
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

    for idx in chow_indexes.into_iter() {
        let chow_set = [set[idx[0]], set[idx[1]], set[idx[2]]];
        if is_chow(&chow_set) {
            let new_meld = Meld::new(chow_set[0], MeldKind::ConcealedChow);
            let new_sb = sb.clone().add_meld(new_meld).unwrap();
            let remains: ArrayVec<Tile, ONESET> = set
                .to_owned()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !idx.contains(i))
                .map(|(_, t)| t)
                .collect();
            assert_eq!(remains.len(), ONESET);

            result.push((new_sb, remains));
            break;
        }
    }

    result
}

fn get_last_melds(set: &[Tile], sb: SetBuilder) -> Option<HandChecker> {
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
            Some(
                sb.add_meld(new_meld)
                    .ok()?
                    .build()
                    .ok()?
                    .to_handchecker(Tile::East),
            )
        }
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_all_pungs() {
        let raw = [
            b'A', b'A', b'B', b'B', b'B', b'C', b'C', b'C', b'D', b'D', b'D', b'E', b'E', b'E',
        ];

        assert_eq!(comb(&raw), 6 * 4 * 4 * 4 * 4);
    }
}
