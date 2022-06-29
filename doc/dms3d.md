3D DMS
======

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

DMS 3D comprises a latitude angle expressed as DMS,
a longitude angle expressed as DMS,
and an optionnal altitude.

When dealing with angles in DMS3d, `Cardinals` are now mandatory,
whereas DMS structures allows optionnal Cardinals:

* latitude (ϕ) where |ϕ| <= 90°, Northern latitude means positive angle,
Southern latitude means negative angle

* longitude (λ) where |λ| <= 180°, Eastern longitude means positive angle,
Western longitude means negative angle

* Build 3D DMS coordinates

```rust
let lat = DMS::new(10, 20, 45.0, Some(Cardinal::North));
let lon = DMS::new(20, 10, 30.0, Some(Cardinal::West));
let coords = DMS3d::new(lat, lon, None).unwrap();
assert_eq!(coords.latitude.degrees, 10);
assert_eq!(coords.longitude.minutes, 10);
assert_eq!(coords.altitude, None); 
```

When building a 3D DMS from DMS coordinates, we do not allow
Cardinals to be ommitted, and they must be properly used:

```rust
let lat = DMS::new(10, 20, 45.0, None);
let lon = DMS::new(20, 10, 30.0, Some(Cardinal::West));
let coords = DMS3d::new(lat, lon, None);
assert_eq!(coords.is_ok(), false);
let lat = lat.with_cardinal(Cardinal::South);
let coords = DMS3d::new(lat, lon, None);
assert_eq!(coords.is_ok(), true);
```

Altitude is totally optionnal, it is expressed in meters
and default value 0 corresponds to sea level
```rust
let coords = DMS3d::new(lat, lon, None);
assert_eq!(coords.altitude, None);
let coords = coords.with_altitude(30.0E3);
assert_eq!(coords.altitude, Some(30.0E3);
```

* Altitude

Some basic methods are provided to interact with altitude values
and perform mathematical operations

* `DMS3d::with_altitude(f64)` 
* `DMS3d::add_altitude(f64)`
* `DMS3d::sub_altitude(f64)`
* `DMS3d::with_altitude_feet(f64)`
* `DMS3d::add_altitude_feet(f64) `
* `DMS3d::sub_altitude_feet(f64)` 
