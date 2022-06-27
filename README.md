# DMS Coordinates 

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

Rust Crate for D°M'S" coordinates manipulation, used in navigation :sailboat: :airplane: :ship:

This crate exposes several structures
* D° M' S" is core struct, which represents an angle |ɑ| < 360°
* DMS1d to represent either 
 * a latitude angle |ɑ| < 90°
 * or a longitude angle |ɑ| < 180°
* DMS3d to represent 3D coordinates on Earth
* Bearing: for navigation purposes

### `D°M'S"` (1D coordinates)

`D°M'S"` represents 1D coordinates

```rust
use dms_coordinates::{DMS, Bearing};
let dms = DMS::new(40, 43, 50.196, Bearing::North);
assert_eq!(dms.degrees, 40);
assert_eq!(dms.minutes, 43);
assert_eq!(dms.bearing, Bearing::North);
```

As long as you're not specifying a `Bearing`,
the `D°M'S"` constructore is flexible and will accept any angle values.
For example:
```rust
// S" overflow
let dms = DMS::new(90, 10, 61.0, None);
assert_eq!(dms.is_ok(), true); // is tolerated
assert_eq!(dms.degrees, 90);
assert_eq!(dms.minutes, 11);
assert_eq!(dms.seconds, 1.0);

// M' + S" overflow
let dms = DMS::new(88, 60, 61.0, None);
assert_eq!(dms.is_ok(), true); // is tolerated
assert_eq!(dms.degrees, 90);
assert_eq!(dms.minutes, 0);
assert_eq!(dms.seconds, 1.0);

// D° + M' + S" overflow
let dms = DMS::new(360, 61, 61.0, None);
assert_eq!(dms.is_ok(), true); // is tolerated
assert_eq!(dms.degrees, 1);
assert_eq!(dms.minutes, 2);
assert_eq!(dms.seconds, 1.0);
```

But when using N, S, E, W bearings,
we expect D° < 180:

```rust
// NE for example
let dms = DMS::new(44, 0, 59.0, Some(Bearing::NorthEast));
assert_eq!(dms.is_ok(), true); // valid NE heading

// E heading with S" overflow
let dms = DMS::new(179, 58, 60.0, Some(Bearing::East));
assert_eq!(dms.is_ok(), true); // still a valid E heading
assert_eq!(dms.degrees, 179);
assert_eq!(dms.minutes, 59); // overflow
assert_eq!(dms.minutes, 0.0); // overflow
```

```rust
// 89° is ok for North bearing
assert_eq!(DMS::new(89, 0, 0.0, Bearing::North).is_ok(), true);
// 91° is nok for North bearing
assert_eq!(DMS::new(91, 0, 0.0, Bearing::North).is_err(), true);
// 89° is nok for NorthEast bearing
assert_eq!(DMS::new(89, 0, 0.0, Bearing::NorthEast).is_err(), true);
// 46° is nok for NorthEast bearing
assert_eq!(DMS::new(46, 0, 0.0, Bearing::NorthEast).is_err(), true);
// 44° is ok for NorthEast bearing
assert_eq!(DMS::new(44, 0, 0.0, Bearing::NorthEast).is_ok(), true);
```

Convert `D°M'S"` into Decimal degrees angle (WGS84),
angle is 0 <= angle < 360:

```rust
let ddeg = ny.to_ddeg_angle();
assert!((ddeg - 40.730).abs(), < 1e-3); // expected NY latitude
```

Convenient method to build `D°M'S"` coordinates from coordinates
in Decimal Degrees (`WGS84`):

```rust
// Sydney longitude coordinates
let sydney = DMS::from_ddeg_longitude(151.209)
    .unwrap();
assert_eq!(dms.degrees, 151); 
assert_eq!(dms.minutes, 12);
assert_eq!(dms.bearing, Bearing::East);
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
