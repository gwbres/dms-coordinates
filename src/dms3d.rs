//! 3D D°M'S" coordinates
use crate::Error;
use crate::EARTH_RADIUS;
use crate::{projected_distance, Cardinal, DMS};

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "gpx")]
use gpx::Waypoint;

/// 3D D°M'S" coordinates, comprises
/// a latitude: D°M'S" angle
/// a longitude: D°M'S" angle
/// and optionnal altitude
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DMS3d {
    /// Latitude angle in D°M'S", cardinal is mandatory
    pub latitude: DMS,
    /// Longitude angle in D°M'S", cardinal is mandatory
    pub longitude: DMS,
    /// Optionnal altitude / depth
    pub altitude: Option<f64>,
}

impl core::fmt::Display for DMS3d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "lat: \"{}\"  lon: \"{}\" alt: \"{}\"",
            self.latitude,
            self.longitude,
            self.altitude.unwrap_or(0.0_f64)
        )
    }
}

impl Default for DMS3d {
    /// Default DMS3D with null coordinates and null altitude
    fn default() -> Self {
        Self {
            latitude: DMS::from_ddeg_latitude(0.0_f64),
            longitude: DMS::from_ddeg_longitude(0.0_f64),
            altitude: None,
        }
    }
}

impl From<DMS3d> for (f64, f64) {
    /// Converts self to (latddeg, londdeg)
    fn from(val: DMS3d) -> Self {
        (val.latitude.to_ddeg_angle(), val.longitude.to_ddeg_angle())
    }
}

impl From<rust_3d::Point3D> for DMS3d {
    /// Builds 3D D°M'S" coordinates from cartesian (ECEF) coordinates
    fn from(item: rust_3d::Point3D) -> Self {
        Self::from_cartesian(item)
    }
}

impl core::ops::Add<DMS3d> for DMS3d {
    type Output = Result<DMS3d, Error>;
    fn add(self, rhs: Self) -> Result<Self, Error> {
        if let Some(a0) = self.altitude {
            if let Some(a1) = rhs.altitude {
                Ok(DMS3d {
                    latitude: (self.latitude + rhs.latitude)?,
                    longitude: (self.longitude + rhs.longitude)?,
                    altitude: Some(a1),
                })
            } else {
                Ok(DMS3d {
                    latitude: (self.latitude + rhs.latitude)?,
                    longitude: (self.longitude + rhs.longitude)?,
                    altitude: Some(a0),
                })
            }
        } else {
            Ok(DMS3d {
                latitude: (self.latitude + rhs.latitude)?,
                longitude: (self.longitude + rhs.longitude)?,
                altitude: None,
            })
        }
    }
}
impl DMS3d {
    /// Builds `3D D°M'S"` coordinates
    pub fn new(latitude: DMS, longitude: DMS, altitude: Option<f64>) -> Result<DMS3d, Error> {
        let cardlat = latitude.cardinal.ok_or(Error::MissingLatitude)?;
        if !cardlat.is_latitude() {
            return Err(Error::InvalidLatitude);
        }
        let cardlon = longitude.cardinal.ok_or(Error::MissingLongitude)?;
        if !cardlon.is_longitude() {
            return Err(Error::InvalidLongitude);
        }
        Ok(DMS3d {
            latitude,
            longitude,
            altitude,
        })
    }
    /// Builds 3D DMS copy with given altitude attribute in `meters`,
    /// if altitude data was already present, it gets overwritten
    pub fn with_altitude(&self, altitude: f64) -> DMS3d {
        DMS3d {
            latitude: self.latitude,
            longitude: self.longitude,
            altitude: Some(altitude),
        }
    }

    /// Same as [with_altitude] but quantity is expressed in `feet`
    pub fn with_altitude_feet(&self, altitude: f64) -> DMS3d {
        self.with_altitude(altitude / 3.28084)
    }

    /// Adds given altitude quantity to self,
    /// if altitude was not defined yet, it takes this value
    pub fn add_altitude(&mut self, altitude: f64) {
        if let Some(a) = self.altitude {
            self.altitude = Some(a + altitude)
        } else {
            self.altitude = Some(altitude)
        }
    }

    /// Same as [add_altitude] but quantity is expressed in `feet`
    pub fn add_altitude_feet(&mut self, altitude: f64) {
        self.add_altitude(altitude / 3.28084)
    }

