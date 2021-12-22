# DMS Coordinates 
Rust package to manipulate D°M'S'' coordinates.

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

### Basic usage 

Single coordinate: 

```rust
    let dms = DMS::new(40, 43, 50.196_f64, true); // NY latitude
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
