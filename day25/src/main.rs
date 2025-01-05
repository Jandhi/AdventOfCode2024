fn main() {
    let file = include_str!("input.txt").replace("\r", "");

    let mut keys : Vec<Vec<u8>> = vec![];
    let mut locks : Vec<Vec<u8>> = vec![];

    for part in file.split("\n\n") {
        let mut item : Vec<u8> = vec![0, 0, 0, 0, 0];

        for line in part.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    item[i] += 1;
                }
            }
        }

        if part.starts_with(".") {
            keys.push(item);
        } else {
            locks.push(item);
        }
    }


    let target_num = 7;

    let amt : usize = keys.iter().map(|key| {
        locks.iter().filter(|lock| {
            key.iter().zip(lock.iter()).all(|(k, l)| {
                *k + *l <= target_num
            })
        }).count()
    }).sum();

    println!("PART 1: {}", amt);
}
