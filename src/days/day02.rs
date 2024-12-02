use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::days::common::generic_day;

pub struct Day02 {
    input_file: String,
    reports: Vec<Vec<i64>>,
}

impl Day02 {
    pub fn new(input_folder: String) -> Day02 {
        let mut day02: Day02 = Day02 {
            input_file: format!("{}/day02.txt", input_folder),
            reports: Vec::new(),
        };
        day02.parse_input();
        day02
    }

    fn parse_line(&self, line: String) -> Vec<i64> {
        line.split_whitespace()
            .map(|y| y.parse().unwrap())
            .collect::<Vec<i64>>()
    }

    fn parse_input(&mut self) {
        self.reports = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| self.parse_line(x.unwrap()))
            .collect::<Vec<_>>();
    }

    fn are_levels_gradually_changing(&self, diff: i64) -> bool {
        diff.abs() <= 3
    }

    fn are_levels_still_changing_in_same_direction(&self, diff: i64, sign: i64) -> bool {
        diff * sign > 0
    }

    fn is_report_safe(&self, report: Vec<i64>) -> bool {
        let len = report.len();
        let sign = report[1] - report[0];
        let mut is_report_safe = true;
        for i in 1..len {
            let diff = report[i] - report[i - 1];

            if !self.are_levels_still_changing_in_same_direction(diff, sign)
                || !self.are_levels_gradually_changing(diff)
            {
                is_report_safe = false;
                break;
            }
        }
        is_report_safe
    }

    fn try_if_report_is_safe_by_removing_one_level(&self, report: &Vec<i64>) -> bool {
        let len = report.len();
        let mut is_report_safe = false;

        for i in 0..len {
            let mut test_levels = report.clone();
            // remove the i-th level from the report
            test_levels.drain(i..i + 1);
            if self.is_report_safe(test_levels) {
                is_report_safe = true;
                break;
            }
        }

        is_report_safe
    }
}

impl generic_day::GenericDay for Day02 {
    fn part1(&self) -> i64 {
        let mut result: i64 = 0;
        for report in self.reports.iter() {
            if self.is_report_safe(report.clone()) {
                result += 1;
            }
        }
        result
    }

    fn part2(&self) -> i64 {
        let mut result: i64 = 0;
        for levels in self.reports.iter() {
            if self.is_report_safe(levels.clone()) {
                result += 1;
            } else {
                if self.try_if_report_is_safe_by_removing_one_level(&levels) {
                    result += 1;
                }
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
        let day02: Day02 = Day02::new(String::from("input_examples"));
        assert_eq!(day02.part1(), 2);
    }

    #[test]
    fn result_part2() {
        let day02: Day02 = Day02::new(String::from("input_examples"));
        assert_eq!(day02.part2(), 4);
    }
}
