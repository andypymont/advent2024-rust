use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    Northwest,
}

const SEARCH_DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::Southwest,
    Direction::South,
    Direction::Southeast,
];

#[derive(Debug, PartialEq)]
struct WordSearch {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    fn get(&self, position: Option<(usize, usize)>) -> char {
        match position {
            Some((r, c)) => self.grid[r][c],
            None => '.',
        }
    }

    fn word_positions(
        &self,
        position: Option<(usize, usize)>,
        direction: Direction,
    ) -> impl Iterator<Item = Option<(usize, usize)>> + use<'_> {
        (0..4).map(move |steps| self.relative_position(position, direction, steps))
    }

    fn relative_position(
        &self,
        position: Option<(usize, usize)>,
        direction: Direction,
        steps: usize,
    ) -> Option<(usize, usize)> {
        let (row, col) = position?;

        let row = match direction {
            Direction::East => Some(row),
            Direction::Northwest | Direction::Northeast => row.checked_sub(steps),
            Direction::Southwest | Direction::South | Direction::Southeast => {
                let row = row + steps;
                if row >= self.height {
                    None
                } else {
                    Some(row)
                }
            }
        };
        let row = row?;

        let col = match direction {
            Direction::South => Some(col),
            Direction::Northwest | Direction::Southwest => col.checked_sub(steps),
            Direction::Northeast | Direction::East | Direction::Southeast => {
                let col = col + steps;
                if col >= self.width {
                    None
                } else {
                    Some(col)
                }
            }
        };

        col.map(|col| (row, col))
    }

    fn xmas_count(&self) -> u32 {
        let mut count = 0;

        for (r, row) in self.grid.iter().enumerate() {
            for (c, letter) in row.iter().enumerate() {
                let position = Some((r, c));

                if letter == &'X' || letter == &'S' {
                    for direction in SEARCH_DIRECTIONS {
                        let mut letters = self
                            .word_positions(position, direction)
                            .map(|pos| self.get(pos));
                        let letters = [
                            letters.next().unwrap_or('.'),
                            letters.next().unwrap_or('.'),
                            letters.next().unwrap_or('.'),
                            letters.next().unwrap_or('.'),
                        ];
                        if letters == ['X', 'M', 'A', 'S'] || letters == ['S', 'A', 'M', 'X'] {
                            count += 1;
                        };
                    }
                }
            }
        }

        count
    }

    fn cross_mas_at(&self, position: Option<(usize, usize)>) -> bool {
        if self.get(position) != 'A' {
            return false;
        }

        let nw = self.get(self.relative_position(position, Direction::Northwest, 1));
        let ne = self.get(self.relative_position(position, Direction::Northeast, 1));
        let sw = self.get(self.relative_position(position, Direction::Southwest, 1));
        let se = self.get(self.relative_position(position, Direction::Southeast, 1));

        let nw_se = (nw == 'M' && se == 'S') || (nw == 'S' && se == 'M');
        let ne_sw = (ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M');
        nw_se && ne_sw
    }

    fn cross_mas_count(&self) -> u32 {
        let mut count = 0;

        for (r, row) in self.grid.iter().enumerate() {
            for c in 0..row.len() {
                count += u32::from(self.cross_mas_at(Some((r, c))));
            }
        }

        count
    }
}

#[derive(Debug, PartialEq)]
struct ParseWordSearchError;

impl FromStr for WordSearch {
    type Err = ParseWordSearchError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                let row: Vec<char> = line.chars().collect();
                width = width.max(row.len());
                row
            })
            .collect();
        let height = grid.len();
        Ok(Self {
            height,
            width,
            grid,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    input
        .parse::<WordSearch>()
        .map_or(None, |ws| Some(ws.xmas_count()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    input
        .parse::<WordSearch>()
        .map_or(None, |ws| Some(ws.cross_mas_count()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_word_search() -> WordSearch {
        WordSearch {
            height: 10,
            width: 10,
            grid: vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
            ],
        }
    }

    #[test]
    fn test_word_search_get() {
        let word_search = example_word_search();
        assert_eq!(word_search.get(None), '.');
        assert_eq!(word_search.get(Some((0, 0))), 'M');
        assert_eq!(word_search.get(Some((1, 1))), 'S');
    }

    #[test]
    fn test_relative_position() {
        let word_search = example_word_search();
        let position = Some((4, 4));
        assert_eq!(
            word_search.relative_position(position, Direction::Northwest, 1),
            Some((3, 3)),
        );
        assert_eq!(
            word_search.relative_position(position, Direction::Southeast, 2),
            Some((6, 6)),
        );
        assert_eq!(
            word_search.relative_position(position, Direction::Southwest, 5),
            None,
        );
    }

    #[test]
    fn test_word_positions() {
        let word_search = example_word_search();
        let expected = vec![Some((4, 4)), Some((3, 3)), Some((2, 2)), Some((1, 1))];
        assert_eq!(
            word_search
                .word_positions(Some((4, 4)), Direction::Northwest)
                .collect::<Vec<Option<(usize, usize)>>>(),
            expected
        );

        let expected = vec![Some((2, 2)), Some((3, 1)), Some((4, 0)), None];
        assert_eq!(
            word_search
                .word_positions(Some((2, 2)), Direction::Southwest)
                .collect::<Vec<Option<(usize, usize)>>>(),
            expected
        );
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
    fn test_cross_mas() {
        let word_search = example_word_search();
        assert_eq!(word_search.cross_mas_at(Some((1, 2))), true);
        assert_eq!(word_search.cross_mas_at(Some((2, 2))), false);
        assert_eq!(word_search.cross_mas_at(Some((2, 6))), true);
        assert_eq!(word_search.cross_mas_at(Some((4, 2))), false);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
