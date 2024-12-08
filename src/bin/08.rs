use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Mul, Sub};
use std::str::FromStr;

advent_of_code::solution!(8);

const fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    if a == 0 || b == 0 {
        a | b
    } else {
        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros();
        b >>= b.trailing_zeros();

        while a != b {
            if a > b {
                a -= b;
                a >>= a.trailing_zeros();
            } else {
                b -= a;
                b >>= b.trailing_zeros();
            }
        }

        a << shift
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn gradient(self) -> Self {
        if self.x == 0 || self.y == 0 {
            return self;
        }

        let divisor = gcd(self.x.abs(), self.y.abs());
        Self {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }

    const fn halved(self) -> Option<Self> {
        if self.x % 2 == 0 && self.y % 2 == 0 {
            Some(Self {
                x: self.x / 2,
                y: self.y / 2,
            })
        } else {
            None
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub<Self> for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Antenna {
    position: Position,
    frequency: char,
}

#[derive(Debug, PartialEq)]
struct SignalTracker {
    position: Position,
    signals: BTreeSet<(Position, char)>,
}

impl SignalTracker {
    const fn new(position: Position) -> Self {
        Self {
            position,
            signals: BTreeSet::new(),
        }
    }

    fn contains(&self, gradient: Position, frequency: char) -> bool {
        if self.signals.contains(&(gradient, frequency)) {
            return true;
        }

        if self.signals.contains(&(Position { x: 0, y: 0 }, frequency)) {
            return true;
        }

        if gradient == (Position { x: 0, y: 0 }) {
            return self.signals.iter().any(|(_g, f)| *f == frequency);
        }

        false
    }

    fn insert(&mut self, antenna: &Antenna) -> bool {
        let gradient = (antenna.position - self.position).gradient();

        if self.contains(gradient, antenna.frequency) {
            return true;
        }

        self.signals.insert((gradient, antenna.frequency));
        false
    }
}

#[derive(Debug, PartialEq)]
struct City {
    antennae: Vec<Antenna>,
    max_x: i32,
    max_y: i32,
}

impl City {
    fn antinode_at(&self, position: Position) -> bool {
        let mut signals = BTreeMap::new();

        self.antennae.iter().any(|antenna| {
            let antenna_pos = antenna.position - position;

            if let Some(other_freq) = signals.get(&(antenna_pos * 2)) {
                if *other_freq == antenna.frequency {
                    return true;
                }
            }
            if let Some(halved) = antenna_pos.halved() {
                if let Some(other_freq) = signals.get(&halved) {
                    if *other_freq == antenna.frequency {
                        return true;
                    }
                }
            }

            signals.insert(antenna_pos, antenna.frequency);
            false
        })
    }

    fn any_distance_antinode_at(&self, position: Position) -> bool {
        let mut signals = SignalTracker::new(position);
        self.antennae.iter().any(|antenna| signals.insert(antenna))
    }

    fn antinode_count(&self, allow_any_distance: bool) -> usize {
        (0..=self.max_x)
            .flat_map(|y| (0..=self.max_y).map(move |x| Position { x, y }))
            .filter(|pos| {
                if allow_any_distance {
                    self.any_distance_antinode_at(*pos)
                } else {
                    self.antinode_at(*pos)
                }
            })
            .count()
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
    fn test_antinode_at() {
        let city = example_city();

        assert_eq!(city.antinode_at(Position { x: 6, y: 0 }), true);
        assert_eq!(city.antinode_at(Position { x: 11, y: 0 }), true);
        assert_eq!(city.antinode_at(Position { x: 3, y: 1 }), true);
        assert_eq!(city.antinode_at(Position { x: 4, y: 2 }), true);
        assert_eq!(city.antinode_at(Position { x: 2, y: 3 }), true);
        assert_eq!(city.antinode_at(Position { x: 9, y: 4 }), true);
        assert_eq!(city.antinode_at(Position { x: 1, y: 5 }), true);
        assert_eq!(city.antinode_at(Position { x: 6, y: 5 }), true);
        assert_eq!(city.antinode_at(Position { x: 3, y: 6 }), true);
        assert_eq!(city.antinode_at(Position { x: 0, y: 7 }), true);
        assert_eq!(city.antinode_at(Position { x: 7, y: 7 }), true);
        assert_eq!(city.antinode_at(Position { x: 10, y: 10 }), true);
        assert_eq!(city.antinode_at(Position { x: 10, y: 11 }), true);

        assert_eq!(city.antinode_at(Position { x: 0, y: 0 }), false);
        assert_eq!(city.antinode_at(Position { x: 5, y: 0 }), false);
        assert_eq!(city.antinode_at(Position { x: 2, y: 4 }), false);
        assert_eq!(city.antinode_at(Position { x: 5, y: 7 }), false);
        assert_eq!(city.antinode_at(Position { x: 9, y: 10 }), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_any_distance_antinode_at() {
        let city = example_city();

        assert_eq!(city.any_distance_antinode_at(Position { x: 0, y: 0 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 1, y: 0 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 6, y: 0 }), true);
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 11, y: 0 }),
            true
        );
        assert_eq!(city.any_distance_antinode_at(Position { x: 1, y: 1 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 3, y: 1 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 8, y: 1 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 2, y: 2 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 4, y: 2 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 5, y: 2 }), true);
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 10, y: 2 }),
            true
        );
        assert_eq!(city.any_distance_antinode_at(Position { x: 2, y: 3 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 3, y: 3 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 7, y: 3 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 4, y: 4 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 9, y: 4 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 1, y: 5 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 5, y: 5 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 6, y: 5 }), true);
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 11, y: 5 }),
            true
        );
        assert_eq!(city.any_distance_antinode_at(Position { x: 3, y: 6 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 6, y: 6 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 0, y: 7 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 5, y: 7 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 7, y: 7 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 2, y: 8 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 8, y: 8 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 4, y: 9 }), true);
        assert_eq!(city.any_distance_antinode_at(Position { x: 9, y: 9 }), true);
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 1, y: 10 }),
            true
        );
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 10, y: 10 }),
            true
        );
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 10, y: 11 }),
            true
        );
        assert_eq!(
            city.any_distance_antinode_at(Position { x: 11, y: 11 }),
            true
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
