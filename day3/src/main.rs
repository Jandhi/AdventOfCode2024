fn main() {
    let file = include_str!("input.txt");
    
    let prod : i32 = get_prods(file);

    println!("PART 1: {}", prod);

    let parts : Vec<&str> = file.split("don't()").collect();

    let mut sum = get_prods(parts[0]);

    for part in parts.iter().skip(1) {
        let do_index = part.find("do()");

        if do_index == None {
            continue;
        }

        let do_part = &part[do_index.unwrap()..];

        sum += get_prods(do_part);
    }

    println!("PART 2: {}", sum);
}

fn get_prods(string : &str) -> i32 {
    string.split("mul(")
    .skip(1)
    .map(|part|{

        let index = part.find(")");

        if index == None {
            return 0;
        }

        let content = &part[0..index.unwrap()];

        if !content.contains(",") {
            return 0;
        }

        let (left, right) = content.split_at(content.find(",").unwrap());

        let left_int = left.parse::<i32>();
        let right_int = right[1..].parse::<i32>();

        if left_int.is_err() || right_int.is_err() {
            return 0;
        }

        return left_int.unwrap() * right_int.unwrap();
    })
    .sum()
}