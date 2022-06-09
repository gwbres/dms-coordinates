# DMS Coordinates 

[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)


Rust Crate for D°M'S'' coordinates manipulation, used in navigation  :sailboat: :airplane: :ship:

### `D°M'S''` 

`D°M'S''` represents coordinates with an associated bearing

```rust
let ny = DMS::new(40, 43, 50.196, Bearing::North); // New York (lat)
let (deg, min, sec) = (ny.degrees, ny.minutes, ny.seconds);
let _bearing = &ny.bearing;
let ddeg = ny.to_decimal_degrees(); // WGS84
println!("New York (lat) {}", ny);
println!("New York - {}° {}' {}''", deg, min, sec);
```

Build `D°M'S''` coordinates from decimal degrees coordinates (`WGS84`):

```rust
let dms = DMS::from_decimal_degrees(-73.935242_f64, false)
    .unwrap();
assert_eq!(dms.bearing, Bearing::West); // NY 
assert_eq!(dms.degrees, 73); // NY 
assert_eq!(dms.minutes, 56); // NY 
```

Another way to convert to `WGS84` is to cast
a D°M'S'' into a floating point number:
```rust
let ddeg : f64 = dms.into(); // cast to f64 (prefered)
let ddeg : f32 = dms.into(); // cast to f32
```

Convert `D°M'S''` to `Azimuth`.   
Azimuth is still expressed as D°M'S'' but 0 <= D° < 360
and bearing is dropped:

```rust
let (deg, min, sec) = dms.to_azimuth(); 
```

Build a `D°M'S''` with associated `bearing`, from an azimuth angle.   
Angle must also be given in `D°M'S''`, but 0 <= D° < 360:
```rust
let dms = DMS::from_azimuth((135, 0, 0.0)).unwrap(); // 135°0'0''
assert_eq!(dms,
    DMS { // 45°0'0''SE
        degrees: 45,
        minutes: 0, 
        seconds: 0.0, 
        bearing: Bearing::SouthEast
    }
);
let dms = DMS::from_azimuth((270, 0, 0.0)).unwrap(); // 270°0'0''
assert_eq!(dms,
    DMS { // 90°0'0''NW
        degrees: 90, 
        minutes: 0, 
        seconds: 0.0, 
        bearing: Bearing::NorthWest
    }
);
```

Convenient arithmetics ops are feasible: 

```rust
let p1 = DMS::from_decimal_degrees(-73.93, false);
let p2 = DMS::from_decimal_degrees(-74.0,  false);
let p = p1 + p2;
let p = p1 * 2;
let p = p1 * 3.14;
let p = p1 / 4;
let p = p1 + 1; // adds 1 second
let p = p1 + 60; // adds 1 minute 
let p = p1 + 1.0; // adds 1 second 
let p = p1 + 61.0; // adds 61 seconds
```

## `DMS3d`: 3D coordinates

3D coordinates (Latitude, Longitude & optionnal Altitude):

```rust
let dms = DMS3d::from_decimal_degrees(
    40.730610_f64, // NY
    -73.935242_f64, // NY
    Some(10.0) // Altitude
);
assert_eq!(dms.latitude.bearing, Bearing::North);
assert_eq!(dms.latitude.minutes, 43);
assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
assert_eq!(dms.longitude.bearing, Bearing::West);
assert_eq!(dms.longitude.minutes, 56);
assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);
```

Build `DMS3D` object from `Cartesian` (x,y,z) coordinates.
In cartesian coordinates, Earth center is considered as 
the origin (x=0, y=0, z=0), 
longitude = 0 is equator and
z axis goes through the poles.

```rust
let cartesian = rust_3d::Point3D {
    x : 10.0,
    y : 20.0,
    z : 30.0,
};
let dms3d = DMS3d::from_cartesian(cartesian);
```

Another way to build from `Cartesian` (x,y,z) coordinates,
is to cast  a `rust_3d::Point3D` object into a `DMS3d`:
```rust
let p = rust_3d::Point3D {
    x : 10.0,
    y : 20.0,
    z : 30.0,
};
let dms3d = DMS3d::from(p); // :) 
```

Convert a DMS3d to Cartesian coordinates:
```rust
let dms3d = DMS3d::from_decimal_degrees(
    -33.8698439,
    151.2082848,
    None);
let cartesian = dms3d.to_cartesian();
let xyz = rust_3d::Point3D::new(-4646844.502,2553749.458,-3535154.018);
assert!((cartesian.x/1000.0 - xyz.x/1000.0).abs() < 50.0);
assert!((cartesian.y/1000.0 - xyz.y/1000.0).abs() < 50.0);
assert!((cartesian.z/1000.0 - xyz.z/1000.0).abs() < 50.0);
```

Distance (in [m]) between two 3D coordinates:
```rust
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
```

`Azimuth` (angle in [°], where 0 <= x < 359.9), between Self and targetted waypoint.   
Azimuth is the angle between target and straigh line to North Pole.
```rust
let nyork  = DMS3d::from_decimal_degrees(40.73,-73.93,None);
let sydney = DMS3d::from_decimal_degrees(48.85,2.2321,None);
assert!((53.78 - nyork.azimuth(sydney)) < 0.01);
```

Arithmetics over DMS3d objects are feasible:
```rust
let p1 = DMS3d::from_decimal_degrees(-73.8, 50.0, Some(10.0));
let p2 = DMS3d::from_decimal_degrees(2.30, 50.0, Some(100.0));
let p3 = p1 + p2;
let p4 = p2 - p1;
```
