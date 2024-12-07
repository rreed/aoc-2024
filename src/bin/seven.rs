// https://adventofcode.com/2024/day/7
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // go ahead and consume the leading whitespace
        let mut parts = line.split(": ");
        let first = parts.next().unwrap().parse::<i64>().unwrap();
        let second = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        lines.push((first, second));
    }

    Ok(lines)
}

fn generate_combinations_one(n: usize) -> Vec<String> {
    let mut combinations = vec!["".to_string()];

    for _ in 0..n {
        combinations = combinations
            .into_iter()
            .flat_map(|s| vec![format!("{}+", s), format!("{}*", s)].into_iter())
            .collect();
    }

    combinations
}

fn generate_combinations_two(n: usize) -> Vec<String> {
    let mut combinations = vec!["".to_string()];

    for _ in 0..n {
        combinations = combinations
            .into_iter()
            // yeah yeah I know it's supposed to be a double pipe but
            .flat_map(|s| vec![format!("{}+", s), format!("{}|", s), format!("{}*", s)].into_iter())
            .collect();
    }

    combinations
}

fn num_concat(a: i64, b: i64) -> i64 {
    return format!("{}{}", a, b).parse::<i64>().unwrap();
}

fn can_reach_target_one(target: i64, vals: &Vec<i64>) -> bool {
    // assert!(vals.len() > 2);
    let num_spaces = vals.len() - 1;
    let combinations = generate_combinations_one(num_spaces);
    for comb in combinations {
        // hey kids don't try to use `meval` for this, it's too PEMDAS-pilled
        let mut ops = comb.chars();
        let mut v_iter = vals.iter();
        let mut retval = *v_iter.next().unwrap();
        while let Some(next_num) = v_iter.next() {
            let nextop = ops.next().unwrap();
            match nextop {
                '+' => retval += next_num,
                '*' => retval *= next_num,
                _ => unreachable!("Unexpected operator: {}", nextop),
            }
        }

        if retval == target {
            return true;
        }
    }
    return false;
}

fn can_reach_target_two(target: i64, vals: &Vec<i64>) -> bool {
    assert!(vals.len() > 2);
    let num_spaces = vals.len() - 1;
    let combinations = generate_combinations_two(num_spaces);
    for comb in combinations {
        let mut ops = comb.chars();
        let mut v_iter = vals.iter();
        let mut retval = *v_iter.next().unwrap();
        while let Some(next_num) = v_iter.next() {
            let nextop = ops.next().unwrap();
            match nextop {
                '+' => retval += next_num,
                '*' => retval *= next_num,
                '|' => retval = num_concat(retval, *next_num),
                _ => unreachable!("Unexpected operator: {}", nextop),
            }
        }
        if retval == target {
            return true;
        }
    }
    return false;
}

fn main() -> io::Result<()> {
    let inputs = read_input("seven.in")?;
    let ans_one = inputs
        .iter()
        .map(|(target, vals)| {
            let mut ret: i64 = 0;
            if can_reach_target_one(*target, &vals) {
                ret = *target;
            }
            ret
        })
        .fold(0, |acc, x| acc + x);

    let ans_two = inputs
        .iter()
        .map(|(target, vals)| {
            let mut ret: i64 = 0;
            if can_reach_target_two(*target, &vals) {
                ret = *target;
            }
            ret
        })
        .fold(0, |acc, x| acc + x);

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
