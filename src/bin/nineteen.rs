use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

fn count_possible(
    towel: &str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&result) = cache.get(towel) {
        return result;
    }

    let mut combs = 0;
    for &p in patterns {
        if towel == p {
            combs += 1;
        }
        if towel.starts_with(p) {
            let new_towel = towel.replacen(p, "", 1);
            combs += count_possible(&new_towel, patterns, cache);
        }
    }

    cache.insert(towel.to_string(), combs);
    combs
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("nineteen.in")?;
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels: HashSet<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();
    let mut memo: HashMap<String, usize> = HashMap::new();

    let ans_one = designs
        .iter()
        .filter(|design| count_possible(design, &towels, &mut memo) != 0)
        .count();
    println!("Ans Part One: {ans_one}");

    let ans_two = designs
        .iter()
        .map(|design| count_possible(design, &towels, &mut memo))
        .fold(0, |acc, x| acc + x);
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
