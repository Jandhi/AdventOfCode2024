use std::collections::HashSet;

fn prune(num : i64) -> i64 {
    num % 16777216
}

fn next(mut num : i64) -> i64 {
    num = prune(num ^ (num * 64));
    num = prune(num ^ (num / 32));
    num = prune(num ^ (num * 2048));
    num
}

fn nth_secret_number(mut num : i64, n : i64) -> i64 {
    for _ in 0..n {
        num = next(num);
    }
    num
}

fn generate_numbers(num : i64, n : i64) -> Vec<i64> {
    let mut nums = Vec::new();
    let mut num = num;
    for _ in 0..n {
        nums.push(num);
        num = next(num);
    }
    nums
}

fn use_sequence(prices : &Vec<i8>, diffs : &Vec<i8>, sequence : &[i8]) -> i64 {
    for i in 0..diffs.len() - sequence.len() + 1 {
        if sequence[0] == diffs[i] && sequence[1] == diffs[i + 1] && sequence[2] == diffs[i + 2] && sequence[3] == diffs[i + 3] {
            return prices[i + 4] as i64;
        }
    }

    0
}

fn main() {
    let file = include_str!("input.txt");

    let nums = file.lines().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let sum = nums.iter().map(|num| nth_secret_number(*num, 2000)).sum::<i64>();

    println!("PART 1: {}", sum);
    
    let prices : Vec<Vec<i8>> = nums.iter().map(|seed| {
        let mut num = *seed;
        let mut sequence = Vec::new();

        for _ in 0..2000 {
            sequence.push((num % 10) as i8);
            num = next(num);
        }

        sequence
    }).collect();

    let diffs : Vec<Vec<i8>> = prices.iter().map(|sequence| {
        sequence.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i8>>()
    }).collect();

    let sequences : HashSet<Vec<i8>> = diffs.iter().map(|diff_vec| {
        diff_vec.windows(4).map(|x| x.to_vec()).collect::<HashSet<Vec<i8>>>()
    }).flatten().collect();

    let best_bananas = sequences.iter().enumerate().map(|(index, seq)| {
        let bananas : i64 = nums.iter().enumerate().map(|(i, _)| {
            use_sequence(&prices[i as usize], &diffs[i as usize], seq)
        }).sum();

        println!("{}/{} : {}", index, sequences.len(), bananas);
        bananas
    }).max().unwrap();

    println!("PART 2: {}", best_bananas);
}
