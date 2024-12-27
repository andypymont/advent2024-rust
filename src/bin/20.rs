use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(20);

const GRID_SIZE: usize = 140;

const fn taxicab_distance(first: usize, second: usize) -> usize {
    (first / GRID_SIZE).abs_diff(second / GRID_SIZE)
        + (first % GRID_SIZE).abs_diff(second % GRID_SIZE)
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Maze {
    walls: Vec<bool>,
    start: usize,
    end: usize,
}

impl Maze {
    fn step_from(position: usize, direction: Direction) -> Option<usize> {
        let row = position / GRID_SIZE;
        let col = position % GRID_SIZE;

        let row = match direction {
            Direction::North => row.checked_sub(1),
            Direction::South => {
                let south = row + 1;
                if south >= GRID_SIZE {
                    None
                } else {
                    Some(south)
                }
            }
            Direction::West | Direction::East => Some(row),
        };
        let row = row?;

        let col = match direction {
            Direction::West => col.checked_sub(1),
            Direction::East => {
                let east = col + 1;
                if east >= GRID_SIZE {
                    None
                } else {
                    Some(east)
                }
            }
            Direction::North | Direction::South => Some(col),
        };
        col.map(|col| (row * GRID_SIZE) + col)
    }

    fn open_neighbours(&self, position: usize) -> impl Iterator<Item = usize> + use<'_> {
        COMPASS.into_iter().filter_map(move |direction| {
            Self::step_from(position, direction).and_then(|pos| {
                if self.walls[pos] {
                    None
                } else {
                    Some(pos)
                }
            })
        })
    }

    fn distances_from_start(&self) -> Vec<Option<usize>> {
        let mut distance = vec![None; GRID_SIZE * GRID_SIZE];
        let mut queue = VecDeque::new();
        queue.push_back((self.end, 0));
        while let Some((position, steps)) = queue.pop_front() {
            if steps < distance[position].unwrap_or(usize::MAX) {
                distance[position] = Some(steps);
                for neighbour in self.open_neighbours(position) {
                    queue.push_back((neighbour, steps + 1));
                }
            }
        }
        distance
    }

    fn find_cheats(&self, max_cheat: usize, min_saving: usize) -> usize {
        let distance = self.distances_from_start();
        let mut count = 0;
        for (i, first) in distance.iter().enumerate() {
            for (j, second) in distance.iter().enumerate().skip(i) {
                let Some(first) = first else {
                    continue;
                };
                let Some(second) = second else {
                    continue;
                };
                let dist = taxicab_distance(i, j);
                if dist > max_cheat {
                    continue;
                }
                let (first, second) = if first > second {
                    (second, first)
                } else {
                    (first, second)
                };
                if second.saturating_sub(first + dist) >= min_saving {
                    count += 1;
                }
            }
        }

        count
    }
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut walls = vec![true; GRID_SIZE * GRID_SIZE];
        let mut start = Err(ParseMazeError);
        let mut end = Err(ParseMazeError);

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row * GRID_SIZE) + col;
                match ch {
                    '.' => walls[pos] = false,
                    'S' => {
                        start = Ok(pos);
                        walls[pos] = false;
                    }
                    'E' => {
                        end = Ok(pos);
                        walls[pos] = false;
                    }
                    '#' => (),
                    _ => return Err(ParseMazeError),
                }
            }
        }

        let start = start?;
        let end = end?;
        Ok(Self { walls, start, end })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Maze::from_str(input)
        .ok()
        .map(|maze| maze.find_cheats(2, 100))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Maze::from_str(input)
        .ok()
        .map(|maze| maze.find_cheats(20, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example_maze() -> Maze {
        let mut walls = vec![true; GRID_SIZE * GRID_SIZE];
        walls[position(1, 1)] = false;
        walls[position(1, 2)] = false;
        walls[position(1, 3)] = false;
        walls[position(1, 5)] = false;
        walls[position(1, 6)] = false;
        walls[position(1, 7)] = false;
        walls[position(1, 9)] = false;
        walls[position(1, 10)] = false;
        walls[position(1, 11)] = false;
        walls[position(1, 12)] = false;
        walls[position(1, 13)] = false;
        walls[position(2, 1)] = false;
        walls[position(2, 3)] = false;
        walls[position(2, 5)] = false;
        walls[position(2, 7)] = false;
        walls[position(2, 9)] = false;
        walls[position(2, 13)] = false;
        walls[position(3, 1)] = false;
        walls[position(3, 3)] = false;
        walls[position(3, 4)] = false;
        walls[position(3, 5)] = false;
        walls[position(3, 7)] = false;
        walls[position(3, 9)] = false;
        walls[position(3, 11)] = false;
        walls[position(3, 12)] = false;
        walls[position(3, 13)] = false;
        walls[position(4, 7)] = false;
        walls[position(4, 9)] = false;
        walls[position(4, 11)] = false;
        walls[position(5, 7)] = false;
        walls[position(5, 9)] = false;
        walls[position(5, 11)] = false;
        walls[position(5, 12)] = false;
        walls[position(5, 13)] = false;
        walls[position(6, 7)] = false;
        walls[position(6, 9)] = false;
        walls[position(6, 13)] = false;
        walls[position(7, 3)] = false;
        walls[position(7, 4)] = false;
        walls[position(7, 5)] = false;
        walls[position(7, 7)] = false;
        walls[position(7, 8)] = false;
        walls[position(7, 9)] = false;
        walls[position(7, 11)] = false;
        walls[position(7, 12)] = false;
        walls[position(7, 13)] = false;
        walls[position(8, 3)] = false;
        walls[position(8, 11)] = false;
        walls[position(9, 1)] = false;
        walls[position(9, 2)] = false;
        walls[position(9, 3)] = false;
        walls[position(9, 7)] = false;
        walls[position(9, 8)] = false;
        walls[position(9, 9)] = false;
        walls[position(9, 11)] = false;
        walls[position(9, 12)] = false;
        walls[position(9, 13)] = false;
        walls[position(10, 1)] = false;
        walls[position(10, 7)] = false;
        walls[position(10, 9)] = false;
        walls[position(10, 13)] = false;
        walls[position(11, 1)] = false;
        walls[position(11, 3)] = false;
        walls[position(11, 4)] = false;
        walls[position(11, 5)] = false;
        walls[position(11, 7)] = false;
        walls[position(11, 9)] = false;
        walls[position(11, 11)] = false;
        walls[position(11, 12)] = false;
        walls[position(11, 13)] = false;
        walls[position(12, 1)] = false;
        walls[position(12, 3)] = false;
        walls[position(12, 5)] = false;
        walls[position(12, 7)] = false;
        walls[position(12, 9)] = false;
        walls[position(12, 11)] = false;
        walls[position(13, 1)] = false;
        walls[position(13, 2)] = false;
        walls[position(13, 3)] = false;
        walls[position(13, 5)] = false;
        walls[position(13, 6)] = false;
        walls[position(13, 7)] = false;
        walls[position(13, 9)] = false;
        walls[position(13, 10)] = false;
        walls[position(13, 11)] = false;

        let start = position(3, 1);
        let end = position(7, 5);

        Maze { walls, start, end }
    }

    #[test]
    fn test_parse_maze() {
        assert_eq!(
            Maze::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_maze()),
        );
    }

    #[test]
    fn test_taxicab_distance() {
        assert_eq!(taxicab_distance(position(4, 7), position(2, 2)), 7);
        assert_eq!(taxicab_distance(position(2, 1), position(9, 8)), 14);
        assert_eq!(taxicab_distance(position(1, 1), position(1, 1)), 0);
    }

    #[test]
    fn test_distances_from_start() {
        let distances = example_maze().distances_from_start();
        assert_eq!(distances[position(3, 1)], Some(84));
        assert_eq!(distances[position(1, 1)], Some(82));
        assert_eq!(distances[position(1, 3)], Some(80));
        assert_eq!(distances[position(7, 5)], Some(0));
        assert_eq!(distances[position(2, 2)], None);
    }

    #[test]
    fn test_find_cheats() {
        let maze = example_maze();
        assert_eq!(maze.find_cheats(2, 2), 44);
        assert_eq!(maze.find_cheats(2, 3), 30);
        assert_eq!(maze.find_cheats(2, 64), 1);
        assert_eq!(maze.find_cheats(20, 76), 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
