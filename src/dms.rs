//! Angle representation in D°M'S" (sexagesimal format).
//! Supports arithmetics operation, up to double precision,
//! for easy navigation calculations.
use thiserror::Error;
use crate::cardinal::Cardinal;

#[cfg(feature = "serde")]
use serde_derive::{Serialize, Deserialize};

/// Angle expressed as `D°M'S"`, 
/// in Degrees D°, Minutes M' and fractionnal
/// Seconds S" (double precision) with an optionnal Cardinal.
/// When a cardinal is associated to this angle,
/// we consider this angle represents either a Latitude
/// or a Longitude angle.
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DMS {
    /// Degrees D° 
    pub degrees: u16,
    /// Minutes M'
    pub minutes: u8,
    /// Seconds with fractionnal part S"
    pub seconds: f64,
    /// Optionnal cardinal associated to this angle
    pub cardinal: Option<Cardinal>,
}

#[derive(Error, Debug)]
pub enum OpsError {
    #[error("incompatible cardinals")]
    IncompatibleCardinals,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Roughly precise scale, used in commercial devices, is 360E-6"
    RoughSurveying,
    /// Extremely precise scale, used in tectnoic plate mapping for instance, is 36E-6"
    PreciseSurveying,
}

impl std::fmt::Display for DMS {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(cardinal) = self.cardinal {
            write!(f, "{}°{}'{}\"{}",
                self.degrees, 
                self.minutes, 
                self.seconds,
                cardinal,
            )
        } else {
            write!(f, "{}°{}'{}\"", 
                self.degrees,
                self.minutes,
                self.seconds,
            )
        }
    }
}

impl Default for DMS {
    /// Builds null angle with no Cardinal associated to it
    fn default() -> Self { 
        Self {
            degrees: 0,
            minutes: 0,
            seconds: 0.0_f64,
            cardinal: None,
        }
    }
}

impl Into<f64> for DMS {
    /// Converts Self to decimal degrees 
    fn into (self) -> f64 { 
        self.to_ddeg_angle() 
    }
}

impl Into<f32> for DMS {
    /// Converts Self into fractionnal seconds with precision loss 
    fn into (self) -> f32 { 
        self.to_ddeg_angle() as f32 
    }
}

impl Into<u64> for DMS {
    /// Returns total amount of seconds in Self, 
    /// loosing fractionnal part
    fn into (self) -> u64 { 
        self.total_seconds().floor() as u64 
    }
}

impl Into<u32> for DMS {
    /// Returns total amount of seconds in Self, 
    /// loosing fractionnal part
    fn into (self) -> u32 { 
        self.total_seconds().floor() as u32 
    }
}

impl Into<u16> for DMS {
    /// Returns total amount of seconds in Self, 
    /// loosing fractionnal part
    fn into (self) -> u16 { 
        self.total_seconds().floor() as u16 
    }
}

impl Into<u8> for DMS {
    /// Returns total amount of seconds in Self, 
    /// loosing fractionnal part
    fn into (self) -> u8 { 
        self.total_seconds().floor() as u8 
    }
}

impl std::ops::Add<DMS> for DMS {
    type Output = Result<Self, OpsError>;
    fn add (self, rhs: Self) -> Result<Self, OpsError> {
        if let Some(c0) = self.cardinal {
            if let Some(c1) = rhs.cardinal {
                let a = self.to_ddeg_angle()
                    + rhs.to_ddeg_angle();
                if c0.is_latitude() && c1.is_latitude() {
                    Ok(Self::from_ddeg_latitude(a))
                } else if c0.is_longitude() && c1.is_longitude() {
                    Ok(Self::from_ddeg_longitude(a))
                } else {
                    Err(OpsError::IncompatibleCardinals)    
                }
            } else {
                Ok(Self::from_seconds(self.total_seconds()
                    + rhs.total_seconds()))
            }
        } else {
            Ok(Self::from_seconds(self.total_seconds()
                + rhs.total_seconds()))
        }
    }
}

impl std::ops::AddAssign<DMS> for DMS {
    fn add_assign (&mut self, rhs: Self) {
        if let Some(c0) = self.cardinal {
            if let Some(c1) = rhs.cardinal {
                let a = self.to_ddeg_angle()
                    + rhs.to_ddeg_angle();
                if c0.is_latitude() && c1.is_latitude() {
                    *self = Self::from_ddeg_latitude(a)
                } else if c0.is_longitude() && c1.is_longitude() {
                    *self = Self::from_ddeg_longitude(a)
                }
            } else {
                *self = Self::from_seconds(
                    self.total_seconds()
                    + rhs.total_seconds())
            }
        } else {
            *self = Self::from_seconds(
                self.total_seconds()
                + rhs.total_seconds())
        }
    }
}

impl std::ops::AddAssign<f64> for DMS {
    fn add_assign (&mut self, rhs: f64) {
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() + rhs;
            if cardinal.is_latitude() {
                *self = Self::from_ddeg_latitude(a)
            } else {
                *self = Self::from_ddeg_longitude(a)
            }
        } else {
            *self = Self::from_seconds(self.total_seconds() + rhs)
        }
    }
}

