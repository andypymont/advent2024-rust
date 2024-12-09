use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Debug, PartialEq)]
struct HardDrive {
    contents: Vec<Option<usize>>,
}

impl HardDrive {
    fn checksum(&self) -> usize {
        self.contents
            .iter()
            .enumerate()
            .map(|(pos, space)| pos * space.unwrap_or(0))
            .sum()
    }

    fn defrag(&mut self) {
        let mut ix = 0;

        loop {
            if ix >= self.contents.len() - 1 {
                break;
            }
            if self.contents[ix].is_some() {
                ix += 1;
                continue;
            }
            let Some(removed) = self.contents.pop() else {
                break;
            };
            if removed.is_some() {
                self.contents[ix] = removed;
                ix += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseHardDriveError;

fn parse_digit(ch: char) -> Result<usize, ParseHardDriveError> {
    let digit = ch.to_digit(10).ok_or(ParseHardDriveError)?;
    digit.try_into().map_err(|_| ParseHardDriveError)
}

impl FromStr for HardDrive {
    type Err = ParseHardDriveError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut contents = Vec::new();
        let mut file_id = 0;

        for (pos, ch) in input.trim().chars().enumerate() {
            let space = if pos % 2 == 0 {
                let id = file_id;
                file_id += 1;
                Some(id)
            } else {
                None
            };
            let length = parse_digit(ch)?;
            for _ in 0..length {
                contents.push(space);
            }
        }

        Ok(Self { contents })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    HardDrive::from_str(input).map_or(None, |mut drive| {
        drive.defrag();
        Some(drive.checksum())
    })
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_hard_drive() -> HardDrive {
        HardDrive {
            contents: vec![
                Some(0),
                Some(0),
                None,
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                Some(4),
                Some(4),
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                Some(9),
                Some(9),
            ],
        }
    }

    fn example_hard_drive_defragged() -> HardDrive {
        HardDrive {
            contents: vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
            ],
        }
    }

    #[test]
    fn test_parse_hard_drive() {
        assert_eq!(
            HardDrive::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_hard_drive()),
        );
    }

    #[test]
    fn test_defrag() {
        let mut hard_drive = example_hard_drive();
        hard_drive.defrag();
        assert_eq!(hard_drive, example_hard_drive_defragged());
    }

    #[test]
    fn test_checksum() {
        assert_eq!(example_hard_drive_defragged().checksum(), 1928);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
