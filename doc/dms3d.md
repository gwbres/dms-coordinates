3D D°M'S"
=========

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

DMS 3D integrates two structures
* DMS1D to represent either a Latitude or a Longitude angle in D°M'S"
* DMS3D to represent 3D coordinates on Earth

## DMS1D (Latitude / Longitude)

DMS1D represents either a Latitude or a Longitude,
it comprises
* an angle expressed in sexagesimal format D°M'S"
* a cardinal: North, South for latitude and East, West for longitude

```rust
let lat = DMS1D::new(10, 20, Cardinal::North);
assert_eq!(lat.degrees, 10);
assert_eq!(lat.cardinal, Cardinal::North);
```

* Latitude: correct angle |λ| <= 90°
* Longitude: correct angle |ϕ| <= 180°

Like the underlying D°M'S" object, we manage overflowing
and wrapp properly:

```rust
// almost overflowing
let lon = DMS1D::new(179, 59, 59.9, Cardinal::West);
assert_eq!(lon.degrees, 179);
assert_eq!(lon.minutes, 59);
assert_eq!(lon.seconds, 59.9);
assert_eq!(lon.cardinal, Cardinal::West);

// add 1" -> overflow
let lon = lon +1;
assert_eq!(lon.degrees, 180);
assert_eq!(lon.minutes, 0);
assert_eq!(lon.seconds, 0.9);
assert_eq!(lon.cardinal, Cardinal::East);
```

## Decimal Degrees

It is most convenient to use Decimal Degrees representation
when dealing with Latitude/Longitude coordinates.

It is possible to build a DMS1D object from coordinates
defined in this format:

```rust
#TODO
let lat = DMS1D::from_ddeg_latitude(3.45534);
let lon = DMS1D::from_ddeg_longitude(90.0);
``̀ 

## `from_str()` special method

It is possible to parse Latitude or Longitude coordinates
from a string description, but the provided descriptor must follow
standard formats properly

* "+DDD.DD" Latitude D° but M = S = 0 ; no fractionnal part
* "-DDD.DD" Latitude D° but M = S = 0 ; no fractionnal part
* "+DDMM.M" Latitude D°M' but S = 0 ; no fractionnal part
* "-DDMM.M" Latitude D°M' but S = 0 ; no fractionnal part
* "+DDMMMSS.SS" Latitude D°M'S" with fractionnal part
* "-DDMMMSS.SS" Latitude D°M'S" with fractionnal part

+/- sign bit describe a North or a South latitude respectively.  
Unlike specified standards, we support the (+) sign to be ommited.

* "+DD.DD" Longitude D° but M = S = 0 ; no fractionnal part
* "-DD.DD" Longitude D° but M = S = 0 ; no fractionnal part
* "+DDMM.M" Longitude D°M' but S = 0 ; no fractionnal part
* "-DDMM.M" Longitude D°M' but S = 0 ; no fractionnal part
* "+DDMMMSS.SS" Longitude D°M'S" with fractionnal part
* "-DDMMMSS.SS" Longitude D°M'S" with fractionnal part

+/- sign bit describe a North or a South latitude respectively.  
Unlike specified standards, we support the (+) sign to be ommited.

```rust
let lat = DMS1D::from_str("+13.33").unwrap();
assert_eq!();
TODO
```

## `to_str()` special method

It is possible to format given Latitude or Longitude coordinates
to string. If user defines a format description, we will use it,
otherwise, +/-DDMMSS.SS and +/-DDMMSSS.SS is prefered, as it allows
fractionnal part to be described

```rust
TODO
```

## DMS3D (3D coordinates)

DMS3D represent 3D coordinates which comprises
* a latitude coordinates in sexagesimal format
* a longitude coordinates in sexagesimal format
* an optionnal altitude / depth in meter


