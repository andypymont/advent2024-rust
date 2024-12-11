use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(10);

const GRID_SIZE: usize = 40;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TrailMapSearchState {
    origin: (usize, usize),
    row: usize,
    col: usize,
}

impl TrailMapSearchState {
    fn neighbours(&self) -> impl Iterator<Item = Self> + use<'_> {
        COMPASS.iter().filter_map(|dir| self.step(dir))
    }

    fn step(&self, direction: &Direction) -> Option<Self> {
        let row = match direction {
            Direction::North => self.row.checked_sub(1),
            Direction::South => {
                if (self.row + 1) < GRID_SIZE {
                    Some(self.row + 1)
                } else {
                    None
                }
            }
            Direction::East | Direction::West => Some(self.row),
        };
        let col = match direction {
            Direction::West => self.col.checked_sub(1),
            Direction::East => {
                if (self.col + 1) < GRID_SIZE {
                    Some(self.col + 1)
                } else {
                    None
                }
            }
            Direction::North | Direction::South => Some(self.col),
        };

        let row = row?;
        let col = col?;

        Some(Self { row, col, ..*self })
    }
}

type TrailMapGrid = [[Option<u8>; GRID_SIZE]; GRID_SIZE];

#[derive(Debug, PartialEq)]
struct TrailMap {
    grid: TrailMapGrid,
    queue: VecDeque<TrailMapSearchState>,
}

impl TrailMap {
    fn new(grid: &TrailMapGrid) -> Self {
        let mut queue = VecDeque::new();
        queue.extend(grid.iter().enumerate().flat_map(|(row, heights)| {
            heights.iter().enumerate().filter_map(move |(col, height)| {
                if height.is_some_and(|h| h == 0) {
                    Some(TrailMapSearchState {
                        origin: (row, col),
                        row,
                        col,
                    })
                } else {
                    None
                }
            })
        }));
        Self { grid: *grid, queue }
    }

    fn total_trail_head_rating(self) -> usize {
        let mut rating = 0;
        for _head in self {
            rating += 1;
        }
        rating
    }

    fn total_trail_head_score(self) -> usize {
        let mut score = BTreeSet::new();
        for state in self {
            score.insert(state);
        }
        score.len()
    }
}

impl Iterator for TrailMap {
    type Item = TrailMapSearchState;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.queue.pop_front() {
            let height = self.grid[state.row][state.col];

            if height == Some(9) {
                return Some(state);
            }

            let climb = height.map(|h| h + 1);
            for candidate in state.neighbours() {
                if self.grid[candidate.row][candidate.col] == climb {
                    self.queue.push_back(candidate);
                }
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct ParseTrailMapError;

const fn parse_digit(ch: char) -> Result<u8, ParseTrailMapError> {
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
        _ => Err(ParseTrailMapError),
    }
}

impl FromStr for TrailMap {
    type Err = ParseTrailMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [[None; GRID_SIZE]; GRID_SIZE];
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let digit = parse_digit(ch)?;
                grid[row][col] = Some(digit);
            }
        }
        Ok(Self::new(&grid))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    TrailMap::from_str(input).map_or(None, |trail_map| Some(trail_map.total_trail_head_score()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    TrailMap::from_str(input).map_or(None, |trail_map| Some(trail_map.total_trail_head_rating()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_trail_map() -> TrailMap {
        let mut grid = [[None; GRID_SIZE]; GRID_SIZE];

        grid[0][0] = Some(8);
        grid[0][1] = Some(9);
        grid[0][2] = Some(0);
        grid[0][3] = Some(1);
        grid[0][4] = Some(0);
        grid[0][5] = Some(1);
        grid[0][6] = Some(2);
        grid[0][7] = Some(3);
        grid[1][0] = Some(7);
        grid[1][1] = Some(8);
        grid[1][2] = Some(1);
        grid[1][3] = Some(2);
        grid[1][4] = Some(1);
        grid[1][5] = Some(8);
        grid[1][6] = Some(7);
        grid[1][7] = Some(4);
        grid[2][0] = Some(8);
        grid[2][1] = Some(7);
        grid[2][2] = Some(4);
        grid[2][3] = Some(3);
        grid[2][4] = Some(0);
        grid[2][5] = Some(9);
        grid[2][6] = Some(6);
        grid[2][7] = Some(5);
        grid[3][0] = Some(9);
        grid[3][1] = Some(6);
        grid[3][2] = Some(5);
        grid[3][3] = Some(4);
        grid[3][4] = Some(9);
        grid[3][5] = Some(8);
        grid[3][6] = Some(7);
        grid[3][7] = Some(4);
        grid[4][0] = Some(4);
        grid[4][1] = Some(5);
        grid[4][2] = Some(6);
        grid[4][3] = Some(7);
        grid[4][4] = Some(8);
        grid[4][5] = Some(9);
        grid[4][6] = Some(0);
        grid[4][7] = Some(3);
        grid[5][0] = Some(3);
        grid[5][1] = Some(2);
        grid[5][2] = Some(0);
        grid[5][3] = Some(1);
        grid[5][4] = Some(9);
        grid[5][5] = Some(0);
        grid[5][6] = Some(1);
        grid[5][7] = Some(2);
        grid[6][0] = Some(0);
        grid[6][1] = Some(1);
        grid[6][2] = Some(3);
        grid[6][3] = Some(2);
        grid[6][4] = Some(9);
        grid[6][5] = Some(8);
        grid[6][6] = Some(0);
        grid[6][7] = Some(1);
        grid[7][0] = Some(1);
        grid[7][1] = Some(0);
        grid[7][2] = Some(4);
        grid[7][3] = Some(5);
        grid[7][4] = Some(6);
        grid[7][5] = Some(7);
        grid[7][6] = Some(3);
        grid[7][7] = Some(2);

        TrailMap::new(&grid)
    }

    #[test]
    fn test_trail_map_from_str() {
        assert_eq!(
            TrailMap::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_trail_map()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
