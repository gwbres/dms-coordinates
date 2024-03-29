//! Package to manipulate 1D and 3D sexagesimal coordinates.
//! Homepage: <https://github.com/gwbres/dms-coordinates>
#![cfg_attr(not(feature = "std"), no_std)]

pub mod cardinal;
pub mod dms;
pub mod dms3d;

#[derive(Debug)]
pub enum Error {
    InvalidLatitude,
    MissingLatitude,
    InvalidLongitude,
    MissingLongitude,
    /// When adding two cardinals toghether, they
    /// must be compatible.
    IncompatibleCardinals,
    #[cfg(feature = "gpx")]
    GpxParsingError,
    #[cfg(feature = "gpx")]
    GpxError,
}

pub use crate::{cardinal::Cardinal, dms::DMS, dms3d::DMS3d};

/// Mean radius of the Earth: 6.37 * 10^(6) m
const EARTH_RADIUS: f64 = 6.37e6_f64;

/// Returns distance (m) between two decimal degrees coordinates
/// coord1: (lat,lon), coord2: (lat, lon)
pub fn projected_distance(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    let dphi = map_3d::deg2rad(coord2.0) - map_3d::deg2rad(coord1.0);
    let d_lambda = map_3d::deg2rad(coord2.1) - map_3d::deg2rad(coord1.1);
    let a: f64 = (dphi / 2.0_f64).sin().powf(2.0_f64)
        + map_3d::deg2rad(coord1.0).cos()
            * map_3d::deg2rad(coord2.0).cos()
            * (d_lambda / 2.0_f64).sin().powf(2.0_f64);
    let c = 2.0_f64 * a.powf(0.5_f64).atan2((1.0 - a).powf(0.5_f64));
    EARTH_RADIUS * c
}

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