    /// Builds `3D D°M'S"` coordinates from given angles, expressed
    /// in decimal degrees, and an optionnal altitude.
    pub fn from_ddeg_angles(latitude: f64, longitude: f64, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: {
                let dms = DMS::from_ddeg_angle(latitude);
                if latitude < 0.0 {
                    dms.with_cardinal(Cardinal::South)
                } else {
                    dms.with_cardinal(Cardinal::North)
                }
            },
            longitude: {
                let dms = DMS::from_ddeg_angle(longitude);
                if longitude < 0.0 {
                    dms.with_cardinal(Cardinal::West)
                } else {
                    dms.with_cardinal(Cardinal::East)
                }
            },
            altitude,
        }
    }

    /// Builds 3D D°M'S" coordinates from given Cartesian coordinates
    pub fn from_cartesian(xyz: rust_3d::Point3D) -> DMS3d {
        DMS3d {
            latitude: DMS::from_ddeg_latitude(map_3d::rad2deg((xyz.z / EARTH_RADIUS).asin())),
            longitude: DMS::from_ddeg_longitude(map_3d::rad2deg(xyz.y.atan2(xyz.x))),
            altitude: Some(xyz.z),
        }
    }

    /// Returns distance in meters, between Self and given 3D D°M'S" coordinates
    pub fn distance(&self, other: DMS3d) -> f64 {
        projected_distance(
            (
                self.latitude.to_ddeg_angle(),
                self.longitude.to_ddeg_angle(),
            ),
            (
                other.latitude.to_ddeg_angle(),
                other.longitude.to_ddeg_angle(),
            ),
        )
    }

    /// Returns azimuth angle ɑ, where 0 <= ɑ < 360,
    /// between Self & other 3D D°M'S" coordinates.
    /// ɑ, being the angle between North Pole & `rhs` coordinates
    pub fn azimuth(&self, rhs: Self) -> f64 {
        let (phi1, phi2) = (
            map_3d::deg2rad(self.latitude.to_ddeg_angle()),
            map_3d::deg2rad(rhs.latitude.to_ddeg_angle()),
        );
        let (lambda1, lambda2) = (
            map_3d::deg2rad(self.longitude.to_ddeg_angle()),
            map_3d::deg2rad(rhs.longitude.to_ddeg_angle()),
        );
        let dlambda = lambda2 - lambda1;
        let y = dlambda.sin() * phi2.cos();
        let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * dlambda.cos();
        map_3d::rad2deg(y.atan2(x))
    }

    /// Converts Self to Cartesian Coordinates (x, y, z).
    /// (x = 0, y = 0, z = 0) being Earth center, in Cartesian coordinates.
    pub fn to_cartesian(&self) -> rust_3d::Point3D {
        let (lat, lon) = (
            map_3d::deg2rad(self.latitude.to_ddeg_angle()),
            map_3d::deg2rad(self.longitude.to_ddeg_angle()),
        );
        rust_3d::Point3D {
            x: EARTH_RADIUS * lat.cos() * lon.cos(),
            y: EARTH_RADIUS * lat.cos() * lon.sin(),
            z: EARTH_RADIUS * lat.sin(),
        }
    }
    /// Converts Self from WGS84 to EU50 Data
    pub fn to_europe50(&self) -> Result<DMS3d, Error> {
        Ok(DMS3d {
            latitude: self.latitude.to_europe50()?,
            longitude: self.longitude.to_europe50()?,
            altitude: self.altitude,
        })
    }
}

#[cfg(feature = "gpx")]
impl From<Waypoint> for DMS3d {
    fn from(wpt: Waypoint) -> Self {
        Self::from_ddeg_angles(wpt.point().x(), wpt.point().y(), wpt.elevation)
    }
}

/*
impl core::ops::Sub for DMS3d {
    type Output = DMS3d;
    fn sub (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude - a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(-a),
                    None => None,
                }
            },
        };
        DMS3d {
            latitude : self.latitude - rhs.latitude,
            longitude: self.longitude - rhs.longitude,
            altitude: altitude,
        }
    }
}

impl core::ops::Mul for DMS3d {
    type Output = DMS3d;
    fn mul (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude - a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(-a),
                    None => None,
                }
            },
        };
        DMS3d {
            latitude : self.latitude * rhs.latitude,
            longitude: self.longitude * rhs.longitude,
            altitude: altitude,
        }
    }
}

impl core::ops::Div for DMS3d {
    type Output = DMS3d;
    fn div (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude - a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(-a),
                    None => None,
                }
            },
        };
        DMS3d {
            latitude : self.latitude / rhs.latitude,
            longitude: self.longitude / rhs.longitude,
            altitude: altitude,
        }
    }
*/
