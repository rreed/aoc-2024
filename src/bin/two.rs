// https://adventofcode.com/2024/day/2
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut ret = Vec::new();

    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        let ints: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();

        match ints {
            Ok(nums) => ret.push(nums),
            Err(e) => println!("Error parsing numbers: {}", e),
        }
    }

    Ok(ret)
}

fn is_safe_one(numbers: &Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let mut ascending = true;
    let mut descending = true;

    for (_, window) in numbers.windows(2).enumerate() {
        let (prev, current) = (window[0], window[1]);

        if prev == current {
            return false;
        }

        if (current - prev).abs() > 3 {
            return false;
        }

        if current > prev {
            descending = false;
        } else if current < prev {
            ascending = false;
        }
    }

    ascending || descending
}

fn is_safe_two(numbers: &Vec<i32>) -> bool {
    if is_safe_one(numbers) {
        return true;
    }
    for x in 0..numbers.len() {
        let mut cpy = numbers.clone();
        cpy.remove(x);
        if is_safe_one(&cpy) {
            return true;
        }
    }
    return false;
}

fn main() -> io::Result<()> {
    let lists = read_input("two.in")?;

    let safe_lists: Vec<&Vec<i32>> = lists.iter().filter(|l| is_safe_one(l)).collect();
    let safe_lists_with_dampener: Vec<&Vec<i32>> =
        lists.iter().filter(|l| is_safe_two(l)).collect();
    let ans_one = safe_lists.len();
    let ans_two = safe_lists_with_dampener.len();

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
