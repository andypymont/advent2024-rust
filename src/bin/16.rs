use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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

    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
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
        .map(|(facing, score)| Self {
            score,
            position: maze.start,
            facing,
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
            .map(move |(facing, extra_score)| Self {
                score: self.score + extra_score,
                position,
                facing,
            }),
        )
    }

    fn previous_states(&self) -> impl Iterator<Item = Self> + use<'_> {
        let left = self.facing.turn_left();
        let right = self.facing.turn_right();
        let opposite = self.facing.opposite();

        [
            (left, right, 1001),
            (opposite, self.facing, 1),
            (right, left, 1001),
        ]
        .into_iter()
        .filter_map(move |(step, facing, less_score)| {
            let position = step.step_from(self.position);
            position.map(|position| Self {
                score: self.score.saturating_sub(less_score),
                position,
                facing,
            })
        })
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
    best: Vec<u32>,
}

impl ReindeerStateQueue {
    fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            best: vec![u32::MAX; 4 * GRID_SIZE * GRID_SIZE],
        }
    }

    fn pop(&mut self) -> Option<ReindeerState> {
        self.queue.pop()
    }

    fn push(&mut self, state: ReindeerState) {
        let dir = match state.facing {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        let current = self.best[(state.position * 4) + dir];
        if state.score <= current {
            self.best[(state.position * 4) + dir] = state.score;
            self.queue.push(state);
        }
    }

    fn contains_exact(&self, state: &ReindeerState) -> bool {
        let dir = match state.facing {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        self.best[(state.position * 4) + dir] == state.score
    }

    fn count_reverse_paths(&self, maze: &Maze, score: u32) -> u32 {
        let mut queue = BinaryHeap::new();

        let position = maze.end;
        for facing in COMPASS {
            let state = ReindeerState {
                score,
                position,
                facing,
            };
            if self.contains_exact(&state) {
                queue.push(state);
            }
        }

        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        while let Some(state) = queue.pop() {
            visited[state.position] = true;
            if state.position == maze.start {
                continue;
            }
            for state in state.previous_states() {
                if self.contains_exact(&state) {
                    queue.push(state);
                }
            }
        }

        visited.into_iter().map(u32::from).sum()
    }
}

#[derive(Debug, PartialEq)]
struct Maze {
    grid: Vec<bool>,
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

    fn spaces_in_best_paths(&self) -> u32 {
        let mut best = u32::MAX;
        let mut queue = ReindeerStateQueue::new();
        for state in ReindeerState::initial(self) {
            queue.push(state);
        }

        while let Some(state) = queue.pop() {
            if state.score > best {
                break;
            }

            if state.position == self.end {
                best = state.score;
                continue;
            }

            for next in state.next_states(self) {
                queue.push(next);
            }
        }

        queue.count_reverse_paths(self, best)
    }
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![false; GRID_SIZE * GRID_SIZE];
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
    Maze::from_str(input).ok().and_then(|maze| maze.best_path())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Maze::from_str(input)
        .ok()
        .map(|maze| maze.spaces_in_best_paths())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (GRID_SIZE * row) + col
    }

    fn example_maze() -> Maze {
        let mut grid = vec![false; GRID_SIZE * GRID_SIZE];
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
