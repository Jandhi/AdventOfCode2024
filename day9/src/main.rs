use std::slice::Chunks;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    index: usize,
    length: usize,
    empty: usize,
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

    for i in 0..p2_files.len() {
        let id = p2_files.len() - 1 - i; // start from back
        let index = p2_files.iter().position(|file| file.id == id).unwrap();

        for mut j in 0..index {
            if p2_files[j].empty >= p2_files[index].length {
                //println!("{:?} has space for {:?}", p2_files[j], p2_files[index]);

                let mut file = p2_files.remove(index);
                
                if j > index {
                    j -= 1;
                }

                p2_files[index - 1].empty += file.length + file.empty;
                
                file.index = p2_files[j].index + p2_files[j].length;
                file.empty = p2_files[j].empty - file.length;
                
                p2_files[j].empty = 0;

                p2_files.insert(j + 1, file);
            }
        }
    }

    let mut checksum = 0;
    let mut i = 0;
    for file in p2_files.iter() {
        for _ in 0..file.length {
            //print!("{}", file.id);

            //println!("{} * {}", file.id, i);
            checksum += file.id * i;
            i += 1;
        }

        for _ in 0..file.empty {
            //print!(".");
            i += 1;
        }
    }

    println!("Part 2: {}", checksum);
}