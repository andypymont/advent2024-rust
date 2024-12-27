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

const GRID_SIZE: usize = 140;

fn relative_position(position: Option<usize>, direction: Direction, steps: usize) -> Option<usize> {
    let position = position?;
    let row = position / GRID_SIZE;
    let col = position % GRID_SIZE;

    let row = match direction {
        Direction::East => Some(row),
        Direction::Northwest | Direction::Northeast => row.checked_sub(steps),
        Direction::Southwest | Direction::South | Direction::Southeast => {
            let row = row + steps;
            if row >= GRID_SIZE {
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
            if col >= GRID_SIZE {
                None
            } else {
                Some(col)
            }
        }
    };
    col.map(|c| (row * GRID_SIZE) + c)
}

fn word_positions(
    position: Option<usize>,
    direction: Direction,
) -> impl Iterator<Item = Option<usize>> {
    (1..=3).map(move |steps| relative_position(position, direction, steps))
}

#[derive(Debug, PartialEq)]
struct WordSearch {
    grid: [char; GRID_SIZE * GRID_SIZE],
}

impl WordSearch {
    fn get(&self, position: Option<usize>) -> char {
        position.map_or('.', |pos| self.grid[pos])
    }

    fn xmas_count(&self) -> u32 {
        self.grid
            .iter()
            .enumerate()
            .map(|(position, letter)| {
                if *letter == 'X' || *letter == 'S' {
                    SEARCH_DIRECTIONS
                        .iter()
                        .map(|direction| {
                            let mut letters =
                                word_positions(Some(position), *direction).map(|pos| self.get(pos));
                            let letters = [
                                *letter,
                                letters.next().unwrap_or('.'),
                                letters.next().unwrap_or('.'),
                                letters.next().unwrap_or('.'),
                            ];
                            u32::from(
                                letters == ['X', 'M', 'A', 'S'] || letters == ['S', 'A', 'M', 'X'],
                            )
                        })
                        .sum()
                } else {
                    0
                }
            })
            .sum()
    }

    fn cross_mas_at(&self, position: Option<usize>, letter: char) -> bool {
        if letter != 'A' {
            return false;
        }

        let nw = self.get(relative_position(position, Direction::Northwest, 1));
        let ne = self.get(relative_position(position, Direction::Northeast, 1));
        let sw = self.get(relative_position(position, Direction::Southwest, 1));
        let se = self.get(relative_position(position, Direction::Southeast, 1));

        let nw_se = (nw == 'M' && se == 'S') || (nw == 'S' && se == 'M');
        let ne_sw = (ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M');
        nw_se && ne_sw
    }

    fn cross_mas_count(&self) -> u32 {
        self.grid
            .iter()
            .enumerate()
            .map(|(pos, ch)| u32::from(self.cross_mas_at(Some(pos), *ch)))
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseWordSearchError;

impl FromStr for WordSearch {
    type Err = ParseWordSearchError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = ['.'; GRID_SIZE * GRID_SIZE];
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                grid[(row * GRID_SIZE) + col] = ch;
            }
        }
        Ok(Self { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    WordSearch::from_str(input).ok().map(|ws| ws.xmas_count())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    WordSearch::from_str(input)
        .ok()
        .map(|ws| ws.cross_mas_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example_word_search() -> WordSearch {
        let mut grid = ['.'; GRID_SIZE * GRID_SIZE];

        grid[position(0, 0)] = 'M';
        grid[position(0, 1)] = 'M';
        grid[position(0, 2)] = 'M';
        grid[position(0, 3)] = 'S';
        grid[position(0, 4)] = 'X';
        grid[position(0, 5)] = 'X';
        grid[position(0, 6)] = 'M';
        grid[position(0, 7)] = 'A';
        grid[position(0, 8)] = 'S';
        grid[position(0, 9)] = 'M';

        grid[position(1, 0)] = 'M';
        grid[position(1, 1)] = 'S';
        grid[position(1, 2)] = 'A';
        grid[position(1, 3)] = 'M';
        grid[position(1, 4)] = 'X';
        grid[position(1, 5)] = 'M';
        grid[position(1, 6)] = 'S';
        grid[position(1, 7)] = 'M';
        grid[position(1, 8)] = 'S';
        grid[position(1, 9)] = 'A';

        grid[position(2, 0)] = 'A';
        grid[position(2, 1)] = 'M';
        grid[position(2, 2)] = 'X';
        grid[position(2, 3)] = 'S';
        grid[position(2, 4)] = 'X';
        grid[position(2, 5)] = 'M';
        grid[position(2, 6)] = 'A';
        grid[position(2, 7)] = 'A';
        grid[position(2, 8)] = 'M';
        grid[position(2, 9)] = 'M';

        grid[position(3, 0)] = 'M';
        grid[position(3, 1)] = 'S';
        grid[position(3, 2)] = 'A';
        grid[position(3, 3)] = 'M';
        grid[position(3, 4)] = 'A';
        grid[position(3, 5)] = 'S';
        grid[position(3, 6)] = 'M';
        grid[position(3, 7)] = 'S';
        grid[position(3, 8)] = 'M';
        grid[position(3, 9)] = 'X';

        grid[position(4, 0)] = 'X';
        grid[position(4, 1)] = 'M';
        grid[position(4, 2)] = 'A';
        grid[position(4, 3)] = 'S';
        grid[position(4, 4)] = 'A';
        grid[position(4, 5)] = 'M';
        grid[position(4, 6)] = 'X';
        grid[position(4, 7)] = 'A';
        grid[position(4, 8)] = 'M';
        grid[position(4, 9)] = 'M';

        grid[position(5, 0)] = 'X';
        grid[position(5, 1)] = 'X';
        grid[position(5, 2)] = 'A';
        grid[position(5, 3)] = 'M';
        grid[position(5, 4)] = 'M';
        grid[position(5, 5)] = 'X';
        grid[position(5, 6)] = 'X';
        grid[position(5, 7)] = 'A';
        grid[position(5, 8)] = 'M';
        grid[position(5, 9)] = 'A';

        grid[position(6, 0)] = 'S';
        grid[position(6, 1)] = 'M';
        grid[position(6, 2)] = 'S';
        grid[position(6, 3)] = 'M';
        grid[position(6, 4)] = 'S';
        grid[position(6, 5)] = 'A';
        grid[position(6, 6)] = 'S';
        grid[position(6, 7)] = 'X';
        grid[position(6, 8)] = 'S';
        grid[position(6, 9)] = 'S';

        grid[position(7, 0)] = 'S';
        grid[position(7, 1)] = 'A';
        grid[position(7, 2)] = 'X';
        grid[position(7, 3)] = 'A';
        grid[position(7, 4)] = 'M';
        grid[position(7, 5)] = 'A';
        grid[position(7, 6)] = 'S';
        grid[position(7, 7)] = 'A';
        grid[position(7, 8)] = 'A';
        grid[position(7, 9)] = 'A';

        grid[position(8, 0)] = 'M';
        grid[position(8, 1)] = 'A';
        grid[position(8, 2)] = 'M';
        grid[position(8, 3)] = 'M';
        grid[position(8, 4)] = 'M';
        grid[position(8, 5)] = 'X';
        grid[position(8, 6)] = 'M';
        grid[position(8, 7)] = 'M';
        grid[position(8, 8)] = 'M';
        grid[position(8, 9)] = 'M';

        grid[position(9, 0)] = 'M';
        grid[position(9, 1)] = 'X';
        grid[position(9, 2)] = 'M';
        grid[position(9, 3)] = 'X';
        grid[position(9, 4)] = 'A';
        grid[position(9, 5)] = 'X';
        grid[position(9, 6)] = 'M';
        grid[position(9, 7)] = 'A';
        grid[position(9, 8)] = 'S';
        grid[position(9, 9)] = 'X';

        WordSearch { grid }
    }

    #[test]
    fn test_word_search_get() {
        let word_search = example_word_search();
        assert_eq!(word_search.get(None), '.');
        assert_eq!(word_search.get(Some(position(0, 0))), 'M');
        assert_eq!(word_search.get(Some(position(1, 1))), 'S');
    }

    #[test]
    fn test_relative_position() {
        let pos = Some(position(4, 4));
        assert_eq!(
            relative_position(pos, Direction::Northwest, 1),
            Some(position(3, 3)),
        );
        assert_eq!(
            relative_position(pos, Direction::Southeast, 2),
            Some(position(6, 6)),
        );
        assert_eq!(relative_position(pos, Direction::Southwest, 5), None,);
    }

    #[test]
    fn test_word_positions() {
        let expected = vec![
            Some(position(3, 3)),
            Some(position(2, 2)),
            Some(position(1, 1)),
        ];
        assert_eq!(
            word_positions(Some(position(4, 4)), Direction::Northwest)
                .collect::<Vec<Option<usize>>>(),
            expected
        );

        let expected = vec![Some(position(3, 1)), Some(position(4, 0)), None];
        assert_eq!(
            word_positions(Some(position(2, 2)), Direction::Southwest)
                .collect::<Vec<Option<usize>>>(),
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
        assert_eq!(word_search.cross_mas_at(Some(position(1, 2)), 'A'), true);
        assert_eq!(word_search.cross_mas_at(Some(position(2, 2)), 'X'), false);
        assert_eq!(word_search.cross_mas_at(Some(position(2, 6)), 'A'), true);
        assert_eq!(word_search.cross_mas_at(Some(position(4, 2)), 'A'), false);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
