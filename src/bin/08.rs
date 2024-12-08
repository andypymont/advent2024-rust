use std::collections::BTreeSet;
use std::iter::successors;
use std::str::FromStr;

advent_of_code::solution!(8);

const fn out_of_bounds(value: i32, max: i32) -> bool {
    value < 0 || value > max
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Antenna {
    position: Position,
    frequency: char,
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Position,
    finish: Position,
}

impl Line {
    fn all_points(&self, max_x: i32, max_y: i32) -> impl Iterator<Item = Position> {
        let delta_x = self.finish.x - self.start.x;
        let delta_y = self.finish.y - self.start.y;

        let mut start_x = self.start.x;
        let mut start_y = self.start.y;
        loop {
            let candidate_x = start_x - delta_x;
            let candidate_y = start_y - delta_y;
            if out_of_bounds(candidate_x, max_x) || out_of_bounds(candidate_y, max_y) {
                break;
            }
            start_x = candidate_x;
            start_y = candidate_y;
        }

        let x_values = successors(Some(start_x), move |x| {
            let x = x + delta_x;
            if out_of_bounds(x, max_x) {
                None
            } else {
                Some(x)
            }
        });
        let y_values = successors(Some(start_y), move |y| {
            let y = y + delta_y;
            if out_of_bounds(y, max_y) {
                None
            } else {
                Some(y)
            }
        });
        x_values.zip(y_values).map(|(x, y)| Position { x, y })
    }

    const fn corners(&self, max_x: i32, max_y: i32) -> (Option<Position>, Option<Position>) {
        let delta_x = self.finish.x - self.start.x;
        let delta_y = self.finish.y - self.start.y;

        let bottom_left = {
            let x = self.start.x - delta_x;
            let y = self.start.y - delta_y;
            if out_of_bounds(x, max_x) || out_of_bounds(y, max_y) {
                None
            } else {
                Some(Position { x, y })
            }
        };

        let top_right = {
            let x = self.finish.x + delta_x;
            let y = self.finish.y + delta_y;
            if out_of_bounds(x, max_x) || out_of_bounds(y, max_y) {
                None
            } else {
                Some(Position { x, y })
            }
        };

        (bottom_left, top_right)
    }
}

#[derive(Debug, PartialEq)]
struct City {
    antennae: Vec<Antenna>,
    max_x: i32,
    max_y: i32,
}

impl City {
    fn antinode_locations(&self, extend: bool) -> BTreeSet<Position> {
        let mut antinodes = BTreeSet::new();

        for (ix, start) in self.antennae.iter().enumerate() {
            for finish in &self.antennae[(ix + 1)..] {
                if start.frequency != finish.frequency {
                    continue;
                }

                let line = Line {
                    start: start.position,
                    finish: finish.position,
                };

                if extend {
                    let (a, b) = line.corners(self.max_x, self.max_y);
                    if let Some(a) = a {
                        antinodes.insert(a);
                    }
                    if let Some(b) = b {
                        antinodes.insert(b);
                    }
                } else {
                    line.all_points(self.max_x, self.max_y).for_each(|a| {
                        antinodes.insert(a);
                    });
                }
            }
        }

        antinodes
    }

    fn antinode_count(&self, allow_any_distance: bool) -> usize {
        self.antinode_locations(!allow_any_distance).len()
    }
}

#[derive(Debug, PartialEq)]
struct ParseCityError;

impl FromStr for City {
    type Err = ParseCityError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut antennae = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in (0..).zip(text.lines()) {
            max_y = y;
            for (x, frequency) in (0..).zip(line.chars()) {
                max_x = x.max(max_x);
                if frequency != '.' {
                    antennae.push(Antenna {
                        position: Position { x, y },
                        frequency,
                    });
                }
            }
        }

        Ok(Self {
            antennae,
            max_x,
            max_y,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    City::from_str(input).map_or(None, |city| Some(city.antinode_count(false)))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    City::from_str(input).map_or(None, |city| Some(city.antinode_count(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_city() -> City {
        City {
            antennae: vec![
                Antenna {
                    position: Position { x: 8, y: 1 },
                    frequency: '0',
                },
                Antenna {
                    position: Position { x: 5, y: 2 },
                    frequency: '0',
                },
                Antenna {
                    position: Position { x: 7, y: 3 },
                    frequency: '0',
                },
                Antenna {
                    position: Position { x: 4, y: 4 },
                    frequency: '0',
                },
                Antenna {
                    position: Position { x: 6, y: 5 },
                    frequency: 'A',
                },
                Antenna {
                    position: Position { x: 8, y: 8 },
                    frequency: 'A',
                },
                Antenna {
                    position: Position { x: 9, y: 9 },
                    frequency: 'A',
                },
            ],
            max_x: 11,
            max_y: 11,
        }
    }

    #[test]
    fn test_parse_city() {
        assert_eq!(
            City::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_city())
        )
    }

    #[test]
    fn test_antinodes() {
        let mut expected = BTreeSet::new();
        expected.insert(Position { x: 6, y: 0 });
        expected.insert(Position { x: 11, y: 0 });
        expected.insert(Position { x: 3, y: 1 });
        expected.insert(Position { x: 4, y: 2 });
        expected.insert(Position { x: 10, y: 2 });
        expected.insert(Position { x: 2, y: 3 });
        expected.insert(Position { x: 9, y: 4 });
        expected.insert(Position { x: 1, y: 5 });
        expected.insert(Position { x: 6, y: 5 });
        expected.insert(Position { x: 3, y: 6 });
        expected.insert(Position { x: 0, y: 7 });
        expected.insert(Position { x: 7, y: 7 });
        expected.insert(Position { x: 10, y: 10 });
        expected.insert(Position { x: 10, y: 11 });

        assert_eq!(example_city().antinode_locations(true), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_antinodes_full_line() {
        let mut expected = BTreeSet::new();
        expected.insert(Position { x: 0, y: 0 });
        expected.insert(Position { x: 1, y: 0 });
        expected.insert(Position { x: 6, y: 0 });
        expected.insert(Position { x: 11, y: 0 });
        expected.insert(Position { x: 1, y: 1 });
        expected.insert(Position { x: 3, y: 1 });
        expected.insert(Position { x: 8, y: 1 });
        expected.insert(Position { x: 2, y: 2 });
        expected.insert(Position { x: 4, y: 2 });
        expected.insert(Position { x: 5, y: 2 });
        expected.insert(Position { x: 10, y: 2 });
        expected.insert(Position { x: 2, y: 3 });
        expected.insert(Position { x: 3, y: 3 });
        expected.insert(Position { x: 7, y: 3 });
        expected.insert(Position { x: 4, y: 4 });
        expected.insert(Position { x: 9, y: 4 });
        expected.insert(Position { x: 1, y: 5 });
        expected.insert(Position { x: 5, y: 5 });
        expected.insert(Position { x: 6, y: 5 });
        expected.insert(Position { x: 11, y: 5 });
        expected.insert(Position { x: 3, y: 6 });
        expected.insert(Position { x: 6, y: 6 });
        expected.insert(Position { x: 0, y: 7 });
        expected.insert(Position { x: 5, y: 7 });
        expected.insert(Position { x: 7, y: 7 });
        expected.insert(Position { x: 2, y: 8 });
        expected.insert(Position { x: 8, y: 8 });
        expected.insert(Position { x: 4, y: 9 });
        expected.insert(Position { x: 9, y: 9 });
        expected.insert(Position { x: 1, y: 10 });
        expected.insert(Position { x: 10, y: 10 });
        expected.insert(Position { x: 3, y: 11 });
        expected.insert(Position { x: 10, y: 11 });
        expected.insert(Position { x: 11, y: 11 });

        assert_eq!(example_city().antinode_locations(false), expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
