use dms_coordinates::{Bearing, DMS, DMS3d};

fn main() {
    let ny = DMS::new(40, 43, 50.196, Bearing::North); // New York (lat)
    // internal attributes
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
}
