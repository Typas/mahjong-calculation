use arrayvec::ArrayVec;

use crate::tile::Tile;

pub const HAINUM: usize = 14;
pub const SETNUM: usize = HAINUM / 3;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum MeldKind {
    RevealedChow,  // 明順
    ConcealedChow, // 暗順
    RevealedPung,  // 明刻
    ConcealedPung, // 暗刻
    RevealedKong,  // 明槓
    ConcealedKong, // 暗槓
}

#[allow(dead_code)]
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

    pub fn kind(&self) -> MeldKind {
        self.kind
    }

    pub fn head(&self) -> Tile {
        self.head
    }
}

// 對子
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pair {
    head: Tile,
    concealed: bool,
}

impl Pair {
    pub fn new(head: Tile, concealed: bool) -> Self {
        Self { head, concealed }
    }

    pub fn is_concealed(&self) -> bool {
        self.concealed
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.head.cmp(&other.head)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Deref for Pair {
    type Target = Tile;
    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

#[derive(Debug, Clone)]
pub struct SetBuilder {
    pair: Option<Pair>,
    melds: ArrayVec<Meld, SETNUM>,
}

impl SetBuilder {
    pub fn new() -> Self {
        let melds = ArrayVec::<_, SETNUM>::new_const();
        Self { pair: None, melds }
    }

    pub fn add_pair(mut self, p: Pair) -> Self {
        self.pair = Some(p);
        self
    }

    pub fn add_meld(mut self, m: Meld) -> Result<Self, Box<dyn std::error::Error>> {
        match self.melds.is_full() {
            true => Err("Already full of melds")?,
            false => {
                self.melds.push(m);
                Ok(self)
            }
        }
    }

    pub fn build(self) -> Result<Set, Box<dyn std::error::Error>> {
        match self.melds.is_full() {
            true => Ok(Set {
                pair: self.pair.ok_or("pair not assigned")?,
                melds: self.melds,
            }),
            _ => Err("Not valid set")?,
        }
    }
}

impl Ord for Meld {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.head.cmp(&other.head) {
            std::cmp::Ordering::Equal => self.kind.cmp(&other.kind),
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Meld {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: more general Set, contains open hands
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pair: Pair,
    melds: ArrayVec<Meld, SETNUM>,
}

#[allow(dead_code)]
impl Set {
    pub fn pair(&self) -> Pair {
        self.pair
    }

    pub fn melds(&self) -> &ArrayVec<Meld, SETNUM> {
        &self.melds
    }
}
