use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::common::generic_day;

pub struct Day03 {
    input_file: String,
    instructions: Vec<String>,
}

impl Day03 {
    pub fn new(input_folder: String) -> Day03 {
        let mut day03: Day03 = Day03 {
            input_file: format!("{}/day03.txt", input_folder),
            instructions: Vec::new(),
        };
        day03.parse_input();
        day03
    }

    fn parse_input(&mut self) {
        let program = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        let instruction = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

        for line in program {
            self.instructions.append(
                &mut instruction
                    .find_iter(&line)
                    .map(|m| String::from(m.as_str()))
                    .collect::<Vec<String>>(),
            );
        }
    }
}

impl generic_day::GenericDay for Day03 {
    fn part1(&self) -> i64 {
        let mut result: i64 = 0;

        for instruction in &self.instructions {
            if instruction.starts_with("mul") {
                let splitted_inst = instruction.split(&['(', ',', ')'][..]).collect::<Vec<_>>();
                result += splitted_inst[1].parse::<i64>().unwrap()
                    * splitted_inst[2].parse::<i64>().unwrap();
            }
        }

        result
    }

    fn part2(&self) -> i64 {
        let mut result: i64 = 0;
        let mut mul_enabled = true;

        for instruction in &self.instructions {
            if instruction.starts_with("mul") && mul_enabled {
                let splitted_inst = instruction.split(&['(', ',', ')'][..]).collect::<Vec<_>>();
                result += splitted_inst[1].parse::<i64>().unwrap()
                    * splitted_inst[2].parse::<i64>().unwrap();
            } else if instruction.starts_with("don't(") {
                mul_enabled = false;
            } else if instruction.starts_with("do(") {
                mul_enabled = true;
            }
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
        let day03: Day03 = Day03::new(String::from("input_examples"));
        assert_eq!(day03.part1(), 161);
    }

    #[test]
    fn result_part2() {
        let day03: Day03 = Day03::new(String::from("input_examples"));
        assert_eq!(day03.part2(), 48);
    }
}
