# RUST - BATTLESHIP

Its a battleship board game implementation in RUST for leaning purposes.

## Build

```shell
cargo build
```

```shell
cargo run
```

## Adjusting

You can play around with the field size and the ships available by adjusting the `src/config.rs`.

Example:

```rs
pub const GRID_COUNT: usize = 30;

pub const SHIPS: [Ship; 8] = [
    Ship { name: "Carrier", length: 8 },
    Ship { name: "Battleship", length: 5 },
    Ship { name: "Cruiser", length: 5 },
    Ship { name: "Submarine", length: 3 },
    Ship { name: "Destroyer", length: 1 },
    Ship { name: "Large Bomber Ship", length: 15 },
    Ship { name: "Submarine", length: 10 },
    Ship { name: "Flight Carrier", length: 20 },
];
```
