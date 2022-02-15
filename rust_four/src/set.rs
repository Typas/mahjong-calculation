use arrayvec::ArrayVec;

use crate::tile::Tile;

pub const HAINUM: usize = 14;
pub const SETNUM: usize = HAINUM / 3;

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
    pub(crate) head: Tile,
    pub(crate) kind: MeldKind,
}

impl Meld {
    pub fn new(head: Tile, kind: MeldKind) -> Self {
        Self { head, kind }
    }
}

#[derive(Debug, Clone)]
pub struct SetBuilder {
    pair: Option<Tile>,
    melds: ArrayVec<Meld, SETNUM>,
}

impl SetBuilder {
    pub fn new() -> Self {
        let melds = ArrayVec::<_, SETNUM>::new_const();
        Self { pair: None, melds }
    }

    pub fn add_pair(mut self, p: Tile) -> Self {
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
            true => {
                let pair = self.pair.ok_or("No assigned pair tile")?;
                Ok(Set {
                    pair,
                    melds: self.melds,
                })
            }
            _ => Err("Not valid set")?,
        }
    }
}

// TODO: more general Set, contains open hands
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pub(crate) pair: Tile,
    pub(crate) melds: ArrayVec<Meld, SETNUM>,
}
