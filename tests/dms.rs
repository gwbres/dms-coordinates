use dms_coordinates::{Cardinal, DMS};

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
mod dms_tests {
    use super::*;
    #[test]
    fn constructor() {
        let dms = DMS::new(25, 38, 29.495);
        assert_eq!(dms.degrees, 25);
        assert_eq!(dms.minutes, 38);
        assert_float_relative_eq!(dms.seconds, 29.495, 1E-6);
        
        let dms = DMS::new(90, 59, 59.999);
        assert_eq!(dms.degrees, 90);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 59.999, 1E-6);
        
        let dms = DMS::new(180, 40, 29.495);
        assert_eq!(dms.degrees, 180);
        assert_eq!(dms.minutes, 40);
        assert_float_relative_eq!(dms.seconds, 29.495, 1E-6);
    }
    #[test]
    fn wrapping_constructor() {
        let dms = DMS::new(91, 59, 61.0);
        assert_eq!(dms.degrees, 92);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 1.0, 1E-6);
        
        let dms = DMS::new(359, 59, 61.0);
        assert_eq!(dms.degrees, 0);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 1.0, 1E-6);
        
        let dms = DMS::new(359, 58, 61.0);
        assert_eq!(dms.degrees, 359);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 1.0, 1E-6);
    } 
    #[test]
    fn total_seconds() {
        let dms = DMS::new(0, 0, 59.9);
        assert_float_relative_eq!(dms.total_seconds(), 59.9, 1E-6);
    }
    #[test]
    fn seconds_cast() {
        let dms = DMS::new(0, 0, 59.9);
        let seconds : f64 = dms.into();
        assert_float_relative_eq!(seconds, 59.9, 1E-6);
        let seconds : u32 = dms.into();
        assert_eq!(seconds, 59);
    }
    /*
    #[test]
    fn test_add_dms() {
        let d0 = DMS::new(71, 18, 50.0)
            .unwrap();
        let d1 = DMS::new(83, 2, 40.3)
            .unwrap(); 
        let d = d0 + d1;
        assert_eq!(d.degrees, 154);
        assert_eq!(d.minutes, 21);
        assert_float_relative_eq!(d.seconds, 30.3, 1E-6);
        let d0 = DMS::new(101, 23, 16.3)
            .unwrap();
        let d1 = DMS::new(2, 59, 31.3)
            .unwrap(); 
        let d = d0 + d1;
        assert_eq!(d.degrees, 104);
        assert_eq!(d.minutes, 22);
        assert_float_relative_eq!(d.seconds, 47.6, 1E-6);
        let d0 = DMS::new(68, 45, 53.8)
            .unwrap();
        let d1 = DMS::new(12, 10, 31.3)
            .unwrap();
        let d = d0 + d1;
        assert_eq!(d.degrees, 80);
        assert_eq!(d.minutes, 56);
        assert_float_relative_eq!(d.seconds, 25.1, 1E-6);
    }
    #[test]
    fn test_add_float() {
        let d0 = DMS::new(101, 23, 16.3).unwrap();
        let d = d0 + 1.0_f64;
        assert_eq!(d.degrees, 101);
        assert_eq!(d.minutes, 23);
        assert_float_relative_eq!(d.seconds, 17.3, 1E-6);
        
        let d0 = DMS::new(101, 23, 16.3).unwrap();
        let d = d0 + 1.7_f64;
        assert_eq!(d.degrees, 101);
        assert_eq!(d.minutes, 23);
        assert_float_relative_eq!(d.seconds, 18.0, 1E-6);
        
        let d0 = DMS::new(99, 58, 59.8).unwrap();
        let d = d0 + 0.2_f32;
        assert_eq!(d.degrees, 99);
        assert_eq!(d.minutes, 59);
        assert_float_relative_eq!(d.seconds, 0.0, 1E-6);
    }*/
}
