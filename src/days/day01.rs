use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::common::generic_day;

pub struct Day01 {
    input_file: String,
    first_list: Vec<i64>,
    second_list: Vec<i64>,
}

impl Day01 {
    pub fn new(input_folder: String) -> Day01 {
        let mut day01: Day01 = Day01 {
            input_file: format!("{}/day01.txt", input_folder),
            first_list: Vec::new(),
            second_list: Vec::new(),
        };
        day01.parse_input();
        day01
    }

    fn parse_line(&mut self, line: &str) {
        let values = line
            .split_whitespace()
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        self.first_list.push(values[0]);
        self.second_list.push(values[1]);
    }

    fn parse_input(&mut self) {
        let _ = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .map(|line| self.parse_line(&line))
            .collect::<Vec<_>>();
    }
}

impl generic_day::GenericDay for Day01 {
    fn part1(&self) -> i64 {
        let mut sorted_first_list: Vec<i64> = self.first_list.clone();
        let mut sorted_second_list: Vec<i64> = self.second_list.clone();
        sorted_first_list.sort();
        sorted_second_list.sort();

        let mut result: i64 = 0;
        let len = sorted_first_list.len();

        for i in 0..len {
            let distance: i64 = sorted_first_list[i] - sorted_second_list[i];
            result += distance.abs();
        }

        result
    }

    fn part2(&self) -> i64 {
        let mut result = 0;

        for item in self.first_list.iter() {
            let count = self.second_list.iter().filter(|&n| *n == *item).count();
            result += (count as i64) * item;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::GenericDay;

    #[test]
    fn result_part1() {
        let day01: Day01 = Day01::new(String::from("input_examples"));
        assert_eq!(day01.part1(), 11);
    }

    #[test]
    fn result_part2() {
        let day01: Day01 = Day01::new(String::from("input_examples"));
        assert_eq!(day01.part2(), 31);
    }
}
