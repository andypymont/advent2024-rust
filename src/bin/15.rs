use std::str::FromStr;

advent_of_code::solution!(15);

const GRID_SIZE: usize = 50;

type Position = (usize, usize);
type Grid = [[Tile; GRID_SIZE]; GRID_SIZE];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn step_from(self, position: Position) -> Position {
        let row = match self {
            Self::North => position.0 - 1,
            Self::South => position.0 + 1,
            Self::East | Self::West => position.0,
        };
        let col = match self {
            Self::West => position.1 - 1,
            Self::East => position.1 + 1,
            Self::North | Self::South => position.1,
        };

        (row, col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Box,
}

const fn gps_coordinate(tile: Tile, row: usize, col: usize) -> usize {
    match tile {
        Tile::Box => (row * 100) + col,
        Tile::Wall | Tile::Empty => 0,
    }
}

fn gps_coordinate_total(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, tile)| gps_coordinate(*tile, r, c))
        })
        .sum()
}

#[derive(Debug, PartialEq)]
struct Warehouse {
    grid: Grid,
    instructions: Vec<Direction>,
    start: Position,
}

impl Warehouse {
    fn execute_instructions(mut self) -> Grid {
        let mut position = self.start;

        for direction in self.instructions {
            let mut push = Vec::new();
            let mut check = position;
            loop {
                check = direction.step_from(check);
                let tile = self.grid[check.0][check.1];
                if tile == Tile::Wall {
                    push.clear();
                    break;
                }
                push.push(check);
                if tile == Tile::Empty {
                    break;
                }
            }

            let mut push = push.into_iter();
            if let Some(step) = push.next() {
                position = step;
                self.grid[step.0][step.1] = Tile::Empty;
            }
            for pushed in push {
                self.grid[pushed.0][pushed.1] = Tile::Box;
            }
        }

        self.grid
    }
}

#[derive(Debug, PartialEq)]
struct ParseWarehouseError;

impl TryFrom<char> for Tile {
    type Error = ParseWarehouseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | '@' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '#' => Ok(Self::Wall),
            _ => Err(ParseWarehouseError),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseWarehouseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::North),
            '>' => Ok(Self::East),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            _ => Err(ParseWarehouseError),
        }
    }
}

impl FromStr for Warehouse {
    type Err = ParseWarehouseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((grid_str, instructions_str)) = input.split_once("\n\n") else {
            return Err(ParseWarehouseError);
        };

