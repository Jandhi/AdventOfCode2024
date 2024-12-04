use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let file = include_str!("input.txt");

    let grid : Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let direction_vectors : Vec<Point> = vec![
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(0, -1),
        Point::new(1, 1),
        Point::new(-1, 1),
        Point::new(-1, -1),
        Point::new(1, -1),
    ];

    let mut count = 0;

    for direction in direction_vectors {
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                let p1 = Point::new(x as i32, y as i32);
                let p2 = p1 + direction;
                let p3 = p2 + direction;
                let p4 = p3 + direction;

                if p2.x >= 0 && p2.y >= 0 && p2.x < grid.len() as i32 && p2.y < grid[p2.x as usize].len() as i32 &&
                    p3.x >= 0 && p3.y >= 0 && p3.x < grid.len() as i32 && p3.y < grid[p3.x as usize].len() as i32 &&
                    p4.x >= 0 && p4.y >= 0 && p4.x < grid.len() as i32 && p4.y < grid[p4.x as usize].len() as i32 {
                    if grid[p1.x as usize][p1.y as usize] == 'X' && grid[p2.x as usize][p2.y as usize] == 'M' && grid[p3.x as usize][p3.y as usize] == 'A' && grid[p4.x as usize][p4.y as usize] == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("PART 1: {}", count);
}
