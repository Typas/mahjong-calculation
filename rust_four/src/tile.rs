use std::convert::TryFrom;

#[allow(dead_code)]
pub const TILEVARIANT: usize = 34;

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
    Red,
    Green,
    White,
    East,
    South,
    West,
    North,
    // 索, Bamboo
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    // 萬, Character
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    // 筒, Dot
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

#[allow(dead_code)]
impl Tile {
    pub fn is_same_color(&self, t: Tile) -> bool {
        self.color() == t.color()
    }

    pub fn color(&self) -> TileColor {
        match self {
            &Self::Red
            | &Self::Green
            | &Self::White
            | &Self::East
            | &Self::South
            | &Self::West
            | &Self::North => TileColor::Honor,
            &Self::B1
            | &Self::B2
            | &Self::B3
            | &Self::B4
            | &Self::B5
            | &Self::B6
            | &Self::B7
            | &Self::B8
            | &Self::B9 => TileColor::Bamboo,
            &Self::C1
            | &Self::C2
            | &Self::C3
            | &Self::C4
            | &Self::C5
            | &Self::C6
            | &Self::C7
            | &Self::C8
            | &Self::C9 => TileColor::Character,
            &Self::D1
            | &Self::D2
            | &Self::D3
            | &Self::D4
            | &Self::D5
            | &Self::D6
            | &Self::D7
            | &Self::D8
            | &Self::D9 => TileColor::Dot,
        }
    }

    pub fn number(&self) -> usize {
        match self {
            &Self::Red | &Self::B1 | &Self::C1 | &Self::D1 => 1,
            &Self::Green | &Self::B2 | &Self::C2 | &Self::D2 => 2,
            &Self::White | &Self::B3 | &Self::C3 | &Self::D3 => 3,
            &Self::East | &Self::B4 | &Self::C4 | &Self::D4 => 4,
            &Self::South | &Self::B5 | &Self::C5 | &Self::D5 => 5,
            &Self::West | &Self::B6 | &Self::C6 | &Self::D6 => 6,
            &Self::North | &Self::B7 | &Self::C7 | &Self::D7 => 7,
            &Self::B8 | &Self::C8 | &Self::D8 => 8,
            &Self::B9 | &Self::C9 | &Self::D9 => 9,
        }
    }

    pub fn is_neighbor(&self, t: Tile) -> bool {
        self.is_ascending(t) || self.is_descending(t)
    }

    pub fn is_ascending(&self, t: Tile) -> bool {
        match self {
            &Self::Red
            | &Self::Green
            | &Self::White
            | &Self::East
            | &Self::South
            | &Self::West
            | &Self::North
            | &Self::B9
            | &Self::C9
            | &Self::D9 => false,
            _ => (*self as u8 + 1) == (t as u8),
        }
    }

    pub fn is_descending(&self, t: Tile) -> bool {
        match self {
            &Self::Red
            | &Self::Green
            | &Self::White
            | &Self::East
            | &Self::South
            | &Self::West
            | &Self::North
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
            &Self::B1 | &Self::C1 | &Self::D1 | &Self::B9 | &Self::C9 | &Self::D9 => true,
            _ => false,
        }
    }

    pub fn is_honor(&self) -> bool {
        self.is_wind() || self.is_dragon()
    }

    pub fn is_wind(&self) -> bool {
        match self {
            &Self::East | &Self::South | &Self::West | &Self::North => true,
            _ => false,
        }
    }

    pub fn is_dragon(&self) -> bool {
        match self {
            &Self::Red | &Self::Green | &Self::White => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Tile::Red),
            'B' => Ok(Tile::Green),
            'C' => Ok(Tile::White),
            'D' => Ok(Tile::East),
            'E' => Ok(Tile::South),
            'F' => Ok(Tile::West),
            'G' => Ok(Tile::North),
            'H' => Ok(Tile::B1),
            'I' => Ok(Tile::B2),
            'J' => Ok(Tile::B3),
            'K' => Ok(Tile::B4),
            'L' => Ok(Tile::B5),
            'M' => Ok(Tile::B6),
            'N' => Ok(Tile::B7),
            'O' => Ok(Tile::B8),
            'P' => Ok(Tile::B9),
            'Q' => Ok(Tile::C1),
            'R' => Ok(Tile::C2),
            'S' => Ok(Tile::C3),
            'T' => Ok(Tile::C4),
            'U' => Ok(Tile::C5),
            'V' => Ok(Tile::C6),
            'W' => Ok(Tile::C7),
            'X' => Ok(Tile::C8),
            'Y' => Ok(Tile::C9),
            'Z' => Ok(Tile::D1),
            '[' => Ok(Tile::D2),
            '\\' => Ok(Tile::D3),
            ']' => Ok(Tile::D4),
            '^' => Ok(Tile::D5),
            '_' => Ok(Tile::D6),
            '`' => Ok(Tile::D7),
            'a' => Ok(Tile::D8),
            'b' => Ok(Tile::D9),
            _ => Err(()),
        }
    }
}
