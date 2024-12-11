use std::collections::HashMap;
// https://adventofcode.com/2024/day/11
// alright time for Outer Wilds I guess
use std::fs;
use std::io::{self};

fn read_input(filepath: &str) -> io::Result<Vec<i64>> {
    let input = fs::read_to_string(&filepath)?;
    let numbers: Vec<&str> = input.split_whitespace().collect();
    let ints: Result<Vec<i64>, _> = numbers.iter().map(|s| s.parse::<i64>()).collect();

    ints.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn split_number_in_half(n: i64) -> (i64, i64) {
    let digits = n.to_string();
    let mid = digits.len() / 2;
    let l = digits[..mid].parse::<i64>().unwrap();
    let r = digits[mid..].parse::<i64>().unwrap();
    (l, r)
}

fn process_stone(memory: &mut HashMap<(i32, i64), i64>, stone: i64, iterations: i32) -> i64 {
    if iterations == 0 {
        return 1;
    }
    if stone == 0 {
        if iterations == 1 {
            return 1;
        }
        return process_stone(memory, 2024, iterations - 2);
    }
    if stone.to_string().len() % 2 != 0 {
        return process_stone(memory, stone * 2024, iterations - 1);
    }

    if let Some(&n) = memory.get(&(iterations, stone)) {
        return n;
    }
    let (l, r) = split_number_in_half(stone);
    let m = process_stone(memory, l, iterations - 1) + process_stone(memory, r, iterations - 1);
    memory.insert((iterations, stone), m);
    m
}

fn main() -> io::Result<()> {
    let stones = read_input("eleven.in")?;
    let mut memory = HashMap::new();
    let ans_one = stones
        .iter()
        .copied()
        .map(|s| process_stone(&mut memory, s, 25))
        .fold(0, |acc, x| acc + x);

    println!("Ans Part One: {ans_one}");

    let ans_two = stones
        .iter()
        .copied()
        .map(|s| process_stone(&mut memory, s, 75))
        .fold(0, |acc, x| acc + x);

    println!("Ans Part Two: {ans_two}");

    Ok(())
}
