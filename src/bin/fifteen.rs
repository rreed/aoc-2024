use std::io::{self};
use std::mem;
use std::ops::Add;

#[derive(PartialEq, Clone, Copy, Debug)]
enum TileType {
    Empty,
    Wall,
    Object,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Object,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

impl TileType {
    fn embiggen(self) -> impl Iterator<Item = Self> {
        match self {
            TileType::Empty => [TileType::Empty, TileType::Empty].into_iter(),
            TileType::Wall => [TileType::Wall, TileType::Wall].into_iter(),
            TileType::Object => [TileType::BoxLeft, TileType::BoxRight].into_iter(),
            TileType::Robot => [TileType::Robot, TileType::Empty].into_iter(),
            TileType::BoxLeft | TileType::BoxRight => unreachable!(),
        }
    }
}

impl From<TileType> for char {
    fn from(tile: TileType) -> Self {
        match tile {
            TileType::Empty => '.',
            TileType::Wall => '#',
            TileType::Object => 'O',
            TileType::Robot => '@',
            TileType::BoxLeft => '[',
            TileType::BoxRight => ']',
        }
    }
}

struct WarehouseState {
    grid: Vec<Vec<TileType>>,
    robot: (usize, usize),
    width: usize,
    height: usize,
}

impl WarehouseState {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<TileType>> = Vec::new();
        let mut robot = (0, 0);

        for (row, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                let tile = TileType::from(char);

                if tile == TileType::Robot {
                    robot = (row, col);
                }

                grid_row.push(tile);
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            robot,
            width,
            height,
        }
    }

    fn embiggen_warehouse(&mut self) {
        let mut temp: Vec<Vec<TileType>> = Vec::new();
        mem::swap(&mut self.grid, &mut temp);

        self.grid = temp
            .into_iter()
            .map(|row| row.into_iter().flat_map(TileType::embiggen).collect())
            .collect();

        self.width *= 2;
        self.robot.1 *= 2;
    }

    fn move_robot(&mut self, direction: Direction) {
        let (row, col) = self.robot;

        if self.can_move_tile(row, col, direction) {
            self.move_tile(row, col, direction);
            self.robot = self.robot + direction;
        }
    }

    fn move_tile(&mut self, row: usize, col: usize, direction: Direction) {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            TileType::Empty => {
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = TileType::Empty;
            }
            TileType::Object => {
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = TileType::Empty;
            }
            TileType::BoxRight => {
                self.move_tile(next_row, next_col - 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = TileType::Empty;
            }
            TileType::BoxLeft => {
                self.move_tile(next_row, next_col + 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = TileType::Empty;
            }
            TileType::Wall => unreachable!(),
            TileType::Robot => unreachable!(),
        }
    }

    fn can_move_tile(&self, row: usize, col: usize, direction: Direction) -> bool {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            TileType::Empty => true,
            TileType::Wall => false,
            TileType::Object => self.can_move_tile(next_row, next_col, direction),
            TileType::BoxLeft => {
                if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col + 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col + 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            TileType::BoxRight => {
                if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col - 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col - 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            TileType::Robot => unreachable!(),
        }
    }

    fn gps_coordinate(row: usize, col: usize) -> usize {
        row * 100 + col
    }

    fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == TileType::Object
                    || self.grid[row][col] == TileType::BoxLeft
                {
                    sum += Self::gps_coordinate(row, col);
                }
            }
        }

        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => unreachable!(),
        }
    }
}

pub fn part_two() -> usize {
    let input: &str = include_str!("fifteen.in");

    let (warehouse, moves) = input.split_once("\r\n\r\n").unwrap();

    let mut warehouse = WarehouseState::new(warehouse);
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.embiggen_warehouse();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    warehouse.sum_gps_coordinates()
}

fn main() -> io::Result<()> {
    let input: &str = include_str!("fifteen.in");

    let (warehouse, moves) = input.split_once("\n\n").unwrap();

    let mut warehouse_one = WarehouseState::new(warehouse);
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    for direction in moves.clone() {
        warehouse_one.move_robot(direction);
    }

    let ans_one = warehouse_one.sum_gps_coordinates();
    println!("Ans Part One: {ans_one}");

    let mut warehouse_two = WarehouseState::new(warehouse);
    warehouse_two.embiggen_warehouse();
    for direction in moves.clone() {
        warehouse_two.move_robot(direction);
    }

    let ans_two = warehouse_two.sum_gps_coordinates();
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
