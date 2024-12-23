use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::common::generic_day;

struct EquationInput {
    result: i64,
    inputs: Vec<i64>,
}

pub struct Day07 {
    input_file: String,
    equation_inputs: Vec<EquationInput>,
}

impl Day07 {
    pub fn new(input_folder: String) -> Day07 {
        let mut day07: Day07 = Day07 {
            input_file: format!("{}/day07.txt", input_folder),
            equation_inputs: Vec::new(),
        };
        day07.parse_input();
        day07
    }

    fn parse_line(&mut self, line: &str) {
        let values = line.split(":").collect::<Vec<_>>();
        self.equation_inputs.push(EquationInput {
            result: values[0].parse().unwrap(),
            inputs: values[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>(),
        });
    }

    fn parse_input(&mut self) {
        let _ = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .map(|line| self.parse_line(&line))
            .collect::<Vec<_>>();
    }

    fn evaluate_which_equations_could_be_true(
        &self,
        operations: Vec<&dyn Fn(i64, i64) -> i64>,
    ) -> i64 {
        let mut sum_of_potential_true_equations = 0;
        for equation_input in &self.equation_inputs {
            let mut results = vec![equation_input.inputs[0]];
            for input in equation_input.inputs[1..].iter() {
                let mut tmp_results: Vec<i64> = Vec::new();
                for result in results {
                    for operation in &operations {
                        tmp_results.push(operation(result, *input));
                    }
                }
                results = tmp_results.clone();
            }

            for result in results {
                if equation_input.result == result {
                    sum_of_potential_true_equations += equation_input.result;
                    break;
                }
            }
        }
        sum_of_potential_true_equations
    }
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn concat(a: i64, b: i64) -> i64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

impl generic_day::GenericDay for Day07 {
    fn part1(&self) -> i64 {
        self.evaluate_which_equations_could_be_true(vec![&add, &multiply])
    }

    fn part2(&self) -> i64 {
        self.evaluate_which_equations_could_be_true(vec![&add, &multiply, &concat])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::GenericDay;

    #[test]
    fn result_part1() {
        let day07: Day07 = Day07::new(String::from("input_examples"));
        assert_eq!(day07.part1(), 3749);
    }

    #[test]
    fn result_part2() {
        let day07: Day07 = Day07::new(String::from("input_examples"));
        assert_eq!(day07.part2(), 11387);
    }
}
