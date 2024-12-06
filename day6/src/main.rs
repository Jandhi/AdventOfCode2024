use std::{collections::HashSet, ops::Add};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start_pos: Point,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut guard_pos = Point::new(0, 0);

        let tiles = s.lines()
            .enumerate() 
            .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '^' => {
                        guard_pos = Point::new(x, y);
                        Tile::Empty
                    }
                    c => panic!("Unknown tile type \"{}\"", c)
                }
            }).collect::<Vec<Tile>>()
        }).collect::<Vec<Vec<Tile>>>();

        Grid {
            tiles,
            start_pos: guard_pos
        }
    }
}

impl Grid {
    fn has_loop(&self) -> bool {
        let mut guard_pos = self.start_pos;
        let mut guard_direction = Direction::Up;
        let mut visited : HashSet<(Point, Direction)> = HashSet::new();

        loop {
            if visited.contains(&(guard_pos, guard_direction)) {
                return true;
            }

            visited.insert((guard_pos, guard_direction));

            // reached the bottom
            if guard_direction == Direction::Down && guard_pos.y == self.tiles.len() - 1 {
                return false;
            }
            // reached the left
            if guard_direction == Direction::Left && guard_pos.x == 0 {
                return false;
            }
            // reached the right
            if guard_direction == Direction::Right && guard_pos.x == self.tiles[0].len() - 1 {
                return false;
            }
            // reached the top
            if guard_direction == Direction::Up && guard_pos.y == 0 {
                return false;
            }

            let next = match guard_direction {
                Direction::Up => Point::new(guard_pos.x, guard_pos.y - 1),
                Direction::Down => Point::new(guard_pos.x, guard_pos.y + 1),
                Direction::Left => Point::new(guard_pos.x - 1, guard_pos.y),
                Direction::Right => Point::new(guard_pos.x + 1, guard_pos.y)
            };

            if self.tiles[next.y][next.x] == Tile::Wall {
                guard_direction = match guard_direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down
                };
            } else {
                guard_pos = next;
            }
        }
    }
}

fn main() {
    let file = include_str!("input.txt");

    let grid : Grid = file.into();
    let mut guard_pos = grid.start_pos;
    let mut guard_direction = Direction::Up;

    let mut visited : HashSet<(Point, Direction)> = HashSet::new();

    loop {
        if visited.contains(&(guard_pos, guard_direction)) {
            break;
        }

        visited.insert((guard_pos, guard_direction));

        // reached the bottom
        if guard_direction == Direction::Down && guard_pos.y == grid.tiles.len() - 1 {
            break;
        }
        // reached the left
        if guard_direction == Direction::Left && guard_pos.x == 0 {
            break;
        }
        // reached the right
        if guard_direction == Direction::Right && guard_pos.x == grid.tiles[0].len() - 1 {
            break;
        }
        // reached the top
        if guard_direction == Direction::Up && guard_pos.y == 0 {
            break;
        }

        let next = match guard_direction {
            Direction::Up => Point::new(guard_pos.x, guard_pos.y - 1),
            Direction::Down => Point::new(guard_pos.x, guard_pos.y + 1),
            Direction::Left => Point::new(guard_pos.x - 1, guard_pos.y),
            Direction::Right => Point::new(guard_pos.x + 1, guard_pos.y)
        };

        if grid.tiles[next.y][next.x] == Tile::Wall {
            guard_direction = match guard_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down
            };
        } else {
            guard_pos = next;
        }
    }

    let visited_points : HashSet<Point> = visited.iter().map(|(point, _)| {
        point.clone()
    }).collect();

    // Print the grid
    for (y, row) in grid.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if visited_points.contains(&Point::new(x, y)) {
                print!("X");
                continue;
            }

            match tile {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#")
            }
        }
        println!();
    }

    println!("Part 1: {}", visited_points.len());

    let mut count = 0;

    for point in visited_points.iter() {
        if *point == grid.start_pos {
            continue;
        }

        let mut new_grid = grid.clone();
        new_grid.tiles[point.y][point.x] = Tile::Wall;

        if new_grid.has_loop() {
            count += 1; 
        }
    }

    println!("Part 2: {}", count);
}
