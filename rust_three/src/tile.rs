use std::convert::TryFrom;

#[allow(dead_code)]
pub const TILEVARIANT: usize = 23;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Honor,
    Bamboo,
    Character,
    Dot,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Tile {
    // 字牌, Honor
    Sun,
    Moon,
    Red,
    Green,
    White,
    // 索, Bamboo
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    // 萬, Character
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    // 筒, Dot
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
}

#[allow(dead_code)]
impl Tile {
    pub fn is_same_color(&self, t: Tile) -> bool {
        self.color() == t.color()
    }

    pub fn color(&self) -> TileColor {
        match self {
            &Self::Sun | &Self::Moon | &Self::Red | &Self::Green | &Self::White => TileColor::Honor,
            &Self::B1 | &Self::B2 | &Self::B3 | &Self::B4 | &Self::B5 | &Self::B6 => {
                TileColor::Bamboo
            }

            &Self::C1 | &Self::C2 | &Self::C3 | &Self::C4 | &Self::C5 | &Self::C6 => {
                TileColor::Character
            }
            &Self::D1 | &Self::D2 | &Self::D3 | &Self::D4 | &Self::D5 | &Self::D6 => TileColor::Dot,
        }
    }

    pub fn number(&self) -> usize {
        match self {
            &Self::Sun | &Self::B1 | &Self::C1 | &Self::D1 => 1,
            &Self::Moon | &Self::B2 | &Self::C2 | &Self::D2 => 2,
            &Self::Red | &Self::B3 | &Self::C3 | &Self::D3 => 3,
            &Self::Green | &Self::B4 | &Self::C4 | &Self::D4 => 4,
            &Self::White | &Self::B5 | &Self::C5 | &Self::D5 => 5,
            &Self::B6 | &Self::C6 | &Self::D6 => 6,
        }
    }

    pub fn is_neighbor(&self, t: Tile) -> bool {
        self.is_ascending(t) || self.is_descending(t)
    }

    pub fn is_ascending(&self, t: Tile) -> bool {
        match self {
            &Self::Sun
            | &Self::Moon
            | &Self::Red
            | &Self::Green
            | &Self::White
            | &Self::B6
            | &Self::C6
            | &Self::D6 => false,
            _ => (*self as u8 + 1) == (t as u8),
        }
    }

    pub fn is_descending(&self, t: Tile) -> bool {
        match self {
            &Self::Sun
            | &Self::Moon
            | &Self::Red
            | &Self::Green
            | &Self::White
            | &Self::B1
            | &Self::C1
            | &Self::D1 => false,
            _ => (*self as u8 - 1) == (t as u8),
        }
    }

    pub fn is_simple(&self) -> bool {
        !(self.is_honor() || self.is_terminal())
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            &Self::B1 | &Self::C1 | &Self::D1 | &Self::B6 | &Self::C6 | &Self::D6 => true,
            _ => false,
        }
    }

    pub fn is_honor(&self) -> bool {
        self.is_wind() || self.is_dragon()
    }

    pub fn is_wind(&self) -> bool {
        match self {
            &Self::Red | &Self::Green | &Self::White => true,
            _ => false,
        }
    }

    pub fn is_dragon(&self) -> bool {
        match self {
            &Self::Sun | &Self::Moon => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Tile::Sun),
            'B' => Ok(Tile::Moon),
            'C' => Ok(Tile::Red),
            'D' => Ok(Tile::Green),
            'E' => Ok(Tile::White),
            'F' => Ok(Tile::B1),
            'G' => Ok(Tile::B2),
            'H' => Ok(Tile::B3),
            'I' => Ok(Tile::B4),
            'J' => Ok(Tile::B5),
            'K' => Ok(Tile::B6),
            'L' => Ok(Tile::C1),
            'M' => Ok(Tile::C2),
            'N' => Ok(Tile::C3),
            'O' => Ok(Tile::C4),
            'P' => Ok(Tile::C5),
            'Q' => Ok(Tile::C6),
            'R' => Ok(Tile::D1),
            'S' => Ok(Tile::D2),
            'T' => Ok(Tile::D3),
            'U' => Ok(Tile::D4),
            'V' => Ok(Tile::D5),
            'W' => Ok(Tile::D6),
            _ => Err(()),
        }
    }
}
