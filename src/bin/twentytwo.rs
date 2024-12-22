use std::{
    collections::{HashMap, HashSet},
    fs, io,
    ops::BitXor,
};

fn read_input(filepath: &str) -> io::Result<Vec<isize>> {
    let input = fs::read_to_string(&filepath)?;
    let secrets: Vec<isize> = input
        .split('\n')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    return Ok(secrets);
}

const PRUNE_BY: isize = 16777216;
fn next_secret(secret: isize) -> isize {
    let step_one = (secret * 64).bitxor(secret).rem_euclid(PRUNE_BY);
    let step_two = (step_one / 32).bitxor(step_one).rem_euclid(PRUNE_BY);
    (step_two * 2048).bitxor(step_two).rem_euclid(PRUNE_BY)
}

fn first_n_secrets(secret: isize, n: isize) -> Vec<isize> {
    let mut ret: Vec<isize> = Vec::new();
    let mut next = secret;
    for _ in 0..n {
        next = next_secret(next);
        ret.push(next);
    }
    ret
}

fn ones_digit(n: isize) -> isize {
    n.rem_euclid(10)
}

fn main() {
    let secrets = read_input("twentytwo.in").unwrap();
    const SECRET_ITERATIONS: isize = 2000;
    let ans_one = secrets
        .iter()
        .map(|secret| {
            first_n_secrets(*secret, SECRET_ITERATIONS)
                .last()
                .copied()
                .unwrap()
        })
        .fold(0, |acc, x| acc + x);
    println!("Ans Part One: {ans_one}");

    let mut sale_price_windows = HashMap::new();
    for secret in secrets {
        let mut seen = HashSet::new();
        let mut prices = Vec::new();
        let mut deltas = Vec::new();
        prices.push(ones_digit(secret));
        let next_n_secrets = first_n_secrets(secret, SECRET_ITERATIONS);
        for (idx, sec) in next_n_secrets.iter().enumerate() {
            prices.push(ones_digit(*sec));
            deltas.push(ones_digit(*sec) - prices[idx]);
        }
        for (i, window) in deltas.windows(4).enumerate() {
            if !seen.contains(&window) {
                seen.insert(window);
                *sale_price_windows
                    .entry([window[0], window[1], window[2], window[3]])
                    .or_insert(0) += prices[i + 4]
            }
        }
    }
    let ans_two = *sale_price_windows.values().max().unwrap();
    println!("Ans Part Two: {ans_two}");
}
