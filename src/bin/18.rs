use std::collections::VecDeque;

advent_of_code::solution!(18);

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
struct Grid {
    height: usize,
    width: usize,
    corrupted: usize,
    cells: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct GridTravelState {
    position: usize,
    steps: usize,
}

struct GridTravelStateQueue {
    visited: Vec<bool>,
    queue: VecDeque<GridTravelState>,
}

impl GridTravelStateQueue {
    fn new(height: usize, width: usize) -> Self {
        let visited = vec![false; height * width];
        let mut queue = VecDeque::new();
        queue.push_back(GridTravelState {
            position: 0,
            steps: 0,
        });
        Self { visited, queue }
    }

    fn push(&mut self, state: GridTravelState) {
        if !self.visited[state.position] {
            self.visited[state.position] = true;
            self.queue.push_back(state);
        }
    }

    fn pop(&mut self) -> Option<GridTravelState> {
        self.queue.pop_front()
    }
}

impl Grid {
    fn step(&self, position: usize, direction: Direction) -> Option<usize> {
        let row = position / self.width;
        let col = position % self.width;

        let row = match direction {
            Direction::North => row.checked_sub(1),
            Direction::South => {
                let south = row + 1;
                if south >= self.height {
                    None
                } else {
                    Some(south)
                }
            }
            Direction::East | Direction::West => Some(row),
        };
        let row = row?;

        let col = match direction {
            Direction::West => col.checked_sub(1),
            Direction::East => {
                let east = col + 1;
                if east >= self.width {
                    None
                } else {
                    Some(east)
                }
            }
            Direction::North | Direction::South => Some(col),
        };
        col.map(|col| (row * self.width) + col)
    }

    fn neighbours(&self, position: usize) -> impl Iterator<Item = usize> + use<'_> {
        COMPASS
            .into_iter()
            .filter_map(move |direction| self.step(position, direction))
    }

    fn shortest_path_after(&self, nanoseconds: usize) -> Option<usize> {
        let goal = (self.height * self.width) - 1;
        let mut queue = GridTravelStateQueue::new(self.height, self.width);

        while let Some(state) = queue.pop() {
            if state.position == goal {
                return Some(state.steps);
            }

            for position in self.neighbours(state.position) {
                if self.cells[position] > nanoseconds {
                    queue.push(GridTravelState {
                        position,
                        steps: state.steps + 1,
                    });
                }
            }
        }

        None
    }

    fn first_coordinate_blocking_exit(&self) -> Option<(usize, usize)> {
        // binary search
        let mut lower = 0;
        let mut upper = self.corrupted;

        while lower < upper {
            let mid = (lower + upper) / 2;
            if self.shortest_path_after(mid).is_none() {
                upper = mid;
            } else {
                lower = mid + 1;
            }
        }

        self.cells
            .iter()
            .position(|cell| *cell == upper)
            .map(|pos| {
                let row = pos / self.width;
                let col = pos % self.width;
                (col, row)
            })
    }
}

#[derive(Debug, PartialEq)]
struct ParseGridError;

impl Grid {
    fn from_input(input: &str, height: usize, width: usize) -> Result<Self, ParseGridError> {
        let mut cells = vec![usize::MAX; height * width];
        let mut corrupted = 0;

        for (nanosec, line) in (1..).zip(input.lines()) {
            let Some((x, y)) = line.split_once(',') else {
                return Err(ParseGridError);
            };
            let x: usize = x.parse().map_err(|_| ParseGridError)?;
            let y: usize = y.parse().map_err(|_| ParseGridError)?;
            cells[(y * width) + x] = nanosec;
            corrupted = nanosec;
        }

        Ok(Self {
            height,
            width,
            corrupted,
            cells,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Grid::from_input(input, 71, 71).map_or(None, |grid| grid.shortest_path_after(1024))
}

#[must_use]
pub fn part_two(input: &str) -> Option<String> {
    Grid::from_input(input, 71, 71).map_or(None, |grid| {
        grid.first_coordinate_blocking_exit()
            .map(|coords| format!("{},{}", coords.0, coords.1))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(x: usize, y: usize) -> usize {
        (y * 7) + x
    }

    fn example_grid() -> Grid {
        let mut cells = vec![usize::MAX; 7 * 7];
        cells[position(5, 4)] = 1;
        cells[position(4, 2)] = 2;
        cells[position(4, 5)] = 3;
        cells[position(3, 0)] = 4;
        cells[position(2, 1)] = 5;
        cells[position(6, 3)] = 6;
        cells[position(2, 4)] = 7;
        cells[position(1, 5)] = 8;
        cells[position(0, 6)] = 9;
        cells[position(3, 3)] = 10;
        cells[position(2, 6)] = 11;
        cells[position(5, 1)] = 12;
        cells[position(1, 2)] = 13;
        cells[position(5, 5)] = 14;
        cells[position(2, 5)] = 15;
        cells[position(6, 5)] = 16;
        cells[position(1, 4)] = 17;
        cells[position(0, 4)] = 18;
        cells[position(6, 4)] = 19;
        cells[position(1, 1)] = 20;
        cells[position(6, 1)] = 21;
        cells[position(1, 0)] = 22;
        cells[position(0, 5)] = 23;
        cells[position(1, 6)] = 24;
        cells[position(2, 0)] = 25;

        Grid {
            height: 7,
            width: 7,
            corrupted: 25,
            cells,
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Grid::from_input(&advent_of_code::template::read_file("examples", DAY), 7, 7),
            Ok(example_grid()),
        );
    }

    #[test]
    fn test_parse_neighbours() {
        let grid = example_grid();
        assert_eq!(
            grid.neighbours(position(0, 0)).collect::<Vec<usize>>(),
            vec![position(1, 0), position(0, 1)],
        );
        assert_eq!(
            grid.neighbours(position(2, 2)).collect::<Vec<usize>>(),
            vec![
                position(2, 1),
                position(3, 2),
                position(2, 3),
                position(1, 2)
            ],
        );
    }

    #[test]
    fn test_shortest_path_after() {
        assert_eq!(example_grid().shortest_path_after(12), Some(22))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(146))
    }

    #[test]
    fn test_first_coordinate_blocking_exit() {
        assert_eq!(
            example_grid().first_coordinate_blocking_exit(),
            Some((6, 1))
        );
    }
}