        let mut grid = [[Tile::Wall; GRID_SIZE]; GRID_SIZE];
        let mut start = Err(ParseWarehouseError);
        for (row, line) in grid_str.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '@' {
                    start = Ok((row, col));
                }
                let tile = Tile::try_from(ch)?;
                grid[row][col] = tile;
            }
        }
        let start = start?;

        let mut instructions = Vec::new();
        for ch in instructions_str.lines().flat_map(|line| line.chars()) {
            let direction = Direction::try_from(ch)?;
            instructions.push(direction);
        }

        Ok(Self {
            grid,
            instructions,
            start,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Warehouse::from_str(input).map_or(None, |warehouse| {
        Some(gps_coordinate_total(&warehouse.execute_instructions()))
    })
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn larger_example() -> Warehouse {
        let mut grid = [[Tile::Wall; GRID_SIZE]; GRID_SIZE];
        grid[1][1] = Tile::Empty;
        grid[1][2] = Tile::Empty;
        grid[1][3] = Tile::Box;
        grid[1][4] = Tile::Empty;
        grid[1][5] = Tile::Empty;
        grid[1][6] = Tile::Box;
        grid[1][7] = Tile::Empty;
        grid[1][8] = Tile::Box;
        grid[2][1] = Tile::Empty;
        grid[2][2] = Tile::Empty;
        grid[2][3] = Tile::Empty;
        grid[2][4] = Tile::Empty;
        grid[2][5] = Tile::Empty;
        grid[2][6] = Tile::Empty;
        grid[2][7] = Tile::Box;
        grid[2][8] = Tile::Empty;
        grid[3][1] = Tile::Empty;
        grid[3][2] = Tile::Box;
        grid[3][3] = Tile::Box;
        grid[3][4] = Tile::Empty;
        grid[3][5] = Tile::Empty;
        grid[3][6] = Tile::Box;
        grid[3][7] = Tile::Empty;
        grid[3][8] = Tile::Box;
        grid[4][1] = Tile::Empty;
        grid[4][2] = Tile::Empty;
        grid[4][3] = Tile::Box;
        grid[4][4] = Tile::Empty;
        grid[4][5] = Tile::Empty;
        grid[4][6] = Tile::Empty;
        grid[4][7] = Tile::Box;
        grid[4][8] = Tile::Empty;
        grid[5][1] = Tile::Box;
        grid[5][2] = Tile::Wall;
        grid[5][3] = Tile::Empty;
        grid[5][4] = Tile::Empty;
        grid[5][5] = Tile::Box;
        grid[5][6] = Tile::Empty;
        grid[5][7] = Tile::Empty;
        grid[5][8] = Tile::Empty;
        grid[6][1] = Tile::Box;
        grid[6][2] = Tile::Empty;
        grid[6][3] = Tile::Empty;
        grid[6][4] = Tile::Box;
        grid[6][5] = Tile::Empty;
        grid[6][6] = Tile::Empty;
        grid[6][7] = Tile::Box;
        grid[6][8] = Tile::Empty;
        grid[7][1] = Tile::Empty;
        grid[7][2] = Tile::Box;
        grid[7][3] = Tile::Box;
        grid[7][4] = Tile::Empty;
        grid[7][5] = Tile::Box;
        grid[7][6] = Tile::Empty;
        grid[7][7] = Tile::Box;
        grid[7][8] = Tile::Box;
        grid[8][1] = Tile::Empty;
        grid[8][2] = Tile::Empty;
        grid[8][3] = Tile::Empty;
        grid[8][4] = Tile::Empty;
        grid[8][5] = Tile::Box;
        grid[8][6] = Tile::Empty;
        grid[8][7] = Tile::Empty;
        grid[8][8] = Tile::Empty;

        Warehouse {
            grid,
            instructions: vec![
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::West,
                Direction::East,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::North,
            ],
            start: (4, 4),
        }
    }

    #[test]
    fn test_parse_warehouse() {
        assert_eq!(
            Warehouse::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(larger_example()),
        );
    }

    #[test]
    fn test_execute_instructions() {
        let mut grid = [[Tile::Wall; GRID_SIZE]; GRID_SIZE];
        grid[1][1] = Tile::Empty;
        grid[1][2] = Tile::Box;
        grid[1][3] = Tile::Empty;
        grid[1][4] = Tile::Box;
        grid[1][5] = Tile::Empty;
        grid[1][6] = Tile::Box;
        grid[1][7] = Tile::Box;
        grid[1][8] = Tile::Box;
        grid[2][1] = Tile::Empty;
        grid[2][2] = Tile::Empty;
        grid[2][3] = Tile::Empty;
        grid[2][4] = Tile::Empty;
        grid[2][5] = Tile::Empty;
        grid[2][6] = Tile::Empty;
        grid[2][7] = Tile::Empty;
        grid[2][8] = Tile::Empty;
        grid[3][1] = Tile::Box;
        grid[3][2] = Tile::Box;
        grid[3][3] = Tile::Empty;
        grid[3][4] = Tile::Empty;
        grid[3][5] = Tile::Empty;
        grid[3][6] = Tile::Empty;
        grid[3][7] = Tile::Empty;
        grid[3][8] = Tile::Empty;
        grid[4][1] = Tile::Box;
        grid[4][2] = Tile::Box;
        grid[4][3] = Tile::Empty;
        grid[4][4] = Tile::Empty;
        grid[4][5] = Tile::Empty;
        grid[4][6] = Tile::Empty;
        grid[4][7] = Tile::Empty;
        grid[4][8] = Tile::Empty;
        grid[5][1] = Tile::Box;
        grid[5][2] = Tile::Wall;
        grid[5][3] = Tile::Empty;
        grid[5][4] = Tile::Empty;
        grid[5][5] = Tile::Empty;
        grid[5][6] = Tile::Empty;
        grid[5][7] = Tile::Empty;
        grid[5][8] = Tile::Box;
        grid[6][1] = Tile::Box;
        grid[6][2] = Tile::Empty;
        grid[6][3] = Tile::Empty;
        grid[6][4] = Tile::Empty;
        grid[6][5] = Tile::Empty;
        grid[6][6] = Tile::Empty;
        grid[6][7] = Tile::Box;
        grid[6][8] = Tile::Box;
        grid[7][1] = Tile::Box;
        grid[7][2] = Tile::Empty;
        grid[7][3] = Tile::Empty;
        grid[7][4] = Tile::Empty;
        grid[7][5] = Tile::Empty;
        grid[7][6] = Tile::Empty;
        grid[7][7] = Tile::Box;
        grid[7][8] = Tile::Box;
        grid[8][1] = Tile::Box;
        grid[8][2] = Tile::Box;
        grid[8][3] = Tile::Empty;
        grid[8][4] = Tile::Empty;
        grid[8][5] = Tile::Empty;
        grid[8][6] = Tile::Empty;
        grid[8][7] = Tile::Box;
        grid[8][8] = Tile::Box;
        assert_eq!(larger_example().execute_instructions(), grid);
    }

    #[test]
    fn test_gps_coordinate() {
        assert_eq!(gps_coordinate(Tile::Box, 1, 4), 104);
        assert_eq!(gps_coordinate(Tile::Empty, 1, 4), 0);
        assert_eq!(gps_coordinate(Tile::Wall, 1, 4), 0);
        assert_eq!(gps_coordinate(Tile::Box, 5, 2), 502);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
