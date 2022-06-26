use dms_coordinates::{Bearing, DMS};

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
mod dms_tests {
    use super::*;
    #[test]
    fn constructor() {
        assert_eq!(
            DMS::new(40, 43, 50.196_f64, None),
            DMS {
                degrees: 40,
                minutes: 43,
                seconds: 50.196_f64,
                bearing: None,
            }
        );
        assert_eq!(
            DMS::new(180, 43, 50.196_f64, None),
            DMS {
                degrees: 180,
                minutes: 43,
                seconds: 50.196_f64,
                bearing: None,
            }
        );
        assert_eq!(
            DMS::new(359, 43, 50.196_f64, None),
            DMS {
                degrees: 359,
                minutes: 43,
                seconds: 50.196_f64,
                bearing: None,
            }
        );
        assert_eq!(
            DMS::new(10, 59, 50.196_f64, None),
            DMS {
                degrees: 10,
                minutes: 59,
                seconds: 50.196_f64,
                bearing: None,
            }
        );
        assert_eq!(
            DMS::new(10, 59, 59.99_f64, None),
            DMS {
                degrees: 10,
                minutes: 59,
                seconds: 59.99_f64,
                bearing: None,
            }
        );
    }
    #[test]
    fn wrapping_constructor() {
        assert_eq!(
            DMS::new(10, 10, 60.0_f64, None),
            DMS {
                degrees: 10,
                minutes: 11,
                seconds: 0.0, 
                bearing: None,
            }
        );
        let dms = DMS::new(10, 10, 60.1_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 11);
        assert_float_relative_eq!(dms.seconds, 0.1, 1e-6);
        let dms = DMS::new(10, 10, 60.2_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 11);
        assert_float_relative_eq!(dms.seconds, 0.2, 1e-6);
        let dms = DMS::new(10, 10, 61.2_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 11);
        assert_float_relative_eq!(dms.seconds, 1.2, 1e-6);
        let dms = DMS::new(10, 10, 121.2_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 12);
        assert_float_relative_eq!(dms.seconds, 1.2, 1e-6);
        let dms = DMS::new(10, 59, 0.0_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 60, 0.0_f64, None);
        assert_eq!(dms.degrees, 11);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 120, 0.0_f64, None);
        assert_eq!(dms.degrees, 12);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 400, 0.0_f64, None);
        assert_eq!(dms.degrees, 16);
        assert_eq!(dms.minutes, 40);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 59, 59.99_f64, None);
        assert_eq!(dms.degrees, 10);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 59.99, 1e-6);
        let dms = DMS::new(10, 59, 60.99_f64, None);
        assert_eq!(dms.degrees, 11);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.99, 1e-6);
        let dms = DMS::new(10, 59, 61.99_f64, None);
        assert_eq!(dms.degrees, 11);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 1.99, 1e-6);
        let dms = DMS::new(10, 59, 3600.0_f64, None);
        assert_eq!(dms.degrees, 11);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 59, 3659.99_f64, None);
        assert_eq!(dms.degrees, 11);
        assert_eq!(dms.minutes, 59);
        assert_float_relative_eq!(dms.seconds, 59.99, 1e-6);
        let dms = DMS::new(10, 59, 3660.00_f64, None);
        assert_eq!(dms.degrees, 12);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.0, 1e-6);
        let dms = DMS::new(10, 59, 3660.01_f64, None);
        assert_eq!(dms.degrees, 12);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.01, 1e-6);
        let dms = DMS::new(10, 59, 3660.99_f64, None);
        assert_eq!(dms.degrees, 12);
        assert_eq!(dms.minutes, 0);
        assert_float_relative_eq!(dms.seconds, 0.99, 1e-6);
    }
    /*
    #[test]
    fn to_ddeg_angle() {
        let dms = DMS::new(40, 43, 50.196_f64, Bearing::North)
            .unwrap(); // NY (lat)
        let lat = dms.to_ddeg_angle();
        let expected = 40.730; // NY
        assert!((lat - expected).abs() < 1E-3);
        
        let ddeg : f64 = dms.into();
        assert!((ddeg - expected).abs() < 1E-3);
        
        let dms = DMS::new(33, 51, 45.36_f64, Bearing::South)
            .unwrap(); // SYDNEY (lat)
        let lat = dms.to_ddeg_angle();
        let expected = -33.867; // SYDNEY 
        assert!((lat - expected).abs() < 1E-2);
        
        let ddeg : f64 = dms.into();
        assert!((ddeg - expected).abs() < 1E-2);
    }
    #[test]
    fn from_ddeg() {
        let dms = DMS::from_ddeg_latitude(4.23349);
        assert_eq!(dms.degrees, 4);
        assert_eq!(dms.minutes, 14);
        assert_eq!(dms.bearing, Bearing::North);
        assert!((dms.seconds - 0.564).abs() < 1E-3);
        
        let dms = DMS::from_ddeg_latitude(40.866389);
        assert_eq!(dms.degrees, 40);
        assert_eq!(dms.minutes, 51);
        assert_eq!(dms.bearing, Bearing::North);
        assert!((dms.seconds - 59.0).abs() < 1E-3);

        let dms = DMS::from_ddeg_longitude(151.209);
        assert_eq!(dms.degrees, 151);
        assert_eq!(dms.minutes, 12);
        assert_eq!(dms.bearing, Bearing::East);
        assert!((dms.seconds - 32.4).abs() < 1E-3);
        
        let dms = DMS::from_ddeg_longitude(-34.603);
        assert_eq!(dms.degrees, 34); 
        assert_eq!(dms.minutes, 36); 
        assert_eq!(dms.bearing, Bearing::West);
        assert!((dms.seconds - 10.8).abs() < 1E-3);
        
        let dms = DMS::from_ddeg_longitude(32.14);
        assert_eq!(dms.degrees, 32);
        assert_eq!(dms.minutes, 8); 
        assert_eq!(dms.bearing, Bearing::East);
        assert!((dms.seconds - 24.0).abs() < 1E-3);
    }
    #[test]
    fn add_ops() {
        ////////////////////////////
        // DMS' + DMS''
        ////////////////////////////
        assert_eq!(
            DMS::new(44, 12, 48.0) + DMS::new(12, 30, 20.0)
            DMS::from_azimuth((154,21,30.0)).unwrap(),
        );
        
        let p = DMS::from_azimuth((2,59,31.0)).unwrap();
        assert_eq!(p + p,
            DMS::from_azimuth((5,59,2.0)).unwrap(),
        );
        
        /*let p = p1 + 1;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 18,
            seconds: 51.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + 9;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 18,
            seconds: 59.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + 10;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 19,
            seconds: 0.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + 11;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 19,
            seconds: 1.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + 10 * 60;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 28,
            seconds: 50.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + (10 * 60 +10);
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 29,
            seconds: 0.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + (10* 60 +9);
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 28,
            seconds: 59.0,
            bearing: Bearing::NorthEast,
        });
        
        let p = p1 + (10 * 60 +11);
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 29,
            seconds: 1.0,
            bearing: Bearing::NorthEast,
        });

        let p = p1 + 10 * 60 +11;
        assert_eq!(p, DMS {
            degrees: 71,
            minutes: 29,
            seconds: 1.0,
            bearing: Bearing::NorthEast,
        
        });
        let p = DMS::from_azimuth((59, 10, 0.0)).unwrap();
        let p1 = p + 39 * 60;
        assert_eq!(p1, DMS::from_azimuth((59, 49, 0.0)).unwrap());
        let p1 = p + 49 * 60;
        assert_eq!(p1, DMS::from_azimuth((59, 59, 0.0)).unwrap());
        let p1 = p + 59 * 60;
        assert_eq!(p1, DMS::from_azimuth((60, 09, 0.0)).unwrap());
        let p = DMS::from_azimuth((359, 10, 0.0)).unwrap();
        //let p1 = p + 1;*/
    } 
    /*#[test]
    fn sub_ops() {
        let p1 = DMS::from_azimuth((68,45,53.0)).unwrap();
        let p2 = DMS::from_azimuth((12,40,29.0)).unwrap();
        let p = p1 - p2;
        assert_eq!(p,
            DMS::from_azimuth((56,05,24.0)).unwrap());
        
        let p1 = DMS::from_azimuth((68,45,53.0)).unwrap();
        let p2 = DMS::from_azimuth((12,40,29.0)).unwrap();
        let p = p1 - p2;
        assert_eq!(p,
            DMS::from_azimuth((56,05,24.0)).unwrap());
    } */
    /*#[test]
    fn mul_ops() {
        let p1 = DMS::from_azimuth((80,30,15.0)).unwrap();
        let p = p1 * 2;
        assert_eq!(p,
            DMS::from_azimuth((80*2,30*2,15.0*2.0)).unwrap());
        let p = p1 * 4;
        assert_eq!(p,
            DMS::from_azimuth((80*4,30*4,15.0*4.0)).unwrap());
        let p = p1 * 5;
        assert_eq!(p,
            DMS::from_azimuth((80*5,30*5,15.0*5.0)).unwrap());
        let p = p1 * 8;
        assert_eq!(p,
            DMS::from_azimuth((80*8,30*8,15.0*8.0)).unwrap());
    }*/
    /*#[test]
    fn div_ops() {
        let p1 = DMS::from_azimuth((80,30,15.0)).unwrap();
        let p = p1 / 2;
        assert_eq!(p,
            DMS::from_azimuth((80/2,30/2,15.0/2.0)).unwrap());
        let p = p1 / 4;
        assert_eq!(p,
            DMS::from_azimuth((80/4,30/4,15.0/4.0)).unwrap());
        let p = p1 / 5;
        assert_eq!(p,
            DMS::from_azimuth((80/5,30/5,15.0/5.0)).unwrap());
        let p = p1 / 8;
        assert_eq!(p,
            DMS::from_azimuth((80/8,30/8,15.0/8.0)).unwrap());
    } */
    */
}
