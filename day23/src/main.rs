use std::collections::{HashMap, HashSet};



fn main() {
    let file = include_str!("input.txt");
    let connections : Vec<(&str, &str)> = file.lines().map(|line| {
        let mut parts = line.split("-");
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect();

    let connections_by_node : HashMap<&str, HashSet<&str>> = connections.iter().fold(HashMap::new(), |mut acc, (a, b)| {
        acc.entry(a).or_insert(HashSet::new()).insert(b);
        acc.entry(b).or_insert(HashSet::new()).insert(a);
        acc
    });

    let nodes : HashSet<&str> = connections_by_node.keys().map(|&x| x).collect();

    let mut triples : HashSet<(&str, &str, &str)> = HashSet::new();

    for node in connections_by_node.keys() {
        for other_node in connections_by_node.get(node).unwrap() {
            if node > other_node {
                continue;
            }

            let third_nodes = connections_by_node.get(node).unwrap().intersection(connections_by_node.get(other_node).unwrap());

            for third_node in third_nodes {
                if node > third_node || other_node > third_node {
                    continue;
                }

                if !node.starts_with("t") && !other_node.starts_with("t") && !third_node.starts_with("t") {
                    continue;
                }

                triples.insert((node, other_node, third_node));
            }
        }
    }

    println!("PART 1: {}", triples.len());

    let mut queue : Vec<HashSet<&str>> = vec![];

    for node in nodes.iter() {
        let mut set = HashSet::new();
        set.insert(*node);
        queue.push(set);
    }

    let mut maximum_clique : Option<HashSet<&str>> = None;

    while queue.len() > 0 {
        let next = queue.remove(0);

        println!("{:?}", next);

        if maximum_clique == None || next.len() > maximum_clique.as_ref().unwrap().len() {
            maximum_clique = Some(next.clone());
        }

        for node in nodes.difference(&next) {
            if next.iter().any(|n| n > node) {
                continue;
            }

            if next.iter().all(|&x| connections_by_node.get(x).unwrap().contains(node)) {
                let mut new_set = next.clone();
                new_set.insert(node);
                queue.push(new_set);
            }
        }
    }

    let mut password_set : Vec<&str> = maximum_clique.unwrap().iter().map(|&x| x).collect();
    password_set.sort();
    let password = password_set.join(",");
    println!("PART 2: {}", password);
}
