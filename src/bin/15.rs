advent_of_code::solution!(15);

const GRID_SIZE: usize = 100;

type Position = (usize, usize);
type Grid = Vec<Vec<Tile>>;

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
    Box(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WarehouseBox {
    Small(usize, usize),
    Large(usize, usize),
}

impl WarehouseBox {
    fn gps_coordinate(&self) -> usize {
        match self {
            Self::Small(r, c) | Self::Large(r, c) => (r * 100) + c,
        }
    }

    const fn move_in_direction(&self, direction: Direction) -> Self {
        match self {
            Self::Small(r, c) => {
                let (r, c) = direction.step_from((*r, *c));
                Self::Small(r, c)
            }
            Self::Large(r, c) => {
                let (r, c) = direction.step_from((*r, *c));
                Self::Large(r, c)
            }
        }
    }

    const fn left(&self) -> Position {
        match self {
            Self::Small(r, c) | Self::Large(r, c) => (*r, *c),
        }
    }

    fn right(&self) -> Position {
        match self {
            Self::Small(r, c) => (*r, *c),
            Self::Large(r, c) => (*r, c + 1),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Warehouse {
    grid: Grid,
    boxes: Vec<WarehouseBox>,
    instructions: Vec<Direction>,
    start: Position,
}

impl Warehouse {
    fn get(&self, row: usize, col: usize) -> Tile {
        self.grid
            .get(row)
            .map_or(Tile::Wall, |row| *row.get(col).unwrap_or(&Tile::Wall))
    }

    fn push_box(
        &self,
        ix: usize,
        direction: Direction,
        pushes: &mut Vec<Option<(WarehouseBox, WarehouseBox)>>,
    ) -> bool {
        if pushes[ix].is_some() {
            return true;
        }

        let before = self.boxes[ix];
        let after = before.move_in_direction(direction);
        pushes[ix] = Some((before, after));

        let (left, right) = match direction {
            Direction::East => {
                let check = after.right();
                (check, check)
            }
            Direction::West => {
                let check = after.left();
                (check, check)
            }
            Direction::South | Direction::North => (after.left(), after.right()),
        };
        let left = self.grid[left.0][left.1];
        let right = self.grid[right.0][right.1];

        match (left, right) {
            (Tile::Wall, _) | (_, Tile::Wall) => false,
            (Tile::Empty, Tile::Empty) => true,
            (Tile::Box(a), Tile::Empty) | (Tile::Empty, Tile::Box(a)) => {
                self.push_box(a, direction, pushes)
            }
            (Tile::Box(a), Tile::Box(b)) => {
                if a == b {
                    self.push_box(a, direction, pushes)
                } else {
                    self.push_box(a, direction, pushes) && self.push_box(b, direction, pushes)
                }
            }
        }
    }

    fn execute_instructions(mut self) -> Vec<WarehouseBox> {
        let mut position = self.start;

        for direction in &self.instructions {
            let mut pushes = vec![None; self.boxes.len()];
            let check = direction.step_from(position);
            let step = match self.get(check.0, check.1) {
                Tile::Wall => false,
                Tile::Empty => true,
                Tile::Box(ix) => self.push_box(ix, *direction, &mut pushes),
            };

            if !step {
                continue;
            }

            for (ix, push) in pushes.iter().enumerate() {
                if let Some((before, after)) = push {
                    let (r, c) = before.left();
                    if self.grid[r][c] == Tile::Box(ix) {
                        self.grid[r][c] = Tile::Empty;
                    }
                    let (r, c) = before.right();
                    if self.grid[r][c] == Tile::Box(ix) {
                        self.grid[r][c] = Tile::Empty;
                    }

                    self.boxes[ix] = *after;
                    let (r, c) = after.left();
                    self.grid[r][c] = Tile::Box(ix);
                    let (r, c) = after.right();
                    self.grid[r][c] = Tile::Box(ix);
                }
            }

            position = check;
        }

        self.boxes
    }
}

#[derive(Debug, PartialEq)]
struct ParseWarehouseError;

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

impl Warehouse {
    fn from_input(input: &str, explode: bool) -> Result<Self, ParseWarehouseError> {
        let Some((grid_str, instructions_str)) = input.split_once("\n\n") else {
            return Err(ParseWarehouseError);
        };

        let mut grid = vec![vec![Tile::Wall; GRID_SIZE]; GRID_SIZE];
        let mut boxes = Vec::new();
        let mut start = Err(ParseWarehouseError);
        for (row, line) in grid_str.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '@' {
                    start = Ok(if explode { (row, col * 2) } else { (row, col) });
                }
                let tile = match ch {
                    '.' | '@' => Tile::Empty,
                    'O' => {
                        let ix = boxes.len();
                        boxes.push(if explode {
                            WarehouseBox::Large(row, col * 2)
                        } else {
                            WarehouseBox::Small(row, col)
                        });
                        Tile::Box(ix)
                    }
                    '#' => Tile::Wall,
                    _ => return Err(ParseWarehouseError),
                };
                if explode {
                    grid[row][col * 2] = tile;
                    grid[row][(col * 2) + 1] = tile;
                } else {
                    grid[row][col] = tile;
                }
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
            boxes,
            instructions,
            start,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Warehouse::from_input(input, false).ok().map(|warehouse| {
        warehouse
            .execute_instructions()
            .iter()
            .map(WarehouseBox::gps_coordinate)
            .sum()
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Warehouse::from_input(input, true).ok().map(|warehouse| {
        warehouse
            .execute_instructions()
            .iter()
            .map(WarehouseBox::gps_coordinate)
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn larger_example() -> Warehouse {
        let mut grid = vec![vec![Tile::Wall; GRID_SIZE]; GRID_SIZE];
        grid[1][1] = Tile::Empty;
        grid[1][2] = Tile::Empty;
        grid[1][3] = Tile::Box(0);
        grid[1][4] = Tile::Empty;
        grid[1][5] = Tile::Empty;
        grid[1][6] = Tile::Box(1);
        grid[1][7] = Tile::Empty;
        grid[1][8] = Tile::Box(2);
        grid[2][1] = Tile::Empty;
        grid[2][2] = Tile::Empty;
        grid[2][3] = Tile::Empty;
        grid[2][4] = Tile::Empty;
        grid[2][5] = Tile::Empty;
        grid[2][6] = Tile::Empty;
        grid[2][7] = Tile::Box(3);
        grid[2][8] = Tile::Empty;
        grid[3][1] = Tile::Empty;
        grid[3][2] = Tile::Box(4);
        grid[3][3] = Tile::Box(5);
        grid[3][4] = Tile::Empty;
        grid[3][5] = Tile::Empty;
        grid[3][6] = Tile::Box(6);
        grid[3][7] = Tile::Empty;
        grid[3][8] = Tile::Box(7);
        grid[4][1] = Tile::Empty;
        grid[4][2] = Tile::Empty;
        grid[4][3] = Tile::Box(8);
        grid[4][4] = Tile::Empty;
        grid[4][5] = Tile::Empty;
        grid[4][6] = Tile::Empty;
        grid[4][7] = Tile::Box(9);
        grid[4][8] = Tile::Empty;
        grid[5][1] = Tile::Box(10);
        grid[5][2] = Tile::Wall;
        grid[5][3] = Tile::Empty;
        grid[5][4] = Tile::Empty;
        grid[5][5] = Tile::Box(11);
        grid[5][6] = Tile::Empty;
        grid[5][7] = Tile::Empty;
        grid[5][8] = Tile::Empty;
        grid[6][1] = Tile::Box(12);
        grid[6][2] = Tile::Empty;
        grid[6][3] = Tile::Empty;
        grid[6][4] = Tile::Box(13);
        grid[6][5] = Tile::Empty;
        grid[6][6] = Tile::Empty;
        grid[6][7] = Tile::Box(14);
        grid[6][8] = Tile::Empty;
        grid[7][1] = Tile::Empty;
        grid[7][2] = Tile::Box(15);
        grid[7][3] = Tile::Box(16);
        grid[7][4] = Tile::Empty;
        grid[7][5] = Tile::Box(17);
        grid[7][6] = Tile::Empty;
        grid[7][7] = Tile::Box(18);
        grid[7][8] = Tile::Box(19);
        grid[8][1] = Tile::Empty;
        grid[8][2] = Tile::Empty;
        grid[8][3] = Tile::Empty;
        grid[8][4] = Tile::Empty;
        grid[8][5] = Tile::Box(20);
        grid[8][6] = Tile::Empty;
        grid[8][7] = Tile::Empty;
        grid[8][8] = Tile::Empty;

        let boxes = vec![
            WarehouseBox::Small(1, 3),
            WarehouseBox::Small(1, 6),
            WarehouseBox::Small(1, 8),
            WarehouseBox::Small(2, 7),
            WarehouseBox::Small(3, 2),
            WarehouseBox::Small(3, 3),
            WarehouseBox::Small(3, 6),
            WarehouseBox::Small(3, 8),
            WarehouseBox::Small(4, 3),
            WarehouseBox::Small(4, 7),
            WarehouseBox::Small(5, 1),
            WarehouseBox::Small(5, 5),
            WarehouseBox::Small(6, 1),
            WarehouseBox::Small(6, 4),
            WarehouseBox::Small(6, 7),
            WarehouseBox::Small(7, 2),
            WarehouseBox::Small(7, 3),
            WarehouseBox::Small(7, 5),
            WarehouseBox::Small(7, 7),
            WarehouseBox::Small(7, 8),
            WarehouseBox::Small(8, 5),
        ];

        Warehouse {
            grid,
            boxes,
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
            Warehouse::from_input(&advent_of_code::template::read_file("examples", DAY), false),
            Ok(larger_example()),
        );
    }

    #[test]
    fn test_execute_instructions() {
        assert_eq!(
            larger_example().execute_instructions(),
            vec![
                WarehouseBox::Small(1, 2),
                WarehouseBox::Small(1, 6),
                WarehouseBox::Small(1, 8),
                WarehouseBox::Small(1, 7),
                WarehouseBox::Small(3, 1),
                WarehouseBox::Small(3, 2),
                WarehouseBox::Small(1, 4),
                WarehouseBox::Small(5, 8),
                WarehouseBox::Small(4, 2),
                WarehouseBox::Small(6, 7),
                WarehouseBox::Small(4, 1),
                WarehouseBox::Small(8, 2),
                WarehouseBox::Small(5, 1),
                WarehouseBox::Small(7, 7),
                WarehouseBox::Small(7, 8),
                WarehouseBox::Small(6, 1),
                WarehouseBox::Small(7, 1),
                WarehouseBox::Small(6, 8),
                WarehouseBox::Small(8, 7),
                WarehouseBox::Small(8, 8),
                WarehouseBox::Small(8, 1),
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    fn larger_example_exploded() -> Warehouse {
        let mut grid = vec![vec![Tile::Wall; GRID_SIZE]; GRID_SIZE];

        grid[1][2] = Tile::Empty;
        grid[1][3] = Tile::Empty;
        grid[1][4] = Tile::Empty;
        grid[1][5] = Tile::Empty;
        grid[1][6] = Tile::Box(0);
        grid[1][7] = Tile::Box(0);
        grid[1][8] = Tile::Empty;
        grid[1][9] = Tile::Empty;
        grid[1][10] = Tile::Empty;
        grid[1][11] = Tile::Empty;
        grid[1][12] = Tile::Box(1);
        grid[1][13] = Tile::Box(1);
        grid[1][14] = Tile::Empty;
        grid[1][15] = Tile::Empty;
        grid[1][16] = Tile::Box(2);
        grid[1][17] = Tile::Box(2);
        grid[2][2] = Tile::Empty;
        grid[2][3] = Tile::Empty;
        grid[2][4] = Tile::Empty;
        grid[2][5] = Tile::Empty;
        grid[2][6] = Tile::Empty;
        grid[2][7] = Tile::Empty;
        grid[2][8] = Tile::Empty;
        grid[2][9] = Tile::Empty;
        grid[2][10] = Tile::Empty;
        grid[2][11] = Tile::Empty;
        grid[2][12] = Tile::Empty;
        grid[2][13] = Tile::Empty;
        grid[2][14] = Tile::Box(3);
        grid[2][15] = Tile::Box(3);
        grid[2][16] = Tile::Empty;
        grid[2][17] = Tile::Empty;
        grid[3][2] = Tile::Empty;
        grid[3][3] = Tile::Empty;
        grid[3][4] = Tile::Box(4);
        grid[3][5] = Tile::Box(4);
        grid[3][6] = Tile::Box(5);
        grid[3][7] = Tile::Box(5);
        grid[3][8] = Tile::Empty;
        grid[3][9] = Tile::Empty;
        grid[3][10] = Tile::Empty;
        grid[3][11] = Tile::Empty;
        grid[3][12] = Tile::Box(6);
        grid[3][13] = Tile::Box(6);
        grid[3][14] = Tile::Empty;
        grid[3][15] = Tile::Empty;
        grid[3][16] = Tile::Box(7);
        grid[3][17] = Tile::Box(7);
        grid[4][2] = Tile::Empty;
        grid[4][3] = Tile::Empty;
        grid[4][4] = Tile::Empty;
        grid[4][5] = Tile::Empty;
        grid[4][6] = Tile::Box(8);
        grid[4][7] = Tile::Box(8);
        grid[4][8] = Tile::Empty;
        grid[4][9] = Tile::Empty;
        grid[4][10] = Tile::Empty;
        grid[4][11] = Tile::Empty;
        grid[4][12] = Tile::Empty;
        grid[4][13] = Tile::Empty;
        grid[4][14] = Tile::Box(9);
        grid[4][15] = Tile::Box(9);
        grid[4][16] = Tile::Empty;
        grid[4][17] = Tile::Empty;
        grid[5][2] = Tile::Box(10);
        grid[5][3] = Tile::Box(10);
        grid[5][6] = Tile::Empty;
        grid[5][7] = Tile::Empty;
        grid[5][8] = Tile::Empty;
        grid[5][9] = Tile::Empty;
        grid[5][10] = Tile::Box(11);
        grid[5][11] = Tile::Box(11);
        grid[5][12] = Tile::Empty;
        grid[5][13] = Tile::Empty;
        grid[5][14] = Tile::Empty;
        grid[5][15] = Tile::Empty;
        grid[5][16] = Tile::Empty;
        grid[5][17] = Tile::Empty;
        grid[6][2] = Tile::Box(12);
        grid[6][3] = Tile::Box(12);
        grid[6][4] = Tile::Empty;
        grid[6][5] = Tile::Empty;
        grid[6][6] = Tile::Empty;
        grid[6][7] = Tile::Empty;
        grid[6][8] = Tile::Box(13);
        grid[6][9] = Tile::Box(13);
        grid[6][10] = Tile::Empty;
        grid[6][11] = Tile::Empty;
        grid[6][12] = Tile::Empty;
        grid[6][13] = Tile::Empty;
        grid[6][14] = Tile::Box(14);
        grid[6][15] = Tile::Box(14);
        grid[6][16] = Tile::Empty;
        grid[6][17] = Tile::Empty;
        grid[7][2] = Tile::Empty;
        grid[7][3] = Tile::Empty;
        grid[7][4] = Tile::Box(15);
        grid[7][5] = Tile::Box(15);
        grid[7][6] = Tile::Box(16);
        grid[7][7] = Tile::Box(16);
        grid[7][8] = Tile::Empty;
        grid[7][9] = Tile::Empty;
        grid[7][10] = Tile::Box(17);
        grid[7][11] = Tile::Box(17);
        grid[7][12] = Tile::Empty;
        grid[7][13] = Tile::Empty;
        grid[7][14] = Tile::Box(18);
        grid[7][15] = Tile::Box(18);
        grid[7][16] = Tile::Box(19);
        grid[7][17] = Tile::Box(19);
        grid[8][2] = Tile::Empty;
        grid[8][3] = Tile::Empty;
        grid[8][4] = Tile::Empty;
        grid[8][5] = Tile::Empty;
        grid[8][6] = Tile::Empty;
        grid[8][7] = Tile::Empty;
        grid[8][8] = Tile::Empty;
        grid[8][9] = Tile::Empty;
        grid[8][10] = Tile::Box(20);
        grid[8][11] = Tile::Box(20);
        grid[8][12] = Tile::Empty;
        grid[8][13] = Tile::Empty;
        grid[8][14] = Tile::Empty;
        grid[8][15] = Tile::Empty;
        grid[8][16] = Tile::Empty;
        grid[8][17] = Tile::Empty;

        Warehouse {
            grid,
            boxes: vec![
                WarehouseBox::Large(1, 6),
                WarehouseBox::Large(1, 12),
                WarehouseBox::Large(1, 16),
                WarehouseBox::Large(2, 14),
                WarehouseBox::Large(3, 4),
                WarehouseBox::Large(3, 6),
                WarehouseBox::Large(3, 12),
                WarehouseBox::Large(3, 16),
                WarehouseBox::Large(4, 6),
                WarehouseBox::Large(4, 14),
                WarehouseBox::Large(5, 2),
                WarehouseBox::Large(5, 10),
                WarehouseBox::Large(6, 2),
                WarehouseBox::Large(6, 8),
                WarehouseBox::Large(6, 14),
                WarehouseBox::Large(7, 4),
                WarehouseBox::Large(7, 6),
                WarehouseBox::Large(7, 10),
                WarehouseBox::Large(7, 14),
                WarehouseBox::Large(7, 16),
                WarehouseBox::Large(8, 10),
            ],
            start: (4, 8),
            instructions: larger_example().instructions,
        }
    }

    #[test]
    fn test_parse_warehouse_exploded() {
        assert_eq!(
            Warehouse::from_input(&advent_of_code::template::read_file("examples", DAY), true),
            Ok(larger_example_exploded()),
        );
    }

    #[test]
    fn test_execute_instructions_exploded() {
        assert_eq!(
            larger_example_exploded().execute_instructions(),
            vec![
                WarehouseBox::Large(1, 11),
                WarehouseBox::Large(1, 14),
                WarehouseBox::Large(1, 16),
                WarehouseBox::Large(2, 15),
                WarehouseBox::Large(5, 12),
                WarehouseBox::Large(3, 12),
                WarehouseBox::Large(3, 14),
                WarehouseBox::Large(3, 16),
                WarehouseBox::Large(3, 2),
                WarehouseBox::Large(4, 16),
                WarehouseBox::Large(1, 2),
                WarehouseBox::Large(4, 10),
                WarehouseBox::Large(2, 2),
                WarehouseBox::Large(6, 4),
                WarehouseBox::Large(7, 14),
                WarehouseBox::Large(4, 2),
                WarehouseBox::Large(8, 8),
                WarehouseBox::Large(7, 11),
                WarehouseBox::Large(8, 14),
                WarehouseBox::Large(7, 16),
                WarehouseBox::Large(8, 10),
            ]
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
