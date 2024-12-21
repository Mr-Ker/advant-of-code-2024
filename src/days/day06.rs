use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

use super::common::generic_day;
use super::common::position::Position;

#[derive(Clone)]
pub struct Day06 {
    input_file: String,
    positions_with_obstacle: HashSet<Position>,
    guard_initial_position: Position,
    grid_size: i64,
}

impl Day06 {
    pub fn new(input_folder: String) -> Day06 {
        let mut day06: Day06 = Day06 {
            input_file: format!("{}/day06.txt", input_folder),
            positions_with_obstacle: HashSet::new(),
            guard_initial_position: Position::new(),
            grid_size: 0,
        };
        day06.parse_input();
        day06
    }

    fn parse_line(&mut self, i: usize, line: String) {
        self.grid_size = line.len() as i64;
        for (j, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    self.positions_with_obstacle.insert(Position {
                        x: j as i64,
                        y: i as i64,
                    });
                }
                '^' => {
                    let position = Position {
                        x: j as i64,
                        y: i as i64,
                    };
                    self.guard_initial_position = position.clone();
                }

                _ => (),
            }
        }
    }

    fn parse_input(&mut self) {
        let mut i = 0;
        let _ = BufReader::new(File::open(&self.input_file).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                self.parse_line(i, line);
                i += 1
            })
            .collect::<Vec<_>>();
    }

    fn is_an_obstacle_present_in_the_direction_faced_by_the_guard(
        &self,
        guard_patrol_path: &mut GuardPatrolPath,
    ) -> bool {
        let mut result = false;
        let mut smallest_diff = self.grid_size * 2;
        for obstacle in self.positions_with_obstacle.iter() {
            let x_diff = obstacle.x - guard_patrol_path.position.x;
            let y_diff = obstacle.y - guard_patrol_path.position.y;
            if (x_diff.abs() == x_diff * guard_patrol_path.direction[0])
                && (y_diff.abs() == y_diff * guard_patrol_path.direction[1])
            {
                if !result || x_diff.abs() + y_diff.abs() < smallest_diff {
                    guard_patrol_path.next_obstacle_position = obstacle.clone();
                    smallest_diff = x_diff.abs() + y_diff.abs();
                }
                result = true;
            }
        }
        if !result {
            let next_obstacle_position: Position = match guard_patrol_path.direction {
                [0, -1] => Position {
                    x: guard_patrol_path.position.x,
                    y: -1,
                },
                [1, 0] => Position {
                    x: self.grid_size,
                    y: guard_patrol_path.position.y,
                },
                [0, 1] => Position {
                    x: guard_patrol_path.position.x,
                    y: self.grid_size,
                },
                [-1, 0] => Position {
                    x: -1,
                    y: guard_patrol_path.position.y,
                },
                _ => panic!("This should not happen!"),
            };
            guard_patrol_path.next_obstacle_position = next_obstacle_position.clone();
        }
        result
    }
}

#[derive(Debug)]
struct GuardPatrolPath {
    visited_positions: HashSet<Position>,
    next_obstacle_position: Position,
    position: Position,
    past_positions: HashSet<Position>,
    direction: [i64; 2],
}

impl GuardPatrolPath {
    fn turn_guard_90degrees(&mut self) {
        match self.direction {
            [0, -1] => self.direction = [1, 0],
            [1, 0] => self.direction = [0, 1],
            [0, 1] => self.direction = [-1, 0],
            [-1, 0] => self.direction = [0, -1],
            _ => panic!("This should not happen!"),
        }
    }

    fn walk_until_next_obstacle(&mut self) {
        if self.position.x == self.next_obstacle_position.x {
            let mut end_position = Position {
                x: self.position.x,
                y: 0,
            };
            let y_range: Range<i64>;

            if self.position.y > self.next_obstacle_position.y {
                y_range = Range {
                    start: self.next_obstacle_position.y + 1,
                    end: self.position.y,
                };
                end_position.y = self.next_obstacle_position.y + 1;
            } else {
                y_range = Range {
                    start: self.position.y,
                    end: self.next_obstacle_position.y,
                };
                end_position.y = self.next_obstacle_position.y - 1;
            }

            for y in y_range {
                self.visited_positions.insert(Position {
                    x: self.position.x,
                    y,
                });
            }

            self.position = end_position.clone();
        } else {
            let x_range: Range<i64>;
            let mut end_position = Position {
                x: 0,
                y: self.position.y,
            };
            if self.position.x > self.next_obstacle_position.x {
                x_range = Range {
                    start: self.next_obstacle_position.x + 1,
                    end: self.position.x,
                };
                end_position.x = self.next_obstacle_position.x + 1;
            } else {
                x_range = Range {
                    start: self.position.x,
                    end: self.next_obstacle_position.x,
                };
                end_position.x = self.next_obstacle_position.x - 1;
            }

            for x in x_range {
                self.visited_positions.insert(Position {
                    x,
                    y: self.position.y,
                });
            }

            self.position = end_position.clone();
        }
        self.turn_guard_90degrees();
    }

