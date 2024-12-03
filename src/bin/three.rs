// https://adventofcode.com/2024/day/3
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<String> {
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut ret = String::new();

    for line in reader.lines() {
        let line = line?;
        ret.push_str(&line);
        // no line breaks
    }

    Ok(ret)
}

fn extract_valid_muls(input: &str) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for captures in re.captures_iter(input) {
        if let (Some(l), Some(r)) = (captures.get(1), captures.get(2)) {
            if let (Ok(l), Ok(r)) = (l.as_str().parse::<i32>(), r.as_str().parse::<i32>()) {
                ret.push((l, r));
            }
        }
    }
    ret
}

fn input_without_dont(input: &str) -> String {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    re.replace_all(input, "").to_string()
}

fn main() -> io::Result<()> {
    let input_str = read_input("three.in")?;

    let mults_one = extract_valid_muls(&input_str);
    let ans_one = mults_one
        .iter()
        .map(|(l, r)| l * r)
        .fold(0, |acc, x| acc + x);
    println!("Ans Part One: {ans_one}");

    let no_donts = input_without_dont(&input_str);
    let mults_two = extract_valid_muls(&no_donts);
    let ans_two = mults_two
        .iter()
        .map(|(l, r)| l * r)
        .fold(0, |acc, x| acc + x);
    println!("Ans Part Two: {ans_two}");
    Ok(())
}
