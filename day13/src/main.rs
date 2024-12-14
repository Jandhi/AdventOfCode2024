use point::{get_ratio, Point};

mod point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClawMachine {
    button_a : Point<i128>,
    button_b : Point<i128>,
    prize : Point<i128>,
}

impl ClawMachine {
    fn new(button_a: Point<i128>, button_b: Point<i128>, prize: Point<i128>) -> ClawMachine {
        ClawMachine {
            button_a,
            button_b,
            prize,
        }
    }

    fn clever_tokens_to_win(&self) -> Option<i128> {
        let top = self.prize.x * self.button_a.y - self.prize.y * self.button_a.x;
        let bottom = self.button_b.x * self.button_a.y - self.button_b.y * self.button_a.x;
 
        if top % bottom != 0 {
            return None;
        }

        let k_b = top / bottom;

        if (self.prize.x - k_b * self.button_b.x) % self.button_a.x != 0 {
            return None;
        }

        let k_a = (self.prize.x - k_b * self.button_b.x) / self.button_a.x;

        return Some(ClawMachine::tokens(Point::new(k_a, k_b)));

    }

    fn tokens_to_win(&self) -> Option<i128> {
        let mut best_k : Option<Point<i128>> = None;

        for k_a in 0..=100 {
            let target = self.prize - self.button_a * k_a;

            if target.x % self.button_b.x != 0 || target.y % self.button_b.y != 0 {
                continue;
            }

            let k_b = target.x / self.button_b.x;
            
            if k_b * self.button_b.y != target.y {
                continue;
            }

            if k_b < 0 || k_b > 100 {
                continue;
            }

            let k = Point::new(k_a, k_b);

            match best_k {
                Some(best_k_point) => {
                    if ClawMachine::tokens(k) < ClawMachine::tokens(best_k_point) {
                        best_k = Some(k);
                    }
                },
                None => best_k = Some(k),
            }
        }

        match best_k {
            Some(best_k_point) => {
                let tokens = ClawMachine::tokens(best_k_point);
                Some(tokens)
            }
            None => None,
        }
    }

    fn tokens(k: Point<i128>) -> i128 {
        k.x * 3 + k.y * 1
    }
}

fn main() {
    let file = include_str!("input.txt");

    let mut claw_machines : Vec<ClawMachine> = file.replace("\r", "").split("\n\n").map(|machine| {
        let mut lines = machine.lines();
        let button_a = lines.next().unwrap()
            .replace("Button A: X+", "")
            .replace("Y+", "")
            .split(", ")
            .map(|n| n.parse::<i128>().unwrap())
            .collect();
        let button_b = lines.next().unwrap()
            .replace("Button B: X+", "")
            .replace("Y+", "")
            .split(", ")
            .map(|n| n.parse::<i128>().unwrap())
            .collect();
        let prize = lines.next().unwrap()
            .replace("Prize: X=", "")
            .replace("Y=", "")
            .split(", ")
            .map(|n| n.parse::<i128>().unwrap())
            .collect();
        ClawMachine::new(button_a, button_b, prize)
    }).collect();

    println!("PART 1: {}", claw_machines.iter().map(|machine| machine.clever_tokens_to_win().unwrap_or(0)).sum::<i128>());

    for machine in claw_machines.iter_mut() {
        machine.prize += Point::new(10000000000000, 10000000000000);
    }

    println!("PART 2: {}", claw_machines.iter().map(|machine| machine.clever_tokens_to_win().unwrap_or(0)).sum::<i128>());

    // its not 31469
}


// tx = k_a * ax + k_b * bx
// ty = k_a * ay + k_b * by

// k_a = (tx - k_b * bx) / ax
// k_a = (ty - k_b * by) / ay

// (tx - k_b * bx) / ax = (ty - k_b * by) / ay
// (tx - k_b * bx) * ay = (ty - k_b * by) * ax
// tx * ay - k_b * bx * ay = ty * ax - k_b * by * ax
// tx * ay - ty * ax = k_b * bx * ay - k_b * by * ax
// tx * ay - ty * ax = k_b * (bx * ay - by * ax)
// (tx * ay - ty * ax) / (bx * ay - by * ax) = k_b