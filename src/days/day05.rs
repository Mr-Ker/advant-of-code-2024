use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::days::common::generic_day;

pub struct Day05 {
    input_file: String,
    page_ordering_rules: HashMap<i64, Vec<i64>>,
    updates: Vec<Vec<i64>>,
}

impl Day05 {
    pub fn new(input_folder: String) -> Day05 {
        let mut day05: Day05 = Day05 {
            input_file: format!("{}/day05.txt", input_folder),
            page_ordering_rules: HashMap::new(),
            updates: Vec::new(),
        };
        day05.parse_input();
        day05
    }

    fn process_page_ordering_rules(&mut self, line: String) {
        let pages = line
            .split("|")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        self.page_ordering_rules
            .entry(pages[0])
            .and_modify(|x| x.push(pages[1]))
            .or_insert(vec![pages[1]]);
    }

    fn process_update(&mut self, line: String) {
        self.updates.push(
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    fn parse_input(&mut self) {
        let lines = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let mut all_page_ordering_rules_processed: bool = false;
        for line in lines {
            if line == "" {
                all_page_ordering_rules_processed = true;
            } else if !all_page_ordering_rules_processed {
                self.process_page_ordering_rules(line);
            } else {
                self.process_update(line);
            }
        }
    }
    fn is_update_correctly_ordered(&self, update: &Vec<i64>) -> bool {
        let mut result = true;

        for (position, page) in update.iter().enumerate() {
            if self.page_ordering_rules.contains_key(page) {
                for not_allowed_previous_page in &self.page_ordering_rules[page] {
                    if update[0..position].contains(&not_allowed_previous_page) {
                        result = false;
                        break;
                    }
                    if !result {
                        break;
                    }
                }
            }
        }
        result
    }

    fn correct_update(&self, update: &Vec<i64>) -> Vec<i64> {
        let mut corrected_update = update.clone();

        while !self.is_update_correctly_ordered(&corrected_update) {
            let mut positions_to_swap = vec![0, 0];
            for (position, page) in corrected_update.iter().enumerate() {
                if self.page_ordering_rules.contains_key(page) {
                    for not_allowed_previous_page in &self.page_ordering_rules[page] {
                        if corrected_update[0..position].contains(&not_allowed_previous_page) {
                            let position_to_swap_with = corrected_update
                                .iter()
                                .position(|p| p == not_allowed_previous_page)
                                .unwrap();
                            positions_to_swap[0]=position;
                            positions_to_swap[1]=position_to_swap_with;
                            break;
                        }
                    }
                }
            }
            corrected_update.swap(positions_to_swap[0], positions_to_swap[1]);
        }
        corrected_update
    }
}

impl generic_day::GenericDay for Day05 {
    fn part1(&self) -> i64 {
        let mut result = 0;
        for update in &self.updates {
            if self.is_update_correctly_ordered(update) {
                result += update[update.len() / 2];
            }
        }
        result
    }

    fn part2(&self) -> i64 {
        let mut result = 0;
        for update in &self.updates {
            if !self.is_update_correctly_ordered(update) {
                let corrected_update = self.correct_update(update);
                result += corrected_update[corrected_update.len() / 2];
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
        let day05: Day05 = Day05::new(String::from("input_examples"));
        assert_eq!(day05.part1(), 143);
    }

    #[test]
    fn result_part2() {
        let day05: Day05 = Day05::new(String::from("input_examples"));
        assert_eq!(day05.part2(), 123);
    }
}