    fn walk_until_next_obstacle_part2(&mut self) {
        let end_position: Position;
        if self.position.x == self.next_obstacle_position.x {
            if self.position.y > self.next_obstacle_position.y {
                end_position = Position {
                    x: self.position.x,
                    y: self.next_obstacle_position.y + 1,
                }
            } else {
                end_position = Position {
                    x: self.position.x,
                    y: self.next_obstacle_position.y - 1,
                }
            }
        } else if self.position.x > self.next_obstacle_position.x {
            end_position = Position {
                x: self.next_obstacle_position.x + 1,
                y: self.position.y,
            };
        } else {
            end_position = Position {
                x: self.next_obstacle_position.x - 1,
                y: self.position.y,
            };
        }
        self.position = end_position.clone();
        self.turn_guard_90degrees();
    }
}

impl generic_day::GenericDay for Day06 {
    fn part1(&self) -> i64 {
        let mut guard_patrol_path = GuardPatrolPath {
            visited_positions: HashSet::new(),
            next_obstacle_position: Position::new(),
            position: self.guard_initial_position.clone(),
            past_positions: HashSet::new(),
            direction: [0, -1],
        };
        guard_patrol_path
            .visited_positions
            .insert(self.guard_initial_position.clone());
        guard_patrol_path
            .past_positions
            .insert(self.guard_initial_position.clone());

        loop {
            if self
                .is_an_obstacle_present_in_the_direction_faced_by_the_guard(&mut guard_patrol_path)
            {
                guard_patrol_path.walk_until_next_obstacle();
            } else {
                guard_patrol_path.walk_until_next_obstacle();
                break;
            }
        }

        guard_patrol_path.visited_positions.len() as i64
    }

    fn part2(&self) -> i64 {
        let mut result = 0;
        let mut guard_patrol_path = GuardPatrolPath {
            visited_positions: HashSet::new(),
            next_obstacle_position: Position::new(),
            position: self.guard_initial_position.clone(),
            past_positions: HashSet::new(),
            direction: [0, -1],
        };
        guard_patrol_path
            .visited_positions
            .insert(self.guard_initial_position.clone());
        guard_patrol_path
            .past_positions
            .insert(self.guard_initial_position.clone());

        loop {
            if self
                .is_an_obstacle_present_in_the_direction_faced_by_the_guard(&mut guard_patrol_path)
            {
                guard_patrol_path.walk_until_next_obstacle();
            } else {
                guard_patrol_path.walk_until_next_obstacle();
                break;
            }
        }

        let mut test_day = self.clone();
        guard_patrol_path
            .visited_positions
            .remove(&self.guard_initial_position);

        for position in guard_patrol_path.visited_positions {
            if test_day.positions_with_obstacle.insert(position.clone()) {
                let guard_stuck_in_loop: bool;
                let mut new_guard_patrol_path = GuardPatrolPath {
                    visited_positions: HashSet::new(),
                    next_obstacle_position: Position::new(),
                    position: test_day.guard_initial_position.clone(),
                    past_positions: HashSet::new(),
                    direction: [0, -1],
                };
                new_guard_patrol_path
                    .past_positions
                    .insert(new_guard_patrol_path.position.clone());

                loop {
                    if test_day.is_an_obstacle_present_in_the_direction_faced_by_the_guard(
                        &mut new_guard_patrol_path,
                    ) {
                        let previous_guard_position = new_guard_patrol_path.position.clone();
                        new_guard_patrol_path.walk_until_next_obstacle_part2();

                        if !new_guard_patrol_path
                            .past_positions
                            .insert(new_guard_patrol_path.position.clone())
                            && previous_guard_position != new_guard_patrol_path.position
                        {
                            guard_stuck_in_loop = true;
                            break;
                        }
                    } else {
                        guard_stuck_in_loop = false;
                        break;
                    }
                }

                if guard_stuck_in_loop {
                    result += 1;
                }

                test_day.positions_with_obstacle.remove(&position);
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
        let day06: Day06 = Day06::new(String::from("input_examples"));
        assert_eq!(day06.part1(), 41);
    }

    #[test]
    fn result_part2() {
        let day06: Day06 = Day06::new(String::from("input_examples"));
        assert_eq!(day06.part2(), 6);
    }
}
