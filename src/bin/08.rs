use std::collections::BTreeSet;
use std::iter::successors;
use std::str::FromStr;

advent_of_code::solution!(8);

type Position = (i32, i32);

#[derive(Debug, PartialEq)]
struct Antenna {
    position: Position,
    frequency: char,
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

                if extend {
                    let (a, b) = self.line_corners(start.position, finish.position);
                    if let Some(a) = a {
                        antinodes.insert(a);
                    }
                    if let Some(b) = b {
                        antinodes.insert(b);
                    }
                } else {
                    self.line_points(start.position, finish.position)
                        .for_each(|a| {
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

    const fn line_corners(
        &self,
        start: Position,
        finish: Position,
    ) -> (Option<Position>, Option<Position>) {
        let delta_x = finish.0 - start.0;
        let delta_y = finish.1 - start.1;

        let bottom_left = {
            let x = start.0 - delta_x;
            let y = start.1 - delta_y;
            if x < 0 || x > self.max_x || y < 0 || y > self.max_y {
                None
            } else {
                Some((x, y))
            }
        };

        let top_right = {
            let x = finish.0 + delta_x;
            let y = finish.1 + delta_y;
            if x < 0 || x > self.max_x || y < 0 || y > self.max_y {
                None
            } else {
                Some((x, y))
            }
        };

        (bottom_left, top_right)
    }

    fn line_points(
        &self,
        start: Position,
        finish: Position,
    ) -> impl Iterator<Item = Position> + use<'_> {
        let delta_x = finish.0 - start.0;
        let delta_y = finish.1 - start.1;

        let mut start_x = start.0;
        let mut start_y = start.1;
        loop {
            let candidate_x = start_x - delta_x;
            let candidate_y = start_y - delta_y;
            if candidate_x < 0
                || candidate_x > self.max_x
                || candidate_y < 0
                || candidate_y > self.max_y
            {
                break;
            }
            start_x = candidate_x;
            start_y = candidate_y;
        }

        let x_values = successors(Some(start_x), move |x| {
            let x = x + delta_x;
            if x < 0 || x > self.max_x {
                None
            } else {
                Some(x)
            }
        });
        let y_values = successors(Some(start_y), move |y| {
            let y = y + delta_y;
            if y < 0 || y > self.max_y {
                None
            } else {
                Some(y)
            }
        });
        x_values.zip(y_values)
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
                        position: (x, y),
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
                    position: (8, 1),
                    frequency: '0',
                },
                Antenna {
                    position: (5, 2),
                    frequency: '0',
                },
                Antenna {
                    position: (7, 3),
                    frequency: '0',
                },
                Antenna {
                    position: (4, 4),
                    frequency: '0',
                },
                Antenna {
                    position: (6, 5),
                    frequency: 'A',
                },
                Antenna {
                    position: (8, 8),
                    frequency: 'A',
                },
                Antenna {
                    position: (9, 9),
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
        expected.insert((6, 0));
        expected.insert((11, 0));
        expected.insert((3, 1));
        expected.insert((4, 2));
        expected.insert((10, 2));
        expected.insert((2, 3));
        expected.insert((9, 4));
        expected.insert((1, 5));
        expected.insert((6, 5));
        expected.insert((3, 6));
        expected.insert((0, 7));
        expected.insert((7, 7));
        expected.insert((10, 10));
        expected.insert((10, 11));
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
        expected.insert((0, 0));
        expected.insert((1, 0));
        expected.insert((6, 0));
        expected.insert((11, 0));
        expected.insert((1, 1));
        expected.insert((3, 1));
        expected.insert((8, 1));
        expected.insert((2, 2));
        expected.insert((4, 2));
        expected.insert((5, 2));
        expected.insert((10, 2));
        expected.insert((2, 3));
        expected.insert((3, 3));
        expected.insert((7, 3));
        expected.insert((4, 4));
        expected.insert((9, 4));
        expected.insert((1, 5));
        expected.insert((5, 5));
        expected.insert((6, 5));
        expected.insert((11, 5));
        expected.insert((3, 6));
        expected.insert((6, 6));
        expected.insert((0, 7));
        expected.insert((5, 7));
        expected.insert((7, 7));
        expected.insert((2, 8));
        expected.insert((8, 8));
        expected.insert((4, 9));
        expected.insert((9, 9));
        expected.insert((1, 10));
        expected.insert((10, 10));
        expected.insert((3, 11));
        expected.insert((10, 11));
        expected.insert((11, 11));
        assert_eq!(example_city().antinode_locations(false), expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
