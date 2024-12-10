use std::{collections::{HashMap, HashSet}, ops::{Add, Mul, Sub}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


fn main() {
    let file = include_str!("input.txt");

    let grid : Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let mut nodes : HashMap<char, Vec<Point>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                if !nodes.contains_key(c) {
                    nodes.insert(*c, vec![]);
                }

                nodes.get_mut(c).unwrap().push(Point {x : x as i32, y : y as i32 });
            }
        }
    }

    let mut antinodes : HashSet<Point> = HashSet::new();

    for (_, points) in nodes.iter() {
        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }

                let p1 = points[i];
                let p2 = points[j];

                
                let antinode_point = (p1 * 2) - p2;
                let in_bounds = antinode_point.x >= 0 && antinode_point.y >= 0 && antinode_point.x < grid[0].len() as i32 && antinode_point.y < grid.len() as i32;

                if !in_bounds {
                    continue;
                }

                antinodes.insert(antinode_point);
            }
        }
    }

    // Debug print map
    // for (row, y) in grid.iter().enumerate() {
    //     for (x, c) in y.iter().enumerate() {
    //         if antinodes.contains(&Point::new(x as i32, row as i32)) {
    //             print!("#");
    //         } else {
    //             print!("{}", c);
    //         }
    //     }

    //     println!();
    // }

    println!("PART 1: {}", antinodes.len());

    let mut antinodes : HashSet<Point> = HashSet::new();

    for (_, points) in nodes.iter() {
        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }

                let p1 = points[i];
                let p2 = points[j];

                let diff = p2 - p1;

                // Not finding bounds on k, fuck it
                for k in -100..=100 {
                    let antinode_point = p1 + (diff * k);

                    let in_bounds = antinode_point.x >= 0 && antinode_point.y >= 0 && antinode_point.x < grid[0].len() as i32 && antinode_point.y < grid.len() as i32;

                    if !in_bounds {
                        continue;
                    }
                    antinodes.insert(antinode_point);
                }
            }
        }
    }

    println!("PART 2: {}", antinodes.len());

    
}
