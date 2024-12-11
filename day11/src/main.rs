use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Stones {
    stones: Vec<i64>,
}

impl Stones {
    fn new (stones: Vec<i64>) -> Stones {
        Stones { stones }
    }

    fn blink(&mut self) {
        let mut offset = 0;

        for i in 0..self.stones.len() {
            match self.stones[i + offset] {
                0 => self.stones[i + offset] = 1,
                _ if self.stones[i + offset].to_string().len() % 2 == 0 => {
                    let num = self.stones[i + offset];
                    let left = num.to_string()[0..num.to_string().len() / 2].parse::<i64>().unwrap();
                    let right = num.to_string()[num.to_string().len() / 2..].parse::<i64>().unwrap();
                    self.stones[i + offset] = left;
                    self.stones.insert(i + offset + 1, right);
                    offset += 1;
                },
                _ => self.stones[i + offset] = self.stones[i + offset] * 2024,
            }
        }
    }
}

struct StonesCalculator {
    stones_after_5_blinks : HashMap<i64, HashMap<i64, usize>>,
}

impl StonesCalculator {
    fn new() -> StonesCalculator {
        StonesCalculator {
            stones_after_5_blinks : HashMap::new(),
        }
    }

    fn get_stones_after_x5_blinks(&mut self, mut stones : HashMap<i64, usize>, x : i64) -> HashMap<i64, usize> {

        for _ in 0..x {
            stones = self.get_stones_after_5_blinks(stones);
        }

        return stones;
    }

    fn get_stones_after_5_blinks(&mut self, stones : HashMap<i64, usize>) -> HashMap<i64, usize> {
        let mut new_stones = HashMap::new();

        for (stone, amount) in stones.iter() {
            let stones = self.get_stones_for_value_after_5_blinks(*stone);

            for (end_stone, end_amount) in stones.iter() {
                *new_stones.entry(*end_stone).or_insert(0) += amount * end_amount;
            }
        }

        new_stones
    }

    fn get_stones_for_value_after_5_blinks(&mut self, value : i64) -> HashMap<i64, usize> {
        if !self.stones_after_5_blinks.contains_key(&value) {
            let mut stones = Stones::new(vec![value]);

            for _ in 0..5 {
                stones.blink();
            }

            self.stones_after_5_blinks.insert(value, stones.stones.iter().fold(HashMap::new(), |mut acc, x| {
                *acc.entry(*x).or_insert(0) += 1;
                acc
            }));
        }

        return self.stones_after_5_blinks.get(&value).unwrap().clone();
    }
}

fn main() {
    let file = include_str!("input.txt");

    let stones : HashMap<i64, usize> = file.split_whitespace().map(|x| x.parse::<i64>().unwrap()).fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let mut calc=  StonesCalculator::new();

    let stones= calc.get_stones_after_x5_blinks(stones, 5);

    println!("PART 1: {}", stones.iter().map(|(_, a)| a).sum::<usize>());

    let stones = calc.get_stones_after_x5_blinks(stones, 10);

    println!("PART 2: {}", stones.iter().map(|(_, a)| a).sum::<usize>());
}
