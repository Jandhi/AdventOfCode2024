
use core::num;
use std::collections::{HashMap, HashSet};

use point::Point;

mod point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EmptyPosition {
    TopLeft,
    BottomLeft,
}

struct Keypad {
    grid: HashMap<char, Point<i32>>,
    empty: EmptyPosition,
}

impl Keypad {
    fn new_numpad() -> Keypad {
        let mut grid = HashMap::new();
        grid.insert(' ', Point::new(0, 0));
        grid.insert('0', Point::new(1, 0));
        grid.insert('A', Point::new(2, 0));
        grid.insert('1', Point::new(0, 1));
        grid.insert('2', Point::new(1, 1));
        grid.insert('3', Point::new(2, 1));
        grid.insert('4', Point::new(0, 2));
        grid.insert('5', Point::new(1, 2));
        grid.insert('6', Point::new(2, 2));
        grid.insert('7', Point::new(0, 3));
        grid.insert('8', Point::new(1, 3));
        grid.insert('9', Point::new(2, 3));
        Keypad { grid, empty: EmptyPosition::BottomLeft }
    }

    fn new_control_pad() -> Keypad {
        let mut grid = HashMap::new();
        grid.insert('<', Point::new(0, 0));
        grid.insert('v', Point::new(1, 0));
        grid.insert('>', Point::new(2, 0));
        grid.insert(' ', Point::new(0, 1));
        grid.insert('^', Point::new(1, 1));
        grid.insert('A', Point::new(2, 1));
        Keypad { grid, empty: EmptyPosition::TopLeft }
    }

    fn decode(&self, instructions : &str) -> String {
        let mut position = self.grid[&'A'];
        let mut decoded = "".to_string();

        for c in instructions.chars() {
            match c {
                '>' => {
                    position.x += 1;
                },
                '<' => {
                    position.x -= 1;
                },
                '^' => {
                    position.y += 1;
                },
                'v' => {
                    position.y -= 1;
                },
                'A' => {
                    decoded += &self.grid.iter().find(|(_, &v)| v == position).unwrap().0.to_string();
                }
                _ => {}
            }

            if position == self.grid[&' '] {
                println!("ILLEGAL POSITION!");
            }
        }

        decoded
    }

    fn get_sequence(&self, sequence : &str) -> String {
        let mut answer = "".to_string();

        for i in 0..sequence.len() {
            let start = if i == 0 { 'A' } else { sequence.chars().nth(i - 1).unwrap() };
            let end = sequence.chars().nth(i).unwrap();

            let sequence = self.get_sequences_from(start, end);

            answer += &sequence;
        }

        answer
    }

    fn get_sequences_from(&self, start : char, target : char) -> String {
        let mut queue = vec![(self.grid[&start], "".to_string())];
        let mut visited : HashSet<Point<i32>> = HashSet::new();

        let mut successes = vec![];

        while queue.len() > 0 {
            let (pos, seq) = queue.remove(0);

            if pos == self.grid[&target] {
                successes.push(seq + &'A'.to_string());
                continue;
            }
            
            for diff in vec![
                (Point::new(-1, 0), '<'),
                (Point::new(0, -1), 'v'),
                (Point::new(1, 0), '>'),
                (Point::new(0, 1), '^'),
            ]{
                let new_pos = pos + diff.0;

                if visited.contains(&new_pos) {
                    continue;
                }
                if self.grid.iter().find(|(_, &v)| v == new_pos).is_none() {
                    continue;
                }
                if self.grid[&' '] == new_pos {
                    continue;
                }

                visited.insert(new_pos);
                let new_seq = seq.clone() + &diff.1.to_string();
                

                queue.push((new_pos, new_seq));
            }
        }

        if successes[0] == "<v<A" {
            return "v<<A".to_string();
        }

        if successes[0] == "v>vA" {
            return ">vvA".to_string();
        }

        successes[0].clone()
    }
}

// fn get_least_expensive_numpad_sequence(seq : &str, numpad : &Keypad, controlpad : &Keypad) -> String {
    

//     seq.chars().enumerate().map(|(i, _)| {
//         let start = if i == 0 { 'A' } else { seq.chars().nth(i - 1).unwrap() };
//         let end = seq.chars().nth(i).unwrap();

//         let least_expensive = numpad.get_sequences_from(start, end).iter().map(|seq| {
//             get_least_expensive_controlpad_sequence(seq, 2, controlpad)
//         }).min_by_key(|seq| seq.len()).unwrap();

//         println!("{} -> {}: {}", start, end, least_expensive);
//         least_expensive 
//     }).fold("".to_string(), |acc, next| acc + next.as_str())
// }

// fn get_least_expensive_controlpad_sequence(seq : &str, depth : i32, controlpad : &Keypad) -> String {
//     if depth == 0 {
//         return seq.to_string();
//     }

//     seq.chars().enumerate().map(|(i, _)| {
//         let start = if i == 0 { 'A' } else { seq.chars().nth(i - 1).unwrap() };
//         let end = seq.chars().nth(i).unwrap();

//         let least_expensive = controlpad.get_sequences_from(start, end).iter().map(|seq| {
//             get_least_expensive_controlpad_sequence(seq, depth - 1, controlpad)
//         }).min_by_key(|seq| seq.len()).unwrap();

//         println!("{} -> {}: {}", start, end, least_expensive);
//         least_expensive 
//     }).fold("".to_string(), |acc, next| acc + next.as_str())
// }

fn main() {
    let file = include_str!("input.txt");

    let inputs = file.lines().collect::<Vec<&str>>();

    let numpad = Keypad::new_numpad();
    let control_pad = Keypad::new_control_pad();    

    // let p1 = numpad.get_sequence("029A");
    // println!("{} vs \n<A^A>^^AvvvA", p1);
    // let p2 = control_pad.get_sequence(&p1);
    // println!("{} vs \nv<<A>>^A<A>AvA<^AA>A<vAAA>^A", p2);
    // let p3 = control_pad.get_sequence(&p2);
    // println!("{} vs \n<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A", p3);

    // let seq = get_least_expensive_numpad_sequence("029A", &numpad, &control_pad);
    // println!("{}", seq);

    let part1 : usize = inputs.iter().map(|line| {
        let numeric_part : usize = line[0..3].parse().unwrap();

        let input1 = numpad.get_sequence(&line);
        let input2 = control_pad.get_sequence(&input1);
        let input3 = control_pad.get_sequence(&input2);

        println!("{} {} {}", input3.len(), input3, numeric_part);
        println!("{}", numpad.decode(&control_pad.decode(&control_pad.decode(&input3))));

        input3.len() * numeric_part
    }).sum();


    println!("Part 1: {}", part1);

    // <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A
    // <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

    // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // <<vA>>^AvA^A<<vA>>^AA<<vA>A>^AAvAA<^A>A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A

    //println!("{}", numpad.decode(&control_pad.decode(&control_pad.decode("<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"))));
}
