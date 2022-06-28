D°M'S"
======

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

`D°M'S"` represents an angle in sexagesimal format, where
* D° for degrees, positive integer, 0 <= D° < 360° in case
no cardinal associated to it
* M' for minutes, 0 <= M' < 60, 60'=1D°
* S" for fractionnal seconds, double precision, 0 <= S" < 60, 60"=1'
* Optionnal Cardinal point, among "N", "S", "E", "W"

```rust
let dms = dms_coordinates::DMS::new(40, 43, 50.196, None);
assert_eq!(dms.degrees, 40);
assert_eq!(dms.minutes, 43);
assert_float_relative_eq!(dms.seconds, 50.196, 1E-6); 
assert_eq!(dms.cardinal, None);
```

A couple other methods are available

```rust
// Total amount of S" (unit basis) in D°M'S"
let dms = DMS::new(0, 10, 55.0);
total = dms.total_seconds(); 
assert_eq!(total, 10*60+55);

// Build D°M'S" from total amount of seconds
let d = DMS::from_seconds(total);
assert_eq!(dms, d);
```

D°M'S" supports overflow and wrapps internal so D°, M' and S"
remain within predefined ranges.

For instance, overflows are allowed when building a D°M'S":
```rust
let dms = DMS::new(10, 61, 0.0);
assert_eq!(dms.degrees, 11); // 61' = 1°+1'
assert_eq!(dms.minutes,  1);
```

It is possible to cast a D°M'S" angle into
* a `f64` number: you get the total amount of seconds, with fractionnal and double precision
* a `f32` number: you get the total amount of seconds, with fractionnal part and precision loss (6 digits)
* `u64`, `u32`, `u16`, `u8` : you get the total amount of seconds, but fractionnal part is lost

```rust
let dms = DMS::new(0, 1, 30.33);
let secs : f64 = dms.into();
assert_eq!(secs, 60.0+30.33);

let secs : u32 = dms.into();
assert_eq!(secs, 1*60+30); // fractionnal part is lost
```

## Arithmetics

D°M'S" supports all basic mathematical operations, for convenient
operations performed in navigation (like course and angle calculations).

It is possible to perform (*) (/) (+) and (-) 

* between two D°M'S" angles
* between one D°M'S" angle and a number

### Operations with another angle

D°M'S" (+) D°M'S"
```rust
let d0 = DMS::new(0, 10, 55.0);
let d1 = DMS::new(0, 20, 5.0);
let d = d0 + d1;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 31);
assert_eq!(d.seconds, 0.0);
```

D°M'S" (-) D°M'S"
```rust
let d0 = DMS::new(0, 20, 55.0);
let d1 = DMS::new(0, 3, 5.0);
let d = d0 - d1;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 17);
assert_eq!(d.seconds, 50.0);
```

* D°M'S" (*) D°M'S" is not feasible
* D°M'S" (/) D°M'S" is not feasible

I doubt these have a practical use

### Operations with a number

D°M'S" (+) ɑ: we consider ɑ in seconds

```rust
// add 5"
let d = DMS::new(0, 20, 55.0) + 5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 21);
assert_eq!(d.seconds, 0.0);

// add 10" with fractionnal part
let d = DMS::new(0, 59, 50.0) + 10.3;
assert_eq!(d.degrees, 1);
assert_eq!(d.minutes, 0);
assert_eq!(d.seconds, 0.3);
```

D°M'S" (-) ɑ: we consider ɑ in seconds

```rust
// substract 5"
let d = DMS::new(0, 20, 55.0) + 5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 20);
assert_eq!(d.seconds, 50.0);

// substract 10" with fractionnal part
let d = DMS::new(0, 59, 9.0) - 10.3;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 58);
assert_eq!(d.seconds, 59.7);
```

D°M'S" (*) ɑ: we consider ɑ as an integral ratio 

```rust
// multiply by factor of 5
let d = DMS::new(0, 1, 10.0) * 5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 5);
assert_eq!(d.seconds, 50.0);

// multiply by factor of 5.5
let d = DMS::new(0, 1, 10.0) * 5.5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 5);
assert_eq!(d.seconds, 55.0);

// divide by 2
let d = DMS::new(0, 1, 10.0) *0.5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 0);
assert_eq!(d.seconds, 5.0);
```

D°M'S" (/) ɑ: we consider ɑ as an integral ratio 

```rust
// divide by factor of 5
let d = DMS::new(0, 1, 10.0) / 5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 0);
assert_eq!(d.seconds, 2.0);

// divide by fractionnal factor 
let d = DMS::new(0, 1, 10.0) / 5.5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 1.8181);

// multiply by 2 
let d = DMS::new(0, 1, 10.0) /0.5;
assert_eq!(d.degrees, 0);
assert_eq!(d.minutes, 2);
assert_eq!(d.seconds, 20.0);
```
