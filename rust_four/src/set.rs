use arrayvec::ArrayVec;
use bitvec::array::BitArray;
use bitvec::BitArr;

use crate::hand::Hand;
use crate::hand::HANDMAXSCORE;
use crate::hand::HANDVARIANT;
use crate::tile::Tile;
use crate::tile::TileColor;

pub const HAINUM: usize = 14;
pub const SETNUM: usize = HAINUM / 3;

#[derive(Clone, Hash, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MeldKind {
    RevealedChow,  // 明順
    RevealedPung,  // 明刻
    RevealedKong,  // 明槓
    ConcealedChow, // 暗順
    ConcealedPung, // 暗刻
    ConcealedKong, // 暗槓
}

impl MeldKind {
    pub fn is_chow(&self) -> bool {
        match self {
            &Self::RevealedChow | &Self::ConcealedChow => true,
            _ => false,
        }
    }

    pub fn is_pung(&self) -> bool {
        !self.is_chow()
    }

    #[allow(dead_code)]
    pub fn is_kong(&self) -> bool {
        match self {
            &Self::RevealedKong | &Self::ConcealedKong => true,
            _ => false,
        }
    }

    pub fn is_revealed(&self) -> bool {
        match self {
            &Self::RevealedChow | &Self::RevealedPung | &Self::RevealedKong => true,
            _ => false,
        }
    }

    pub fn is_concealed(&self) -> bool {
        !self.is_revealed()
    }
}

// 面子
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Meld {
    head: Tile,
    kind: MeldKind,
}

impl Meld {
    pub fn new(head: Tile, kind: MeldKind) -> Self {
        Self { head, kind }
    }
}

#[derive(Clone)]
pub struct SetBuilder {
    pair: Option<Tile>,
    melds: ArrayVec<Meld, SETNUM>,
    wind: Tile,
}

impl SetBuilder {
    pub fn new(wind: Tile) -> Self {
        let melds = ArrayVec::<_, SETNUM>::new_const();
        Self {
            pair: None,
            melds,
            wind,
        }
    }

    pub fn add_pair(&mut self, p: Tile) {
        self.pair = Some(p);
    }

    pub fn add_meld(&mut self, m: Meld) -> Result<(), Box<dyn std::error::Error>> {
        match self.melds.is_full() {
            true => Err("Already full of melds")?,
            false => {
                self.melds.push(m);
                Ok(())
            }
        }
    }

