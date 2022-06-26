//! Bearing, compass quadrants / headings
use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Bearing {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    East,
}

impl std::fmt::Display for Bearing {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bearing::North => write!(f, "N"),
            Bearing::NorthEast => write!(f, "NE"),
            Bearing::NorthWest => write!(f, "NW"),
            Bearing::South => write!(f, "S"),
            Bearing::SouthEast => write!(f, "SE"),
            Bearing::SouthWest => write!(f, "SW"),
            Bearing::East => write!(f, "E"),
            Bearing::West => write!(f, "W"),
        }
    }
}

impl Bearing {
    /// Returns True if Self matches a latitude bearing
    pub fn is_latitude (&self) -> bool { 
        match self {
            Bearing::North | Bearing::South => true,
            _ => false,
        }
    }
    /// Returns True if Self matches a longitude bearing
    pub fn is_longitude (&self) -> bool {
        match self {
            Bearing::East | Bearing::West => true,
            _ => false,
        }
    }
    /// Returns True if Self is a Northern bearing 
    pub fn is_northern (&self) -> bool {
        match self {
            Bearing::North | Bearing::NorthEast | Bearing::NorthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self is a Southern bearing 
    pub fn is_southern (&self) -> bool {
        match self {
            Bearing::South | Bearing::SouthEast | Bearing::SouthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self is an Eastern bearing 
    pub fn is_eastern (&self) -> bool {
        match self {
            Bearing::East | Bearing::NorthEast | Bearing::SouthEast => true,
            _ => false,
        }
    }
    /// Returns True if Self is a Western bearing 
    pub fn is_western (&self) -> bool {
        match self {
            Bearing::West | Bearing::NorthWest | Bearing::SouthWest => true,
            _ => false,
        }
    }
    /// Returns True if Self matches a subquadrabt bearing, like NE or SW
    pub fn is_sub_quadrant (&self) -> bool {
        match self {
            Bearing::NorthEast | Bearing::SouthEast | Bearing::NorthWest | Bearing::SouthWest => true,
            _ => false,
        }
    }
}
