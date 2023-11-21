# Advent Of Code in RUST 🦀

[![Rust](https://github.com/ismaelJimenez/advent-of-code/actions/workflows/rust.yml/badge.svg)](https://github.com/ismaelJimenez/advent-of-code/actions/workflows/rust.yml)

Learning Rust by implementing solutions for [Advent of Code](https://adventofcode.com/) problems.

## How to run tests for all exercises

Simply execute:

```bash
cargo test
```

If you want to get the result for only one project run:

```bash
cargo test --package y2022day01
```

## How to get the results

To get for example the result of part 1 of day 1, run:

```bash
cargo run --package y2022day01 --bin part-02
```
## Create a new exercise

Cd into the specific **year folder** (e.g. `2022`) and run:

```bash
cargo new --lib day-NN
```

Replace `NN` with the number of day for the given year. For instance:


```bash
cargo new --lib day-01
```

Then add part-01 and part-02 binaries.

Finally add the new subproject in the workspace by editing the main [`Cargo.toml`](/Cargo.toml). For instance, assuming you just created `2022/day-10`:


```toml
[workspace]
members = [
  "2022/day-01",
  # ...
  "2022/day-10" # <- new entry
]
```

## How to run benchmarks

To get for example the benchmark result ofday 1, run:

```bash
cargo bench --package y2022day01
```

## License

Licensed under [MIT License](LICENSE).

## Others

See also `https://github.com/lmammino/rust-advent`