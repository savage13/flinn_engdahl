flinn_engdahl
=============

[![Documentation](https://docs.rs/flinn_engdahl/badge.svg)](https://docs.rs/flinn_engdahl)

Flinn-Engdahl Seismic And Geographic Regionalization - Rust Implementation

Converts a (latitude, longitude) location into a named region of the world

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
flinn_engdahl = "0.1.0"
```

## Example

```rust
use flinn_engdahl as fe;
let name = fe::region(-77.845753, 166.675927).unwrap();
assert_eq!(name, "VICTORIA LAND, ANTARCTICA");
```

## License

This version was ported from agg-2.4 (BSD 3-Clause) and is released
under the BSD 2-Clause License.


