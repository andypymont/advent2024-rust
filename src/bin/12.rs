use std::collections::{BTreeSet, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(12);

const GRID_SIZE: usize = 140;

type Grid = [[Option<char>; GRID_SIZE]; GRID_SIZE];

const fn grid_add(lhs: usize, rhs: usize) -> Option<usize> {
    let check = lhs + rhs;
    if check >= GRID_SIZE {
        None
    } else {
        Some(check)
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
    fn step_from(self, row: usize, col: usize) -> Option<(usize, usize)> {
        let row = match self {
            Self::North => row.checked_sub(1),
            Self::East | Self::West => Some(row),
            Self::South => grid_add(row, 1),
        };
        let row = row?;

        let col = match self {
            Self::North | Self::South => Some(col),
            Self::East => grid_add(col, 1),
            Self::West => col.checked_sub(1),
        };
        col.map(|col| (row, col))
    }
}

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, PartialEq)]
struct Region {
    plant: char,
    area: usize,
    sides: BTreeSet<(Direction, usize, usize)>,
}

impl Region {
    fn distinct_sides(&self) -> usize {
        self.sides
            .iter()
            .filter(|(direction, row, col)| {
                let left_dir = match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };
                left_dir
                    .step_from(*row, *col)
                    .map_or(true, |(r, c)| !self.sides.contains(&(*direction, r, c)))
            })
            .count()
    }
}

#[derive(Debug, PartialEq)]
struct Farm {
    grid: Grid,
}

impl Farm {
    fn find_region(
        &self,
        row: usize,
        col: usize,
        visited: &mut [[bool; GRID_SIZE]; GRID_SIZE],
    ) -> Option<Region> {
        let plant = self.grid[row][col]?;
        let mut sides = BTreeSet::new();
        let mut area = 0;
        let mut queue = VecDeque::new();
        queue.push_back((row, col));

        while let Some((row, col)) = queue.pop_front() {
            if visited[row][col] {
                continue;
            }
            visited[row][col] = true;
            area += 1;

            for direction in COMPASS {
                let Some((new_row, new_col)) = direction.step_from(row, col) else {
                    // No neighbour on this side == edge of grid
                    sides.insert((direction, row, col));
                    continue;
                };
                let Some(other) = self.grid[new_row][new_col] else {
                    // Empty space on this side == edge of grid
                    sides.insert((direction, row, col));
                    continue;
                };

                if other == plant {
                    // matching plant; part of area
                    queue.push_back((new_row, new_col));
                } else {
                    // different plant == edge of this area
                    sides.insert((direction, row, col));
                }
            }
        }

        Some(Region { plant, area, sides })
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited = [[false; GRID_SIZE]; GRID_SIZE];

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if visited[row][col] {
                    continue;
                }
                if let Some(region) = self.find_region(row, col, &mut visited) {
                    regions.push(region);
                }
            }
        }

        regions
    }
}

#[derive(Debug, PartialEq)]
struct ParseFarmError;

impl FromStr for Farm {
    type Err = ParseFarmError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [[None; GRID_SIZE]; GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                grid[row][col] = Some(ch);
            }
        }

        Ok(Self { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Farm::from_str(input).ok().map(|farm| {
        farm.find_regions()
            .iter()
            .map(|r| r.area * r.sides.len())
            .sum()
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Farm::from_str(input).ok().map(|farm| {
        farm.find_regions()
            .iter()
            .map(|r| r.area * r.distinct_sides())
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_farm() -> Farm {
        let mut grid = [[None; GRID_SIZE]; GRID_SIZE];

        grid[0][0] = Some('R');
        grid[0][1] = Some('R');
        grid[0][2] = Some('R');
        grid[0][3] = Some('R');
        grid[0][4] = Some('I');
        grid[0][5] = Some('I');
        grid[0][6] = Some('C');
        grid[0][7] = Some('C');
        grid[0][8] = Some('F');
        grid[0][9] = Some('F');
        grid[1][0] = Some('R');
        grid[1][1] = Some('R');
        grid[1][2] = Some('R');
        grid[1][3] = Some('R');
        grid[1][4] = Some('I');
        grid[1][5] = Some('I');
        grid[1][6] = Some('C');
        grid[1][7] = Some('C');
        grid[1][8] = Some('C');
        grid[1][9] = Some('F');
        grid[2][0] = Some('V');
        grid[2][1] = Some('V');
        grid[2][2] = Some('R');
        grid[2][3] = Some('R');
        grid[2][4] = Some('R');
        grid[2][5] = Some('C');
        grid[2][6] = Some('C');
        grid[2][7] = Some('F');
        grid[2][8] = Some('F');
        grid[2][9] = Some('F');
        grid[3][0] = Some('V');
        grid[3][1] = Some('V');
        grid[3][2] = Some('R');
        grid[3][3] = Some('C');
        grid[3][4] = Some('C');
        grid[3][5] = Some('C');
        grid[3][6] = Some('J');
        grid[3][7] = Some('F');
        grid[3][8] = Some('F');
        grid[3][9] = Some('F');
        grid[4][0] = Some('V');
        grid[4][1] = Some('V');
        grid[4][2] = Some('V');
        grid[4][3] = Some('V');
        grid[4][4] = Some('C');
        grid[4][5] = Some('J');
        grid[4][6] = Some('J');
        grid[4][7] = Some('C');
        grid[4][8] = Some('F');
        grid[4][9] = Some('E');
        grid[5][0] = Some('V');
        grid[5][1] = Some('V');
        grid[5][2] = Some('I');
        grid[5][3] = Some('V');
        grid[5][4] = Some('C');
        grid[5][5] = Some('C');
        grid[5][6] = Some('J');
        grid[5][7] = Some('J');
        grid[5][8] = Some('E');
        grid[5][9] = Some('E');
        grid[6][0] = Some('V');
        grid[6][1] = Some('V');
        grid[6][2] = Some('I');
        grid[6][3] = Some('I');
        grid[6][4] = Some('I');
        grid[6][5] = Some('C');
        grid[6][6] = Some('J');
        grid[6][7] = Some('J');
        grid[6][8] = Some('E');
        grid[6][9] = Some('E');
        grid[7][0] = Some('M');
        grid[7][1] = Some('I');
        grid[7][2] = Some('I');
        grid[7][3] = Some('I');
        grid[7][4] = Some('I');
        grid[7][5] = Some('I');
        grid[7][6] = Some('J');
        grid[7][7] = Some('J');
        grid[7][8] = Some('E');
        grid[7][9] = Some('E');
        grid[8][0] = Some('M');
        grid[8][1] = Some('I');
        grid[8][2] = Some('I');
        grid[8][3] = Some('I');
        grid[8][4] = Some('S');
        grid[8][5] = Some('I');
        grid[8][6] = Some('J');
        grid[8][7] = Some('E');
        grid[8][8] = Some('E');
        grid[8][9] = Some('E');
        grid[9][0] = Some('M');
        grid[9][1] = Some('M');
        grid[9][2] = Some('M');
        grid[9][3] = Some('I');
        grid[9][4] = Some('S');
        grid[9][5] = Some('S');
        grid[9][6] = Some('J');
        grid[9][7] = Some('E');
        grid[9][8] = Some('E');
        grid[9][9] = Some('E');

        Farm { grid }
    }

    #[test]
    fn test_parse_farm() {
        assert_eq!(
            Farm::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_farm()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