    pub fn build(self) -> Result<Set, Box<dyn std::error::Error>> {
        match (self.pair, self.melds.is_full()) {
            (Some(_), true) => Ok(Set {
                pair: self.pair.unwrap(),
                melds: self.melds,
                wind: self.wind,
            }),
            _ => Err("Not valid set")?,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Set {
    pair: Tile,
    melds: ArrayVec<Meld, SETNUM>,
    wind: Tile, // 自風
}

impl Set {
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

        if checker.any() == false {
            checker.set(Hand::NoPoint as usize, true);
        }

        checker
    }
}

macro_rules! perm3_match {
    (($m1: expr, $m2: expr, $m3: expr), ($pat1: pat, $pat2: pat, $pat3: pat)) => {
        match ($m1, $m2, $m3) {
            ($pat1, $pat2, $pat3)
            | ($pat1, $pat3, $pat2)
            | ($pat2, $pat1, $pat3)
            | ($pat2, $pat3, $pat1)
            | ($pat3, $pat1, $pat2)
            | ($pat3, $pat2, $pat1) => true,
            _ => false,
        }
    };
}

macro_rules! comb3_fn {
    ($self: ident, $f: ident, $checker: ident, $variant: expr) => {
        if let true = $f($self.melds[0], $self.melds[1], $self.melds[2]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[0], $self.melds[1], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[0], $self.melds[2], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[1], $self.melds[2], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
    };
}

macro_rules! comb2_fn {
    ($self: ident, $f: ident, $checker: ident, $variant: expr) => {
        if let true = $f($self.melds[0], $self.melds[1]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[0], $self.melds[2]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[0], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[1], $self.melds[2]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[1], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
        if let true = $f($self.melds[2], $self.melds[3]) {
            $checker.set($variant as usize, true);
        }
    };
}

impl Set {
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
        match self.melds.iter().any(|m| m.head == Tile::Red) {
            true => checker.set(Hand::RedPung as usize, true),
            false => (),
        }

        match self.melds.iter().any(|m| m.head == Tile::Green) {
            true => checker.set(Hand::GreenPung as usize, true),
            false => (),
        }

        match self.melds.iter().any(|m| m.head == Tile::White) {
            true => checker.set(Hand::WhitePung as usize, true),
            false => (),
        }

        match self.melds.iter().any(|m| m.head == self.wind) {
            true => checker.set(Hand::WindPung as usize, true),
            false => (),
        }
    }

    // 字牌類
    fn honors(&self, checker: &mut HandList) {
        // 四喜和
        let wind_melds = self.melds.iter().filter(|m| m.head.is_wind()).count();
        match wind_melds {
            4 => {
                checker.set(Hand::BigFourWinds as usize, true);
                return;
            }
            3 => match self.pair.is_wind() {
                true => {
                    checker.set(Hand::LittleFourWinds as usize, true);
                    return;
                }
                false => (),
            },
            _ => (),
        }

        // 三元和
        let dragon_melds = self.melds.iter().filter(|m| m.head.is_dragon()).count();
        match dragon_melds {
            3 => {
                checker.set(Hand::BigThreeDragons as usize, true);
            }
            2 => match self.pair.is_dragon() {
                true => {
                    checker.set(Hand::LittleThreeDragons as usize, true);
                }
                false => (),
            },
            _ => (),
        }
    }

    // 一氣、幺九類
    fn straight_simple_terminal(&self, checker: &mut HandList) {
        // 一氣
        let is_straight = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_chow(), m2.kind.is_chow(), m3.kind.is_chow()) {
                (true, true, true) => {
                    perm3_match!((m1.head, m2.head, m3.head), (Tile::B1, Tile::B4, Tile::B7))
                        || perm3_match!((m1.head, m2.head, m3.head), (Tile::C1, Tile::C4, Tile::C7))
                        || perm3_match!((m1.head, m2.head, m3.head), (Tile::D1, Tile::D4, Tile::D7))
                }
                _ => false,
            }
        };

        comb3_fn!(self, is_straight, checker, Hand::PureStraight);
        if checker[Hand::PureStraight as usize] == true {
            return;
        }

        // 斷幺
        let pair_simple = self.pair.is_simple();
        let melds_simple = self.melds.iter().all(|m| match m.kind.is_chow() {
            true => match m.head {
                Tile::B1 | Tile::B7 | Tile::C1 | Tile::C7 | Tile::D1 | Tile::D7 => false,
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
                        Tile::B1 | Tile::B7 | Tile::C1 | Tile::C7 | Tile::D1 | Tile::D7 => true,
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
                        Tile::B1 | Tile::B7 | Tile::C1 | Tile::C7 | Tile::D1 | Tile::D7 => true,
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
                    perm3_match!(
                        (m1.head.color(), m2.head.color(), m3.head.color()),
                        (TileColor::Bamboo, TileColor::Character, TileColor::Dot)
                    ) && m1.head.number() == m2.head.number()
                        && m2.head.number() == m3.head.number()
                }
                _ => false,
            }
        };

        comb3_fn!(self, is_mixed_triple_chow, checker, Hand::MixedTripleChow);
        if checker[Hand::MixedTripleChow as usize] == true {
            return;
        }

        // 三色同刻
        let is_mixed_triple_pung = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung(), m3.kind.is_pung()) {
                (true, true, true) => {
                    perm3_match!(
                        (m1.head.color(), m2.head.color(), m3.head.color()),
                        (TileColor::Bamboo, TileColor::Character, TileColor::Dot)
                    ) && m1.head.number() == m2.head.number()
                        && m2.head.number() == m3.head.number()
                }
                _ => false,
            }
        };

