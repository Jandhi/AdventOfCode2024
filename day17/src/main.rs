#[derive(Debug, Clone)]
struct State {
    register_a : i64,
    register_b : i64,
    register_c : i64,
    program_counter : usize,
    program : Vec<i64>,
    output : String,
}

impl State {
    fn run(&mut self) {
        while self.advance() {}
    }

    fn advance(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return false;
        }

        let mut increment_program_counter = true;

        let opcode = OpCode::from(self.program[self.program_counter]);
        let operand = self.program[self.program_counter + 1];
        
        let literal_operand = operand;
        let combo_operand = self.calculate_combo(operand);

        match opcode {
            OpCode::ADV => {
                self.register_a /= 2_i64.pow(combo_operand as u32)
            },
            OpCode::BXL => {
                self.register_b ^= literal_operand;
            },
            OpCode::BST => {
                self.register_b = combo_operand % 8;
            },
            OpCode::JNZ => {
                if self.register_a != 0 {
                    increment_program_counter = false;
                    self.program_counter = literal_operand as usize;
                }
            },
            OpCode::BXC => {
                self.register_b = self.register_b ^ self.register_c;
            },
            OpCode::OUT => {
                if self.output.len() > 0 {
                    self.output.push_str(",");
                }
                self.output.push_str(format!("{}", combo_operand % 8).as_str());
            },
            OpCode::BDV => {
                self.register_b = self.register_a / 2_i64.pow(combo_operand as u32);
            },
            OpCode::CDV => {
                self.register_c = self.register_a / 2_i64.pow(combo_operand as u32);
            },
        }

        if increment_program_counter {
            self.program_counter += 2;
        }

        return true;
    }

    fn calculate_combo(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid Operand")
        }
    }
}

enum OpCode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

impl OpCode {
    fn from(value: i64) -> OpCode {
        match value {
            0 => OpCode::ADV,
            1 => OpCode::BXL,
            2 => OpCode::BST,
            3 => OpCode::JNZ,
            4 => OpCode::BXC,
            5 => OpCode::OUT,
            6 => OpCode::BDV,
            7 => OpCode::CDV,
            _ => panic!("Invalid OpCode")
        }
    }
}

fn main() {
    let file = include_str!("input.txt");
    let mut lines = file.lines();

    let reg_a = lines.next().unwrap().replace("Register A: ", "").parse::<i64>().unwrap();
    let reg_b = lines.next().unwrap().replace("Register B: ", "").parse::<i64>().unwrap();
    let reg_c = lines.next().unwrap().replace("Register C: ", "").parse::<i64>().unwrap();
    lines.next();
    let program_str = lines.next().unwrap().replace("Program: ", "");
    let program = program_str.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let original_state = State {
        register_a: reg_a,
        register_b: reg_b,
        register_c: reg_c,
        program_counter: 0,
        program: program.clone(),
        output: String::new(),
    };

    let mut p1_state = original_state.clone();
    p1_state.run();

    println!("PART 1: {}", p1_state.output);

    // let mut state = original_state.clone();
    // state.register_a = combine_bits(&vec![1, 0, 3, 5, 1, 0, 0]);

    // state.run();
    

    let mut bits : Vec<Vec<i64>> = Vec::new();

    for i in 0..program.len() {
        bits.push(Vec::new());
        let nums = program[program.len()-1-i..].to_vec();
        let target = nums.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        println!("target: {}", target); 

        let mut new_bits = bits.clone();
        
        for val in 0..=7 {
            new_bits.last_mut().unwrap().push(val);

            if make_possibilities(&new_bits).iter().any(|possibility| {
                let mut state = original_state.clone();
                state.register_a = *possibility;
                state.run();
                return state.output == target;
            }) {
                bits.last_mut().unwrap().push(val);
            }

            new_bits.last_mut().unwrap().pop();
        }

        println!("target: {} bits: {:?}", target, bits);
    }

    for possibility in make_possibilities(&bits) {
        let mut state = original_state.clone();
        state.register_a = possibility;
        state.run();
        if state.output == program_str {
            println!("PART 2: {}", possibility);
            break;
        }
    }

    //println!("PART 2: {}", a);
}

fn make_possibilities(bits : &Vec<Vec<i64>>) -> Vec<i64> {
    let mut possibilities = Vec::new();
    possibilities.push(0);

    for bit in bits {
        possibilities = add_possibilities(possibilities, bit);
    }

    return possibilities;
}

fn add_possibilities(possibilities : Vec<i64>, bit : &Vec<i64>) -> Vec<i64> {
    let mut new_possibilities = Vec::new();

    for possibility in possibilities {
        if bit.len() == 0 {
            for val in 0..=7 {
                new_possibilities.push(possibility << 3 | val);
            }
        } else {
            for val in bit {
                new_possibilities.push(possibility << 3 | val);
            }
        }
    }

    return new_possibilities;
    
}