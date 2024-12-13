# Advent of Code 2024

![Build Status](https://github.com/Mr-Ker/advent-of-code-2024/actions/workflows/rust.yml/badge.svg)

The provided solutions for the 2024 edition of Advent of Code are implemented in
Rust.

## Run for a specific day

`cargo run -- --day N --input-folder <some_folder>`

The program will look for a text file named `dayXX.txt`, where `XX` is the day
number led by `0` for days `< 9`, within the provided `<input_folder>` and use
it as input file.

## Run the tests

`cargo test`
