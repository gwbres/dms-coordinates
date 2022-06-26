//! 1D D°M'S" coordinates
use thiserror::Error;
use std::io::{ErrorKind};
use crate::Bearing;
use serde_derive::{Serialize, Deserialize};

/// Wrapping modulo ops to use on `Seconds` in D°M'S" structure.
/// Returns (extra degrees, extra minutes, wrapped) 
fn seconds_wrapping_modulo (secs: f64) -> (u16, u16, f64) {
    if secs < 60.0 {
        (0, 0, secs)
    } else if secs < 3600.0 {
        let minutes = (secs / 60.0).floor();
        let integer = (secs.floor() as u16)%60;
        let fract = secs.fract();
        (0, minutes as u16, integer as f64 + secs.fract())
    } else {
        let degrees = (secs / 3600.0).floor();
        let minutes = ((secs - degrees*3600.0) / 60.0).floor(); 
        let integer = (secs.floor() as u16)%60;
        let fract = secs.fract();
        (degrees as u16, minutes as u16, integer as f64 + secs.fract())
    }
}

/// Wrapping modulo ops on integer value for `Minutes` field in D°M'S"
fn minutes_wrapping_modulo (value: u16) -> (u16, u16) { (value/60, value %60) }

/// `D°M'S"` coordinates,
/// to represent an angle Degrees / Minutes / Seconds,
/// where 1° is corresponds to the rotation of Earth in 1 hour.   
/// 0 <= D° < 360 when no Bearing is used.
/// 0 <= D° < 90 when Bearing is used.
/// 0 <= D° < 45 when Sub quadrant (NE,SE,SW,NW quandrants) is used.
#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS {
    pub degrees: u16,
    pub minutes: u16,
    pub seconds: f64,
    pub bearing: Option<Bearing>,
}

pub enum Scale {
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
        if let Some(bearing) = self.bearing {
            write!(f, "{}°{}'{}\"{}", 
                self.degrees, 
                self.minutes, 
                self.seconds,
                bearing)
        } else {
            write!(f, "{}°{}'{}\"", 
                self.degrees, 
                self.minutes, 
                self.seconds)
        }
    }
}

impl Default for DMS {
    /// Builds a null coordinates, with no Bearing associated to it
    fn default() -> DMS { 
        DMS {
            degrees: 0,
            minutes: 0,
            seconds: 0.0_f64,
            bearing: None,
        }
    }
}

/*
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
*/

/*
impl std::ops::Add<DMS> for DMS {
    type Output = DMS;
    fn add (self, rhs: Self) -> Self {
        let matches_lat = self.bearing.is_latitude() && rhs.bearing.is_latitude();
        let matches_lon = self.bearing.is_longitude() && rhs.bearing.is_longitude();
        if !(matches_lat || matches_lon) {
            panic!("cannot add {} and {} together", self, rhs);
        }
        let (d0, rhs) = (self.to_ddeg_angle(), rhs.to_ddeg_angle());
        if matches_lat {
            Self::from_ddeg_latitude(d0 + rhs) 
        } else {
            Self::from_ddeg_longitude(d0 + rhs) 
        }
    }
}
*/

/*
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
    /// D°, M', S" and optionnal Bearing.
    /// This method never fails, it adapts to unusually large values by wrapping and apply
    /// correct modulo operations
    pub fn new (degrees: u16, minutes: u16, seconds: f64, bearing: Option<Bearing>) -> DMS {
        if let Some(bearing) = bearing {
            Self::default()
        } else {
            let (s_degs, s_mins, seconds) = seconds_wrapping_modulo(seconds); 
            let (m_degs, minutes) = minutes_wrapping_modulo(minutes + s_mins);
            DMS {
                degrees: (degrees + s_degs + m_degs)%360, 
                minutes, 
                seconds, 
                bearing: None,
            }
        }
    }
}
/*
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
    /// ɑ expressed in decimal degrees
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
} */

mod tests {
    use super::*;
    #[test]
    fn test_seconds_modulo() {
        assert_eq!(seconds_wrapping_modulo(59.0), (0, 0, 59.0));
        assert_eq!(seconds_wrapping_modulo(59.8), (0, 0, 59.8));
        assert_eq!(seconds_wrapping_modulo(59.9), (0, 0, 59.9));
        assert_eq!(seconds_wrapping_modulo(60.0), (0, 1, 0.0));
        let (d,m,s) = seconds_wrapping_modulo(60.1);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 0.1, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(60.2);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 0.2, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(60.4);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 0.4, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(61.0);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 1.0, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(61.1);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 1.1, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(61.2);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 1.2, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(119.99);
        assert_eq!((d, m), (0, 1)); 
        assert_float_relative_eq!(s, 59.99, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(120.0);
        assert_eq!((d, m), (0, 2)); 
        assert_float_relative_eq!(s, 0.0, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(120.1);
        assert_eq!((d, m), (0, 2)); 
        assert_float_relative_eq!(s, 0.1, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(120.2);
        assert_eq!((d, m), (0, 2)); 
        assert_float_relative_eq!(s, 0.2, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(121.2);
        assert_eq!((d, m), (0, 2));
        assert_float_relative_eq!(s, 1.2, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3599.98);
        assert_eq!((d, m), (0, 59));
        assert_float_relative_eq!(s, 59.98, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3599.998);
        assert_eq!((d, m), (0, 59));
        assert_float_relative_eq!(s, 59.998, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3599.999);
        assert_eq!((d, m), (0, 59));
        assert_float_relative_eq!(s, 59.999, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3600.0);
        assert_eq!((d, m), (1, 0));
        assert_float_relative_eq!(s, 0.0, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3600.1);
        assert_eq!((d, m), (1, 0));
        assert_float_relative_eq!(s, 0.1, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3600.2);
        assert_eq!((d, m), (1, 0));
        assert_float_relative_eq!(s, 0.2, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3799.99);
        assert_eq!((d, m), (1, 3));
        assert_float_relative_eq!(s, 19.99, 1e-6);
        let (d,m,s) = seconds_wrapping_modulo(3899.99);
        assert_eq!((d, m), (1, 4));
        assert_float_relative_eq!(s, 59.99, 1e-6);
    }
    #[test]
    fn test_minute__modulo() {
        assert_eq!(minutes_wrapping_modulo(59), (0, 59));
        assert_eq!(minutes_wrapping_modulo(60), (1, 0));
        assert_eq!(minutes_wrapping_modulo(61), (1, 1));
        assert_eq!(minutes_wrapping_modulo(62), (1, 2));
        assert_eq!(minutes_wrapping_modulo(122), (2, 2));
        assert_eq!(minutes_wrapping_modulo(3599), (59, 59));
        assert_eq!(minutes_wrapping_modulo(3600), (60, 0));
        assert_eq!(minutes_wrapping_modulo(3601), (60, 1));
        assert_eq!(minutes_wrapping_modulo(3602), (60, 2));
    }
}
