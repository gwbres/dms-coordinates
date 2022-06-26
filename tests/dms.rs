use dms_coordinates::{Bearing, DMS};

#[cfg(test)]
mod tests {
    use super::*;
    fn test_constructor() {
        assert_eq!(
            DMS::new(40, 43, 50.196_f64, Bearing::North).is_ok(),
            true);
        assert_eq!(
            DMS::new(40, 43, 50.196_f64, Bearing::NorthEast).is_err(),
            true);
    }
    fn test_to_ddeg_angle() {
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
    fn test_from_ddeg() {
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
        assert_eq!(dms.degrees, 151); // SYDNEY
        assert_eq!(dms.minutes, 12); // SYDNEY
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
    /*#[test]
    fn test_from_azimuth() {
        assert_eq!(
            DMS::from_azimuth((135,0,0.0)).unwrap(),
            DMS {
                degrees: 45,
                minutes: 0,
                seconds: 0.0,
                bearing: Bearing::SouthEast,
            });
        assert_eq!(
            DMS::from_azimuth((270,0,0.0)).unwrap(),
            DMS {
                degrees: 90,
                minutes: 0,
                seconds: 0.0,
                bearing: Bearing::SouthWest,
            });
        assert_eq!(
            DMS::from_azimuth((85,57,10.0)).unwrap(),
            DMS {
                degrees: 85,
                minutes: 57,
                seconds: 10.0,
                bearing: Bearing::NorthEast,
            });
        assert_eq!(
            DMS::from_azimuth((146,29,37.0)).unwrap(),
            DMS {
                degrees: 34,
                minutes: 29,
                seconds: 37.0,
                bearing: Bearing::SouthEast,
            });
        assert_eq!(
            DMS::from_azimuth((237,18,02.0)).unwrap(),
            DMS {
                degrees: 57,
                minutes: 18,
                seconds: 2.0,
                bearing: Bearing::SouthWest,
            });
        assert_eq!(
            DMS::from_azimuth((325,47,28.0)).unwrap(),
            DMS {
                degrees: 35,
                minutes: 47,
                seconds: 28.0,
                bearing: Bearing::NorthWest,
            });
        assert_eq!(
            DMS::from_azimuth((101,23,16.0)).unwrap(),
            DMS {
                degrees: 79,
                minutes: 23,
                seconds: 16.0,
                bearing: Bearing::SouthEast,
            });
        assert_eq!(DMS::from_azimuth((5,59,2.0)).is_err(), false);
        assert_eq!(DMS::from_azimuth((5,61,2.0)).is_err(), true);
        assert_eq!(DMS::from_azimuth((5,0,62.0)).is_err(), true);
        assert_eq!(DMS::from_azimuth((361,0,0.0)).is_err(), true);
        assert_eq!(DMS::from_azimuth((359, 10, 0.0)).is_err(), false);
        assert_eq!(DMS::from_azimuth((359, 59, 59.9)).is_err(), false);
    }*/
    /* #[test]
    fn test_add_ops() {
        let p1 = DMS::from_azimuth((71,18,50.0)).unwrap();
        let p2 = DMS::from_azimuth((83,02,40.0)).unwrap();
        assert_eq!(p1 + p2,
            DMS::from_azimuth((154,21,30.0)).unwrap(),
        );
        
        let p = DMS::from_azimuth((2,59,31.0)).unwrap();
        assert_eq!(p + p,
            DMS::from_azimuth((5,59,2.0)).unwrap(),
        );
        
        let p = p1 + 1;
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
        //let p1 = p + 1;
    } */
    /*#[test]
    fn test_sub_ops() {
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
    fn test_mul_ops() {
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
    fn test_div_ops() {
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
}
