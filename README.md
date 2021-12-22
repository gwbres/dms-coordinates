# DMS Coordinates 
Rust package to manipulate D°M'S'' coordinates.

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)

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
    let dms = DMS::from_decimal_degrees(-73.9893_f64); // NY longitude
    assert_eq!(dms.get_bearing(), 'N'); // 'N' bearing
    assert_eq!(dms.get_degrees(), 73); // NY::lon D°
    assert_eq!(dms.get_minutes(), 56); // NY::lon M'
    assert!((dms.get_seconds() - 6.871).abs() < 1E-3)
```