impl std::ops::Add<f64> for DMS {
    type Output = Self;
    fn add (self, rhs: f64) -> Self { 
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() + rhs;
            if cardinal.is_latitude() {
                Self::from_ddeg_latitude(a)
            } else {
                Self::from_ddeg_longitude(a)
            }
        } else {
            Self::from_seconds(self.total_seconds() + rhs)
        }
    }
}

impl std::ops::Sub<f64> for DMS {
    type Output = Self;
    fn sub (self, rhs: f64) -> Self { 
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() - rhs;
            if cardinal.is_latitude() {
                Self::from_ddeg_latitude(a)
            } else {
                Self::from_ddeg_longitude(a)
            }
        } else {
            Self::from_seconds(self.total_seconds() - rhs)
        }
    }
}

impl std::ops::SubAssign<f64> for DMS {
    fn sub_assign (&mut self, rhs: f64) { 
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() - rhs;
            if cardinal.is_latitude() {
                *self = Self::from_ddeg_latitude(a)
            } else {
                *self = Self::from_ddeg_longitude(a)
            }
        } else {
            *self = Self::from_seconds(self.total_seconds() - rhs)
        }
    }
}

impl std::ops::Mul<f64> for DMS {
    type Output = DMS;
    fn mul (self, rhs: f64) -> DMS {
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() * rhs;
            if cardinal.is_latitude() {
                Self::from_ddeg_latitude(a)
            } else {
                Self::from_ddeg_longitude(a)
            }
        } else {
            Self::from_seconds(self.total_seconds() * rhs)
        }
    }
}

impl std::ops::Div<f64> for DMS {
    type Output = DMS;
    fn div (self, rhs: f64) -> DMS {
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() / rhs;
            if cardinal.is_latitude() {
                Self::from_ddeg_latitude(a)
            } else {
                Self::from_ddeg_longitude(a)
            }
        } else {
            Self::from_seconds(self.total_seconds() / rhs)
        }
    }
}

impl std::ops::MulAssign<f64> for DMS {
    fn mul_assign (&mut self, rhs: f64) {
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() * rhs;
            if cardinal.is_latitude() {
                *self = Self::from_ddeg_latitude(a)
            } else {
                *self = Self::from_ddeg_longitude(a)
            }
        } else {
            *self = Self::from_seconds(self.total_seconds() * rhs)
        }
    }
}

impl std::ops::DivAssign<f64> for DMS {
    fn div_assign (&mut self, rhs: f64) {
        if let Some(cardinal) = self.cardinal {
            let a = self.to_ddeg_angle() / rhs;
            if cardinal.is_latitude() {
                *self = Self::from_ddeg_latitude(a)
            } else {
                *self = Self::from_ddeg_longitude(a)
            }
        } else {
            *self = Self::from_seconds(self.total_seconds() / rhs)
        }
    }
}

impl DMS {
    /// Builds `D°M'S"` angle, from given D°, M', S" values.
    /// This method allows overflow, it will wrapp values to correct range
    /// itself.
    pub fn new (degrees: u16, minutes: u8, seconds: f64, cardinal: Option<Cardinal>) -> DMS { 
        let d =  Self::from_seconds(
            degrees as f64 * 3600.0 
                + minutes as f64 * 60.0
                    + seconds);
        if let Some(cardinal) = cardinal {
            d.with_cardinal(cardinal)
        } else {
            d
        }
    }

    /// Builds `D°M'S"` angle from total amount of seconds
    pub fn from_seconds (seconds: f64) -> Self {
        let degrees = (seconds / 3600.0).floor();
        let minutes = ((seconds - degrees * 3600.0) /60.0).floor();
        let integer = ((seconds - degrees * 3600.0 - minutes*60.0).floor() as u8)%60;
        Self {
            degrees: (degrees as u16)%360,
            minutes: minutes as u8,
            seconds: integer as f64 + seconds.fract(),
            cardinal: None,
        }
    }

    /// Returns same D°M'S" angle but attaches a cardinal to it.
    /// Useful to convert make this D°M'S" angle a Latitude or a
    /// Longitude.
    pub fn with_cardinal (&self, cardinal: Cardinal) -> Self {
        Self {
            degrees: self.degrees,
            minutes: self.minutes,
            seconds: self.seconds,
            cardinal: Some(cardinal),
        }
    }

    /// Builds D°M'S" angle from given angle expressed in 
    /// decimal degrees, with no cardinal associated to returned value
    pub fn from_ddeg_angle (angle: f64) -> Self {
        let degrees = angle.abs().floor();
        let minutes = ((angle.abs() - degrees) * 60.0).floor();
        let seconds = (angle.abs() - degrees - minutes/60.0_f64) * 3600.0_f64;
        Self {
            degrees: degrees as u16,
            minutes: minutes as u8,
            seconds,
            cardinal: None,
        }
    }

