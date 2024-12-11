use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::str::FromStr;

advent_of_code::solution!(9);

fn checksum(id: usize, start: usize, length: usize) -> usize {
    if id == 0 {
        return 0;
    }
    (start..start + length).fold(0, |sum, pos| sum + (pos * id))
}

#[derive(Debug, PartialEq)]
struct Record {
    id: Option<usize>,
    start: usize,
    length: usize,
}

impl Record {
    fn checksum(&self) -> usize {
        checksum(self.id.unwrap_or(0), self.start, self.length)
    }

    const fn is_file(&self) -> bool {
        self.id.is_some()
    }

    const fn is_free_space(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug)]
struct SpaceAllocator {
    position: usize,
    cache: Vec<BinaryHeap<Reverse<usize>>>,
}

impl SpaceAllocator {
    fn new() -> Self {
        let mut cache = Vec::new();
        for _ in 0..=9 {
            cache.push(BinaryHeap::new());
        }
        Self { position: 0, cache }
    }

    fn find_leftmost_matching_cache(&self, length: usize) -> Option<usize> {
        (length..=9)
            .filter_map(|len| self.cache[len].peek().map(|pos| (len, pos)))
            .max_by_key(|(_len, pos)| *pos)
            .map(|(len, _pos)| len)
    }

    fn find_in_cache(&mut self, length: usize) -> Option<(usize, usize)> {
        let length = self.find_leftmost_matching_cache(length)?;
        if let Some(Reverse(pos)) = self.cache[length].pop() {
            Some((pos, length))
        } else {
            None
        }
    }

    fn find_in_disk_map(&mut self, disk_map: &DiskMap, length: usize) -> Option<(usize, usize)> {
        while self.position < disk_map.records.len() {
            let record = &disk_map.records[self.position];

            if record.is_free_space() {
                if record.length >= length {
                    self.position += 1;
                    return Some((record.start, record.length));
                }

                self.insert_in_cache(record.start, record.length);
            }

            self.position += 1;
        }

        None
    }

    fn insert_in_cache(&mut self, pos: usize, length: usize) {
        if length == 0 {
            return;
        }
        self.cache[length].push(Reverse(pos));
    }

    fn next(&mut self, disk_map: &DiskMap, length: usize) -> Option<usize> {
        let (pos, space) = match self.find_in_cache(length) {
            Some(cached) => cached,
            None => self.find_in_disk_map(disk_map, length)?,
        };
        self.insert_in_cache(pos + length, space - length);
        Some(pos)
    }
}

#[derive(Debug, PartialEq)]
struct DiskMap {
    records: Vec<Record>,
}

impl DiskMap {
    fn defragged_checksum(mut self) -> usize {
        let mut total_checksum = 0;

        // track from the front and back of memory at the same time
        let mut front = 0;
        let mut back = self.records.len() - 1;

        loop {
            if front == self.records.len() {
                break;
            }
            if self.records[front].length == 0 {
                front += 1;
                continue;
            }
            if self.records[front].is_file() {
                // record checksum as files found at front of memory
                total_checksum += self.records[front].checksum();
                front += 1;
                continue;
            }
            while back > front
                && (self.records[back].is_free_space() || self.records[back].length == 0)
            {
                // skip over free space at back of memory, look for files
                back -= 1;
            }
            if back <= front {
                // we can no longer moves files to the front of memory into free space, and
                // everything we passed moving forwards from the back is already exhausted, so we
                // are done
                break;
            }

            // we're located at free space at front of memory and a file at the back of memory (due
            // to gate logic above)
            let moved = self.records[front].length.min(self.records[back].length);
            total_checksum += checksum(
                self.records[back].id.unwrap_or(0),
                self.records[front].start,
                moved,
            );

            // checksum now adjusted so reduce both elements in size
            self.records[front].start += moved;
            self.records[front].length -= moved;
            self.records[back].length -= moved;
        }

        total_checksum
    }

    fn defragged_whole_files_checksum(&self) -> usize {
        let mut total_checksum = 0;
        let mut alloc = SpaceAllocator::new();

        for pos in (0..self.records.len()).rev() {
            let record = &self.records[pos];

            if !record.is_file() {
                continue;
            }

            let start = alloc
                .next(self, record.length)
                .map_or(record.start, |start| start.min(record.start));
            total_checksum += checksum(record.id.unwrap_or(0), start, record.length);
        }

        total_checksum
    }
}

#[derive(Debug, PartialEq)]
struct ParseDiskMapError;

const fn parse_digit(ch: char) -> Result<usize, ParseDiskMapError> {
    match ch {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(ParseDiskMapError),
    }
}

