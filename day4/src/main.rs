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

    fn in_bounds(&self, grid : &Vec<Vec<char>>) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < grid.len() as i32 && self.y < grid[self.x as usize].len() as i32
    }

    fn get(&self, grid : &Vec<Vec<char>>) -> char {
        grid[self.x as usize][self.y as usize]
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

                if p2.in_bounds(&grid) && p3.in_bounds(&grid) && p4.in_bounds(&grid) {
                    if p1.get(&grid) == 'X' && p2.get(&grid) == 'M' && p3.get(&grid) == 'A' && p4.get(&grid) == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("PART 1: {}", count);

    count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let p0 = Point::new(x as i32, y as i32);
            
            let p1 = p0 + Point::new(-1, -1);
            let p2 = p0 + Point::new(1, 1);

            let p3 = p0 + Point::new(-1, 1);
            let p4 = p0 + Point::new(1, -1);
        
            if p1.in_bounds(&grid) && p2.in_bounds(&grid) && p3.in_bounds(&grid) && p4.in_bounds(&grid) {
                if p0.get(&grid) == 'A' &&
                    (p1.get(&grid) == 'M' && p2.get(&grid) == 'S' || p1.get(&grid) == 'S' && p2.get(&grid) == 'M') &&
                    (p3.get(&grid) == 'M' && p4.get(&grid) == 'S' || p3.get(&grid) == 'S' && p4.get(&grid) == 'M') {
                        count += 1;
                }
            }
        }
    }

    println!("PART 2: {}", count);
}
