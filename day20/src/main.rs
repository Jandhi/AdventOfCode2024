use std::collections::HashMap;

use point::Point;

mod point;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End
}

#[derive(Clone)]
struct Path {
    points : Vec<Point<i32>>,
}

impl Path {
    fn next_paths(&self, grid : &Vec<Vec<Tile>>) -> Vec<Path> {
        let mut paths = vec![];

        for new_end in self.points.last().unwrap().cardinal_neighbours() {
            if !new_end.is_in_bounds(grid) || self.points.contains(&new_end) {
                continue;
            }

            if let Tile::Wall = grid[new_end.y as usize][new_end.x as usize] {
                continue;
            }
            
            let mut new_path = self.clone();
            new_path.points.push(new_end);
            paths.push(new_path);
        }

        paths
    }
}

fn print_grid(grid : &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#"),
                Tile::Start => print!("S"),
                Tile::End => print!("E")
            }
        }
        println!();
    }
}

fn print_grid_with_path(grid : &Vec<Vec<Tile>>, path : &Path) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if path.points.contains(&point) {
                print!("O");
            } else {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::Start => print!("S"),
                    Tile::End => print!("E")
                }
            }
        }
        println!();
    }
}

fn main() {
    let file = include_str!("input.txt");

    let mut start : Point<i32> = Point::new(0, 0);
    let mut end : Point<i32> = Point::new(0, 0);

    let grid : Vec<Vec<Tile>> = file.trim().lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'S' => {
                    start = Point::new(x as i32, y as i32);
                    Tile::Start
                },
                'E' => {
                    end = Point::new(x as i32, y as i32);
                    Tile::End
                }
                _ => panic!("Invalid character in input")
            }
        }).collect()
    })
    .collect();

    let mut queue : Vec<Path> = vec![Path { points: vec![start] }];
    let mut best_path : Option<Path> = None;

    while queue.len() > 0 {
        let path = queue.pop().unwrap();

        if *path.points.last().unwrap() == end {
            
            best_path = Some(path);
            println!("Found best path");
            break;
        }

        for new_path in path.next_paths(&grid) {
            queue.push(new_path);
        }
    }

    let points_map : HashMap<Point<i32>, usize> = best_path.unwrap().points.iter().enumerate().map(|(i, point)| (*point, i)).collect();
    let mut times : HashMap<usize, usize> = HashMap::new();

    for (point, time_at_point) in points_map.iter() {
        
        for neighbour in point.cardinal_neighbours() {
            if !neighbour.is_in_bounds(&grid) || grid[neighbour.y as usize][neighbour.x as usize] != Tile::Wall {
                continue;
            }

            for next_neighbour in neighbour.cardinal_neighbours() {
                if !next_neighbour.is_in_bounds(&grid) || grid[next_neighbour.y as usize][next_neighbour.x as usize] == Tile::Wall {
                    continue;
                }

                let time_at_next_neighbour = points_map.get(&next_neighbour).unwrap();

                if *time_at_next_neighbour > time_at_point + 2 {
                    let shortcut_size = time_at_next_neighbour - time_at_point - 2;

                    times.insert(shortcut_size, times.get(&shortcut_size).unwrap_or(&0) + 1);
                }
            }
        }
    }

    let supershortcuts : usize = times.iter().filter(|(time, _)| **time >= 100).map(|(_, amt)| amt).sum();
    println!("PART 1: {:?}", supershortcuts);

    let mut times : HashMap<usize, usize> = HashMap::new();

    for (point, time_at_point) in points_map.iter() {
        for (other_point, time_at_other_point) in points_map.iter() {
            let distance = (point.x - other_point.x).abs() + (point.y - other_point.y).abs();

            if distance > 20 {
                continue;
            }

            if *time_at_other_point <= *time_at_point + (distance as usize) {
                continue;
            }

            let shortcut_size = time_at_other_point - time_at_point - distance as usize;
            times.insert(shortcut_size, times.get(&shortcut_size).unwrap_or(&0) + 1);
        }
    }

    let supershortcuts : usize = times.iter().filter(|(time, _)| **time >= 100).map(|(_, amt)| amt).sum();
    println!("PART 2: {:?}", supershortcuts);
}
