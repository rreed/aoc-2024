// https://adventofcode.com/2024/day/5
use std::fs;
use std::io;

fn read_input(filepath: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let input = fs::read_to_string(&filepath)?;

    let mut sections = input.splitn(2, "\n\n");

    let ordering_rules: Vec<String> = sections
        .next()
        .unwrap_or("Failed to find ordering rules")
        .lines()
        .map(|line| line.to_string())
        .collect();
    let page_orderings: Vec<String> = sections
        .next()
        .unwrap_or("Failed to find page orderings")
        .lines()
        .map(|line| line.to_string())
        .collect();

    Ok((ordering_rules, page_orderings))
}

fn a_is_before_b(list: &Vec<i32>, a: i32, b: i32) -> bool {
    let idx_a = list.iter().position(|i| *i == a);
    let idx_b = list.iter().position(|i| *i == b);
    if idx_a.is_some() && idx_b.is_some() {
        return Some(idx_a) < Some(idx_b);
    }
    return true; // ignore any time that both numbers aren't present
}

fn is_correctly_ordered(list: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    rules.iter().all(|(a, b)| a_is_before_b(list, *a, *b))
}

fn get_middle_element(list: &Vec<i32>) -> i32 {
    let middle_idx = list.len() / 2;
    return list[middle_idx];
}

fn fix_ordering(list: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut cpy = list.clone();
    rules.iter().for_each(|(a, b)| {
        let idx_a = cpy.iter().position(|i| *i == *a);
        let idx_b = cpy.iter().position(|i| *i == *b);
        // if both elements aren't present, do no rearranging
        if let (Some(idx_a), Some(idx_b)) = (idx_a, idx_b) {
            if !a_is_before_b(&cpy, *a, *b) {
                let element = cpy.remove(idx_b);
                cpy.insert(idx_a, element);
                cpy = fix_ordering(&cpy, &rules);
            }
        }
    });
    return cpy;
}

fn main() -> io::Result<()> {
    let (ordering_rules, page_orderings) = read_input("five.in")?;
    let parsed_rules: Vec<(i32, i32)> = ordering_rules
        .iter()
        .map(|s| {
            let mut parts = s.split('|');
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .collect();
    let parsed_orderings: Vec<Vec<i32>> = page_orderings
        .iter()
        .map(|s| s.split(",").map(|i| i.parse::<i32>().unwrap()).collect())
        .collect();

    let ans_one = parsed_orderings
        .iter()
        .filter(|v| is_correctly_ordered(v, &parsed_rules))
        .fold(0, |acc, l| acc + get_middle_element(l));

    let ans_two = parsed_orderings
        .iter()
        .filter(|v| !is_correctly_ordered(v, &parsed_rules))
        .fold(0, |acc, l| {
            acc + get_middle_element(&fix_ordering(l, &parsed_rules))
        });

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
