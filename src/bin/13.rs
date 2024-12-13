use std::str::FromStr;

advent_of_code::solution!(13);

type Point = (i64, i64);

#[derive(Debug, PartialEq)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

const DISTANT_CLAW: i64 = 10_000_000_000_000;

impl Machine {
    const fn distant(&self) -> Self {
        Self {
            prize: (self.prize.0 + DISTANT_CLAW, self.prize.1 + DISTANT_CLAW),
            ..*self
        }
    }

    const fn win_prize(&self) -> Option<i64> {
        let denom = (self.a.1 * self.b.0) - (self.a.0 * self.b.1);
        if denom == 0 {
            return None;
        }

        let a = ((self.b.0 * self.prize.1) - (self.b.1 * self.prize.0)) / denom;
        let b = ((self.a.1 * self.prize.0) - (self.a.0 * self.prize.1)) / denom;

        if (a * self.a.0) + (b * self.b.0) == self.prize.0
            && (a * self.a.1) + (b * self.b.1) == self.prize.1
        {
            Some((a * 3) + b)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Arcade {
    machines: Vec<Machine>,
}

impl Arcade {
    fn distant(&self) -> Self {
        Self {
            machines: self.machines.iter().map(Machine::distant).collect(),
        }
    }

    fn win_all_prizes(&self) -> i64 {
        self.machines
            .iter()
            .map(|machine| machine.win_prize().unwrap_or(0))
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseArcadeError;

fn parse_point(text: &str) -> Result<Point, ParseArcadeError> {
    let (_prefix, coords) = text.split_once(": ").ok_or(ParseArcadeError)?;
    let (x, y) = coords.split_once(", ").ok_or(ParseArcadeError)?;
    let x = x[2..].parse().map_err(|_| ParseArcadeError)?;
    let y = y[2..].parse().map_err(|_| ParseArcadeError)?;
    Ok((x, y))
}

impl FromStr for Machine {
    type Err = ParseArcadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let a = lines.next().map_or(Err(ParseArcadeError), parse_point)?;
        let b = lines.next().map_or(Err(ParseArcadeError), parse_point)?;
        let prize = lines.next().map_or(Err(ParseArcadeError), parse_point)?;
        Ok(Self { a, b, prize })
    }
}

impl FromStr for Arcade {
    type Err = ParseArcadeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut machines = Vec::new();

        for chunk in input.split("\n\n") {
            let machine = chunk.parse()?;
            machines.push(machine);
        }

        Ok(Self { machines })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    Arcade::from_str(input).map_or(None, |arcade| Some(arcade.win_all_prizes()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    Arcade::from_str(input).map_or(None, |arcade| Some(arcade.distant().win_all_prizes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_arcade() -> Arcade {
        Arcade {
            machines: vec![
                Machine {
                    a: (94, 34),
                    b: (22, 67),
                    prize: (8400, 5400),
                },
                Machine {
                    a: (26, 66),
                    b: (67, 21),
                    prize: (12748, 12176),
                },
                Machine {
                    a: (17, 86),
                    b: (84, 37),
                    prize: (7870, 6450),
                },
                Machine {
                    a: (69, 23),
                    b: (27, 71),
                    prize: (18641, 10279),
                },
            ],
        }
    }

    #[test]
    fn test_parse_arcade() {
        assert_eq!(
            Arcade::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_arcade())
        )
    }

    #[test]
    fn test_win_prize() {
        let arcade = example_arcade();

        assert_eq!(arcade.machines[0].win_prize(), Some(280));
        assert_eq!(arcade.machines[1].win_prize(), None);
        assert_eq!(arcade.machines[2].win_prize(), Some(200));
        assert_eq!(arcade.machines[3].win_prize(), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_win_distant_prize() {
        let arcade = example_arcade().distant();

        assert_eq!(arcade.machines[0].win_prize(), None);
        assert_eq!(arcade.machines[1].win_prize(), Some(459_236_326_669));
        assert_eq!(arcade.machines[2].win_prize(), None);
        assert_eq!(arcade.machines[3].win_prize(), Some(416_082_282_239));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875_318_608_908));
    }
}
