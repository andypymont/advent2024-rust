use std::str::FromStr;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl Machine {
    fn find_a_presses(&self, b: u32) -> Option<u32> {
        let rem = self.prize.x.checked_sub(b * self.button_b.x)?;
        if rem % self.button_a.x == 0 {
            let a = rem / self.button_a.x;
            if (a * self.button_a.y) + (b * self.button_b.y) == self.prize.y {
                Some(a)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn win_prize(&self) -> Option<u32> {
        (1..=100)
            .filter_map(|b| self.find_a_presses(b).map(|a| (a * 3) + b))
            .min()
    }
}

#[derive(Debug, PartialEq)]
struct Arcade {
    machines: Vec<Machine>,
}

impl Arcade {
    fn win_all_prizes(&self) -> u32 {
        self.machines
            .iter()
            .map(|machine| machine.win_prize().unwrap_or(0))
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseArcadeError;

impl FromStr for Point {
    type Err = ParseArcadeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_prefix, coords) = line.split_once(": ").ok_or(ParseArcadeError)?;
        let (x, y) = {
            let (x, y) = coords.split_once(", ").ok_or(ParseArcadeError)?;

            let x = x[2..].parse().map_err(|_| ParseArcadeError)?;
            let y = y[2..].parse().map_err(|_| ParseArcadeError)?;

            (x, y)
        };

        Ok(Self { x, y })
    }
}

impl FromStr for Machine {
    type Err = ParseArcadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let button_a = lines.next().map_or(Err(ParseArcadeError), str::parse)?;
        let button_b = lines.next().map_or(Err(ParseArcadeError), str::parse)?;
        let prize = lines.next().map_or(Err(ParseArcadeError), str::parse)?;
        Ok(Self {
            button_a,
            button_b,
            prize,
        })
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
pub fn part_one(input: &str) -> Option<u32> {
    Arcade::from_str(input).map_or(None, |arcade| Some(arcade.win_all_prizes()))
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_arcade() -> Arcade {
        Arcade {
            machines: vec![
                Machine {
                    button_a: Point { x: 94, y: 34 },
                    button_b: Point { x: 22, y: 67 },
                    prize: Point { x: 8400, y: 5400 },
                },
                Machine {
                    button_a: Point { x: 26, y: 66 },
                    button_b: Point { x: 67, y: 21 },
                    prize: Point { x: 12748, y: 12176 },
                },
                Machine {
                    button_a: Point { x: 17, y: 86 },
                    button_b: Point { x: 84, y: 37 },
                    prize: Point { x: 7870, y: 6450 },
                },
                Machine {
                    button_a: Point { x: 69, y: 23 },
                    button_b: Point { x: 27, y: 71 },
                    prize: Point { x: 18641, y: 10279 },
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