    /// Builds Latitude angle, expressed in D°M'S", from
    /// given angle expressed in decimal degrees
    pub fn from_ddeg_latitude (angle: f64) -> Self {
        let degrees = angle.abs().floor();
        let minutes = ((angle.abs() - degrees) * 60.0).floor();
        let seconds = (angle.abs() - degrees - minutes/60.0_f64) * 3600.0_f64;
        let cardinal = if angle < 0.0 {
            Cardinal::South
        } else {
            Cardinal::North
        };
        Self {
            degrees: (degrees as u16)%90,
            minutes: minutes as u8,
            seconds,
            cardinal: Some(cardinal),
        }
    }

    /// Builds Longitude angle, expressed in D°M'S",
    /// from given angle expressed in decimal degrees
    pub fn from_ddeg_longitude (angle: f64) -> Self {
        let degrees = angle.abs().floor();
        let minutes = (angle.abs() - degrees) * 60.0;
        let seconds = (angle.abs() - degrees - minutes/60.0_f64) * 3600.0_f64;
        let cardinal = if angle < 0.0 {
            Cardinal::South
        } else {
            Cardinal::North
        };
        Self {
            degrees: (degrees as u16)%180,
            minutes: minutes as u8,
            seconds,
            cardinal: Some(cardinal),
        }
    }

    /// Returns Self expressed in decimal degrees
    /// If no cardinal is associated, returned angle strictly > 0.
    pub fn to_ddeg_angle (&self) -> f64 {
        let d = self.degrees as f64
            + self.minutes as f64 / 60.0_f64
                + self.seconds as f64 / 3600.0_f64;
        match self.cardinal {
            Some(cardinal) => {
                if cardinal.is_southern() || cardinal.is_western() {
                    -d
                } else {
                    d
                }
            },
            None => d,
        }
    }

    /// Adds given angle to Self, angle expressed a decimal degrees
    pub fn add_ddeg (&mut self, angle: f64) {
        *self = Self::from_ddeg_angle(
            self.to_ddeg_angle() + angle);
    }

    /// Returns copy of Self with given angle added, as decimal degrees
    pub fn with_ddeg_angle (&self, angle: f64) -> Self {
        Self::from_ddeg_angle(
            self.to_ddeg_angle() + angle)
    }

    /// Returns total of seconds (base unit) contained in Self
    pub fn total_seconds (&self) -> f64 {
        self.degrees as f64 * 3600.0
        + self.minutes as f64 * 60.0
            + self.seconds
    }

    /// Converts self to radians
    pub fn to_radians (&self) -> f64 {
        self.to_ddeg_angle() / 180.0 * std::f64::consts::PI 
    }
/*    
    /// Descriptor must follow standard formats:
    ///     +DDD.D  : sign + 3 digit "." + 1digit
    ///     Degrees specified, minutes = 0, seconds = 0
    ///     +DDDMM.M : sign + 3 digit D° + 2 digit M' "." 1 digit M'
    ///     Degrees + minutes specified
    ///     +DDDMMSS.S : sign + 3 digit D° + 2 digit M' + fractionnal seconds
    /// <!> Although standards says "+" is mandatory to describe positive D°,
    /// this method tolerates a missing '+' and we interprate D° as positive value.
    pub fn from_str (s: &str) -> Result<Self, ParseError> {
        let lon_positive_d = Regex::new(r"^+\d{3}.\d{1}$")
            .unwrap();
        let lon_negative_d = Regex::new(r"^-\d{3}.\d{1}$")
            .unwrap();
        let lon_positive_dm = Regex::new(r"^+\d{3}d{2}.\d{1}$")
            .unwrap();
        let lon_negative_dm = Regex::new(r"^-\d{d}d{2}.\d{1}$")
            .unwrap();
        let lon_positive_dms = Regex::new(r"^+\d{3}d{2}d{2}.\d{1}$")
            .unwrap();
        let lon_negative_dms = Regex::new(r"^-\d{3}d{2}d{2}.\d{1}$")
            .unwrap();
        if lon_positive_d.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::East),
            })
        } else if lon_negative_d.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::West),
            })

        } else if lon_positive_dm.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::East),
            })

        } else if lon_negative_dm.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::West),
            })

        } else if lon_positive_dms.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::East),
            })

        } else if lon_negative_dms.is_match(s) {
            let degrees = u16::from_str_radix(&s[0..3], 10)?; //attention au '+'
            Ok(DMS {
                degrees,
                minutes: 0,
                seconds: 0.0,
                cardinal: Some(Cardinal::West),
            })
        
        } else {
            Err(ParseError::FormatNotRecognized)
        }
    }
*/
    
    /// Returns D°M'S" angle copy with
    /// WGS84 to EU50 conversion applied.
    /// For conversion to be applied, we need a cardinal to be associated,
    /// otherwise this simply returns a copy
    pub fn to_europe50 (&self) -> Result<DMS, OpsError> {
        if let Some(cardinal) = self.cardinal {
            if cardinal.is_latitude() {
                *self + DMS::new(0, 0, 3.6, Some(Cardinal::North))
            } else {
                *self + DMS::new(0, 0, 2.4, Some(Cardinal::East))
            }
        } else {
            Ok(self.clone())
        }
    }
}
