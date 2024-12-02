mod day01;
mod day02;

pub fn run_day(day: u8, input_folder: String) {
    let mut result_part1: i64 = -1;
    let mut result_part2: i64 = -1;
    match day {
        1 => {
            let day01: day01::Day01 = day01::Day01::new(input_folder);
            result_part1 = day01.part1();
            result_part2 = day01.part2();
        }
        2 => {
            let day02: day02::Day02 = day02::Day02::new(input_folder);
            result_part1 = day02.part1();
            result_part2 = day02.part2();
        }
        _ => println!("Day not implemented yet"),
    }
    println!("Result of part 1 is {}", result_part1);
    println!("Result of part 2 is {}", result_part2)
}
