/// List of known bearings to construct a `D°M'S''`
pub const KNOWN_BEARINGS: &'static [char] = &['N','S','E','W'];

/// `DMS` Structure to manipulate
/// D°M'S'' coordinates
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DMS {
    degrees: i32,
    minutes: i32,
    seconds: f64,
    bearing: char,
}

macro_rules! single_line_if_else {
    ($c: expr, $v: expr, $v1: expr) => {
        if $c {$v} else {$v1}
    };
}

impl DMS {
    /// Builds new `D°M'S''` coordinates
    pub fn new (degrees: i32, minutes: i32, seconds: f64, bearing: char) -> Result<DMS, std::io::Error> {
        match KNOWN_BEARINGS.contains(&bearing) {
            true => Ok(DMS {degrees, minutes, seconds, bearing}),
            false => Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Invalid bearing value")),
        }
    }

    /// Buils new `D°M'S''` coordinates
    /// from given decimal coordinates.   
    /// is_lat: true to express result as latitude coordinates,
    /// longitude otherwise.
    pub fn from_decimal_degrees (ddeg: f64, is_lat: bool) -> DMS {
        let d = ddeg.abs().trunc() as i32;
        //let m = ((ddeg - d as f64) * 60.0).trunc() as i32;
        let m = ((ddeg.abs() - d as f64) * 60.0).trunc() as i32;
        //let s = ((ddeg - d as f64 - (m as f64)/60.0_f64)) * 3600_f64;
        let s = (ddeg.abs() - d as f64 - (m as f64)/60.0_f64) * 3600.0_f64;

        let bearing = match is_lat {
            true => single_line_if_else!(ddeg<0.0,'S','N'),
            false => single_line_if_else!(ddeg<0.0,'W','E'),
        };

        DMS {
            degrees: d,  
            minutes: m, 
            seconds: s,
            bearing: bearing,
        }
    }

    // Returns D°
    pub fn get_degrees (&self) -> i32 { return self.degrees }
    // Returns M'
    pub fn get_minutes (&self) -> i32 { return self.minutes }
    // Returns S''
    pub fn get_seconds (&self) -> f64 { return self.seconds }
    // Returns bearing 
    pub fn get_bearing (&self) -> char { return self.bearing }

    // Converts `D°M'S''` coordinates to `Decimal Degrees` coordinates
    pub fn to_decimal_degrees (&self) -> f64 {
        let ddeg: f64 = self.degrees as f64
            + self.minutes as f64 / 60.0_f64
            + self.seconds as f64 / 3600.0_f64;

        if self.get_bearing() == 'S' || self.get_bearing() == 'W' {
            -ddeg
        } else {
            ddeg
        }
    }
}

/// `3D D°M'S''` coordinates   
/// (latitude, longitude, optionnal altitude)
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DMS3d {
   latitude: DMS,
   longitude: DMS,
   altitude: Option<f64>,
}

