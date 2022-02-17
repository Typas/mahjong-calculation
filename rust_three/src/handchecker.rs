use arrayvec::ArrayVec;
use bitvec::array::BitArray;
use bitvec::BitArr;

use crate::hand::Hand;
use crate::hand::HANDMAXSCORE;
use crate::hand::HANDVARIANT;
use crate::set::{Meld, Set, SETNUM};
use crate::tile::{Tile, TileColor};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct HandList(BitArr!(for HANDVARIANT, in u32));

impl HandList {
    pub fn new() -> HandList {
        HandList(BitArray::ZERO)
    }

    pub fn score(&self) -> u16 {
        let s: u16 = self
            .iter()
            .enumerate()
            .take(HANDVARIANT)
            .map(|(i, b)| Hand::try_from(i).unwrap().score() * *b as u16)
            .sum();

        if s > HANDMAXSCORE {
            HANDMAXSCORE
        } else {
            s
        }
    }
}

impl std::ops::Deref for HandList {
    type Target = BitArr!(for HANDVARIANT, in u32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HandList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Set {
    pub fn to_handchecker(&self, wind: Tile) -> HandChecker {
        HandChecker {
            pair: self.pair,
            melds: self.melds.clone(),
            wind,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HandChecker {
    pair: Tile,
    melds: ArrayVec<Meld, SETNUM>,
    wind: Tile, // 自風
}

impl HandChecker {
    pub fn sort(&mut self) {
        self.melds.sort();
    }

    pub fn hands(&self) -> HandList {
        let mut checker = HandList::new();
        // all_chow_pung must be first
        self.all_chow_pung(&mut checker);
        self.score_pungs(&mut checker);
        self.honors(&mut checker);
        self.straight_simple_terminal(&mut checker);
        self.pure_mix(&mut checker);
        self.conceal_pungs(&mut checker);
        self.same_chows(&mut checker);
        self.shift_pungs(&mut checker);

        match checker[Hand::AllHonors as usize] {
            false => (),
            _ => {
                checker
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, c)| match i {
                        i if i == Hand::AllHonors as usize => None,
                        _ => Some(c),
                    })
                    .for_each(|mut c| c.set(false));
            }
        }
        if checker.any() == false {
            checker.set(Hand::NoPoint as usize, true);
        }

        checker
    }
}

impl Ord for HandChecker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pair.cmp(&other.pair) {
            std::cmp::Ordering::Equal => self.melds.cmp(&other.melds),
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for HandChecker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! comb2_fn {
    ($self: ident, $f: ident, $checker: ident, $variant: expr) => {
        if let true = $f($self.melds[0], $self.melds[1]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[0], $self.melds[2]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[1], $self.melds[2]) {
            $checker.set($variant as usize, true);
        }
    };
}

impl HandChecker {
    // 平和、對對和
    fn all_chow_pung(&self, checker: &mut HandList) {
        // 平和
        match self.melds.iter().all(|m| m.kind.is_chow()) {
            true => {
                checker.set(Hand::AllChows as usize, true);
                return;
            }
            false => (),
        }

        // 對對
        match self.melds.iter().all(|m| m.kind.is_pung()) {
            true => {
                checker.set(Hand::AllPungs as usize, true);
            }
            false => (),
        }
    }

    // 役牌
    fn score_pungs(&self, checker: &mut HandList) {
        match self.melds.iter().any(|m| m.head == Tile::Dark) {
            true => checker.set(Hand::DarkPung as usize, true),
            false => (),
        }

        match self.melds.iter().any(|m| m.head == Tile::Light) {
            true => checker.set(Hand::LightPung as usize, true),
            false => (),
        }

        match self.melds.iter().any(|m| m.head == self.wind) {
            true => checker.set(Hand::WindPung as usize, true),
            false => (),
        }
    }

    // 字牌類
    fn honors(&self, checker: &mut HandList) {
        // 三元和
        let wind_melds = self.melds.iter().filter(|m| m.head.is_wind()).count();
        match wind_melds {
            3 => {
                checker.set(Hand::BigThreeWinds as usize, true);
                return;
            }
            2 => match self.pair.is_wind() {
                true => {
                    checker.set(Hand::LittleThreeWinds as usize, true);
                    return;
                }
                false => (),
            },
            _ => (),
        }

        // 雙喜和
        let dragon_melds = self.melds.iter().filter(|m| m.head.is_dragon()).count();
        match dragon_melds {
            2 => {
                checker.set(Hand::TwoDragons as usize, true);
            }
            _ => (),
        }
    }

    // 幺九類
    fn straight_simple_terminal(&self, checker: &mut HandList) {
        // 斷幺
        let pair_simple = self.pair.is_simple();
        let melds_simple = self.melds.iter().all(|m| match m.kind.is_chow() {
            true => match m.head {
                Tile::B1 | Tile::B4 | Tile::C1 | Tile::C4 | Tile::D1 | Tile::D4 => false,
                _ => true,
            },
            false => m.head.is_simple(),
        });

        if let (true, true) = (pair_simple, melds_simple) {
            checker.set(Hand::AllSimples as usize, true);
            return;
        }

        // 清老、混老、清全、混全
        match checker[Hand::AllPungs as usize] {
            true => {
                if let (true, true) = (
                    self.pair.is_terminal(),
                    self.melds.iter().all(|m| m.head.is_terminal()),
                ) {
                    checker.set(Hand::AllTerminals as usize, true);
                    return;
                }

                if let (true, true) = (
                    !self.pair.is_simple(),
                    self.melds.iter().all(|m| !m.head.is_simple()),
                ) {
                    checker.set(Hand::AllTerminalsAndHonors as usize, true);
                };
            }
            false => {
                let pair_terminal = self.pair.is_terminal();
                let melds_terminal = self.melds.iter().all(|m| match m.kind.is_chow() {
                    true => match m.head {
                        Tile::B1 | Tile::B4 | Tile::C1 | Tile::C4 | Tile::D1 | Tile::D4 => true,
                        _ => false,
                    },
                    false => m.head.is_terminal(),
                });

                if let (true, true) = (pair_terminal, melds_terminal) {
                    checker.set(Hand::TerminalsInAllSets as usize, true);
                    return;
                }

                let pair_honor_or_terminal = !pair_simple;
                let melds_honor_or_terminal = self.melds.iter().all(|m| match m.kind.is_chow() {
                    true => match m.head {
                        Tile::B1 | Tile::B4 | Tile::C1 | Tile::C4 | Tile::D1 | Tile::D4 => true,
                        _ => false,
                    },
                    false => !m.head.is_simple(),
                });

                if let (true, true) = (pair_honor_or_terminal, melds_honor_or_terminal) {
                    checker.set(Hand::OutsideHands as usize, true);
                }
            }
        }
    }

    // 一色類、三色類
    fn pure_mix(&self, checker: &mut HandList) {
        // 字一色
        let pair_honor = self.pair.is_honor();
        let melds_honor = self.melds.iter().all(|m| m.head.is_honor());
        if let (true, true) = (pair_honor, melds_honor) {
            checker.set(Hand::AllHonors as usize, true);
            return;
        }

        // 清一色
        if let true = self.melds.iter().all(|m| m.head.is_same_color(self.pair)) {
            checker.set(Hand::FullFlush as usize, true);
            return;
        }

        // 混一色
        match self.pair.is_honor() {
            true => {
                // set first non-honor meld color
                let color_tile = self
                    .melds
                    .iter()
                    .filter(|m| !m.head.is_honor())
                    .take(1)
                    .map(|m| m.head)
                    .next()
                    .unwrap();

                if let true = self
                    .melds
                    .iter()
                    .all(|m| m.head.is_same_color(color_tile) || m.head.is_honor())
                {
                    checker.set(Hand::HalfFlush as usize, true);
                }
            }
            false => {
                if let true = self
                    .melds
                    .iter()
                    .all(|m| m.head.is_same_color(self.pair) || m.head.is_honor())
                {
                    checker.set(Hand::HalfFlush as usize, true);
                    return;
                }
            }
        }

        // 三色同順
        let is_mixed_triple_chow = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_chow(), m2.kind.is_chow(), m3.kind.is_chow()) {
                (true, true, true) => {
                    matches!(
                        (m1.head.color(), m2.head.color(), m3.head.color()),
                        (TileColor::Bamboo, TileColor::Character, TileColor::Dot)
                    ) && m1.head.number() == m2.head.number()
                        && m2.head.number() == m3.head.number()
                }
                _ => false,
            }
        };

        if let true = is_mixed_triple_chow(self.melds[0], self.melds[1], self.melds[2]) {
            checker.set(Hand::MixedTripleChow as usize, true);
            return;
        }

        // 三色同刻
        let is_mixed_triple_pung = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung(), m3.kind.is_pung()) {
                (true, true, true) => {
                    matches!(
                        (m1.head.color(), m2.head.color(), m3.head.color()),
                        (TileColor::Bamboo, TileColor::Character, TileColor::Dot)
                    ) && m1.head.number() == m2.head.number()
                        && m2.head.number() == m3.head.number()
                }
                _ => false,
            }
        };

        if let true = is_mixed_triple_pung(self.melds[0], self.melds[1], self.melds[2]) {
            checker.set(Hand::TriplePung as usize, true);
        }
    }

