//! 1D D°M'S" coordinates
use thiserror::Error;
use std::io::{ErrorKind};
use crate::Bearing;
use serde_derive::{Serialize, Deserialize};

/// `D°M'S"` coordinates
#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS {
    pub degrees: u16,
    pub minutes: u16,
    pub seconds: f64,
    pub bearing: Bearing,
}

pub enum DMSScale {
    /// Countries scale is 1°0'0"
    Country,
    /// Large cities scale is 0°6'0"
    LargeCity,
    /// Cities scale is 0°0'36"
    City,
    /// Neighborhood, Strees scale is 0°0'3.6" 
    Neighborhood,
    /// Single street / large buildings scale is 0°0'0.360"
    Street,
    /// Trees / small buildings scale is 0.036" 
    Tree,
    /// Human / single individual scale is 3.6E-3"
    Human,
    /// Rougthly precise scale, used in commercial devices, is 360E-6"
    RoughSurveying,
    /// Extremely precise scale, used in tectnoic plate mapping for instance, is 36E-6"
    PreciseSurveying,
}

impl std::fmt::Display for DMS {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}°{}'{}\"{}", 
            self.degrees, 
            self.minutes, 
            self.seconds,
            self.bearing)
    }
}

impl Default for DMS {
    fn default() -> DMS { DMS::from_ddeg_latitude(0.0_f64) }
}

impl Into<f32> for DMS {
    fn into (self) -> f32 {
        self.to_ddeg_angle() as f32
    }
}

impl Into<f64> for DMS {
    fn into (self) -> f64 {
        self.to_ddeg_angle()
    }
}

/*
impl std::ops::Add<DMS> for DMS {
    type Output = DMS;
    fn add (self, rhs: Self) -> Self {
        let (d0, m0, s0) = self.to_azimuth();
        let (d1, m1, s1) = rhs.to_azimuth();
        println!("\n({} {} {}) + ({} {} {})", d0, m0, s0, d1, m1, s1);
        let seconds = s0 as u64 + s1 as u64; 
        let minutes = (m0 as u64 + m1 as u64) + num_integer::div_floor(seconds, 60);
        let degrees = (d0 as u64 + d1 as u64) + num_integer::div_floor(minutes, 60);
        println!("{}° {}' {}\"", degrees, minutes, seconds);
        Self::from_azimuth(((degrees as u16)%360, (minutes as u16)%60, (seconds % 60) as f64))
            .unwrap()
    }
}

impl std::ops::Sub<DMS> for DMS {
    type Output = DMS;
    fn sub (self, rhs: Self) -> Self {
        let (d0, m0, s0) = self.to_azimuth();
        let (d1, m1, s1) = rhs.to_azimuth();
        println!("\n({} {} {}) - ({} {} {})", d0, m0, s0, d1, m1, s1);
        let degrees = d0 as i64 - d1 as i64; 
        let minutes = (m0 as i64 - m1 as i64) - num_integer::div_floor(degrees, 360); 
        let seconds = (s0 as i64 - s1 as i64) - num_integer::div_floor(minutes, 60);
        println!("{}° {}' {}\"", degrees, minutes, 0.0);
        DMS::from_azimuth(((degrees % 360) as u16, (minutes % 60) as u16, (seconds % 60) as f64)) 
            .unwrap()
    }
}

impl std::ops::Add<u64> for DMS {
    type Output = DMS;
    fn add (self, rhs: u64) -> Self { 
        let (d, m, s) = self.to_azimuth();
        println!("\n({} {} {}) + {}", d, m, s, rhs);
        let seconds = s as u64 + rhs;
        let minutes = m as u64 + num_integer::div_floor(seconds, 60);
        let degrees = d as u64 + num_integer::div_floor(minutes, 60);
        println!("{}° {}' {}\"", degrees, minutes, seconds);
        Self::from_azimuth(((degrees % 360) as u16, (minutes % 60) as u16, (seconds % 60) as f64))
            .unwrap()
    }
}

impl std::ops::Sub<u64> for DMS {
    type Output = DMS;
    fn sub (self, rhs: u64) -> Self { self.clone() }
}

impl std::ops::Add<i8> for DMS {
    type Output = DMS;
    fn add (self, rhs: i8) -> Self { self + rhs as u64 }
}

impl std::ops::Add<i16> for DMS {
    type Output = DMS;
    fn add (self, rhs: i16) -> Self { self + rhs as u64 }
}

impl std::ops::Add<i32> for DMS {
    type Output = DMS;
    fn add (self, rhs: i32) -> Self { self + rhs as u64 }
}

impl std::ops::Add<f32> for DMS {
    type Output = DMS;
    fn add (self, rhs: f32) -> Self { self + rhs as u64 }
}

impl std::ops::Add<f64> for DMS {
    type Output = DMS;
    fn add (self, rhs: f64) -> Self { self + rhs as u64 }
}

impl std::ops::Sub<i8> for DMS {
    type Output = DMS;
    fn sub (self, rhs: i8) -> Self { self - rhs as u64 }
}

impl std::ops::Sub<i16> for DMS {
    type Output = DMS;
    fn sub (self, rhs: i16) -> Self { self - rhs as u64 }
}

impl std::ops::Sub<i32> for DMS {
    type Output = DMS;
    fn sub (self, rhs: i32) -> Self { self - rhs as u64 }
}

impl std::ops::Sub<f32> for DMS {
    type Output = DMS;
    fn sub (self, rhs: f32) -> Self { self - rhs as u64 }
}

impl std::ops::Sub<f64> for DMS {
    type Output = DMS;
    fn sub (self, rhs: f64) -> Self { self - rhs as u64 }
}

impl std::ops::Mul<i8> for DMS {
    type Output = DMS;
    fn mul (self, rhs: i8) -> Self { self * (rhs as i64) }
}

impl std::ops::Mul<i16> for DMS {
    type Output = DMS;
    fn mul (self, rhs: i16) -> Self { self * (rhs as i64) }
}

impl std::ops::Mul<i32> for DMS {
    type Output = DMS;
    fn mul (self, rhs: i32) -> Self { self * (rhs as i64) }
}

impl std::ops::Mul for DMS {
    type Output = DMS;
    fn mul (self, rhs: Self) -> Self {
        self.clone()
    }
}

impl std::ops::Div for DMS {
    type Output = DMS;
    fn div (self, rhs: Self) -> Self {
        let (d0, m0, s0) = self.to_azimuth();
        let (d1, m1, s1) = rhs.to_azimuth();
        let degrees = match d1 > d0 {
            true => 0,
            false => d0 / d1,
        };
        DMS::from_azimuth((degrees, 0, 0.0))
            .unwrap()
    }
}

impl std::ops::Div<i8> for DMS {
    type Output = DMS;
    fn div (self, rhs: i8) -> Self { self / (rhs as i64) }
}

impl std::ops::Div<i16> for DMS {
    type Output = DMS;
    fn div (self, rhs: i16) -> Self { self / (rhs as i64) }
}

impl std::ops::Div<i32> for DMS {
    type Output = DMS;
    fn div (self, rhs: i32) -> Self { self / (rhs as i64) }
}

impl std::ops::Mul<i64> for DMS {
    type Output = DMS;
    fn mul (self, rhs: i64) -> Self {
        let (d, m, s) = self.to_azimuth();
        let mut degrees = (d as i64 * rhs) % 360; 
        let mut minutes = (m as i64 * rhs) % 60; 
        let mut seconds = (s as i64 * rhs) % 60;
        DMS::from_azimuth((degrees as u16, minutes as u16, seconds as f64))
            .unwrap()
    }
}

impl std::ops::Div<i64> for DMS {
    type Output = DMS;
    fn div (self, rhs: i64) -> Self {
        let (d0,m0,s0) = self.to_azimuth();
        let degrees = d0 / (rhs as u16); 
        let minutes = m0 / (rhs as u16); 
        let seconds = s0 / rhs as f64;
        DMS::from_azimuth((degrees,minutes,seconds))
            .unwrap()
    }
} */

