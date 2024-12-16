use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::str::FromStr;

advent_of_code::solution!(16);

const GRID_SIZE: usize = 140;

const fn grid_add(lhs: usize, rhs: usize) -> Option<usize> {
    let rv = lhs + rhs;
    if rv >= GRID_SIZE {
        None
    } else {
        Some(rv)
    }
}

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step_from(self, (row, col): Position) -> Option<Position> {
        let row = match self {
            Self::North => row.checked_sub(1),
            Self::South => grid_add(row, 1),
            Self::West | Self::East => Some(row),
        };
        let row = row?;

        let col = match self {
            Self::West => col.checked_sub(1),
            Self::East => grid_add(col, 1),
            Self::North | Self::South => Some(col),
        };
        col.map(|col| (row, col))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ReindeerState {
    score: u32,
    position: Position,
    facing: Direction,
}

impl ReindeerState {
    fn step_forward(&self, maze: &Maze) -> Option<Self> {
        let position = self.facing.step_from(self.position)?;
        if maze.grid[position.0][position.1] {
            Some(Self {
                score: self.score + 1,
                position,
                facing: self.facing,
            })
        } else {
            None
        }
    }

    const fn turn_left(&self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            score: self.score + 1000,
            position: self.position,
            facing,
        }
    }

    const fn turn_right(&self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            score: self.score + 1000,
            position: self.position,
            facing,
        }
    }
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // this struct will go in a max heap, but we want to return lower scores first
        match self.score.cmp(&other.score) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => (self.position, self.facing).cmp(&(other.position, other.facing)),
        }
    }
}

impl PartialOrd for ReindeerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct ReindeerStateQueue {
    queue: BinaryHeap<ReindeerState>,
    best: BTreeMap<(Position, Direction), u32>,
}

impl ReindeerStateQueue {
    const fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            best: BTreeMap::new(),
        }
    }

    fn push(&mut self, state: ReindeerState) {
        let current = self
            .best
            .entry((state.position, state.facing))
            .and_modify(|s| *s -= s.saturating_sub(state.score))
            .or_insert(state.score);
        if state.score <= *current {
            self.queue.push(state);
        }
    }

    fn pop(&mut self) -> Option<ReindeerState> {
        self.queue.pop()
    }
}

#[derive(Debug, PartialEq)]
struct Maze {
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    start: Position,
    end: Position,
}

impl Maze {
    const fn initial_state(&self) -> ReindeerState {
        ReindeerState {
            score: 0,
            position: self.start,
            facing: Direction::East,
        }
    }

    fn best_path(&self) -> Option<u32> {
        let mut queue = ReindeerStateQueue::new();
        queue.push(self.initial_state());

        while let Some(state) = queue.pop() {
            if state.position == self.end {
                return Some(state.score);
            }

            if let Some(forward) = state.step_forward(self) {
                if self.grid[forward.position.0][forward.position.1] {
                    queue.push(forward);
                }
            }
            queue.push(state.turn_left());
            queue.push(state.turn_right());
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
        let mut start = Err(ParseMazeError);
        let mut end = Err(ParseMazeError);

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' => grid[row][col] = true,
                    'S' => {
                        grid[row][col] = true;
                        start = Ok((row, col));
                    }
                    'E' => {
                        grid[row][col] = true;
                        end = Ok((row, col));
                    }
                    '#' => (),
                    _ => return Err(ParseMazeError),
                }
            }
        }

        let start = start?;
        let end = end?;

        Ok(Self { grid, start, end })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Maze::from_str(input).map_or(None, |maze| maze.best_path())
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_maze() -> Maze {
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
        grid[1][1] = true;
        grid[1][2] = true;
        grid[1][3] = true;
        grid[1][4] = true;
        grid[1][5] = true;
        grid[1][6] = true;
        grid[1][7] = true;
        grid[1][9] = true;
        grid[1][10] = true;
        grid[1][11] = true;
        grid[1][12] = true;
        grid[1][13] = true;
        grid[2][1] = true;
        grid[2][3] = true;
        grid[2][7] = true;
        grid[2][9] = true;
        grid[2][13] = true;
        grid[3][1] = true;
        grid[3][2] = true;
        grid[3][3] = true;
        grid[3][4] = true;
        grid[3][5] = true;
        grid[3][7] = true;
        grid[3][9] = true;
        grid[3][10] = true;
        grid[3][11] = true;
        grid[3][13] = true;
        grid[4][1] = true;
        grid[4][5] = true;
        grid[4][11] = true;
        grid[4][13] = true;
        grid[5][1] = true;
        grid[5][3] = true;
        grid[5][5] = true;
        grid[5][6] = true;
        grid[5][7] = true;
        grid[5][8] = true;
        grid[5][9] = true;
        grid[5][10] = true;
        grid[5][11] = true;
        grid[5][13] = true;
        grid[6][1] = true;
        grid[6][3] = true;
        grid[6][9] = true;
        grid[6][13] = true;
        grid[7][1] = true;
        grid[7][2] = true;
        grid[7][3] = true;
        grid[7][4] = true;
        grid[7][5] = true;
        grid[7][6] = true;
        grid[7][7] = true;
        grid[7][8] = true;
        grid[7][9] = true;
        grid[7][10] = true;
        grid[7][11] = true;
        grid[7][13] = true;
        grid[8][3] = true;
        grid[8][5] = true;
        grid[8][11] = true;
        grid[8][13] = true;
        grid[9][1] = true;
        grid[9][2] = true;
        grid[9][3] = true;
        grid[9][5] = true;
        grid[9][6] = true;
        grid[9][7] = true;
        grid[9][8] = true;
        grid[9][9] = true;
        grid[9][11] = true;
        grid[9][13] = true;
        grid[10][1] = true;
        grid[10][3] = true;
        grid[10][5] = true;
        grid[10][9] = true;
        grid[10][11] = true;
        grid[10][13] = true;
        grid[11][1] = true;
        grid[11][2] = true;
        grid[11][3] = true;
        grid[11][4] = true;
        grid[11][5] = true;
        grid[11][7] = true;
        grid[11][8] = true;
        grid[11][9] = true;
        grid[11][11] = true;
        grid[11][13] = true;
        grid[12][1] = true;
        grid[12][5] = true;
        grid[12][7] = true;
        grid[12][9] = true;
        grid[12][11] = true;
        grid[12][13] = true;
        grid[13][1] = true;
        grid[13][2] = true;
        grid[13][3] = true;
        grid[13][5] = true;
        grid[13][6] = true;
        grid[13][7] = true;
        grid[13][8] = true;
        grid[13][9] = true;
        grid[13][11] = true;
        grid[13][12] = true;
        grid[13][13] = true;
        Maze {
            grid,
            start: (13, 1),
            end: (1, 13),
        }
    }

    #[test]
    fn test_parse_maze() {
        assert_eq!(
            Maze::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_maze()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
