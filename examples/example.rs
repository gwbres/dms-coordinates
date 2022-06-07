use dms_coordinates::{Bearing, DMS, DMS3d};

fn main() {
    ///////////////////////////
    // DMS
    ///////////////////////////
    let ny = DMS::new(40, 43, 50.196, Bearing::North); // New York (lat)
    let (deg, min, sec) = (ny.degrees, ny.minutes, ny.seconds);
    let _bearing = &ny.bearing;
    let ddeg = ny.to_decimal_degrees();
    println!("New York Latitude {}", ny);
    println!("New York deg:{} min:{} sec:{}", deg,min,sec);
    println!("Decimal Degrees {}", ddeg); 
    let dms = DMS::from_decimal_degrees(ddeg, true);
    println!("{} || {}", ny, dms);
    let ddeg : f32 = dms.into();
    println!("{}", ddeg);

    let dms = DMS::from_decimal_degrees(-73.935242_f64, false);
    let (_deg, _min, _sec) = (dms.degrees, dms.minutes, dms.seconds);
    assert_eq!(dms.bearing, Bearing::West); // NY
    assert_eq!(dms.degrees, 73); // NY
    assert_eq!(dms.minutes, 56); // NY

    // Cast to f64 (prefered)
    let _ddeg : f64 = dms.into();
    // Cast to f32
    let _ddeg : f32 = dms.into();
    // Convert to azimuth
    let (_deg, _min, _sec) = dms.to_azimuth(); 

    // Convenient arithmetics ops are feasible
    let _p1 = DMS::from_decimal_degrees(-73.93, false);
    let _p2 = DMS::from_decimal_degrees(-74.0, false);
    let _p3 = _p1 + _p2;
    let _p4 = _p2 - _p1;

    ///////////////////////////
    // DMS 3D
    ///////////////////////////
    let dms = DMS3d::from_decimal_degrees(
        40.730610_f64, // NY
        -73.935242_f64, // NY
        Some(10.0) // Altitude
    );
    // Testing New York attributes:
    assert_eq!(dms.latitude.degrees, 40);
    assert_eq!(dms.latitude.minutes, 43);
    assert_eq!(dms.latitude.bearing, Bearing::North);
    assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
    assert_eq!(dms.longitude.degrees, 73);
    assert_eq!(dms.longitude.minutes, 56);
    assert_eq!(dms.longitude.bearing, Bearing::West);
    assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);

    // Azimuth / angle
    let nyork  = DMS3d::from_decimal_degrees(40.73,-73.93,None);
    let sydney = DMS3d::from_decimal_degrees(48.85,2.2321,None);
    assert!((53.78 - nyork.azimuth(sydney)) < 0.01);

    // Cartesian conversion
    let dms3d = DMS3d::from_decimal_degrees(
        -33.8698439,
        151.2082848,
        None);
    let cartesian = dms3d.to_cartesian();
    let xyz = rust_3d::Point3D::new(-4646844.502,2553749.458,-3535154.018);
    assert!((cartesian.x/1000.0 - xyz.x/1000.0).abs() < 50.0);
    assert!((cartesian.y/1000.0 - xyz.y/1000.0).abs() < 50.0);
    assert!((cartesian.z/1000.0 - xyz.z/1000.0).abs() < 50.0);

    // Projected distance [m]
    let new_york = DMS3d::from_decimal_degrees(
        40.730610_f64,
        -73.935242_f64,
        Some(10.0)
    );
    let paris = DMS3d::from_decimal_degrees(
        48.856614, 
        2.3522219,
        Some(10.0)
    );
    let dist_km = new_york.distance(paris) / 1000.0;
    assert!((5831.0 - dist_km).abs() < 1.0);

    // Arithmetics
    let p1 = DMS3d::from_decimal_degrees(-73.8, 50.0, Some(10.0));
    let p2 = DMS3d::from_decimal_degrees(2.30, 50.0, Some(100.0));
    let _p3 = p1 + p2;
    let _p4 = p2 - p1;
}
