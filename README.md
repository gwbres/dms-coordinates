# DMS Coordinates 
Rust package to manipulate D°M'S'' coordinates.

[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

### Basic usage 

Single coordinate: 

```rust
    let dms = DMS::new(40, 43, 50.196_f64, 'N') // NY latitude
        .unwrap();
    prinln!("{:#?}", dms);
    prinln!("{}", dms.get_degrees());
    prinln!("{}", dms.to_decimal_degrees());
    prinln!("{}", dms.get_bearing());
```

Build D°M'S'' from decimal degrees coordinates
```rust
    let dms = DMS::from_decimal_degrees(-73.935242_f64); // NY longitude
    println!("{}", dms.get_seconds());
    assert_eq!(dms.get_bearing(), 'N'); // 'N' bearing
    assert_eq!(dms.get_degrees(), 73); // NY::lon D°
    assert_eq!(dms.get_minutes(), 56); // NY::lon M'
```

3D coordinates

```rust
    let dms = DMS3d::from_decimal_degrees(
        40.730610_f64, // NY
        -73.935242_f64, // NY
        Some(10.0)
    );
    assert_eq!(dms.latitude.get_degrees(), 40); // NY
    assert_eq!(dms.latitude.get_minutes(), 43); // NY
    assert_eq!(dms.latitude.get_bearing(), 'N');
    assert!((dms.latitude.get_seconds() - 50.1960).abs() < 1E-3);
    assert_eq!(dms.longitude.get_degrees(), 73); // NY
    assert_eq!(dms.longitude.get_minutes(), 56); // NY
    assert_eq!(dms.longitude.get_bearing(), 'W');
    assert!((dms.longitude.get_seconds() - 6.8712).abs() < 1E-3);
```