    // 暗刻類
    fn conceal_pungs(&self, checker: &mut HandList) {
        // 三暗刻
        if let true = checker[Hand::AllPungs as usize] {
            if let true = self.melds.iter().all(|m| m.kind.is_concealed()) {
                checker.set(Hand::ThreeConcealedPungs as usize, true);
                return;
            }
        }

        // 二暗刻
        let is_two_conceal_pung = |m1: Meld, m2: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung()) {
                (true, true) => match (m1.kind.is_concealed(), m2.kind.is_concealed()) {
                    (true, true) => true,
                    _ => false,
                },
                _ => false,
            }
        };

        comb2_fn!(self, is_two_conceal_pung, checker, Hand::TwoConcealedPungs);
    }
    // 同順類
    fn same_chows(&self, checker: &mut HandList) {
        let is_same_chow = |m1: Meld, m2: Meld| -> bool {
            match (m1.kind.is_chow(), m2.kind.is_chow()) {
                (true, true) => m1.head == m2.head,
                _ => false,
            }
        };

        if let true = checker[Hand::AllChows as usize] {
            // 三同順
            if let true = self.melds.windows(2).all(|w| w[0].head == w[1].head) {
                checker.set(Hand::PureTripleChow as usize, true);
                return;
            }
        }

        comb2_fn!(self, is_same_chow, checker, Hand::PureDoubleChow);
    }

    // 連刻類
    fn shift_pungs(&self, checker: &mut HandList) {
        // 三連刻
        if checker[Hand::AllPungs as usize]
            && self
                .melds
                .windows(2)
                .all(|w| w[0].head.is_same_color(w[1].head))
            && !self.melds[0].head.is_honor()
        {
            let mut tmp: ArrayVec<u8, SETNUM> =
                self.melds.iter().cloned().map(|m| m.head as u8).collect();
            tmp.sort();
            let tmp: ArrayVec<Tile, SETNUM> = tmp
                .into_iter()
                .map(|t| Tile::try_from((t + b'A') as char).unwrap())
                .collect();
            if tmp.windows(2).all(|w| w[0].is_ascending(w[1])) {
                checker.set(Hand::ThreePureShiftedPungs as usize, true);
                return;
            }
        }

        // 二連刻

        let is_shifted_pungs = |m1: Meld, m2: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung()) {
                (true, true) => {
                    match (m1.head.is_ascending(m2.head), m2.head.is_ascending(m1.head)) {
                        (false, false) => false,
                        _ => true,
                    }
                }
                _ => false,
            }
        };

        comb2_fn!(self, is_shifted_pungs, checker, Hand::PureShiftedPungs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlist_score() {
        let hl = HandList::new();
        assert_eq!(hl.score(), 0);
    }

    #[test]
    fn handlist_pattern_score() {
        let mut hl = HandList::new();
        hl.set(Hand::AllChows as usize, true);
        hl.set(Hand::HalfFlush as usize, true);
        hl.set(Hand::OutsideHands as usize, true);

        // assert_eq!(hl.score(), 16);
    }
}
