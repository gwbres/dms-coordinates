# DMS Coordinates 
Rust package to manipulate D°M'S'' coordinates.

[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)

### D°M'S'' Object 

```rust
let dms = DMS::new(40, 43, 50.196_f64, 'N') // NY latitude
    .unwrap();
prinln!("{:#?}", dms);
// internal attributes
let (deg, min, sec) = (dms.degrees, dms.minutes, dms.seconds);
let bearing = dms.bearing;
// convert to decimal degrees
let ddeg = dms.to_decimal_degrees();
```

Build D°M'S'' from decimal degrees coordinates:

```rust
let dms = DMS::from_decimal_degrees(-73.935242_f64);
// internal attributes
let (deg, min, sec) = (dms.degrees, dms.minutes, dms.seconds);
assert_eq!(dms.bearing, 'N'); // NY longitude bearing
assert_eq!(dms.degrees, 73); // NY longitude D°
assert_eq!(dms.minutes, 56); // NY longitude M'
```

D°M'S'' to Azimuth conversion: azimuth
is still expressed as D°M'S'' but 0 <= D° < 360
and bearing is dropped:

```rust
let dms = DMS::from_decimal_degrees(-73.935242_f64);
let (deg, min, sec) = dms.to_azimuth(); 
```

Convenient arithmetics ops: 

```rust
let p1 = DMS::from_decimal_degrees(-73.93); // NY longitude
let p2 = DMS::from_decimal_degrees(-74.0);
let p3 = p1 + p2;
let p4 = p2 - p1;
```

## DMS3d: 3D coordinates

3D coordinates, to represent a Latitude, a Longitude
and optionnal Altitude information:

```rust
let dms = DMS3d::from_decimal_degrees(
    40.730610_f64, // NY
    -73.935242_f64, // NY
    Some(10.0) // Altitude
);
// Testing New York attributes:
assert_eq!(dms.latitude.degrees, 40);
assert_eq!(dms.latitude.minutes, 43);
assert_eq!(dms.latitude.bearing, 'N');
assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
assert_eq!(dms.longitude.degrees, 73);
assert_eq!(dms.longitude.minutes, 56);
assert_eq!(dms.longitude.bearing, 'W');
assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);
```

Build DMS3D object from two Cartesian (x,y,z) coordinates
```rust
let p0 = rust_3d::Point3D {
    x: 10.0,
    y: 20.0,
    z: 30.0,
};
let p1 = rust_3d::Point3D {
    x: 40.0,
    y: 50.0,
    z: 60.0,
};
let dms = DMS3d::from_cartesian(p0, p1);
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

Azimuth (angle in [°], where 0 <= x < 359.9), between Self and targetted waypoint.
Azimuth is the angle between target and straigh line to North Pole.
```rust
let angle = new_york.azimuth(paris);
assert!((angle - 53.74).abs() < 0.01);
```

Convenient D°M'S'' to Cartesian coordinates (x,y,z) conversion.
In cartesian coordinates, we consider the center of the Earth
as origin (x=0,y=0,z=0), longitude = 0 is equator and
z axis goes through the poles.
```rust
let dms3d = DMS3d::from_decimal_degrees(-73.8, 50.0, None);
let cartesian = dms3d.to_cartesian();
```

Another way to convert to Cartesian coordinates is to use
the proposed cast:
```rust
let dms = DMS3d::from_decimal_degrees(-73.8, 50.0, Some(10.0));
let cartesian : rust_3d::Point3D = dms.into(); // :) 
```

Convenient arithmetics:
```rust
let p1 = DMS3d::from_decimal_degrees(-73.8, 50.0, Some(10.0));
let p2 = DMS3d::from_decimal_degrees(2.30, 50.0, Some(100.0));
let p3 = p1 + p2;
let p4 = p2 - p1;
```
