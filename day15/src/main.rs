use point::Point;

mod point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

impl Move {
    fn to_point(&self) -> Point<i32> {
        match self {
            Move::Up => Point::new(0, -1),
            Move::Down => Point::new(0, 1),
            Move::Left => Point::new(-1, 0),
            Move::Right => Point::new(1, 0)
        }
    }

    fn make_move(&self, pos : Point<i32>, grid: &mut Vec<Vec<Tile>>) -> Point<i32> {
        let next_pos = pos + self.to_point();

        match get(next_pos, grid) {
            Tile::Empty => {
                // Move the robot
                grid[pos.y as usize][pos.x as usize] = Tile::Empty;
                grid[next_pos.y as usize][next_pos.x as usize] = Tile::Robot;
                return next_pos;
            },
            Tile::Box | Tile::BoxLeft | Tile::BoxRight => {
                if self.is_empty_or_pushable(next_pos, grid) {
                    grid[pos.y as usize][pos.x as usize] = Tile::Empty;
                    self.push(next_pos, Tile::Robot, grid);
                    return next_pos;
                } else {
                    return pos;
                }
            },
            _ => {
                return pos;
            }
        }
    }

    fn push(&self, pos : Point<i32>, new_tile : Tile, grid: &mut Vec<Vec<Tile>>) {
        let old_tile = get(pos, grid);
        
        grid[pos.y as usize][pos.x as usize] = new_tile;

        match old_tile {
            Tile::Box => {
                let next_pos = pos + self.to_point();
                self.push(next_pos, old_tile, grid);
            },
            Tile::BoxLeft => {
                match self {
                    Move::Up | Move::Down => {
                        let next_pos = pos + self.to_point();

                        grid[pos.y as usize][pos.x as usize + 1] = Tile::Empty;
                        self.push(next_pos + Point::new(1, 0), Tile::BoxRight, grid);

                        self.push(next_pos, old_tile, grid);
                    },
                    Move::Left | Move::Right => {
                        let next_pos = pos + self.to_point();
                        self.push(next_pos, old_tile, grid);
                    }
                }
            }

            Tile::BoxRight => {
                match self {
                    Move::Up | Move::Down => {
                        let next_pos = pos + self.to_point();

                        grid[pos.y as usize][pos.x as usize - 1] = Tile::Empty;
                        self.push(next_pos + Point::new(-1, 0), Tile::BoxLeft, grid);

                        self.push(next_pos, old_tile, grid);
                    },
                    Move::Left | Move::Right => {
                        let next_pos = pos + self.to_point();
                        self.push(next_pos, old_tile, grid);
                    }
                }
            }
            Tile::Empty => (),
            _ => {
                print_grid(grid);
                println!("{:?}", self);
                panic!("Invalid tile to push")
            }
        }
    }

    fn is_empty_or_pushable(&self, pos : Point<i32>, grid: &Vec<Vec<Tile>>) -> bool {
        match get(pos, grid) {
            Tile::Empty => true,
            Tile::Box => self.is_empty_or_pushable(pos + self.to_point(), grid),
            Tile::BoxLeft => {
                match self {
                    Move::Up | Move::Down => self.is_empty_or_pushable(pos + self.to_point(), grid) && self.is_empty_or_pushable(pos + Point::new(1, 0) + self.to_point(), grid),
                    Move::Left | Move::Right => self.is_empty_or_pushable(pos + self.to_point(), grid),
                }
            }
            Tile::BoxRight => {
                match self {
                    Move::Up | Move::Down => self.is_empty_or_pushable(pos + self.to_point(), grid) && self.is_empty_or_pushable(pos + Point::new(-1, 0) + self.to_point(), grid),
                    Move::Left | Move::Right => self.is_empty_or_pushable(pos + self.to_point(), grid),
                }
            },
            _ => false
        }
    }
}

fn get(pos: Point<i32>, grid: &Vec<Vec<Tile>>) -> Tile {
    grid[pos.y as usize][pos.x as usize]
}

fn gps_sum(grid : &Vec<Vec<Tile>>) -> i32 {
    grid.iter().enumerate()
        .map(|(y, line)| line.iter().enumerate()
            .map(|(x, tile)| {
                match tile {
                    Tile::Box | Tile::BoxLeft => (y as i32) * 100 + (x as i32),
                    _ => 0
                }
            }).sum::<i32>()
        ).sum()
}

fn print_grid(grid : &Vec<Vec<Tile>>) {
    for line in grid {
        for tile in line {
            match tile {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#"),
                Tile::Box => print!("O"),
                Tile::Robot => print!("@"),
                Tile::BoxLeft => print!("["),
                Tile::BoxRight => print!("]")
            }
        }
        println!();
    }
}

fn main() {
    let file = include_str!("input.txt").replace("\r", "");

    let mut parts = file.split("\n\n");

    let mut robot_pos : Point<i32> = Default::default();

    let mut grid : Vec<Vec<Tile>> = parts.next().unwrap().lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '@' => {
                    robot_pos = Point::new(x as i32, y as i32);
                    Tile::Robot
                },
                _ => panic!("Invalid character in grid")
            }
        }).collect()
    }).collect();


    let moves : Vec<Move> = parts.next().unwrap().trim().chars().map(|char| {
        match char {
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None
        }
    }).filter_map(|m| m).collect();

    for mv in moves.iter() {
        robot_pos = mv.make_move(robot_pos, &mut grid);
    }

    println!("PART 1: {}", gps_sum(&grid));

    let mut parts = file.split("\n\n");
    let mut big_grid : Vec<Vec<Tile>> = parts.next().unwrap().lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => vec![Tile::Empty, Tile::Empty],
                '#' => vec![Tile::Wall, Tile::Wall],
                'O' => vec![Tile::BoxLeft, Tile::BoxRight],
                '@' => {
                    robot_pos = Point::new(x as i32 * 2, y as i32);
                    vec![Tile::Robot, Tile::Empty]
                },
                _ => panic!("Invalid character in grid")
            }
        }).flatten().collect()
    }).collect();

    for mv in moves.iter() {
        robot_pos = mv.make_move(robot_pos, &mut big_grid);
    }

    println!("PART 2: {}", gps_sum(&big_grid));
}