impl FromStr for DiskMap {
    type Err = ParseDiskMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut records = Vec::new();

        let mut start = 0;
        let mut id = 0..;
        let mut file = true;

        for ch in input.trim().chars() {
            let length = parse_digit(ch)?;
            let id = if file {
                Some(id.next().unwrap_or(0))
            } else {
                None
            };
            records.push(Record { id, start, length });

            start += length;
            file = !file;
        }

        Ok(Self { records })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    DiskMap::from_str(input).map_or(None, |dm| Some(dm.defragged_checksum()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    DiskMap::from_str(input).map_or(None, |dm| Some(dm.defragged_whole_files_checksum()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_disk_map() -> DiskMap {
        DiskMap {
            records: vec![
                Record {
                    id: Some(0),
                    start: 0,
                    length: 2,
                },
                Record {
                    id: None,
                    start: 2,
                    length: 3,
                },
                Record {
                    id: Some(1),
                    start: 5,
                    length: 3,
                },
                Record {
                    id: None,
                    start: 8,
                    length: 3,
                },
                Record {
                    id: Some(2),
                    start: 11,
                    length: 1,
                },
                Record {
                    id: None,
                    start: 12,
                    length: 3,
                },
                Record {
                    id: Some(3),
                    start: 15,
                    length: 3,
                },
                Record {
                    id: None,
                    start: 18,
                    length: 1,
                },
                Record {
                    id: Some(4),
                    start: 19,
                    length: 2,
                },
                Record {
                    id: None,
                    start: 21,
                    length: 1,
                },
                Record {
                    id: Some(5),
                    start: 22,
                    length: 4,
                },
                Record {
                    id: None,
                    start: 26,
                    length: 1,
                },
                Record {
                    id: Some(6),
                    start: 27,
                    length: 4,
                },
                Record {
                    id: None,
                    start: 31,
                    length: 1,
                },
                Record {
                    id: Some(7),
                    start: 32,
                    length: 3,
                },
                Record {
                    id: None,
                    start: 35,
                    length: 1,
                },
                Record {
                    id: Some(8),
                    start: 36,
                    length: 4,
                },
                Record {
                    id: None,
                    start: 40,
                    length: 0,
                },
                Record {
                    id: Some(9),
                    start: 40,
                    length: 2,
                },
            ],
        }
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(4, 5, 3), 72);
        assert_eq!(
            Record {
                id: Some(4),
                start: 5,
                length: 3
            }
            .checksum(),
            72
        );
        assert_eq!(checksum(2, 7, 5), 90);
        assert_eq!(
            Record {
                id: Some(2),
                start: 7,
                length: 5
            }
            .checksum(),
            90
        );

        assert_eq!(checksum(0, 4, 13), 0);
        assert_eq!(
            Record {
                id: None,
                start: 5,
                length: 2
            }
            .checksum(),
            0
        );
        assert_eq!(
            Record {
                id: None,
                start: 4,
                length: 4
            }
            .checksum(),
            0
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            DiskMap::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_disk_map()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_space_allocator() {
        let disk_map = example_disk_map();

        let mut allocator = SpaceAllocator::new();
        assert_eq!(allocator.next(&disk_map, 1), Some(2));
        assert_eq!(allocator.next(&disk_map, 1), Some(3));
        assert_eq!(allocator.next(&disk_map, 1), Some(4));
        assert_eq!(allocator.next(&disk_map, 1), Some(8));

        let mut allocator = SpaceAllocator::new();
        assert_eq!(allocator.next(&disk_map, 2), Some(2));
        assert_eq!(allocator.next(&disk_map, 2), Some(8));
        assert_eq!(allocator.next(&disk_map, 2), Some(12));
        assert_eq!(allocator.next(&disk_map, 2), None);
        assert_eq!(allocator.next(&disk_map, 1), Some(4));
        assert_eq!(allocator.next(&disk_map, 1), Some(10));
        assert_eq!(allocator.next(&disk_map, 1), Some(14));
        assert_eq!(allocator.next(&disk_map, 1), Some(18));

        let mut allocator = SpaceAllocator::new();
        assert_eq!(allocator.next(&disk_map, 3), Some(2));
        assert_eq!(allocator.next(&disk_map, 3), Some(8));
        assert_eq!(allocator.next(&disk_map, 3), Some(12));
        assert_eq!(allocator.next(&disk_map, 3), None);
        assert_eq!(allocator.next(&disk_map, 1), Some(18));

        let mut allocator = SpaceAllocator::new();
        assert_eq!(allocator.next(&disk_map, 4), None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
