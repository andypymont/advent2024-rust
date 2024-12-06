use std::str::FromStr;

advent_of_code::solution!(6);

const GRID_SIZE: usize = 130;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn step_from(&self, position: usize) -> Option<usize> {
        let row = position / GRID_SIZE;
        let col = position % GRID_SIZE;

        let row = match self {
            Self::West | Self::East => Some(row),
            Self::North => row.checked_sub(1),
            Self::South => {
                let row = row + 1;
                if row >= GRID_SIZE {
                    None
                } else {
                    Some(row)
                }
            }
        };
        let row = row?;

        let col = match self {
            Self::North | Self::South => Some(col),
            Self::West => col.checked_sub(1),
            Self::East => {
                let col = col + 1;
                if col >= GRID_SIZE {
                    None
                } else {
                    Some(col)
                }
            }
        };

        col.map(|col| (row * GRID_SIZE) + col)
    }
}

type Grid = [Option<bool>; GRID_SIZE * GRID_SIZE];

#[derive(Debug, PartialEq)]
struct PatrolArea {
    grid: Grid,
    start: usize,
}

impl PatrolArea {
    fn patrol_visits(&self) -> [bool; GRID_SIZE * GRID_SIZE] {
        let mut visits = [false; GRID_SIZE * GRID_SIZE];

        let mut position = self.start;
        let mut facing = Direction::North;

        loop {
            visits[position] = true;
            let Some(ahead) = facing.step_from(position) else {
                break;
            };
            match self.grid[ahead] {
                None => break,
                Some(true) => facing = facing.turn_right(),
                Some(false) => position = ahead,
            }
        }

        visits
    }
}

#[derive(Debug, PartialEq)]
struct ParsePatrolAreaError;

impl FromStr for PatrolArea {
    type Err = ParsePatrolAreaError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut grid = [None; GRID_SIZE * GRID_SIZE];
        let mut start: Result<usize, ParsePatrolAreaError> = Err(ParsePatrolAreaError);

