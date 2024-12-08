struct Equation {
    left : i64,
    right : Vec<i64>,
}

impl Equation {
    fn new(left: i64, right: Vec<i64>) -> Equation {
        Equation { left, right }
    }

    fn is_solvable_p1(&self) -> bool {
        for i in 0..i64::pow(2, self.right.len() as u32 - 1) {
            let mut operations = Vec::new();
            for j in 0..self.right.len() - 1 {
                if i & (1 << j) != 0 {
                    operations.push(Operation::Multiply);
                } else {
                    operations.push(Operation::Add);
                }
            }

            let solved = self.solve(self.right.clone(), operations.clone());

            if solved == self.left {
                return true;
            }
        }

        return false
    }

    fn is_solvable_p2(&self) -> bool {
        for i in 0..i64::pow(3, self.right.len() as u32 - 1) {
            let mut operations = Vec::new();
            for j in 0..self.right.len() - 1 {
                match i / i64::pow(3, j as u32) % 3 {
                    0 => operations.push(Operation::Add),
                    1 => operations.push(Operation::Multiply),
                    2 => operations.push(Operation::Concatenate),
                    _ => panic!(),
                }
            }

            let solved = self.solve(self.right.clone(), operations.clone());

            if solved == self.left {
                return true;
            }
        }

        return false
    }

    fn solve(&self, nums : Vec<i64>, operations : Vec<Operation>) -> i64 {
        let mut sum = nums[0];

        for (num, op) in nums.iter().skip(1).zip(operations.iter()) {
            match op {
                Operation::Add => sum += num,
                Operation::Multiply => sum *= num,
                Operation::Concatenate => sum = format!("{}{}", sum, num).parse().unwrap(),
            }
        }

        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn main() {
    let file = include_str!("input.txt");

    let equations : Vec<Equation> = file.lines().map(|line| {
        let mut parts = line.split(": ");
        let left = parts.next().unwrap().parse().unwrap();
        let right = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
        Equation::new(left, right)
    }).collect();

    let sum = equations.iter().filter(|eq| eq.is_solvable_p1()).map(|eq| eq.left).sum::<i64>();

    println!("PART 1: {}", sum);

    let sum = equations.iter().filter(|eq| eq.is_solvable_p2()).map(|eq| eq.left).sum::<i64>();

    println!("PART 2: {}", sum);
}
