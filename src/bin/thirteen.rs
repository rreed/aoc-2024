// https://adventofcode.com/2024/day/12
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// apparently this is called a "Linear Diophantine Equation"
// cool :3

struct ClawMachine {
    a_button: (f64, f64),
    b_button: (f64, f64),
    prize: (f64, f64),
}

fn min_steps(a: (f64, f64), b: (f64, f64), target: (f64, f64)) -> i64 {
    // do a bunch of linear algebra
    // I did not have a clue what a "Linear Diophantine Equation" was before this but
    // well, had to dust my linear algebra to do this
    let factor = 1.0 / (a.0 * b.1 - a.1 * b.0);
    let mut inverse_matrix: Vec<Vec<f64>> = vec![vec![0.0; 2]; 2];
    inverse_matrix[0][0] = factor * b.1;
    inverse_matrix[1][0] = factor * -b.0;
    inverse_matrix[0][1] = factor * -a.1;
    inverse_matrix[1][1] = factor * a.0;

    let a_presses = inverse_matrix[0][0] * target.0 + inverse_matrix[1][0] * target.1;
    let b_presses = inverse_matrix[0][1] * target.0 + inverse_matrix[1][1] * target.1;
    // you can, in fact, half-press A
    if (b_presses.fract() < 0.001 || b_presses.fract() > 0.999)
        && (a_presses.fract() < 0.01 || a_presses.fract() > 0.999)
    {
        let a = a_presses.round() as i64;
        let b = b_presses.round() as i64;
        return a * 3 + b;
    }
    0
}

fn read_coordinates(s: String) -> (f64, f64) {
    let re = Regex::new(r"[-+]?\d+").unwrap();

    let numbers: Vec<f64> = re
        .find_iter(&s)
        .filter_map(|mat| mat.as_str().parse::<f64>().ok())
        .collect();

    return (numbers[0], numbers[1]);
}

fn read_input(filepath: &str) -> io::Result<Vec<ClawMachine>> {
    let reader = BufReader::new(File::open(filepath).unwrap());
    let mut machines: Vec<ClawMachine> = Vec::new();
    let mut next_a = (0f64, 0f64);
    let mut next_b = (0f64, 0f64);
    let mut next_prize = (0f64, 0f64);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("Button A:") {
            next_a = read_coordinates(line);
        } else if line.starts_with("Button B:") {
            next_b = read_coordinates(line);
        } else if line.starts_with("Prize:") {
            next_prize = read_coordinates(line);
        } else {
            machines.push(ClawMachine {
                a_button: next_a,
                b_button: next_b,
                prize: next_prize,
            });
        }
    }

    Ok(machines)
}

fn min_tokens_one(machine: &ClawMachine) -> i64 {
    return min_steps(machine.a_button, machine.b_button, machine.prize);
}

fn min_tokens_two(machine: &mut ClawMachine) -> i64 {
    machine.prize.0 = machine.prize.0 + 10000000000000f64;
    machine.prize.1 = machine.prize.1 + 10000000000000f64;
    return min_steps(machine.a_button, machine.b_button, machine.prize);
}

fn main() -> io::Result<()> {
    let mut machines = read_input("thirteen.in")?;

    let ans_one = machines
        .iter()
        .map(|machine| min_tokens_one(machine))
        .fold(0, |acc, x| acc + x);
    println!("Ans Part One: {ans_one}");

    let ans_two = machines
        .iter_mut()
        .map(|machine| min_tokens_two(machine))
        .fold(0, |acc, x| acc + x);
    println!("Ans Part Two: {ans_two}");
    Ok(())
}
