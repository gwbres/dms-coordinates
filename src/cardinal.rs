//! Cardinal points, only integer angles (N, NE, E, ..) are supported

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u16)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cardinal {
    /// Northern Cardinal
    North = 0,
    /// North Eastern Cardinal
    NorthEast = 45,
    /// Eastern Cardinal
    East = 90,
    /// South Eastern Cardinal
    SouthEast = 135,
    /// Southern Cardinal
    South = 180,
    /// South Western Cardinal
    SouthWest = 225,
    /// Western Cardinal
    West = 270,
    /// North Western Cardinal
    NorthWest = 315,
}

impl std::ops::Add<u16> for Cardinal {
    type Output = Cardinal;
    /// Adds given angle (°) to Self
    fn add (self, rhs: u16) -> Self {
        Cardinal::from_angle((self.to_angle() + rhs) % 360)
    }
}

impl Default for Cardinal {
    /// Builds default Northern Cardinal
    fn default() -> Self {
        Self::North
    }
}

impl std::fmt::Display for Cardinal {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cardinal::North => write!(f, "N"),
            Cardinal::NorthEast => write!(f, "NE"),
            Cardinal::East => write!(f, "E"),
            Cardinal::SouthEast => write!(f, "SE"),
            Cardinal::South => write!(f, "S"),
            Cardinal::SouthWest => write!(f, "SW"),
            Cardinal::West => write!(f, "W"),
            Cardinal::NorthWest => write!(f, "NW"),
        }
    }
}

impl Cardinal {
    /// Returns True if Self matches a latitude cardinal
    pub fn is_latitude (&self) -> bool { 
        match self {
            Cardinal::North | Cardinal::South => true,
            _ => false,
        }
    }
    /// Returns True if Self matches a longitude cardinal
    pub fn is_longitude (&self) -> bool {
        match self {
            Cardinal::East | Cardinal::West => true,
            _ => false,
        }
    }
    /// Returns True if Cardinal and `rhs` represents
    /// same kind of coordinates
    pub fn same_kind (&self, rhs: Self) -> bool {
        (self.is_latitude() && rhs.is_latitude())
        || (self.is_longitude() && rhs.is_longitude())
    }
    /// Returns True if Self is a Northern cardinal 
    pub fn is_northern (&self) -> bool {
        match self {
            Cardinal::North | Cardinal::NorthEast | Cardinal::NorthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self is a Southern cardinal 
    pub fn is_southern (&self) -> bool {
        match self {
            Cardinal::South | Cardinal::SouthEast | Cardinal::SouthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self is an Eastern cardinal 
    pub fn is_eastern (&self) -> bool {
        match self {
            Cardinal::East | Cardinal::NorthEast | Cardinal::SouthEast => true,
            _ => false,
        }
    }
    /// Returns True if Self is a Western cardinal 
    pub fn is_western (&self) -> bool {
        match self {
            Cardinal::West | Cardinal::NorthWest | Cardinal::SouthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self matches a subquadrant cardinal, like NE or SW
    pub fn is_sub_quadrant (&self) -> bool {
        (self.to_angle() / 45)%2 > 0
    }
    /// Returns compass angle (in D°) associated to Self,
    /// 0° being North Cardinal
    pub fn to_angle (&self) -> u16 {
        match self {
            Cardinal::North => 0,
            Cardinal::NorthEast => 45,
            Cardinal::East => 90,
            Cardinal::SouthEast => 135,
            Cardinal::South => 180,
            Cardinal::SouthWest => 225,
            Cardinal::West => 270,
            Cardinal::NorthWest => 315,
        }
    }
    /// Builds a Cardinal from given compass angle (in D°),
    /// 0° being North Cardinal
    pub fn from_angle (angle: u16) -> Cardinal {
        if angle < 45 {
            Cardinal::North
        } else if angle < 90 {
            Cardinal::NorthEast
        } else if angle < 135 {
            Cardinal::East
        } else if angle < 180 {
            Cardinal::SouthEast
        } else if angle < 225 {
            Cardinal::South
        } else if angle < 270 {
            Cardinal::SouthWest
        } else if angle < 315 {
            Cardinal::West
        } else { 
            Cardinal::NorthWest
        }
    }
}
