mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use common::generic_day::GenericDay;

fn run_my_day(day: impl GenericDay) {
    println!("Result of part 1 is {}", day.part1());
    println!("Result of part 2 is {}", day.part2());
}

pub fn run_day(day: u8, input_folder: String) {
    match day {
        1 => run_my_day(day01::Day01::new(input_folder)),
        2 => run_my_day(day02::Day02::new(input_folder)),
        3 => run_my_day(day03::Day03::new(input_folder)),
        4 => run_my_day(day04::Day04::new(input_folder)),
        5 => run_my_day(day05::Day05::new(input_folder)),
        6 => run_my_day(day06::Day06::new(input_folder)),
        _ => panic!("Day not found!"),
    }
}
