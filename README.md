# DMS Coordinates 

[![Rust](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/dms-coordinates/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/dms-coordinates/badge.svg)](https://docs.rs/dms-coordinates/badge.svg)
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)   
[![crates.io](https://img.shields.io/crates/v/dms-coordinates.svg)](https://crates.io/crates/dms-coordinates)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/dms-coordinates/blob/main/LICENSE-MIT) 

Rust Crate for D°M'S" coordinates manipulation, used in navigation :sailboat: :airplane: :ship:

This crate exposes several structures

* [Cardinal points](doc/cardinal.md) of a compass rose
* [D° M' S"](doc/dms.md) to represent an angle as Degrees, Minutes and fractionnal seconds,
so called "sexagesimal" format, with an optionnal Cardinal. This object
can be used to represent Latitude / Longitude angles
* [DMS 3D](doc/dms3d.md) comprises a Latitude, a Longitude angle and optionnal altitude
(3D coordinates)

## Features

* std: this lib supports "no-std" by default
* serde: enable `DMS`, `DMS3d`, `Cardinal` serdes ops, requires "std"
* gpx: enables cast from Waypoint to DMS3D, requires "std"

## Other solutions :crab:

Other solutions exist nowadays in Rust to deal with sexagesimal coordiantes.  
The `geodesy` library is one of them, and most likely more consistent than this library.  
The sole interest you can find here is the few dependencies that this library has. 
