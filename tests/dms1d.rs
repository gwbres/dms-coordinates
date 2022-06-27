use dms_coordinates::{Cardinal, DMS1d};

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
mod dms1d {
    use super::*;
    #[test]
    fn constructor() {
        let coordinates = DMS1d::new(25, 38, 29.495, Cardinal::North);
    }
    #[test]
    fn from_ddeg_latitude() {
        let coords = DMS1d::from_ddeg_latitude(53.120405);
        assert_eq!(coords.dms.degrees, 53);
        assert_eq!(coords.dms.minutes, 7);
        assert_eq!(coords.cardinal, Cardinal::North);
        assert_float_relative_eq!(coords.dms.seconds, 13.459, 1E-3);
    }
    #[test]
    fn from_ddeg_longitude() {
        let coords = DMS1d::from_ddeg_longitude(6.1631049);
        assert_eq!(coords.dms.degrees, 6);
        assert_eq!(coords.dms.minutes, 9);
        assert_eq!(coords.cardinal, Cardinal::East);
        assert_float_relative_eq!(coords.dms.seconds, 47.177, 1E-3);
        let coords = DMS1d::from_ddeg_longitude(-86.649);
        assert_eq!(coords.dms.degrees, 86);
        assert_eq!(coords.dms.minutes, 38);
        assert_eq!(coords.cardinal, Cardinal::West);
        assert_float_relative_eq!(coords.dms.seconds, 57.822, 1E-3);
    }
}