        comb3_fn!(self, is_mixed_triple_pung, checker, Hand::TriplePung);
    }

    // 暗刻類
    fn conceal_pungs(&self, checker: &mut HandList) {
        // 四暗刻
        if let true = checker[Hand::AllPungs as usize] {
            if let true = self.melds.iter().all(|m| m.kind.is_concealed()) {
                checker.set(Hand::FourConcealedPungs as usize, true);
                return;
            }
        }

        // 三暗刻
        let is_three_conceal_pung = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung(), m3.kind.is_pung()) {
                (true, true, true) => match (
                    m1.kind.is_concealed(),
                    m2.kind.is_concealed(),
                    m3.kind.is_concealed(),
                ) {
                    (true, true, true) => true,
                    _ => false,
                },
                _ => false,
            }
        };

        comb3_fn!(
            self,
            is_three_conceal_pung,
            checker,
            Hand::ThreeConcealedPungs
        );
        if checker[Hand::ThreeConcealedPungs as usize] == true {
            return;
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
            // 四同順
            if let true = self.melds.windows(2).all(|w| w[0].head == w[1].head) {
                checker.set(Hand::QuadrupleChow as usize, true);
                return;
            }

            // 二般高
            match (
                self.melds[0].head == self.melds[1].head
                    && self.melds[2].head == self.melds[3].head,
                self.melds[0].head == self.melds[2].head
                    && self.melds[1].head == self.melds[3].head,
                self.melds[0].head == self.melds[3].head
                    && self.melds[1].head == self.melds[2].head,
            ) {
                (false, false, false) => {}
                _ => {
                    checker.set(Hand::TwicePureDoubleChow as usize, true);
                    return;
                }
            }
        }

        let is_triple_chow =
            |m1: Meld, m2: Meld, m3: Meld| -> bool { is_same_chow(m1, m2) && is_same_chow(m2, m3) };

        comb3_fn!(self, is_triple_chow, checker, Hand::PureTripleChow);
        if checker[Hand::PureTripleChow as usize] == true {
            return;
        }

        comb2_fn!(self, is_same_chow, checker, Hand::PureDoubleChow);
    }

    // 連刻類
    fn shift_pungs(&self, checker: &mut HandList) {
        // 四連刻
        if checker[Hand::AllPungs as usize]
            && self
                .melds
                .windows(2)
                .all(|w| w[0].head.is_same_color(w[1].head))
            && !self.melds[0].head.is_honor()
        {
            let mut tmp: ArrayVec<u8, 4> =
                self.melds.iter().cloned().map(|m| m.head as u8).collect();
            tmp.sort();
            let tmp: ArrayVec<Tile, 4> = tmp
                .into_iter()
                .map(|t| Tile::try_from((t + b'A') as char).unwrap())
                .collect();
            if tmp.windows(2).all(|w| w[0].is_ascending(w[1])) {
                checker.set(Hand::FourPureShiftedPungs as usize, true);
                return;
            }
        }

        // 三連刻

        let is_shifted_pungs = |m1: Meld, m2: Meld, m3: Meld| -> bool {
            match (m1.kind.is_pung(), m2.kind.is_pung(), m3.kind.is_pung()) {
                (true, true, true) => match (
                    m1.head.is_ascending(m2.head) && m2.head.is_ascending(m3.head),
                    m1.head.is_ascending(m3.head) && m3.head.is_ascending(m2.head),
                    m2.head.is_ascending(m1.head) && m1.head.is_ascending(m3.head),
                    m2.head.is_ascending(m3.head) && m3.head.is_ascending(m1.head),
                    m3.head.is_ascending(m1.head) && m1.head.is_ascending(m2.head),
                    m3.head.is_ascending(m2.head) && m2.head.is_ascending(m1.head),
                ) {
                    (false, false, false, false, false, false) => false,
                    _ => true,
                },
                _ => false,
            }
        };

        comb3_fn!(self, is_shifted_pungs, checker, Hand::PureShiftedPungs);
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
        hl.set(Hand::TwicePureDoubleChow as usize, true);
        hl.set(Hand::OutsideHands as usize, true);

        assert_eq!(hl.score(), 16);
    }
}