        for (row, line) in text.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                grid[(row * GRID_SIZE) + col] = match ch {
                    '.' => Some(false),
                    '#' => Some(true),
                    '^' => {
                        start = Ok((row * GRID_SIZE) + col);
                        Some(false)
                    }
                    _ => None,
                }
            }
        }

        let start = start?;
        Ok(Self { grid, start })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    PatrolArea::from_str(input).map_or(None, |area| {
        Some(area.patrol_visits().into_iter().map(u32::from).sum())
    })
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example_patrol_area() -> PatrolArea {
        let mut grid = [None; GRID_SIZE * GRID_SIZE];

        grid[position(0, 0)] = Some(false);
        grid[position(0, 1)] = Some(false);
        grid[position(0, 2)] = Some(false);
        grid[position(0, 3)] = Some(false);
        grid[position(0, 4)] = Some(true);
        grid[position(0, 5)] = Some(false);
        grid[position(0, 6)] = Some(false);
        grid[position(0, 7)] = Some(false);
        grid[position(0, 8)] = Some(false);
        grid[position(0, 9)] = Some(false);

        grid[position(1, 0)] = Some(false);
        grid[position(1, 1)] = Some(false);
        grid[position(1, 2)] = Some(false);
        grid[position(1, 3)] = Some(false);
        grid[position(1, 4)] = Some(false);
        grid[position(1, 5)] = Some(false);
        grid[position(1, 6)] = Some(false);
        grid[position(1, 7)] = Some(false);
        grid[position(1, 8)] = Some(false);
        grid[position(1, 9)] = Some(true);

        grid[position(2, 0)] = Some(false);
        grid[position(2, 1)] = Some(false);
        grid[position(2, 2)] = Some(false);
        grid[position(2, 3)] = Some(false);
        grid[position(2, 4)] = Some(false);
        grid[position(2, 5)] = Some(false);
        grid[position(2, 6)] = Some(false);
        grid[position(2, 7)] = Some(false);
        grid[position(2, 8)] = Some(false);
        grid[position(2, 9)] = Some(false);

        grid[position(3, 0)] = Some(false);
        grid[position(3, 1)] = Some(false);
        grid[position(3, 2)] = Some(true);
        grid[position(3, 3)] = Some(false);
        grid[position(3, 4)] = Some(false);
        grid[position(3, 5)] = Some(false);
        grid[position(3, 6)] = Some(false);
        grid[position(3, 7)] = Some(false);
        grid[position(3, 8)] = Some(false);
        grid[position(3, 9)] = Some(false);

        grid[position(4, 0)] = Some(false);
        grid[position(4, 1)] = Some(false);
        grid[position(4, 2)] = Some(false);
        grid[position(4, 3)] = Some(false);
        grid[position(4, 4)] = Some(false);
        grid[position(4, 5)] = Some(false);
        grid[position(4, 6)] = Some(false);
        grid[position(4, 7)] = Some(true);
        grid[position(4, 8)] = Some(false);
        grid[position(4, 9)] = Some(false);

        grid[position(5, 0)] = Some(false);
        grid[position(5, 1)] = Some(false);
        grid[position(5, 2)] = Some(false);
        grid[position(5, 3)] = Some(false);
        grid[position(5, 4)] = Some(false);
        grid[position(5, 5)] = Some(false);
        grid[position(5, 6)] = Some(false);
        grid[position(5, 7)] = Some(false);
        grid[position(5, 8)] = Some(false);
        grid[position(5, 9)] = Some(false);

        grid[position(6, 0)] = Some(false);
        grid[position(6, 1)] = Some(true);
        grid[position(6, 2)] = Some(false);
        grid[position(6, 3)] = Some(false);
        grid[position(6, 4)] = Some(false);
        grid[position(6, 5)] = Some(false);
        grid[position(6, 6)] = Some(false);
        grid[position(6, 7)] = Some(false);
        grid[position(6, 8)] = Some(false);
        grid[position(6, 9)] = Some(false);

        grid[position(7, 0)] = Some(false);
        grid[position(7, 1)] = Some(false);
        grid[position(7, 2)] = Some(false);
        grid[position(7, 3)] = Some(false);
        grid[position(7, 4)] = Some(false);
        grid[position(7, 5)] = Some(false);
        grid[position(7, 6)] = Some(false);
        grid[position(7, 7)] = Some(false);
        grid[position(7, 8)] = Some(true);
        grid[position(7, 9)] = Some(false);

        grid[position(8, 0)] = Some(true);
        grid[position(8, 1)] = Some(false);
        grid[position(8, 2)] = Some(false);
        grid[position(8, 3)] = Some(false);
        grid[position(8, 4)] = Some(false);
        grid[position(8, 5)] = Some(false);
        grid[position(8, 6)] = Some(false);
        grid[position(8, 7)] = Some(false);
        grid[position(8, 8)] = Some(false);
        grid[position(8, 9)] = Some(false);

        grid[position(9, 0)] = Some(false);
        grid[position(9, 1)] = Some(false);
        grid[position(9, 2)] = Some(false);
        grid[position(9, 3)] = Some(false);
        grid[position(9, 4)] = Some(false);
        grid[position(9, 5)] = Some(false);
        grid[position(9, 6)] = Some(true);
        grid[position(9, 7)] = Some(false);
        grid[position(9, 8)] = Some(false);
        grid[position(9, 9)] = Some(false);

        let start = position(6, 4);

        PatrolArea { grid, start }
    }

    #[test]
    fn test_patrol_visits() {
        let mut visits = [false; GRID_SIZE * GRID_SIZE];
        visits[position(1, 4)] = true;
        visits[position(1, 5)] = true;
        visits[position(1, 6)] = true;
        visits[position(1, 7)] = true;
        visits[position(1, 8)] = true;
        visits[position(2, 4)] = true;
        visits[position(2, 8)] = true;
        visits[position(3, 4)] = true;
        visits[position(3, 8)] = true;
        visits[position(4, 2)] = true;
        visits[position(4, 3)] = true;
        visits[position(4, 4)] = true;
        visits[position(4, 5)] = true;
        visits[position(4, 6)] = true;
        visits[position(4, 8)] = true;
        visits[position(5, 2)] = true;
        visits[position(5, 4)] = true;
        visits[position(5, 6)] = true;
        visits[position(5, 8)] = true;
        visits[position(6, 2)] = true;
        visits[position(6, 3)] = true;
        visits[position(6, 4)] = true;
        visits[position(6, 5)] = true;
        visits[position(6, 6)] = true;
        visits[position(6, 7)] = true;
        visits[position(6, 8)] = true;
        visits[position(7, 1)] = true;
        visits[position(7, 2)] = true;
        visits[position(7, 3)] = true;
        visits[position(7, 4)] = true;
        visits[position(7, 5)] = true;
        visits[position(7, 6)] = true;
        visits[position(7, 7)] = true;
        visits[position(8, 1)] = true;
        visits[position(8, 2)] = true;
        visits[position(8, 3)] = true;
        visits[position(8, 4)] = true;
        visits[position(8, 5)] = true;
        visits[position(8, 6)] = true;
        visits[position(8, 7)] = true;
        visits[position(9, 7)] = true;

        assert_eq!(example_patrol_area().patrol_visits(), visits);
    }

    #[test]
    fn test_parse_patrol_area() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_patrol_area()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
