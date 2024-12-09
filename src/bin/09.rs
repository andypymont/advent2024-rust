use std::collections::BTreeSet;
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
struct File {
    id: usize,
    start: usize,
    length: usize,
}

impl File {
    fn checksum(&self) -> usize {
        (self.start..(self.start + self.length))
            .map(|pos| pos * self.id)
            .sum()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Space {
    start: usize,
    length: usize,
}

impl Space {
    const fn overlaps(&self, other: Self) -> bool {
        !(other.start > self.start + self.length || other.start + other.length < self.start)
    }
}

#[derive(Debug, PartialEq)]
struct FileSystem {
    files: Vec<File>,
    spaces: BTreeSet<Space>,
}

impl FileSystem {
    fn find_space(&self, length: usize, before: usize) -> Option<Space> {
        self.spaces
            .iter()
            .find(|s| s.length >= length && s.start < before)
            .copied()
    }

    fn allocate(&mut self, length: usize, before: usize) -> Option<usize> {
        let space = self.find_space(length, before)?;
        self.spaces.remove(&space);
        let remaining = space.length.saturating_sub(length);
        if remaining > 0 {
            self.spaces.insert(Space {
                start: space.start + length,
                length: remaining,
            });
        }
        Some(space.start)
    }

    fn checksum(&self) -> usize {
        self.files.iter().map(File::checksum).sum()
    }

    fn defrag(&mut self) {
        for ix in (0..self.files.len()).rev() {
            let length = self.files[ix].length;
            if let Some(start) = self.allocate(length, self.files[ix].start) {
                self.free(Space {
                    start: self.files[ix].start,
                    length: self.files[ix].length,
                });
                self.files[ix].start = start;
            }
        }
    }

    fn free(&mut self, space: Space) {
        let mut start = space.start;
        let mut end = space.start + space.length;

        for other in self.spaces.clone() {
            if other.start > end {
                break;
            }
            if space.overlaps(other) {
                start = start.min(other.start);
                end = end.max(other.start + other.length);
                self.spaces.remove(&other);
            }
        }

        self.spaces.insert(Space {
            start,
            length: end - start,
        });
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

impl FromStr for FileSystem {
    type Err = ParseHardDriveError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut spaces = BTreeSet::new();
        let mut id = 0;
        let mut start = 0;

        for (ix, ch) in input.trim().chars().enumerate() {
            let length = parse_digit(ch)?;
            if length == 0 {
                continue;
            }

            if ix % 2 == 0 {
                files.push(File { id, start, length });
                id += 1;
            } else {
                spaces.insert(Space { start, length });
            }

            start += length;
        }

        Ok(Self { files, spaces })
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
    FileSystem::from_str(input).map_or(None, |mut fs| {
        fs.defrag();
        Some(fs.checksum())
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

    fn example_file_system() -> FileSystem {
        let mut spaces = BTreeSet::new();
        spaces.insert(Space {
            start: 2,
            length: 3,
        });
        spaces.insert(Space {
            start: 8,
            length: 3,
        });
        spaces.insert(Space {
            start: 12,
            length: 3,
        });
        spaces.insert(Space {
            start: 18,
            length: 1,
        });
        spaces.insert(Space {
            start: 21,
            length: 1,
        });
        spaces.insert(Space {
            start: 26,
            length: 1,
        });
        spaces.insert(Space {
            start: 31,
            length: 1,
        });
        spaces.insert(Space {
            start: 35,
            length: 1,
        });
        FileSystem {
            files: vec![
                File {
                    id: 0,
                    start: 0,
                    length: 2,
                },
                File {
                    id: 1,
                    start: 5,
                    length: 3,
                },
                File {
                    id: 2,
                    start: 11,
                    length: 1,
                },
                File {
                    id: 3,
                    start: 15,
                    length: 3,
                },
                File {
                    id: 4,
                    start: 19,
                    length: 2,
                },
                File {
                    id: 5,
                    start: 22,
                    length: 4,
                },
                File {
                    id: 6,
                    start: 27,
                    length: 4,
                },
                File {
                    id: 7,
                    start: 32,
                    length: 3,
                },
                File {
                    id: 8,
                    start: 36,
                    length: 4,
                },
                File {
                    id: 9,
                    start: 40,
                    length: 2,
                },
            ],
            spaces,
        }
    }

    fn example_file_system_defragged() -> FileSystem {
        let mut spaces = BTreeSet::new();
        spaces.insert(Space {
            start: 11,
            length: 1,
        });
        spaces.insert(Space {
            start: 14,
            length: 1,
        });
        spaces.insert(Space {
            start: 18,
            length: 4,
        });
        spaces.insert(Space {
            start: 26,
            length: 1,
        });
        spaces.insert(Space {
            start: 31,
            length: 5,
        });
        spaces.insert(Space {
            start: 40,
            length: 2,
        });

        FileSystem {
            files: vec![
                File {
                    id: 0,
                    start: 0,
                    length: 2,
                },
                File {
                    id: 1,
                    start: 5,
                    length: 3,
                },
                File {
                    id: 2,
                    start: 4,
                    length: 1,
                },
                File {
                    id: 3,
                    start: 15,
                    length: 3,
                },
                File {
                    id: 4,
                    start: 12,
                    length: 2,
                },
                File {
                    id: 5,
                    start: 22,
                    length: 4,
                },
                File {
                    id: 6,
                    start: 27,
                    length: 4,
                },
                File {
                    id: 7,
                    start: 8,
                    length: 3,
                },
                File {
                    id: 8,
                    start: 36,
                    length: 4,
                },
                File {
                    id: 9,
                    start: 2,
                    length: 2,
                },
            ],
            spaces,
        }
    }

    #[test]
    fn test_example_file_system_defrag() {
        let mut file_system = example_file_system();
        file_system.defrag();
        assert_eq!(file_system, example_file_system_defragged());
    }

    #[test]
    fn test_parse_file_system() {
        assert_eq!(
            FileSystem::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_file_system()),
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
