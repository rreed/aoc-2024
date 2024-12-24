use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs::{self},
    io,
};

struct Network {
    connections: HashSet<(String, String)>,
    computers: HashSet<String>,
}

fn read_input(filepath: &str) -> io::Result<Network> {
    let input = fs::read_to_string(&filepath).unwrap();
    let mut connections: HashSet<(String, String)> = HashSet::new();
    let mut computers: HashSet<String> = HashSet::new();

    input.split('\n').for_each(|s| {
        let (a, b) = s.split_once('-').unwrap();
        connections.insert((a.to_string(), b.to_string()));
        connections.insert((b.to_string(), a.to_string()));

        computers.insert(a.to_string());
        computers.insert(b.to_string());
    });
    return Ok(Network {
        connections,
        computers,
    });
}

fn main() {
    let network = read_input("twentythree.in").unwrap();
    let mut triangles: HashSet<Vec<String>> = HashSet::new();

    for (a, b) in &network.connections {
        if !a.starts_with('t') {
            continue;
        }

        for comp in &network.computers {
            if comp == a || comp == b {
                continue;
            }

            if network.connections.contains(&(a.clone(), comp.clone()))
                && network.connections.contains(&(b.clone(), comp.clone()))
            {
                let mut triangle = vec![a.clone(), b.clone(), comp.clone()];

                triangle.sort();
                triangles.insert(triangle);
            }
        }
    }

    let ans_one = triangles.len();
    println!("Ans Part One: {ans_one}");

    let mut computer_vec: Vec<String> = network.computers.into_iter().collect();
    computer_vec.sort();
    let mut computer_deque: VecDeque<String> = computer_vec.into_iter().collect();

    let mut max_groups: Vec<Vec<String>> = Vec::new();

    while let Some(seed) = computer_deque.pop_front() {
        let mut curr_group = vec![seed];

        let mut is_maximum = false;
        'outer: while !is_maximum {
            for (idx, computer) in computer_deque.iter().enumerate() {
                if curr_group.iter().all(|curr_computer| {
                    network
                        .connections
                        .contains(&(curr_computer.clone(), computer.clone()))
                }) {
                    curr_group.push(computer_deque.remove(idx).unwrap());
                    continue 'outer;
                }
            }

            is_maximum = true;
        }

        max_groups.push(curr_group);
    }

    let ans_two = max_groups
        .into_iter()
        .max_by_key(Vec::len)
        .unwrap()
        .into_iter()
        .join(",");
    println!("Ans Part Two: {ans_two}");
}
