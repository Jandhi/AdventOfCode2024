use std::collections::HashMap;

fn can_build(pattern : String, towels : &Vec<&str>) -> bool {
    for  towel in towels.iter() {
        if pattern.starts_with(towel) {

            if pattern.len() == towel.len() {

                return true;
            }

            if can_build(pattern[towel.len()..].to_string(), towels) {
                return true;
            }
        }
    }

    false
}

fn build_amounts(pattern : String, towels : &Vec<&str>, cache : &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(&pattern) {
        return *cache.get(&pattern).unwrap();
    }

    let mut amounts=  0;

    for towel in towels.iter() {
        if pattern.starts_with(towel) {

            if pattern.len() == towel.len() {
                amounts += 1;
            } else {
                amounts += build_amounts(pattern[towel.len()..].to_string(), towels, cache);
            }
        }
    }

    cache.insert(pattern, amounts);
    amounts
}

fn main() {
    let file = include_str!("input.txt");
    let mut lines = file.lines();

    let towels : Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next();

    let patterns : Vec<&str> = lines.collect();

    let possible_towels = patterns.iter().filter(|pattern| {
        can_build(pattern.to_string(), &towels)
    }).count();

    println!("PART 1: {}", possible_towels);

    let mut cache = HashMap::new();

    let amounts : usize = patterns.iter().map(|pattern| {
        println!("{}", pattern);
        build_amounts(pattern.to_string(), &towels, &mut cache)
    }).sum();

    println!("PART 2: {}", amounts);
}
