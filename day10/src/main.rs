use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn get(&self, grid: &Vec<Vec<u32>>) -> u32 {
        grid[self.y][self.x]
    }

    fn neighbours(&self, grid : &Vec<Vec<u32>>) -> Vec<Point> {
        let mut neighbours = Vec::new();

        if self.x > 0 {
            neighbours.push(Point::new(self.x - 1, self.y));
        }

        if self.x < grid[0].len() - 1 {
            neighbours.push(Point::new(self.x + 1, self.y));
        }

        if self.y > 0 {
            neighbours.push(Point::new(self.x, self.y - 1));
        }

        if self.y < grid.len() - 1 {
            neighbours.push(Point::new(self.x, self.y + 1));
        }

        neighbours
    }

    fn next(&self, grid : &Vec<Vec<u32>>) -> Vec<Point> {
        let mut points = vec![];

        let val = self.get(grid);
        for neighbour in self.neighbours(grid) {
            if neighbour.get(grid) == val + 1 {
                points.push(neighbour);
            }
        }

        points
    }
}

impl std::ops::Add for Point {
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

    let grid : Vec<Vec<u32>> = file.lines().map(|line| {
        line.chars().map(|num| num.to_digit(10).unwrap()).collect()
    }).collect();

    let mut scores : u32 = grid.iter()
        .enumerate()
        .map(|row| {
            row.1.iter()
                .enumerate()
                .map(|col| {
                    match col.1 {
                        0 => find_score(&grid, Point::new(col.0, row.0)),
                        _ => 0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("PART 1: {}", scores);

    let mut ratings : u32 = grid.iter()
        .enumerate()
        .map(|row| {
            row.1.iter()
                .enumerate()
                .map(|col| {
                    match col.1 {
                        0 => find_rating(&grid, Point::new(col.0, row.0)),
                        _ => 0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("PART 2: {}", ratings);
}

fn find_score(grid : &Vec<Vec<u32>>, position : Point) -> u32 {
    let mut queue : Vec<Point> = vec![position];
    let mut visited : HashSet<Point> = HashSet::new();
    let mut value = 0;

    while queue.len() > 0 {
        let current = queue.remove(0);

        if current.get(grid) == 9 {
            value += 1;
            continue;
        }

        for next in current.next(grid) {
            if !visited.contains(&next) {
                queue.push(next);
                visited.insert(next);
            }
        }
    }

    value
}

fn find_rating(grid : &Vec<Vec<u32>>, position : Point) -> u32 {
    let mut queue : Vec<Point> = vec![position];
    let mut value = 0;

    while queue.len() > 0 {
        let current = queue.remove(0);

        if current.get(grid) == 9 {
            value += 1;
            continue;
        }

        for next in current.next(grid) {
            queue.push(next);
        }
    }

    value
}
