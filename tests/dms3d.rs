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
    #[test]
    fn test_3ddms_from_ddeg() {
        let dms = DMS3d::from_decimal_degrees(
            40.730610_f64, // NY
            -73.935242_f64, // NY
            Some(10.0)
        );
        assert_eq!(dms.latitude.degrees, 40); // NY
        assert_eq!(dms.latitude.minutes, 43); // NY
        assert_eq!(dms.latitude.bearing, Bearing::North);
        assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
        assert_eq!(dms.longitude.degrees, 73); // NY
        assert_eq!(dms.longitude.minutes, 56); // NY
        assert_eq!(dms.longitude.bearing, Bearing::West);
        assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);
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
    #[test]
    fn test_azimuth() {
        let dms1 = DMS3d::from_decimal_degrees( // NY
            40.73,
            -73.93,
            None,
        );
        let dms2 = DMS3d::from_decimal_degrees( // Paris
            48.85, 
            2.2321,
            None,
        );
        assert!((53.78 - dms1.azimuth(dms2)) < 0.01);
        let dms1 = DMS3d::from_decimal_degrees( // Paris 
            48.85, 
            2.2321,
            None,
        );
        let dms2 = DMS3d::from_decimal_degrees( // Sydney
            48.86,
            2.287,
            None,
        );
        assert!((68.49 - dms1.azimuth(dms2)) < 0.01)
    }
    #[test]
    fn test_to_cartesian() {
        let coords = DMS3d::from_decimal_degrees(
            -33.8698439,
            151.2082848,
            None).to_cartesian();
        let xyz = rust_3d::Point3D::new(-4646053.737,2553314.458,-3534283.535);
        assert!((coords.x/1000.0 - xyz.x/1000.0).abs() < 50.0);
        assert!((coords.y/1000.0 - xyz.y/1000.0).abs() < 50.0);
        assert!((coords.z/1000.0 - xyz.z/1000.0).abs() < 50.0);
    }
    #[test]
    fn test_from_cartesian() {
        let xyz = rust_3d::Point3D::new(-4646844.502,2553749.458,-3535154.018);
        let coords1 = DMS3d::from_decimal_degrees(
            -33.8698439,
            151.2082848,
            None);
        let cartesian = coords1.to_cartesian();
        assert!((cartesian.x/1000.0 - xyz.x/1000.0).abs() < 50.0);
        assert!((cartesian.y/1000.0 - xyz.y/1000.0).abs() < 50.0);
        assert!((cartesian.z/1000.0 - xyz.z/1000.0).abs() < 50.0);
    }
    #[test]
    fn test_to_gpx() {
        let dms = DMS3d::from_decimal_degrees(
            40.730610_f64, // NY
            -73.935242_f64, // NY
            Some(10.0)
        );
        assert_eq!(dms.to_gpx("ny.gpx").is_ok(), true);
        let ny = DMS3d::from_gpx("ny.gpx")
            .unwrap()
            .unwrap();
        assert_eq!(ny.distance(dms), 0.0)
    }
}
