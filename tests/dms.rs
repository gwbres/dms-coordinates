use dms_coordinates::{DMS, Cardinal};

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
mod dms_tests {
    use super::*;
    #[test]
    fn constructor() {
        let dms = DMS::new(25, 38, 29.495, None);
        assert_eq!(dms.degrees, 25);
        assert_eq!(dms.minutes, 38);
        assert!((29.495 - dms.seconds).abs() < 1E-6, true);
        
        let dms = DMS::new(90, 59, 59.999, None);
        assert_eq!(dms.degrees, 90);
        assert_eq!(dms.minutes, 59);
        assert!((59.999 - dms.seconds).abs() < 1E-6, true);
        
        let dms = DMS::new(180, 40, 29.495, None);
        assert_eq!(dms.degrees, 180);
        assert_eq!(dms.minutes, 40);
        assert!((29.495 - dms.seconds).abs() < 1E-6, true);
    }
    #[test]
    fn wrapping_constructor() {
        let dms = DMS::new(91, 59, 61.0, None);
        assert_eq!(dms.degrees, 92);
        assert_eq!(dms.minutes, 0);
        assert!((1.0 - dms.seconds).abs() < 1E-6, true);
        
        let dms = DMS::new(359, 59, 61.0, None);
        assert_eq!(dms.degrees, 0);
        assert_eq!(dms.minutes, 0);
        assert!((1.0 - dms.seconds).abs() < 1E-6, true);
        
        let dms = DMS::new(359, 58, 61.0, None);
        assert_eq!(dms.degrees, 359);
        assert_eq!(dms.minutes, 59);
        assert!((1.0 - dms.seconds).abs() < 1E-6, true);
    } 
    #[test]
    fn total_seconds() {
        let dms = DMS::new(0, 0, 59.9, None);
        assert!((dms.total_seconds() - 59.9).abs() < 1E-6);
        let dms = DMS::new(0, 10, 59.9, None);
        assert!((dms.total_seconds() - 659.9).abs() < 1E-6);
    }
    #[test]
    fn test_from_ddeg() {
        let d = DMS::from_ddeg_angle(3.357015);
        assert_eq!(d.degrees, 3);
        assert_eq!(d.minutes, 21);
        assert_float_relative_eq!(d.seconds, 25.254, 1E-6);
        assert_eq!(d.cardinal, None);
    }
    #[test]
    fn test_to_ddeg() {
        let d = DMS::new(3, 21, 25.255, Some(Cardinal::South));
        assert_float_relative_eq!(d.to_ddeg_angle(), -3.3570127, 1E-6);

        let d = DMS::new(43, 49, 54.114, Some(Cardinal::West));
        assert_float_relative_eq!(d.to_ddeg_angle(), -43.83169, 1E-6);
    }
}
