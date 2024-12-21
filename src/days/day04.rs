use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::common::generic_day;
use super::common::position::Position;

struct Diagonal {
    line: String,
    position: Position,
}

pub struct Day04 {
    input_file: String,
    horizontal_word_search: Vec<String>,
    vertical_word_search: Vec<String>,
    diagonal_top_left_to_bottom_right_word_search: Vec<Diagonal>,
    diagonal_bottom_left_to_top_right_word_search: Vec<Diagonal>,
}

impl Day04 {
    pub fn new(input_folder: String) -> Day04 {
        let mut day04: Day04 = Day04 {
            input_file: format!("{}/day04.txt", input_folder),
            horizontal_word_search: Vec::new(),
            vertical_word_search: Vec::new(),
            diagonal_top_left_to_bottom_right_word_search: Vec::new(),
            diagonal_bottom_left_to_top_right_word_search: Vec::new(),
        };
        day04.parse_input();
        day04
    }

    fn transform_horizontal_word_search_input_into_diagonal_top_left_to_bottom_right(&mut self) {
        let mut main_diagonal: String = String::from("");

        for i in 0..self.horizontal_word_search.len() {
            main_diagonal.push(self.horizontal_word_search[i].chars().nth(i).unwrap());
        }
        self.diagonal_top_left_to_bottom_right_word_search
            .push(Diagonal {
                line: main_diagonal,
                position: Position { x: 0, y: 0 },
            });

        for i in 1..self.horizontal_word_search.len() {
            let mut upper_diagonal: String = String::from("");
            let mut lower_diagonal: String = String::from("");

            for j in 0..(self.horizontal_word_search.len() - i) {
                upper_diagonal.push(self.horizontal_word_search[j].chars().nth(i + j).unwrap());
                lower_diagonal.push(self.horizontal_word_search[i + j].chars().nth(j).unwrap());
            }
            self.diagonal_top_left_to_bottom_right_word_search
                .push(Diagonal {
                    line: upper_diagonal,
                    position: Position {
                        x: (i as i64),
                        y: 0,
                    },
                });
            self.diagonal_top_left_to_bottom_right_word_search
                .push(Diagonal {
                    line: lower_diagonal,
                    position: Position {
                        x: 0,
                        y: (i as i64),
                    },
                });
        }
    }

    fn transform_horizontal_word_search_input_into_diagonal_bottom_left_to_top_right(&mut self) {
        let mut main_diagonal: String = String::from("");

        let length = self.horizontal_word_search.len();
        for i in 0..length {
            main_diagonal.push(
                self.horizontal_word_search[length - (i + 1)]
                    .chars()
                    .nth(i)
                    .unwrap(),
            );
        }
        self.diagonal_bottom_left_to_top_right_word_search
            .push(Diagonal {
                line: main_diagonal,
                position: Position {
                    x: 0,
                    y: ((length - 1) as i64),
                },
            });

        for i in 1..length {
            let mut upper_diagonal: String = String::from("");
            let mut lower_diagonal: String = String::from("");

            for j in 0..(length - i) {
                upper_diagonal.push(
                    self.horizontal_word_search[length - (j + 1)]
                        .chars()
                        .nth(i + j)
                        .unwrap(),
                );
                lower_diagonal.push(
                    self.horizontal_word_search[length - (i + j + 1)]
                        .chars()
                        .nth(j)
                        .unwrap(),
                );
            }
            self.diagonal_bottom_left_to_top_right_word_search
                .push(Diagonal {
                    line: lower_diagonal,
                    position: Position {
                        x: 0,
                        y: ((length - i - 1) as i64),
                    },
                });
            self.diagonal_bottom_left_to_top_right_word_search
                .push(Diagonal {
                    line: upper_diagonal,
                    position: Position {
                        x: (i as i64),
                        y: ((length - 1) as i64),
                    },
                });
        }
    }

    fn transform_horizontal_word_search_input_into_vertical(&mut self) {
        let mut vertical_word_search: Vec<String> = self.horizontal_word_search[0]
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();

        for (i, column) in vertical_word_search.iter_mut().enumerate() {
            for j in 1..self.horizontal_word_search.len() {
                column.push(self.horizontal_word_search[j].chars().nth(i).unwrap());
            }
        }

        self.vertical_word_search = vertical_word_search;
    }

