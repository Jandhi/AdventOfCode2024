fn main() {
    rewrite();
}

fn rewrite() {
    let file = include_str!("input.txt");

    let (vec1  , vec2): (Vec<i32>, Vec<i32>) = file
        .lines()
        .map(|line| {
            let parts = line.split("   ").collect::<Vec<&str>>();
            (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
        })
        .unzip();

    let mut vec1sorted = vec1.clone();
    vec1sorted.sort();
    let mut vec2sorted = vec2.clone();
    vec2sorted.sort();

    let diffs: i32 = vec1sorted
        .iter()
        .zip(vec2sorted.iter())
        .map(|(a, b)| i32::abs(a - b))
        .sum();

    println!("Part 1: {}", diffs);

    let score: i32 = vec1
        .iter()
        .map(|num| vec2.iter().filter(|&n| *n == *num).count() as i32 * num)
        .sum();

    println!("Part 2: {}", score);
}


fn first_solution() {
    let file = include_str!("input.txt");

    let mut vec1 = vec![];
    let mut vec2 = vec![];

    for line in file.trim().lines() {
        if line.is_empty() {
            continue;
        }
        let parts = line.split("   ").collect::<Vec<&str>>();

        vec1.push(parts[0].parse::<i32>().unwrap());
        vec2.push(parts[1].parse::<i32>().unwrap());
    }

    let mut vec1copy = vec1.clone();
    let mut vec2copy = vec2.clone();

    vec1.sort();
    vec2.sort();

    let mut diffs = 0;

    for i in 0..vec1.len() {
        diffs += i32::abs(vec1[i] - vec2[i]);
    }

    println!("{}", diffs);

    let mut score = 0;

    for i in 0..vec1copy.len() {
        score += (vec1copy[i] * vec2copy.iter().filter(|num| **num == vec1copy[i]).count() as i32);
    }

    println!("{}", score);
}
