use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
    Other,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    DiagonalRight,
    DiagonalLeft,
}

#[derive(Debug, PartialEq)]
struct WordSearch {
    grid: Vec<Vec<Letter>>,
}

impl WordSearch {
    fn get(&self, row: usize, col: usize) -> Letter {
        *self
            .grid
            .get(row)
            .map_or(&Letter::Other, |row| row.get(col).unwrap_or(&Letter::Other))
    }

    fn find(&self, mut row: usize, col: usize, direction: &Direction) -> bool {
        let mut letters = [Letter::Other; 4];
        let mut col = Some(col);

        for letter in &mut letters {
            let Some(c) = col else {
                continue;
            };

            *letter = self.get(row, c);

            row = match direction {
                Direction::Right => row,
                Direction::Down | Direction::DiagonalRight | Direction::DiagonalLeft => row + 1,
            };
            col = match direction {
                Direction::Right | Direction::DiagonalRight => Some(c + 1),
                Direction::Down => Some(c),
                Direction::DiagonalLeft => {
                    if c == 0 {
                        None
                    } else {
                        Some(c - 1)
                    }
                }
            };
        }

        if letters == [Letter::X, Letter::M, Letter::A, Letter::S] {
            true
        } else {
            letters == [Letter::S, Letter::A, Letter::M, Letter::X]
        }
    }

    fn mas(&self, r: usize, c: usize, direction: &Direction) -> bool {
        if self.get(r, c) != Letter::A {
            return false;
        }

        let row = match direction {
            Direction::Right => r,
            Direction::Down | Direction::DiagonalRight | Direction::DiagonalLeft => r + 1,
        };
        let col = match direction {
            Direction::Right | Direction::DiagonalRight => Some(c + 1),
            Direction::Down => Some(c),
            Direction::DiagonalLeft => {
                if c == 0 {
                    None
                } else {
                    Some(c - 1)
                }
            }
        };
        let Some(col) = col else {
            return false;
        };
        let end = self.get(row, col);
        match end {
            Letter::Other | Letter::X | Letter::A => false,
            Letter::M | Letter::S => {
                let row = match direction {
                    Direction::Right => Some(r),
                    Direction::Down | Direction::DiagonalRight | Direction::DiagonalLeft => {
                        if r == 0 {
                            None
                        } else {
                            Some(r - 1)
                        }
                    }
                };
                let Some(row) = row else {
                    return false;
                };
                let col = match direction {
                    Direction::Right | Direction::DiagonalRight => {
                        if c == 0 {
                            None
                        } else {
                            Some(c - 1)
                        }
                    }
                    Direction::Down => Some(c),
                    Direction::DiagonalLeft => Some(c + 1),
                };
                let Some(col) = col else {
                    return false;
                };
                let start = self.get(row, col);
                matches!(
                    (start, end),
                    (Letter::M, Letter::S) | (Letter::S, Letter::M)
                )
            }
        }
    }

    fn cross_mas(&self, row: usize, col: usize) -> bool {
        let right = self.mas(row, col, &Direction::DiagonalRight);
        let left = self.mas(row, col, &Direction::DiagonalLeft);
        right && left
    }

    fn total_cross_mas(&self) -> u32 {
        let mut count = 0;

        for (r, row) in self.grid.iter().enumerate() {
            for c in 0..row.len() {
                count += u32::from(self.cross_mas(r, c));
            }
        }

        count
    }

    fn total_matches(&self) -> u32 {
        let mut matches = 0;

        for (r, row) in self.grid.iter().enumerate() {
            for c in 0..row.len() {
                matches += u32::from(self.find(r, c, &Direction::Down));
                matches += u32::from(self.find(r, c, &Direction::Right));
                matches += u32::from(self.find(r, c, &Direction::DiagonalLeft));
                matches += u32::from(self.find(r, c, &Direction::DiagonalRight));
            }
        }

        matches
    }
}

#[derive(Debug, PartialEq)]
struct ParseWordSearchError;

impl FromStr for WordSearch {
    type Err = ParseWordSearchError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();

            for ch in line.chars() {
                row.push(match ch {
                    'X' => Letter::X,
                    'M' => Letter::M,
                    'A' => Letter::A,
                    'S' => Letter::S,
                    _ => Letter::Other,
                });
            }

            grid.push(row);
        }

        Ok(Self { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    input
        .parse::<WordSearch>()
        .map_or(None, |word_search| Some(word_search.total_matches()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    input
        .parse::<WordSearch>()
        .map_or(None, |word_search| Some(word_search.total_cross_mas()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_word_search() -> WordSearch {
        WordSearch {
            grid: vec![
                vec![
                    Letter::M,
                    Letter::M,
                    Letter::M,
                    Letter::S,
                    Letter::X,
                    Letter::X,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::M,
                ],
                vec![
                    Letter::M,
                    Letter::S,
                    Letter::A,
                    Letter::M,
                    Letter::X,
                    Letter::M,
                    Letter::S,
                    Letter::M,
                    Letter::S,
                    Letter::A,
                ],
                vec![
                    Letter::A,
                    Letter::M,
                    Letter::X,
                    Letter::S,
                    Letter::X,
                    Letter::M,
                    Letter::A,
                    Letter::A,
                    Letter::M,
                    Letter::M,
                ],
                vec![
                    Letter::M,
                    Letter::S,
                    Letter::A,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::M,
                    Letter::S,
                    Letter::M,
                    Letter::X,
                ],
                vec![
                    Letter::X,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::A,
                    Letter::M,
                    Letter::X,
                    Letter::A,
                    Letter::M,
                    Letter::M,
                ],
                vec![
                    Letter::X,
                    Letter::X,
                    Letter::A,
                    Letter::M,
                    Letter::M,
                    Letter::X,
                    Letter::X,
                    Letter::A,
                    Letter::M,
                    Letter::A,
                ],
                vec![
                    Letter::S,
                    Letter::M,
                    Letter::S,
                    Letter::M,
                    Letter::S,
                    Letter::A,
                    Letter::S,
                    Letter::X,
                    Letter::S,
                    Letter::S,
                ],
                vec![
                    Letter::S,
                    Letter::A,
                    Letter::X,
                    Letter::A,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::A,
                    Letter::A,
                    Letter::A,
                ],
                vec![
                    Letter::M,
                    Letter::A,
                    Letter::M,
                    Letter::M,
                    Letter::M,
                    Letter::X,
                    Letter::M,
                    Letter::M,
                    Letter::M,
                    Letter::M,
                ],
                vec![
                    Letter::M,
                    Letter::X,
                    Letter::M,
                    Letter::X,
                    Letter::A,
                    Letter::X,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::X,
                ],
            ],
        }
    }

    #[test]
    fn test_word_search_get() {
        let word_search = example_word_search();

        assert_eq!(word_search.get(0, 0), Letter::M);
        assert_eq!(word_search.get(1, 1), Letter::S);
        assert_eq!(word_search.get(10, 4), Letter::Other);
    }

    #[test]
    fn test_word_search_find() {
        let word_search = example_word_search();

        assert_eq!(word_search.find(0, 0, &Direction::Right), false);
        assert_eq!(word_search.find(0, 5, &Direction::Right), true);
        assert_eq!(word_search.find(0, 4, &Direction::DiagonalRight), true);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_word_search()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_word_search_cross_mas() {
        let word_search = example_word_search();

        assert_eq!(word_search.cross_mas(1, 2), true);
        assert_eq!(word_search.cross_mas(2, 2), false);
        assert_eq!(word_search.cross_mas(2, 6), true);
        assert_eq!(word_search.cross_mas(4, 2), false);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
