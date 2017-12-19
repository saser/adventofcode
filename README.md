# Advent of Code 2017
This repo contains my solutions for the [Advent of Code 2017](http://adventofcode.com/2017) problems, along with a runner for the solutions. The structure of repo runner was more or less copied from [Linus Färnstrand's solutions for Advent of Code 2016](https://github.com/faern/adventofcode2016).

*   `aoc/` contains the runner, which is the main entry point for running the solutions.
*   `base/` contains (right now) some shared interfaces, and will probably later contain some shared utility structs and enums that can be reused between problems.
*   `dayXX/` contains the crate for the solution to the problems for day XX.

## Usage
Make sure you have Rust installed. I have used stable Rust, but my solutions should probably work fine with both stable, beta, and nightly Rust. You should preferrably use [rustup](https://rustup.rs/) to install Rust.

1.  Go into the `aoc/` folder
2.  Run `$ cargo build` (if you want the optimized version, run `$ cargo build --release`)
3.  Run `$ ./target/release/aoc <day> <part> <input file>`, e.g. `$ ./target/release/aoc 4 2 ../inputs/04` to run day 4, part 2, on the file `../inputs/04`.
