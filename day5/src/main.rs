use itertools::Itertools;

struct Constraint {
    item : i32,
    
}

fn main() {
    let file = include_str!("input.txt").replace("\r", "");

    let parts = file.split("\n\n").collect::<Vec<&str>>();


    let pairs : Vec<(i32, i32)> = parts[0]
        .split("\n")
        .filter(|line| line.trim().len() > 0)
        .map(|pair| {
            pair.split("|").map(|num| {
                num.trim().parse().unwrap()
            })
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let inputs : Vec<Vec<i32>> = parts[1]
        .split("\n")
        .map(|row| {
            row.split(",").map(|num| num.parse().unwrap()).collect()
        })
        .collect();

    let part1 : i32 = inputs.iter()
        .filter(|row| {
            pairs.iter().all(|(a, b)| {
                if !row.contains(a) || !row.contains(b) {
                    return true;
                }

                row.iter().position(|num| num == a).unwrap() < row.iter().position(|num| num == b).unwrap()
            })
        })
        .map(|row| row[row.len() / 2])
        .sum();

    println!("Part 1: {}", part1);

    let part2 : i32 = inputs.iter()
        .filter(|row| {
            pairs.iter().any(|(a, b)| {
                if !row.contains(a) || !row.contains(b) {
                    return false;
                }

                !(row.iter().position(|num| num == a).unwrap() < row.iter().position(|num| num == b).unwrap())
            })
        })
        .map(|row| {
            println!("{:?}", row);
            let sorted = get_sorted(row, &pairs);
            sorted[sorted.len() / 2]
        })
        .sum();

    println!("Part 2: {}", part2);
}

fn get_sorted(row : &Vec<i32>, pairs : &Vec<(i32, i32)>) -> Vec<i32> {
    let sorted : Vec<i32> = vec![];

    for num in row {

    }

    sorted
}