impl DMS {
    /// Builds `D°M'S"` coordinates from given
    /// D°, M', S" and Bearing values
    pub fn new (degrees: u16, minutes: u16, seconds: f64, bearing: Bearing) -> std::io::Result<DMS> {
        if seconds > 60.0 {
            return Err(std::io::Error::new(ErrorKind::InvalidData, "`seconds` must be < 60"))
        }
        if minutes > 60 {
            return Err(std::io::Error::new(ErrorKind::InvalidData, "`minutes` must be < 60"))
        } 
        if bearing.is_sub_quadrant() {
            if degrees > 45 {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "`degrees` should be < 45"))
            }
        } else {
            if degrees > 90 {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "`degrees` should be < 90"))
            }
        }
        Ok(DMS {
            degrees, 
            minutes, 
            seconds, 
            bearing,
        })
    }

    /// Buils `D°M'S"` coordinates from a latitude angle ɑ, 
    /// in decimal degrees. Quadrants are prefered, Subquadrants like NE, NW, SE, SW
    /// are not used at the momoent.
    pub fn from_ddeg_latitude (ddeg: f64) -> DMS {
        let bearing = match ddeg < 0.0 {
            true => Bearing::South,
            false => Bearing::North,
        };
        let degrees = ddeg.abs().trunc() as u16;
        let minutes = ((ddeg.abs() - degrees as f64) * 60.0).trunc() as u16;
        let seconds = (ddeg.abs() - degrees as f64 - (minutes as f64)/60.0_f64) * 3600.0_f64;
        DMS {
            degrees,
            minutes,
            seconds,
            bearing,
        }
    }
    
    /// Buils `D°M'S"` coordinates from a longitude angle ɑ, 
    /// in decimal degrees. Quadrants are prefered, Subquadrants like NE, NW, SE, SW
    /// are not used at the momoent.
    pub fn from_ddeg_longitude (ddeg: f64) -> DMS {
        let bearing = match ddeg < 0.0 {
            true => Bearing::West,
            false => Bearing::East,
        };
        let degrees = ddeg.abs().trunc() as u16;
        let minutes = ((ddeg.abs() - degrees as f64) * 60.0).trunc() as u16;
        let seconds = (ddeg.abs() - degrees as f64 - (minutes as f64)/60.0_f64) * 3600.0_f64;
        DMS {
            degrees,
            minutes,
            seconds,
            bearing,
        }
    }

    /// Converts Self into angle ɑ, 
    /// ɑ e 
    pub fn to_ddeg_angle (&self) -> f64 {
        let ddeg: f64 = self.degrees as f64
            + self.minutes as f64 / 60.0_f64
            + self.seconds as f64 / 3600.0_f64;
        if self.bearing.is_southern() || self.bearing.is_western() {
            -ddeg
        } else {
            ddeg
        }
    }
}
