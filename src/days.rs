mod day01;
mod day02;
mod common;

use common::generic_day::GenericDay;

fn run_my_day(day: impl GenericDay) {
    println!("Result of part 1 is {}", day.part1());
    println!("Result of part 2 is {}", day.part2());
}

pub fn run_day(day: u8, input_folder: String) {
    match day {
        1 => run_my_day(day01::Day01::new(input_folder)),
        2 => run_my_day(day02::Day02::new(input_folder)),
        _ => panic!("Day not found!"),
    }
}
