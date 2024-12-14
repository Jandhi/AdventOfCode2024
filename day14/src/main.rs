use core::time;
use std::{thread::sleep, time::Duration};

use image::{ImageBuffer, RgbImage};
use point::Point;

mod point;

#[derive(Debug, Clone)]
struct Robot {
    position: Point<i32>,
    velocity: Point<i32>,
}

impl Robot {
    fn f1(&self) {}
    fn f2(&mut self) {}
}

fn main() {
    let file = include_str!("input.txt");

    let mut robots : Vec<Robot> = file.lines().map(|line| {
        let mut parts = line.split(" ");
        let position = parts.next().unwrap()
            .replace("p=", "")
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let velocity = parts.next().unwrap()
            .replace("v=", "")
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        Robot {
            position,
            velocity,
        }
    }).collect();

    let mut robots_p2 = robots.clone();

    let grid_size = Point::new(101, 103);

    for i in 0..100 {
        second_passed(&mut robots, grid_size);
    }

    println!("PART 1: {}", calculate_safety_factor(&robots, grid_size));

    for i in 0..10000 {
        println!("--------------\n SECOND PASSED IS {}", i);
        second_passed(&mut robots_p2, grid_size);
        let variance = variance(&robots_p2);

        if variance > 1500 {
            continue;
        }
        display_grid(format!("{} variance {}", i, variance), &robots_p2, grid_size);
    }

    // 26?
}

fn variance(robots : &Vec<Robot>) -> i32 {
    let avg_x = robots.iter().map(|robot| robot.position.x).sum::<i32>() / robots.len() as i32;
    let avg_y = robots.iter().map(|robot| robot.position.y).sum::<i32>() / robots.len() as i32;

    robots.iter().map(|robot| {
        let x = robot.position.x - avg_x;
        let y = robot.position.y - avg_y;
        x * x + y * y
    }).sum::<i32>() / robots.len() as i32
}

fn display_grid(name : impl Into<String>, robots : &Vec<Robot>, grid_size : Point<i32>) {
    let mut image = RgbImage::new(grid_size.x as u32, grid_size.y as u32);
    
    for robot in robots.iter() {
        image.put_pixel(robot.position.x as u32, robot.position.y as u32, image::Rgb([255, 255, 255]));
    }

    image.save(format!("{}.png", name.into())).unwrap();
}

fn second_passed(robots : &mut Vec<Robot>, grid_size : Point<i32>) {
    for robot in robots.iter_mut() {
        robot.position += robot.velocity;

        if robot.position.x < 0 {
            robot.position.x += grid_size.x;
        }
        if robot.position.y < 0 {
            robot.position.y += grid_size.y;
        }
        if robot.position.x >= grid_size.x {
            robot.position.x -= grid_size.x;
        }
        if robot.position.y >= grid_size.y {
            robot.position.y -= grid_size.y;
        }
    }
}

fn calculate_safety_factor(robots : &Vec<Robot>, grid_size : Point<i32>) -> i32 {
    let mut quadrants : Vec<i32> = vec![0, 0, 0, 0];

    for robot in robots.iter() {
        if robot.position.x < grid_size.x / 2 {
            if robot.position.y < grid_size.y / 2 {
                quadrants[0] += 1;
            } else if robot.position.y > grid_size.y / 2 {
                quadrants[1] += 1;
            }
        } else if robot.position.x > grid_size.x / 2 {
            if robot.position.y < grid_size.y / 2 {
                quadrants[2] += 1;
            } else if robot.position.y > grid_size.y / 2 {
                quadrants[3] += 1;
            }
        } 
    }

    return quadrants.iter().product();
}