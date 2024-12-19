use std::slice::Chunks;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    index: usize,
    length: usize,
    empty: usize,
}

fn get_checksum(files : &Vec<File>) -> usize {
    let mut i = 0;
    let mut sum = 0;

    for file in files.iter() {
        let mut indices = (i..i + file.length)
            .fold("(".to_string(), |str, val| format!("{}{}+", str, val));

        indices.pop();
        indices += ")";

        sum += (i..i + file.length).sum::<usize>() * file.id;

        println!("{} * {} = {}", indices, file.id, (i..i + file.length).sum::<usize>() * file.id);


        i += file.length + file.empty;
    }

    sum
}

fn print_files(files : &Vec<File>) {
    for file in files.iter() {
        for _ in 0..file.length {
            print!("{}", file.id);
        }

        for _ in 0..file.empty {
            print!(".");
        }
    }

    println!();
}

fn main() {
    let mut file = include_str!("input.txt").to_string();

    if file.len() % 2 == 1 {
        file.push('0');
    }

    let mut id = 0;
    let mut index = 0;

    let files = file.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let (size, empty) = (chunk[0].to_digit(10).unwrap() as usize, chunk[1].to_digit(10).unwrap() as usize);
            let file = File { id, index, length: size, empty };
            id += 1;
            index += size + empty;
            file
        })
        .collect::<Vec<_>>();

    let mut p1_files = files.clone();

    let mut checksum = 0;
    let mut i = 0;

    while p1_files.len() > 0 {
        let file = p1_files.remove(0);
        
        for _ in 0..file.length {
            checksum += file.id * i;
            i += 1;
        }

        for _ in 0..file.empty {
            let last_file = p1_files.last_mut();

            match last_file {
                Some(last_file) => {
                    checksum += i * last_file.id;
                    i += 1;
                    last_file.length -= 1;

                    if last_file.length == 0 {
                        p1_files.pop();
                    }
                }
                None => break, 
            }
        }
    }

    println!("Part 1: {}", checksum);

    let mut p2_files = files.clone();

    for file_id in (0..id).rev() {
        let pos = p2_files.iter().position(|file| file.id == file_id).unwrap();

        for new_pos in 0..pos {
            if p2_files[new_pos].empty < p2_files[pos].length {
                continue;
            }
            
            // Add empty space to the file originally preceding it
            p2_files[pos - 1].empty += p2_files[pos].empty + p2_files[pos].length;

            // The file only has as much empty space as is left from the new prededing file
            p2_files[pos].empty = p2_files[new_pos].empty - p2_files[pos].length;

            // The new file preceding our file will have no empty spots
            p2_files[new_pos].empty = 0;

            
            let file = p2_files.remove(pos);
            p2_files.insert(new_pos + 1, file);

            break;
        }
    }

    println!("Part 2: {}", get_checksum(&p2_files));
}