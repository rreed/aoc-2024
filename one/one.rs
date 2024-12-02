// https://adventofcode.com/2024/day/1
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if let [left, right] = numbers[..] {
            let l = left.parse::<i32>().expect("Failed to parse left number");
            let r = right.parse::<i32>().expect("Failed to parse right number");

            left_list.push(l);
            right_list.push(r);
        }
    }

    Ok((left_list, right_list))
}

fn similarity_score(i: i32, v: &Vec<i32>) -> i32 {
    return v.iter().filter(|&n| *n == i).count() as i32;
}

fn main() -> io::Result<()> {
    let (mut left, mut right) = read_input("one.in")?;

    left.sort();
    right.sort();

    let sum1 = left
        .iter()
        .zip(&right)
        .map(|(l, r)| (l - r).abs())
        .fold(0, |acc, x| acc + x);

    let sum2 = left
        .iter()
        .map(|l| l * similarity_score(*l, &right))
        .fold(0, |acc, x| acc + x);

    println!("Ans Part One: {sum1}");
    println!("Ans Part Two: {sum2}");

    Ok(())
}
