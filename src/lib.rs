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
    /// is_lat: true if given dec. coordinates represent a latitude
    pub fn from_decimal_degrees (is_lat: bool, ddeg: f64) -> DMS {
        let d = ddeg.trunc() as i32;
        let m = (60.0 * (ddeg - ddeg.trunc().abs())) as i32;
        let s = (ddeg - ddeg.trunc()).abs() - 60.0 * (60.0 * (ddeg - ddeg.trunc()).abs());

        let bearing = match is_lat {
            true => single_line_if_else!(d<0,'S','N'),
            false => single_line_if_else!(d<0,'W','E'),
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

        if self.get_bearing() == 'N' || self.get_bearing() == 'W' {
            -ddeg
        } else {
            ddeg
        }
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
/*
    #[test]
    fn test_dms_to_ddeg_conversion() {
        let new_york_dms = DMS::new(40, 43, 50.196_f64);
        let new_york_lat = new_york_dms.to_decimal_degrees();
        let expected_lat = 40.730; // NY lat
        assert!((new_york_lat - expected_lat).abs() < 1E-3)
    }

    #[test]
    fn test_dms_from_ddeg_construction() {
        let new_york_lat = DMS::from_decimal_degrees(-73.9893_f64); // NY longitude 
        let expected_secs = 6.871_f64; // NY::lon S''
        assert_eq!(new_york_lat.get_degrees(), 73); // NY::lon D°
        assert_eq!(new_york_lat.get_minutes(), 56); // NY::lon M'
        assert!((new_york_lat.get_seconds() - expected_secs).abs() < 1E-3)
    }
*/
}


    
