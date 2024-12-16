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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step_from(self, position: usize) -> Option<usize> {
        let row = position / GRID_SIZE;
        let col = position % GRID_SIZE;

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
        col.map(|col| (row * GRID_SIZE) + col)
    }

    const fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ReindeerState {
    score: u32,
    position: usize,
    facing: Direction,
    visited: [bool; GRID_SIZE * GRID_SIZE],
}

impl ReindeerState {
    fn initial(maze: &Maze) -> impl Iterator<Item = Self> + use<'_> {
        [
            (Direction::East, 0),
            (Direction::North, 1000),
            (Direction::South, 1000),
            (Direction::West, 2000),
        ]
        .into_iter()
        .map(|(facing, score)| {
            let mut visited = [false; GRID_SIZE * GRID_SIZE];
            visited[maze.start] = true;
            Self {
                score,
                position: maze.start,
                facing,
                visited,
            }
        })
    }

    fn next_states(&self, maze: &Maze) -> impl Iterator<Item = Self> + use<'_> {
        let empty: Box<dyn Iterator<Item = Self>> = Box::new(std::iter::empty());
        let Some(position) = self.facing.step_from(self.position) else {
            return empty;
        };
        if !maze.grid[position] {
            return empty;
        };
        Box::new(
            [
                (self.facing, 1),
                (self.facing.turn_left(), 1001),
                (self.facing.turn_right(), 1001),
            ]
            .into_iter()
            .map(move |(facing, extra_score)| {
                let mut visited = self.visited;
                visited[position] = true;
                Self {
                    score: self.score + extra_score,
                    position,
                    facing,
                    visited,
                }
            }),
        )
    }
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // this struct will go in a max heap, and we want to prioritise lower scores
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
    best: BTreeMap<(usize, Direction), u32>,
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
    grid: [bool; GRID_SIZE * GRID_SIZE],
    start: usize,
    end: usize,
}

impl Maze {
    fn best_path(&self) -> Option<u32> {
        let mut queue = ReindeerStateQueue::new();
        for state in ReindeerState::initial(self) {
            queue.push(state);
        }

        while let Some(state) = queue.pop() {
            if state.position == self.end {
                return Some(state.score);
            }

            for next in state.next_states(self) {
                queue.push(next);
            }
        }

        None
    }

