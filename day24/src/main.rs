use std::{clone, collections::{HashMap, HashSet}};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Operation {
    XOR,
    OR,
    AND
}

impl Operation {
    fn calculate(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::XOR => a ^ b,
            Operation::OR => a | b,
            Operation::AND => a & b
        }
    }

    fn string(&self) -> &str {
        match self {
            Operation::XOR => "XOR",
            Operation::OR => "OR",
            Operation::AND => "AND"
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Gate {
    operation: Operation,
    input1: String,
    input2: String,
    output : String
}

impl Gate {
    fn new(operation: Operation, input1: String, input2: String, output: String) -> Self {
        Gate {
            operation,
            input1,
            input2,
            output
        }
    }
}

fn calculate(wires: &HashMap<String, bool>, gates: &Vec<Gate>) -> i64 {
    let mut p1_wires = wires.clone();
    let mut p1_gates = gates.clone();
    let z_gates : HashSet<String> = gates.iter().map(|gate| gate.output.clone()).filter(|name| name.starts_with("z")).collect();

    while p1_gates.len() > 0 {
        if z_gates.iter().all(|name| p1_wires.contains_key(name)) {
            break;
        }

        let next = p1_gates.remove(0);

        if !p1_wires.contains_key(&next.input1) || !p1_wires.contains_key(&next.input2) {
           p1_gates.push(next);
           continue;
        }

        let val = next.operation.calculate(p1_wires[&next.input1], p1_wires[&next.input2]);
        p1_wires.insert(next.output, val);
    }

    

    let mut answer = 0;
    let mut i = 0;

    loop {
        let name = format!("z{:02}", i);

        if !p1_wires.contains_key(name.as_str()) {
            break;
        }

        answer += if p1_wires[name.as_str()] { 2_i64.pow(i) } else { 0 };
        i += 1;
    } 

    answer
}

fn get_wires(x : i64, y : i64, bits : usize) -> HashMap<String, bool> {
    let mut wires = HashMap::new();

    for i in 0..bits {
        wires.insert(format!("x{:02}", i), (x & (1 << i)) != 0);
        wires.insert(format!("y{:02}", i), (y & (1 << i)) != 0);
    }

    wires
}

fn main() {
    let file = include_str!("input.txt").replace("\r", "");

    let mut parts = file.split("\n\n");

    let wires : HashMap<String, bool> = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split(": ");
        let wire = parts.next().unwrap();
        let value = parts.next().unwrap() == "1";
        (wire.to_string(), value)
    }).collect();

    let gates : Vec<Gate> = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split(" ");

        let input1 = parts.next().unwrap();

        let operation = match parts.next().unwrap() {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!()
        };
        
        let input2 = parts.next().unwrap();
        parts.next().unwrap(); // ->
        let output = parts.last().unwrap();
        Gate::new(operation, input1.to_string(), input2.to_string(), output.to_string())
    }).collect();


    println!("Part 1: {}", calculate(&wires, &gates));

    let bits: usize = 45;
    
    for i in 0..bits {
        for j in 0..bits {
            let val = calculate(&get_wires(2_i64.pow(i as u32), 2_i64.pow(j as u32), bits), &gates);

            if val != 2_i64.pow(i as u32) + 2_i64.pow(j as u32) {
                println!("Bits {} and {} are not adding right", i, j);
            }
        }
    }

    // let mut gates_p2 = gates.clone();
    // let mut rename_rules : HashMap<String, String> = HashMap::new();
 
    // // carry rules
    // for gate in gates_p2.iter_mut() {
    //     if gate.input1.starts_with("x") && gate.input2.starts_with("y") || 
    //         gate.input1.starts_with("y") && gate.input2.starts_with("x") {
    //         if gate.operation == Operation::AND && !gate.output.starts_with("z") {
    //             rename_rules.insert(gate.output.clone(), "car".to_string() + &gate.input1[1..]);
    //         }
    //     }
    // }

    // // xor rules
    // for gate in gates_p2.iter_mut() {
    //     if gate.input1.starts_with("x") && gate.input2.starts_with("y") || 
    //         gate.input1.starts_with("y") && gate.input2.starts_with("x") {
    //         if gate.operation == Operation::XOR && !gate.output.starts_with("z") {
    //             rename_rules.insert(gate.output.clone(), "xor".to_string() + &gate.input1[1..]);
    //         }
    //     }
    // }

    // for gate in gates_p2.iter_mut() {
    //     for (rule, replacement) in rename_rules.iter() {
    //         gate.input1 = gate.input1.replace(rule, replacement);
    //         gate.input2 = gate.input2.replace(rule, replacement);
    //         gate.output = gate.output.replace(rule, replacement);
    //     }
    // }

    // // xor rules
    // for gate in gates_p2.iter_mut() {
    //     if gate.input1.starts_with("car") && gate.input2.starts_with("xor") || 
    //         gate.input1.starts_with("xor") && gate.input2.starts_with("car") {
    //         if gate.operation == Operation::AND && !gate.output.starts_with("z")  {
    //             rename_rules.insert(gate.output.clone(), "acr".to_string() + &gate.input1[3..]);
    //         }
    //     }
    // }

    // for gate in gates_p2.iter_mut() {
    //     for (rule, replacement) in rename_rules.iter() {
    //         gate.input1 = gate.input1.replace(rule, replacement);
    //         gate.input2 = gate.input2.replace(rule, replacement);
    //         gate.output = gate.output.replace(rule, replacement);
    //     }

    //     if gate.input1.starts_with("y") && gate.input2.starts_with("x") {
    //         let temp = gate.input1.clone();
    //         gate.input1 = gate.input2.clone();
    //         gate.input2 = temp;
    //     }
    // }

    // gates_p2.sort_by_key(|gate| gate.input1.clone());

    // for gate in gates_p2.iter_mut() {
    //     println!("{} {} {} -> {}", gate.input1, gate.operation.string(), gate.input2, gate.output);
    // }
}
