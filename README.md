# Advent of Code
This repo contains my solutions for the [Advent of Code](http://adventofcode.com/) problems, along with a runner for the solutions. The structure of repo runner was more or less copied from [Linus Färnstrand's solutions for Advent of Code 2016](https://github.com/faern/adventofcode2016).

*   `aoc/` contains the runner, which is the main entry point for running the solutions.
*   `base/` contains (right now) some shared interfaces, and will probably later contain some shared utility structs and enums that can be reused between problems.
*   `yearNNNN/` contains the crate for the solutions to the problems for year NNNN.

## Usage
Make sure you have Rust installed. I have used stable Rust, but my solutions should probably work fine with both stable, beta, and nightly Rust. You should preferrably use [rustup](https://rustup.rs/) to install Rust.

1.  Run `$ cargo build` (if you want the optimized version, run `$ cargo build --release`)
2.  Run `$ target/release/aoc <year> <day> <part> <input file>`, e.g. `$ target/release/aoc 2017 4 2 ../inputs/2017/04` to run year 2017, day 4, part 2, on the file `../inputs/2017/04`.
