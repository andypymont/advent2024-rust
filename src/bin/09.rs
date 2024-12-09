use std::ops::RangeInclusive;
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

    fn defrag_whole_files(&mut self) {
        let mut next_id = self.contents.iter().filter_map(|content| *content).max();

        while let Some(id) = next_id {
            let Some(source) = self.find_file(id) else {
                continue;
            };
            let length = 1 + source.end() - source.start();

            if let Some(dest) = self.find_first_gap(length) {
                if dest.start() < source.start() {
                    self.contents[source].iter_mut().for_each(|c| *c = None);
                    self.contents[dest].iter_mut().for_each(|c| *c = Some(id));
                }
            }

            next_id = id.checked_sub(1);
        }
    }

    fn find_file(&self, id: usize) -> Option<RangeInclusive<usize>> {
        let mut start = None;
        let mut finish = 0;

        for (ix, file) in self.contents.iter().enumerate() {
            match start {
                None => {
                    if file == &Some(id) {
                        start = Some(ix);
                        finish = ix;
                    }
                }
                Some(_) => {
                    if file == &Some(id) {
                        finish = ix;
                    } else {
                        break;
                    }
                }
            }
        }

        let start = start?;
        Some(RangeInclusive::new(start, finish))
    }

    fn find_first_gap(&self, length: usize) -> Option<RangeInclusive<usize>> {
        let mut start = None;
        let mut finish = 0;

        for (ix, file) in self.contents.iter().enumerate() {
            match start {
                None => {
                    if file.is_none() {
                        start = Some(ix);
                        finish = ix;
                    }
                }
                Some(_) => {
                    if file.is_none() {
                        finish = ix;
                    } else {
                        start = None;
                    }
                }
            }
            if let Some(s) = start {
                if 1 + finish - s == length {
                    break;
                }
            }
        }

        let start = start?;
        Some(RangeInclusive::new(start, finish))
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
pub fn part_two(input: &str) -> Option<usize> {
    HardDrive::from_str(input).map_or(None, |mut drive| {
        drive.defrag_whole_files();
        Some(drive.checksum())
    })
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

    fn example_hard_drive_defragged_whole_files() -> HardDrive {
        HardDrive {
            contents: vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(4),
                Some(4),
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                None,
                None,
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
                None,
                None,
                None,
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                None,
                None,
            ],
        }
    }

    #[test]
    fn test_find_file() {
        let mut hard_drive = example_hard_drive();
        assert_eq!(hard_drive.find_file(0), Some(RangeInclusive::new(0, 1)));
        assert_eq!(hard_drive.find_file(1), Some(RangeInclusive::new(5, 7)));
        assert_eq!(hard_drive.find_file(7), Some(RangeInclusive::new(32, 34)));
        assert_eq!(hard_drive.find_file(10), None);
    }

    #[test]
    fn test_find_first_gap() {
        let mut hard_drive = example_hard_drive();
        assert_eq!(
            hard_drive.find_first_gap(1),
            Some(RangeInclusive::new(2, 2))
        );
        assert_eq!(
            hard_drive.find_first_gap(2),
            Some(RangeInclusive::new(2, 3))
        );
        assert_eq!(
            hard_drive.find_first_gap(3),
            Some(RangeInclusive::new(2, 4))
        );
        assert_eq!(hard_drive.find_first_gap(4), None);
    }

    #[test]
    fn test_defrag_whole_files() {
        let mut hard_drive = example_hard_drive();
        hard_drive.defrag_whole_files();
        assert_eq!(hard_drive, example_hard_drive_defragged_whole_files());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
