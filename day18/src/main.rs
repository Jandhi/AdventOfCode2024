use std::collections::HashSet;

use point::Point;

mod point;

fn is_traversible(point : Point<i32>, grid : &Vec<Vec<bool>>) -> bool {
    if point.x < 0 || point.y < 0 {
        return false;
    }
    if point.x >= grid.len() as i32 || point.y >= grid[0].len() as i32 {
        return false;
    }
    return !grid[point.x as usize][point.y as usize];
}

fn get_path(grid : Vec<Vec<bool>>) -> Option<Vec<Point<i32>>> {
    let size = Point::new(grid.len() as i32, grid[0].len() as i32);
    let target = Point::new(size.x - 1, size.y - 1);

    let mut queue : Vec<Vec<Point<i32>>> = vec![vec![Point { x: 0, y: 0 }]];
    let mut visited : HashSet<Point<i32>> = HashSet::new();

    while !queue.is_empty() {
        let path = queue.remove(0);

        let last = path.last().unwrap();
        if *last == target {
            return Some(path);
        }
        if visited.contains(last) {
            continue;
        }
        visited.insert(*last);

        let mut next = vec![];
        
        for diff in vec![Point::new(1, 0), Point::new(-1, 0), Point::new(0, 1), Point::new(0, -1)] {
            let new_point = *last + diff;
            if is_traversible(new_point, &grid) {
                next.push(new_point);
            }
        }

        for point in next.iter() {
            let mut new_path = path.clone();
            new_path.push(*point);
            queue.push(new_path);
        }
    }

    return None;
}

fn main() {
    let file = include_str!("input.txt");
    let mut points = file.lines().map(|line| {
        let mut iter = line.split(",");
        let x : i32 = iter.next().unwrap().parse().unwrap();
        let y : i32 = iter.next().unwrap().parse().unwrap();
        Point { x, y }
    }).collect::<Vec<_>>();

    let size = Point::new(71, 71);
    let falls = 1024;
    let target = Point::new(size.x as i32 - 1, size.y as i32 - 1);

    let mut grid : Vec<Vec<bool>> = vec![vec![false; size.y]; size.x];
    for i in 0..falls {
        let point = points[i as usize];
        grid[point.x as usize][point.y as usize] = true;
    }

    let path = get_path(grid).unwrap();

    println!("PART 1: {}", path.len() - 1);

    let mut grid : Vec<Vec<bool>> = vec![vec![false; size.y]; size.x];

    for i in 0..points.len() {
        grid[points[i].x as usize][points[i].y as usize] = true;

        let path = get_path(grid.clone());

        if path.is_none() {
            println!("PART 2: {},{}", points[i].x, points[i].y);
            break;
        }
    }
}