impl DMS3d {
    /// Builds new `3D D°M'S''`  coordinates
    pub fn new (latitude: DMS, longitude: DMS, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: latitude,
            longitude: longitude,
            altitude: altitude,
        }
    }
    /// Builds a `3D D°M'S''` from given coordinates in decimal degrees
    pub fn from_decimal_degrees (lat: f64, lon: f64, altitude: Option<f64>) -> DMS3d {
        DMS3d {
            latitude: DMS::from_decimal_degrees(lat, true),
            longitude: DMS::from_decimal_degrees(lon, false),
            altitude: altitude
        }
    }

    /// Returns altitude of self
    pub fn get_altitude (&self) -> Option<f64> { self.altitude }

    /// Returns distance (m) between self and another DMS3d
    pub fn distance (&self, other: DMS3d) -> f64 {
        map_3d::distance(
            (self.latitude.to_decimal_degrees(),self.longitude.to_decimal_degrees()),
            (other.latitude.to_decimal_degrees(),other.longitude.to_decimal_degrees())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dms_construction() {
        let dms = DMS::new(10, 20, 100.0_f64, 'N');
        assert_eq!(dms.is_err(), false); // valid values
        let dms = dms.unwrap();
        assert_eq!(dms.get_degrees(), 10);
        assert_eq!(dms.get_minutes(), 20);
        assert_eq!(dms.get_seconds(), 100.0_f64);
        assert_eq!(dms.get_bearing(), 'N');
        let dms = DMS::new(10, 20, 100.0_f64, 'C');
        assert_eq!(dms.is_err(), true); // non valid values
    }
    
    #[test]
    fn test_dms_to_ddeg_conversion() {
        let dms = DMS::new(40, 43, 50.196_f64, 'N').unwrap(); // NY (lat)
        let lat = dms.to_decimal_degrees();
        let expected_lat = 40.730; // NY
        assert!((lat - expected_lat).abs() < 1E-3);
        let dms = DMS::new(33, 51, 45.36_f64, 'S').unwrap(); // SYDNEY (lat)
        let lat = dms.to_decimal_degrees();
        let expected_lat = -33.867; // SYDNEY 
        assert!((lat - expected_lat).abs() < 1E-2)
    }
    
    #[test]
    fn test_dms_from_ddeg_construction() {
        let dms = DMS::from_decimal_degrees(-73.935242_f64, false); // NY (lon) 
        let secs = 6.8712_f64; // NY
        assert_eq!(dms.get_degrees(), 73); // NY
        assert_eq!(dms.get_minutes(), 56); // NY
        assert_eq!(dms.get_bearing(), 'W');
        assert!((dms.get_seconds() - secs).abs() < 1E-3);
        let dms = DMS::from_decimal_degrees(151.209900_f64, false); // SYDNEY (lon) 
        let secs = 35.64_f64; // SYDNEY
        assert_eq!(dms.get_degrees(), 151); // SYDNEY
        assert_eq!(dms.get_minutes(), 12); // SYDNEY
        assert_eq!(dms.get_bearing(), 'E');
        assert!((dms.get_seconds() - secs).abs() < 1E-3);
        let dms = DMS::from_decimal_degrees(-34.603722, true); // Buenos Aires (lon) 
        let secs = 13.3992_f64; // Buenos Aires 
        assert_eq!(dms.get_degrees(), 34); 
        assert_eq!(dms.get_minutes(), 36); 
        assert_eq!(dms.get_bearing(), 'S');
        assert!((dms.get_seconds() - secs).abs() < 1E-3)
    }

    #[test]
    fn test_3ddms_construction() {
        let lat = DMS::new(10, 20, 100.0_f64, 'N')
            .unwrap();
        let lon = DMS::new(30, 40, 200.0_f64, 'E')
            .unwrap();
        let dms = DMS3d::new(lat, lon, Some(150.0_f64)); 
        assert_eq!(dms.latitude.get_degrees(), 10);
        assert_eq!(dms.latitude.get_minutes(), 20);
        assert_eq!(dms.longitude.get_degrees(), 30);
        assert_eq!(dms.longitude.get_minutes(), 40);
        assert_eq!(dms.altitude, Some(150.0_f64));
    }

    #[test]
    fn test_3ddms_from_ddeg() {
        let dms = DMS3d::from_decimal_degrees(
            40.730610_f64, // NY
            -73.935242_f64, // NY
            Some(10.0)
        );
        assert_eq!(dms.latitude.get_degrees(), 40); // NY
        assert_eq!(dms.latitude.get_minutes(), 43); // NY
        assert_eq!(dms.latitude.get_bearing(), 'N');
        assert!((dms.latitude.get_seconds() - 50.1960).abs() < 1E-3);
        assert_eq!(dms.longitude.get_degrees(), 73); // NY
        assert_eq!(dms.longitude.get_minutes(), 56); // NY
        assert_eq!(dms.longitude.get_bearing(), 'W');
        assert!((dms.longitude.get_seconds() - 6.8712).abs() < 1E-3);
    }

    #[test]
    fn test_distance() {
        let dms1 = DMS3d::from_decimal_degrees( // NY
            40.730610_f64,
            -73.935242_f64,
            Some(10.0)
        );
        let dms2 = DMS3d::from_decimal_degrees( // Paris
            48.856614, 
            2.3522219,
            Some(10.0)
        );
        let expected_km = 5831.0_f64; 
        let d_km = dms1.distance(dms2) / 1000.0_f64;
        assert!((expected_km - d_km).abs() < 1.0);
    }
}
