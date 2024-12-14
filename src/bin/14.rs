use std::cmp::Ordering;
use std::str::FromStr;

advent_of_code::solution!(14);

type Point = (i32, i32);

#[derive(Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    const fn position_after(&self, seconds: i32, width: i32, height: i32) -> Point {
        (
            (self.position.0 + (self.velocity.0 * seconds)).rem_euclid(width),
            (self.position.1 + (self.velocity.1 * seconds)).rem_euclid(height),
        )
    }
}

fn robots_in_quadrants_after(
    robots: &[Robot],
    seconds: i32,
    width: i32,
    height: i32,
) -> (u32, u32, u32, u32) {
    let x_centre = width / 2;
    let y_centre = height / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots {
        let (x, y) = robot.position_after(seconds, width, height);

        match (y.cmp(&y_centre), x.cmp(&x_centre)) {
            (Ordering::Less, Ordering::Less) => top_left += 1,
            (Ordering::Less, Ordering::Greater) => top_right += 1,
            (Ordering::Greater, Ordering::Less) => bottom_left += 1,
            (Ordering::Greater, Ordering::Greater) => bottom_right += 1,
            _ => (),
        }
    }

    (top_left, top_right, bottom_left, bottom_right)
}

#[derive(Debug, PartialEq)]
struct ParseRobotError;

fn parse_point(text: &str) -> Result<Point, ParseRobotError> {
    let (x, y) = text.split_once(',').ok_or(ParseRobotError)?;
    let x = x.parse().map_err(|_| ParseRobotError)?;
    let y = y.parse().map_err(|_| ParseRobotError)?;
    Ok((x, y))
}

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((position, velocity)) = line.split_once(' ') else {
            return Err(ParseRobotError);
        };
        let position = parse_point(&position[2..])?;
        let velocity = parse_point(&velocity[2..])?;
        Ok(Self { position, velocity })
    }
}

fn parse_robots(input: &str) -> Result<Vec<Robot>, ParseRobotError> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let robot = line.parse()?;
        robots.push(robot);
    }
    Ok(robots)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    parse_robots(input).map_or(None, |robots| {
        let (a, b, c, d) = robots_in_quadrants_after(&robots, 100, 101, 103);
        Some(a * b * c * d)
    })
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_robots() -> Vec<Robot> {
        vec![
            Robot {
                position: (0, 4),
                velocity: (3, -3),
            },
            Robot {
                position: (6, 3),
                velocity: (-1, -3),
            },
            Robot {
                position: (10, 3),
                velocity: (-1, 2),
            },
            Robot {
                position: (2, 0),
                velocity: (2, -1),
            },
            Robot {
                position: (0, 0),
                velocity: (1, 3),
            },
            Robot {
                position: (3, 0),
                velocity: (-2, -2),
            },
            Robot {
                position: (7, 6),
                velocity: (-1, -3),
            },
            Robot {
                position: (3, 0),
                velocity: (-1, -2),
            },
            Robot {
                position: (9, 3),
                velocity: (2, 3),
            },
            Robot {
                position: (7, 3),
                velocity: (-1, 2),
            },
            Robot {
                position: (2, 4),
                velocity: (2, -3),
            },
            Robot {
                position: (9, 5),
                velocity: (-3, -3),
            },
        ]
    }

    #[test]
    fn test_parse_robot() {
        let robots = example_robots();
        assert_eq!(
            "p=0,4 v=3,-3".parse(),
            Ok(Robot {
                position: (0, 4),
                velocity: (3, -3)
            })
        );
        assert_eq!(
            "p=6,3 v=-1,-3".parse(),
            Ok(Robot {
                position: (6, 3),
                velocity: (-1, -3)
            })
        );
        assert_eq!(
            "p=7,3 v=-1,2".parse(),
            Ok(Robot {
                position: (7, 3),
                velocity: (-1, 2)
            })
        );
    }

    #[test]
    fn test_parse_robots() {
        assert_eq!(
            parse_robots(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_robots())
        );
    }

    #[test]
    fn test_robot_position_after() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
        };
        assert_eq!(robot.position_after(5, 11, 7), (1, 3));
    }

    #[test]
    fn test_robots_in_quadrants_after() {
        let robots = example_robots();
        assert_eq!(robots_in_quadrants_after(&robots, 100, 11, 7), (1, 3, 4, 1),);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
