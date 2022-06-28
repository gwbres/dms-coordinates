Cardinal
========

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

Cardinal points

```rust
let n = Cardinal::NorthEast;
assert_eq!(n.is_northern(), true);
assert_eq!(south.is_eastern(), false);
```

It is possible to add or substract an angle in D°:
```
let south = Cardinal::North + 180; // D°
assert_eq!(south, Cardinal::South);
assert_eq!(south.is_southern(), true);
assert_eq!(south.is_eastern(), false);
```

Value is rounded to lowest closest Cardinal
```rust
let south = Cardinal::North + 190; // D°
assert_eq!(south, Cardinal::South);
```

Build Cardinal from Angle in D°, we consider 0° as North

```rust
let n = Cardinal::from_angle(0);
assert_eq!(n.is_northern(), true);
assert_eq!(n.is_southern(), false);

let e = Cardinal::from_angle(90);
assert_eq!(e.is_eastern(), true);
```