    fn spaces_part_of_best_paths(&self) -> u32 {
        let mut best = u32::MAX;
        let mut seats = [false; GRID_SIZE * GRID_SIZE];

        let mut queue = ReindeerStateQueue::new();
        for state in ReindeerState::initial(self) {
            queue.push(state);
        }

        while let Some(state) = queue.pop() {
            if state.position == self.end {
                if state.score > best {
                    break;
                }
                best = state.score;
                for (pos, is_seat) in state.visited.iter().enumerate() {
                    seats[pos] = seats[pos] || *is_seat;
                }
                continue;
            }

            for next in state.next_states(self) {
                queue.push(next);
            }
        }

        seats.iter().map(|s| u32::from(*s)).sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [false; GRID_SIZE * GRID_SIZE];
        let mut start = Err(ParseMazeError);
        let mut end = Err(ParseMazeError);

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row * GRID_SIZE) + col;
                match ch {
                    '.' => grid[pos] = true,
                    'S' => {
                        grid[pos] = true;
                        start = Ok(pos);
                    }
                    'E' => {
                        grid[pos] = true;
                        end = Ok(pos);
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

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Maze::from_str(input).map_or(None, |maze| Some(maze.spaces_part_of_best_paths()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (GRID_SIZE * row) + col
    }

    fn example_maze() -> Maze {
        let mut grid = [false; GRID_SIZE * GRID_SIZE];
        grid[position(1, 1)] = true;
        grid[position(1, 2)] = true;
        grid[position(1, 3)] = true;
        grid[position(1, 4)] = true;
        grid[position(1, 5)] = true;
        grid[position(1, 6)] = true;
        grid[position(1, 7)] = true;
        grid[position(1, 9)] = true;
        grid[position(1, 10)] = true;
        grid[position(1, 11)] = true;
        grid[position(1, 12)] = true;
        grid[position(1, 13)] = true;
        grid[position(2, 1)] = true;
        grid[position(2, 3)] = true;
        grid[position(2, 7)] = true;
        grid[position(2, 9)] = true;
        grid[position(2, 13)] = true;
        grid[position(3, 1)] = true;
        grid[position(3, 2)] = true;
        grid[position(3, 3)] = true;
        grid[position(3, 4)] = true;
        grid[position(3, 5)] = true;
        grid[position(3, 7)] = true;
        grid[position(3, 9)] = true;
        grid[position(3, 10)] = true;
        grid[position(3, 11)] = true;
        grid[position(3, 13)] = true;
        grid[position(4, 1)] = true;
        grid[position(4, 5)] = true;
        grid[position(4, 11)] = true;
        grid[position(4, 13)] = true;
        grid[position(5, 1)] = true;
        grid[position(5, 3)] = true;
        grid[position(5, 5)] = true;
        grid[position(5, 6)] = true;
        grid[position(5, 7)] = true;
        grid[position(5, 8)] = true;
        grid[position(5, 9)] = true;
        grid[position(5, 10)] = true;
        grid[position(5, 11)] = true;
        grid[position(5, 13)] = true;
        grid[position(6, 1)] = true;
        grid[position(6, 3)] = true;
        grid[position(6, 9)] = true;
        grid[position(6, 13)] = true;
        grid[position(7, 1)] = true;
        grid[position(7, 2)] = true;
        grid[position(7, 3)] = true;
        grid[position(7, 4)] = true;
        grid[position(7, 5)] = true;
        grid[position(7, 6)] = true;
        grid[position(7, 7)] = true;
        grid[position(7, 8)] = true;
        grid[position(7, 9)] = true;
        grid[position(7, 10)] = true;
        grid[position(7, 11)] = true;
        grid[position(7, 13)] = true;
        grid[position(8, 3)] = true;
        grid[position(8, 5)] = true;
        grid[position(8, 11)] = true;
        grid[position(8, 13)] = true;
        grid[position(9, 1)] = true;
        grid[position(9, 2)] = true;
        grid[position(9, 3)] = true;
        grid[position(9, 5)] = true;
        grid[position(9, 6)] = true;
        grid[position(9, 7)] = true;
        grid[position(9, 8)] = true;
        grid[position(9, 9)] = true;
        grid[position(9, 11)] = true;
        grid[position(9, 13)] = true;
        grid[position(10, 1)] = true;
        grid[position(10, 3)] = true;
        grid[position(10, 5)] = true;
        grid[position(10, 9)] = true;
        grid[position(10, 11)] = true;
        grid[position(10, 13)] = true;
        grid[position(11, 1)] = true;
        grid[position(11, 2)] = true;
        grid[position(11, 3)] = true;
        grid[position(11, 4)] = true;
        grid[position(11, 5)] = true;
        grid[position(11, 7)] = true;
        grid[position(11, 8)] = true;
        grid[position(11, 9)] = true;
        grid[position(11, 11)] = true;
        grid[position(11, 13)] = true;
        grid[position(12, 1)] = true;
        grid[position(12, 5)] = true;
        grid[position(12, 7)] = true;
        grid[position(12, 9)] = true;
        grid[position(12, 11)] = true;
        grid[position(12, 13)] = true;
        grid[position(13, 1)] = true;
        grid[position(13, 2)] = true;
        grid[position(13, 3)] = true;
        grid[position(13, 5)] = true;
        grid[position(13, 6)] = true;
        grid[position(13, 7)] = true;
        grid[position(13, 8)] = true;
        grid[position(13, 9)] = true;
        grid[position(13, 11)] = true;
        grid[position(13, 12)] = true;
        grid[position(13, 13)] = true;
        Maze {
            grid,
            start: position(13, 1),
            end: position(1, 13),
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
        assert_eq!(result, Some(45));
    }
}
