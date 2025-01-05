
use core::num;
use std::{collections::{HashMap, HashSet}, hash::Hash};

use point::Point;

mod point;

fn best_route_directional_keypad(start : char, end : char) -> String {
    match start {
        '<' => {
            match end {
                '<' => "A".to_string(),
                '>' => ">>A".to_string(),
                '^' => ">^A".to_string(),
                'v' => ">A".to_string(),
                'A' => ">>^A".to_string(),
                _ => panic!("Invalid end character")
            }
        },
        '>' => {
            match end {
                '<' => "<<A".to_string(),
                '>' => "A".to_string(),
                '^' => "<^A".to_string(),
                'v' => "<A".to_string(),
                'A' => "^A".to_string(),
                _ => panic!("Invalid end character")
            }
        },
        '^' => {
            match end {
                '<' => "v<A".to_string(),
                '>' => "v>A".to_string(),
                '^' => "A".to_string(),
                'v' => "vA".to_string(),
                'A' => ">A".to_string(),
                _ => panic!("Invalid end character")
            }
        },
        'v' => {
            match end {
                '<' => "<A".to_string(),
                '>' => ">A".to_string(),
                '^' => "^A".to_string(),
                'v' => "A".to_string(),
                'A' => ">^A".to_string(),
                _ => panic!("Invalid end character")
            }
        },
        'A' => {
            match end {
                '<' => "v<<A".to_string(),
                '>' => "vA".to_string(),
                '^' => "<A".to_string(),
                'v' => "<vA".to_string(),
                'A' => "A".to_string(),
                _ => panic!("Invalid end character")
            }
        }
        _ => panic!("Invalid start character")
    }
}

fn route_directional(string : &str) -> String {

    let chars : Vec<char> = string.chars().collect();
    let mut solution = String::new();

    for i in 0..chars.len() {
        solution += best_route_directional_keypad(if i == 0 { 'A' } else { chars[i - 1] }, chars[i]).as_str()
    }

    solution
}

fn best_route_numeric_keypad(start : char, end : char) -> String {
    let positions : HashMap<char, Point<i32>> = [
        ('0', Point::new(1, 0)),
        ('A', Point::new(2, 0)),
        ('1', Point::new(0, 1)),
        ('2', Point::new(1, 1)),
        ('3', Point::new(2, 1)),
        ('4', Point::new(0, 2)),
        ('5', Point::new(1, 2)),
        ('6', Point::new(2, 2)),
        ('7', Point::new(0, 3)),
        ('8', Point::new(1, 3)),
        ('9', Point::new(2, 3)),
    ].iter().cloned().collect();

    if positions[&start].y == 0 && positions[&end].y > 0 && positions[&end].x == 0 {
        let x_diff = positions[&end].x - positions[&start].x;
        let y_diff = positions[&end].y - positions[&start].y;

        return "^".repeat(y_diff.abs() as usize) + "<".repeat(x_diff.abs() as usize).as_str() + "A";
    }

    if positions[&start].x == 0 && positions[&end].x > 0 && positions[&end].y == 0 {
        let x_diff = positions[&end].x - positions[&start].x;
        let y_diff = positions[&end].y - positions[&start].y;

        return ">".repeat(x_diff.abs() as usize) + "v".repeat(y_diff.abs() as usize).as_str() + "A";
    }

    let route = (vec![start], String::new());
    let mut queue = vec![route];

    while queue.len() > 0 {
        let (path, instruction) = queue.remove(0);

        let last = path.last().unwrap();

        if *last == end {
            return instruction + "A";
        }

        for (dir, symbol) in vec![
            (Point::new(-1, 0), '<'),
            (Point::new(0, -1), 'v'),
            (Point::new(1, 0), '>'),
            (Point::new(0, 1), '^'),
        ] {
            if !positions.iter().any(|(_, p)| *p == *positions.get(last).unwrap() + dir) {
                continue;
            }

            let mut new_position = path.clone();
            new_position.push(*positions.iter().find(|(_, p)| **p == *positions.get(last).unwrap() + dir).unwrap().0);
            let new_instruction = instruction.clone() + &symbol.to_string();
            queue.push((new_position, new_instruction));
        }
    }

    "".to_string()
}

fn route_numeric(string : &str) -> String {

    let chars : Vec<char> = string.chars().collect();
    let mut solution = String::new();

    for i in 0..chars.len() {
        solution += best_route_numeric_keypad(if i == 0 { 'A' } else { chars[i - 1] }, chars[i]).as_str()
    }

    solution
}

fn repeat_directional_len(mut string : &str, amt : usize, cache : Option<&HashMap<String, u128>>) -> u128 {
    let mut solution = string.to_string();

    if amt < CACHE_SIZE || cache.is_none() {
        for i in 0..amt {
            println!("{}", i+1);
            solution = route_directional(solution.as_str())
        }

        return solution.to_string().len() as u128;
    }

    for i in 0..amt-CACHE_SIZE  {
        println!("{}", i+1);
        solution = route_directional(solution.as_str())
    }

    let mut parts : Vec<&str> = solution.split('A').collect();
    if *parts.last().unwrap() == "" {
        parts.pop(); // Remove last empty part
    }

    let length = parts.iter()
        .map(|part| cache.unwrap()[&(part.to_string() + "A")])
        .sum::<u128>();

    println!("LENGTH: {}", length);
    length
    
}

const CACHE_SIZE : usize = 13;

fn main() { 
    let file = include_str!("input.txt");

    let inputs = file.lines().collect::<Vec<&str>>();

    let mut cache : HashMap<String, u128> = HashMap::new();

    for start in vec!['<', '>', '^', 'v', 'A'] {
        for end in vec!['<', '>', '^', 'v', 'A'] {
            let route = best_route_directional_keypad(start, end);
            cache.insert(route.clone(), repeat_directional_len(route.as_str(), CACHE_SIZE, None));
        }
    }

    let complexities : u128 = inputs.iter().map(|input| {
        let length = repeat_directional_len(route_numeric(input).as_str(), 2, Some(&cache));
        input[0..3].parse::<u128>().unwrap() * length
    }).sum();

    println!("PART 1: {}", complexities);

    let complexities : u128 = inputs.iter().map(|input| {
        let length = repeat_directional_len(route_numeric(input).as_str(), 25, Some(&cache));
        let ans = input[0..3].parse::<u128>().unwrap() * length;
        println!("{} : {}", input, ans);
        ans
    }).sum();

    println!("PART 2: {}", complexities);

    // is        259245930604564
    // should be 226179529377982

    /*
    671A : 60893633732822
    826A : 77504793828476
    670A : 56446219860480
    085A : 6862235364940
    283A : 24472646591264 
     */
}
