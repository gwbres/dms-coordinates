//! 3D D°M'S" coordinates
use regex::Regex;
use thiserror::Error;
use initial_conditions::EARTH_RADIUS;
use serde_derive::{Serialize, Deserialize};
use crate::{DMS, Cardinal, projected_distance};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("format is not recognized")]
    FormatNotRecognized,
    #[error("failed to parse int number")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// 3D D°M'S" coordinates, comprises
/// a latitude: D°M'S" angle with cardinal (no longer an option),
/// a longitude: D°M'S" angle with cardinal (no longer an option),
/// and optionnal altitude
#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DMS3d {
    /// Latitude angle in D°M'S"
    pub latitude: DMS,
    /// Longitude angle in D°M'S"
    pub longitude: DMS1d,
    /// Optionnal altitude / depth
    pub altitude: Option<f64>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open file")]
    IoError(#[from] std::io::Error),
    #[error("GPX parsing error")]
    GpxParsingError,
    #[error("failed to write GPX")]
    GpxWritingError(#[from] gpx::errors::GpxError),
}

impl std::fmt::Display for DMS3d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lat: \"{}\"  lon: \"{}\" alt: \"{}\"", 
            self.latitude,
            self.longitude,
            self.altitude.unwrap_or(0.0_f64))
    }
}

impl Default for DMS3d {
    fn default() -> Self {
        Self {
            latitude: DMS::from_ddeg_latitude(0.0_f64), 
            longitude: DMS::from_ddeg_longitude(0.0_f64),
            altitude: None
        }
    }
}

impl DMS3d {
    /// Builds `3D D°M'S"` coordinates
    pub fn new (latitude: DMS, longitude: DMS, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: latitude,
            longitude: longitude,
            altitude: altitude,
        }
    }

    /// Builds `3D D°M'S"` coordinates from given angles, expressed
    /// in decimal degrees, and an optionnal altitude.
    pub fn from_ddeg_angles (latitude: f64, longitude: f64, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: DMS::from_ddeg_angle(latitude),
            longitude: DMS::from_ddeg_angle(longitude),
            altitude: altitude
        }
    }

    /// Builds 3D D°M'S" coordinates from given Cartesian coordinates
    pub fn from_cartesian (xyz: rust_3d::Point3D) -> DMS3d {
        DMS3d {
            latitude: DMS::from_ddeg_latitude(map_3d::rad2deg((xyz.z / EARTH_RADIUS).asin())),
            longitude: DMS::from_ddeg_longitude(map_3d::rad2deg(xyz.y.atan2(xyz.x))),
            altitude: Some(xyz.z),
        }
    }

    /// Returns distance in meters, between Self and given 3D D°M'S" coordinates
    pub fn distance (&self, other: DMS3d) -> f64 {
        projected_distance(
            (self.latitude.to_ddeg_angle(),self.longitude.to_ddeg_angle()),
            (other.latitude.to_ddeg_angle(),other.longitude.to_ddeg_angle())
        )
    }

    /// Returns azimuth angle ɑ, where 0 <= ɑ < 360, 
    /// between Self & other 3D D°M'S" coordinates. 
    /// ɑ, being the angle between North Pole & `rhs` coordinates
    pub fn azimuth (&self, rhs: Self) -> f64 {
        let (phi1, phi2) = (map_3d::deg2rad(self.latitude.to_ddeg_angle()),
            map_3d::deg2rad(rhs.latitude.to_ddeg_angle()));
        let (lambda1, lambda2) = (map_3d::deg2rad(self.longitude.to_ddeg_angle()),
            map_3d::deg2rad(rhs.longitude.to_ddeg_angle()));
        let dlambda = lambda2 - lambda1;
        let y = dlambda.sin() * phi2.cos();
        let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * dlambda.cos();
        map_3d::rad2deg(y.atan2(x))
    }

    /// Converts Self to Cartesian Coordinates (x, y, z).
    /// (x = 0, y = 0, z = 0) being Earth center, in Cartesian coordinates.
    pub fn to_cartesian (&self) -> rust_3d::Point3D {
        let (lat, lon) = (map_3d::deg2rad(self.latitude.to_ddeg_angle()),
            map_3d::deg2rad(self.longitude.to_ddeg_angle()));
        rust_3d::Point3D {
            x: EARTH_RADIUS * lat.cos() * lon.cos(),
            y: EARTH_RADIUS * lat.cos() * lon.sin(),
            z: EARTH_RADIUS * lat.sin(),
        }
    }
    
    /// Writes self into given file in GPX format.  
    /// Resulting GPX file contains a single waypoint route.
    pub fn to_gpx (&self, fp: &str) -> Result<(), gpx::errors::GpxError> {
        let mut gpx : gpx::Gpx = Default::default();
        gpx.version = gpx::GpxVersion::Gpx11;
        let mut wpt = gpx::Waypoint::new(
            geo_types::Point::new(
                self.latitude.to_ddeg_angle(), 
                self.longitude.to_ddeg_angle()));
        wpt.elevation = self.altitude;
        gpx.waypoints.push(wpt);
        gpx::write(&gpx, std::fs::File::create(fp).unwrap())
    }

    /// Builds 3D D°M'S" coordinates from a GPX file,
    /// which must either contain a single waypoint,
    /// otherwise we use the 1st waypoint encountered in the route.
    pub fn from_gpx (fp: &str) -> Result<Option<DMS3d>, Error> {
        let fd = std::fs::File::open(fp)?;
        let content: Result<gpx::Gpx, gpx::errors::GpxError> = gpx::read(fd);
        match content {
            Ok(mut gpx) => {
                if let Some(wpt) = gpx.waypoints.pop() {
                    Ok(Some(DMS3d::from_ddeg_angles(
                        wpt.point().x(),
                        wpt.point().y(),
                    wpt.elevation))
                )
                } else {
                    Ok(None)
                }
            },
            Err(_) => Err(Error::GpxParsingError)
        }
    }
    
    /// Converts Self to WGS84 European Datum,
    /// Conversion is invalid if Self is not in WGS84 GPS.
    pub fn into_europe_wgs84 (&self) -> DMS3d {
        DMS3d {
            latitude: self.latitude + DMS::new(0, 0, 3.6, Bearing::North),
            longitude: self.longitude + DMS::new(0, 0, 2.4, Bearing::East),
            altitude: self.altitude,
        }
    }

    /// Applies correction to convert from
    /// WGS84 GPS to WGS84 EU. Do not use if Self is not in WGS84 GPS.
    pub fn convert_to_europe_wgs84 (&mut self) {
        self.latitude += DMS::new(0, 0, 3.6, Bearing::North);
        self.longitude += DMS::new(0, 0, 2.4, Bearing::East);
    }
}

/*
impl std::ops::Add for DMS3d {
    type Output = DMS3d;
    fn add (self, rhs: Self) -> Self {
        let altitude : Option<f64> = match self.altitude {
            Some(altitude) => {
                match rhs.altitude {
                    Some(a) => Some(altitude + a),
                    None => Some(altitude),
                }
            },
            None => {
                match rhs.altitude {
                    Some(a) => Some(a),
                    None => None, 
                }
            },
        };
        DMS3d { 
            latitude : self.latitude + rhs.latitude,
            longitude: self.longitude + rhs.longitude, 
            altitude: altitude, 
        }
    }
}

impl std::ops::Sub for DMS3d {
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

impl std::ops::Mul for DMS3d {
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

impl std::ops::Div for DMS3d {
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
impl From<rust_3d::Point3D> for DMS3d {
    fn from (item: rust_3d::Point3D) -> Self {
        Self::from_cartesian(item)
    }
}
