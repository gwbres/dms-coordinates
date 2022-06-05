# DMS Coordinates 
Rust package to manipulate D°M'S'' coordinates.

[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)

### Basic usage 

Single coordinate: 

```rust
let dms = DMS::new(40, 43, 50.196_f64, 'N') // NY latitude
    .unwrap();
prinln!("{:#?}", dms);
prinln!("{}", dms.degrees);
prinln!("{}", dms.to_decimal_degrees());
prinln!("{}", dms.bearing);
```

Build D°M'S'' from decimal degrees coordinates
```rust
let dms = DMS::from_decimal_degrees(-73.935242_f64); // NY longitude
println!("{}", dms.seconds);
assert_eq!(dms.bearing, 'N'); // 'N' bearing
assert_eq!(dms.degrees, 73); // NY::lon D°
assert_eq!(dms.minutes, 56); // NY::lon M'
```

3D coordinates

```rust
let dms = DMS3d::from_decimal_degrees(
    40.730610_f64, // NY
    -73.935242_f64, // NY
    Some(10.0)
);
assert_eq!(dms.latitude.degrees, 40); // NY
assert_eq!(dms.latitude.minutes, 43); // NY
assert_eq!(dms.latitude.bearing, 'N');
assert!((dms.latitude.seconds - 50.1960).abs() < 1E-3);
assert_eq!(dms.longitude.degrees, 73); // NY
assert_eq!(dms.longitude.minutes, 56); // NY
assert_eq!(dms.longitude.bearing, 'W');
assert!((dms.longitude.seconds - 6.8712).abs() < 1E-3);
```

(Projected) Distance (m) between two 3D coordinates:

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
println!("{}", new_york.distance(paris) / 1000.0);
```
