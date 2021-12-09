# Rust

Solutions for Advent of Code 2021, written in Rust.

## Running

Assuming Rust is installed:

```bash
# From this repository's root
cargo run --release --package day_<day num>
```

## Testing

```bash
# From this repository's root
cargo test --release --package day_<day num>
```

## Benchmarking

As of Rust 1.59.0, this requires the use of the `nightly` toolchain.

```bash
# From this repository's root
cargo bench --package day_<day num>
```
