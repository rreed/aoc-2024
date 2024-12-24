use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

#[derive(PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

struct GateMap {
    g1: String,
    g2: String,
    op: Op,
    out: String,
}

struct Board {
    values: HashMap<String, u8>,
    gates: Vec<GateMap>,
}

fn read_input(filepath: &str) -> io::Result<Board> {
    let input = fs::read_to_string(&filepath).unwrap();
    let mut values: HashMap<String, u8> = HashMap::new();
    let mut gates: Vec<GateMap> = Vec::new();

    let mut sections = input.splitn(2, "\n\n");

    sections
        .next()
        .unwrap_or("Failed to find initial values")
        .lines()
        .for_each(|line| {
            let mut split = line.split(": ");
            let gate = split.next().unwrap();
            let val = split.next().unwrap().parse::<u8>().unwrap();
            values.insert(gate.to_string(), val);
        });
    sections
        .next()
        .unwrap_or("Failed to find gates")
        .lines()
        .for_each(|line| {
            let mut split = line.split(" ");
            let g1 = split.next().unwrap();
            let op = match split.next().unwrap() {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Invalid operator"),
            };
            let g2 = split.next().unwrap();
            let _ = split.next(); // ignore " -> "
            let out = split.next().unwrap();
            gates.push(GateMap {
                g1: g1.to_string(),
                g2: g2.to_string(),
                op,
                out: out.to_string(),
            });
        });
    return Ok(Board { values, gates });
}

fn main() {
    let mut board = read_input("twentyfour.in").unwrap();
    let mut remaining_gates: Vec<_> = board.gates.iter().collect();

    while !remaining_gates.is_empty() {
        let mut next_remaining_gates = Vec::new();

        for g in remaining_gates {
            let g1 = match board.values.get(&g.g1) {
                Some(val) => *val,
                None => {
                    next_remaining_gates.push(g);
                    continue;
                }
            };
            let g2 = match board.values.get(&g.g2) {
                Some(val) => *val,
                None => {
                    next_remaining_gates.push(g);
                    continue;
                }
            };
            let op = match g.op {
                Op::AND => g1 & g2,
                Op::OR => g1 | g2,
                Op::XOR => g1 ^ g2,
            };
            board.values.insert(g.out.clone(), op);
        }

        remaining_gates = next_remaining_gates;
    }

    let mut z_gates: Vec<_> = board.values.keys().filter(|k| k.starts_with('z')).collect();
    z_gates.sort_by_key(|k| k[1..].parse::<usize>().unwrap());

    let mut ans_one_binary: String = z_gates
        .iter()
        .map(|k| board.values.get(*k).unwrap().to_string())
        .collect();
    ans_one_binary = ans_one_binary.chars().rev().collect();

    let ans_one = u64::from_str_radix(&ans_one_binary, 2).unwrap();
    println!("Ans Part One: {}", ans_one);

    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for g in &board.gates {
        edges.entry(g.g1.as_str()).or_default().push(g.out.as_str());
        edges.entry(g.g2.as_str()).or_default().push(g.out.as_str());
    }

    let mut broken_nodes = HashSet::new();
    for g in &board.gates {
        // z nodes must be XOR (except for the last one)
        if g.out.starts_with("z") && g.out != **z_gates.last().unwrap() && g.op != Op::XOR {
            broken_nodes.insert(g.out.clone());
        }
        // z nodes must not be the inputs of other nodes
        if g.g1.starts_with("z") {
            broken_nodes.insert(g.g1.clone());
        }
        if g.g2.starts_with("z") {
            broken_nodes.insert(g.g2.clone());
        }

        // inputs of XOR nodes (except for z nodes) must be x and y nodes
        if g.op == Op::XOR
            && !g.out.starts_with("z")
            && !((g.g1.starts_with("x") && g.g2.starts_with("y"))
                || (g.g1.starts_with("y") && g.g2.starts_with("x")))
        {
            broken_nodes.insert(g.out.clone());
        }

        // XOR nodes that are not z nodes must always be the input of exactly two other nodes
        if g.op == Op::XOR && !g.out.starts_with("z") && edges[g.out.as_str()].len() != 2 {
            broken_nodes.insert(g.out.clone());
        }

        // AND nodes must always be input of exactly one other node (except the very first one)
        if g.op == Op::AND
            && !g.out.starts_with("z")
            && edges[g.out.as_str()].len() != 1
            && !((g.g1 == "x00" && g.g2 == "y00") || (g.g1 == "y00" && g.g2 == "x00"))
        {
            broken_nodes.insert(g.out.clone());
        }
    }

    let mut ans_two = broken_nodes.into_iter().collect::<Vec<_>>();
    ans_two.sort();
    println!("Ans Part Two: {}", ans_two.join(","));
}