    fn parse_input(&mut self) {
        self.horizontal_word_search = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        self.transform_horizontal_word_search_input_into_vertical();
        self.transform_horizontal_word_search_input_into_diagonal_top_left_to_bottom_right();
        self.transform_horizontal_word_search_input_into_diagonal_bottom_left_to_top_right();
    }

    fn search_xmas(&self, direction_word_search: &Vec<String>) -> i64 {
        let mut result: i64 = 0;
        for line in direction_word_search {
            result += line.match_indices("XMAS").count() as i64;
            result += line.match_indices("SAMX").count() as i64;
        }
        result
    }

    fn get_center_position_of_str_in_diagonal(
        &self,
        diagonal: &Diagonal,
        y_translation_factor: i64,
        string_to_search: &str,
    ) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        let indices = diagonal.line.match_indices(string_to_search);
        for indice in indices {
            let position = Position {
                x: diagonal.position.x + (indice.0 as i64) + 1,
                y: diagonal.position.y + ((indice.0 as i64) + 1) * y_translation_factor,
            };
            result.push(position);
        }
        result
    }

    fn get_center_position_of_mas_and_sam_in_diagonal(
        &self,
        diagonal: &Diagonal,
        y_translation_factor: i64,
    ) -> Vec<Position> {
        let mut result =
            self.get_center_position_of_str_in_diagonal(diagonal, y_translation_factor, "MAS");
        result.append(&mut self.get_center_position_of_str_in_diagonal(
            diagonal,
            y_translation_factor,
            "SAM",
        ));
        result
    }

    fn search_mas_in_diagonal_top_left_to_bottom_right(&self) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();

        for diagonal in &self.diagonal_top_left_to_bottom_right_word_search {
            result.append(&mut self.get_center_position_of_mas_and_sam_in_diagonal(diagonal, 1));
        }
        result
    }

    fn search_mas_in_diagonal_bottom_left_to_top_right(&self) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();

        for diagonal in &self.diagonal_bottom_left_to_top_right_word_search {
            result.append(&mut self.get_center_position_of_mas_and_sam_in_diagonal(diagonal, -1));
        }
        result
    }
}

impl generic_day::GenericDay for Day04 {
    fn part1(&self) -> i64 {
        let mut result = self.search_xmas(&self.horizontal_word_search);
        result += self.search_xmas(&self.vertical_word_search);
        result += self.search_xmas(
            &self
                .diagonal_top_left_to_bottom_right_word_search
                .iter()
                .map(|x| x.line.clone())
                .collect::<Vec<String>>(),
        );
        result += self.search_xmas(
            &self
                .diagonal_bottom_left_to_top_right_word_search
                .iter()
                .map(|x| x.line.clone())
                .collect::<Vec<String>>(),
        );
        result
    }

    fn part2(&self) -> i64 {
        let mas_positions_in_diagonal_top_left_to_bottom_right =
            self.search_mas_in_diagonal_top_left_to_bottom_right();
        let mas_positions_in_diagonal_bottom_left_to_top_right =
            self.search_mas_in_diagonal_bottom_left_to_top_right();

        // mas_positions_in_diagonal_top_left_to_bottom_right
        //     .iter()
        //     .zip(&mas_positions_in_diagonal_bottom_left_to_top_right)
        //     .filter(|&(a, b)| a.x == b.x && a.y == b.y)
        //     .count() as i64
        let mut result = 0;
        for position_in_first_list in &mas_positions_in_diagonal_top_left_to_bottom_right {
            for position_in_second_list in &mas_positions_in_diagonal_bottom_left_to_top_right {
                if position_in_first_list.x == position_in_second_list.x
                    && position_in_first_list.y == position_in_second_list.y
                {
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
        let day04: Day04 = Day04::new(String::from("input_examples"));
        assert_eq!(day04.part1(), 18);
    }

    #[test]
    fn result_part2() {
        let day04: Day04 = Day04::new(String::from("input_examples"));
        assert_eq!(day04.part2(), 9);
    }
}
