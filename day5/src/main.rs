use itertools::Itertools;

#[derive(Debug)]
struct Graph {
    nodes : Vec<i32>,
    edges : Vec<(i32, i32)>
}

impl Graph {
    fn new(edges:  Vec<(i32, i32)>) -> Graph {
        let mut nodes = vec![];

        for (a, b) in edges.iter() {
            if !nodes.contains(a) {
                nodes.push(*a);
            }

            if !nodes.contains(b) {
                nodes.push(*b);
            }
        }

        Graph {
            nodes,
            edges
        }
    } 

    fn pop_leaf(&mut self, node : i32) {
        let index = self.nodes.iter().position(|n| *n == node).unwrap();
        self.nodes.remove(index);

        self.edges = self.edges.iter().filter(|(a, b)| {
            *a != node && *b != node
        }).map(|(a, b)| {
            (*a, *b)
        }).collect();
    }

    fn first_leaf(&self) -> i32 {
        match self.nodes.iter().find(|node| {
            self.is_leaf(**node)
        }) {
            Some(node) => *node,
            None => panic!("No leaf found in graph {:?}", self)
        }
    }
    
    fn is_leaf(&self, node : i32) -> bool {
        self.edges.iter().all(|(a, _)| {
            *a != node
        })
    }
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
    let mut graph = Graph::new(pairs.iter().filter(|(a, b)| {
        row.contains(a) && row.contains(b)
    })
    .map(|(a, b)| (*a, *b))
    .collect());
    let mut the_grand_order_of_all_things = vec![];

    while graph.nodes.len() > 0 {
        let leaf = graph.first_leaf();
        the_grand_order_of_all_things.insert(0, leaf);
        graph.pop_leaf(leaf);
    }

    the_grand_order_of_all_things
}

