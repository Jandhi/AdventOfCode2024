use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use point::Point;
mod point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    path : Vec<Point<i32>>,
    score : i32,
}

impl State {
    fn new(path : Vec<Point<i32>>) -> State {
        let mut state = State {
            path,
            score: 0,
        };
        state.score_path();
        state
    }

    fn score_path(&mut self) {
        let mut score = self.path.len() - 1;

        if self.path.len() > 1 {
            if self.path[1] - self.path[0] == Point::new(0, 1) {
                score += 1000;
            } else if self.path[1] - self.path[0] == Point::new(0, -1) {
                score += 1000;
            }
        }
    
        for i in 1..self.path.len() - 1 {
            if self.path[i - 1] - self.path[i] != self.path[i] - self.path[i + 1] { // Turn!
                score += 1000;
            }
        }
    
        self.score = score as i32;
    }

    fn next_states(&self, grid : &Vec<Vec<Tile>>) -> Vec<State> {
        let mut states = Vec::new();

        let last_point = self.path.last().unwrap();

        for point in vec![
            Point::new(last_point.x + 1, last_point.y),
            Point::new(last_point.x - 1, last_point.y),
            Point::new(last_point.x, last_point.y + 1),
            Point::new(last_point.x, last_point.y - 1),
        ] {
            if point.x < 0 || point.y < 0 || point.y >= grid.len() as i32 || point.x >= grid[point.y as usize].len() as i32 {
                continue;
            }

            match grid[point.y as usize][point.x as usize] {
                Tile::Empty | Tile::End => {
                    let mut new_path = self.path.clone();
                    new_path.push(point);
                    states.push(State::new(new_path));
                },
                _ => {}
            }
        }

        states
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn main() {
    let file = include_str!("input.txt");

    let mut start_point = Point::<i32>::new(0, 0);

    let grid : Vec<Vec<Tile>> = file.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => {
                start_point = Point::<i32>::new(x as i32, y as i32);
                Tile::Start
            },
            'E' => Tile::End,
            _ => panic!("Invalid character in input")
        }).collect()
    }).collect();

    let mut state = State::new(vec![start_point]);
    let mut visited : HashSet<Point<i32>> = HashSet::new();

    let mut best_paths : Vec<State> = vec![];
    let mut best_score : Option<i32> = None;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(state));

    loop {
        state = heap.pop().unwrap().0;

        if grid[state.path.last().unwrap().y as usize][state.path.last().unwrap().x as usize] == Tile::End {
            best_score = Some(state.score);
            best_paths.push(state);

            state = heap.pop().unwrap().0;

            while state.score == best_score.unwrap() {
                if grid[state.path.last().unwrap().y as usize][state.path.last().unwrap().x as usize] != Tile::End {
                    continue;
                }

                best_paths.push(state);
                state = heap.pop().unwrap().0;
            }

            break;
        }

        if state.path.len() > 1 {
            visited.insert(state.path[state.path.len() - 2]);
        }

        for next in state.next_states(&grid) {
            if visited.contains(next.path.last().unwrap()) {
                continue;
            }

            heap.push(Reverse(next));
        }
    }

    println!("PART 1: {}", best_score.unwrap());

    let set : HashSet<Point<i32>> = best_paths.iter().map(|state| state.path.clone()).flatten().collect();

    println!("PART 2: {}", set.len());

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if set.contains(&Point::new(x as i32, y as i32)) {
                print!("O");
            } else {
                match grid[y][x] {
                    Tile::Empty => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::Start => print!("S"),
                    Tile::End => print!("E"),
                }
            }
        }
        println!();
    }

    
}
