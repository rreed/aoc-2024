// https://adventofcode.com/2024/day/8
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<HashMap<char, HashSet<(usize, usize)>>> {
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut positions: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    for (r_idx, line) in reader.lines().enumerate() {
        let line = line?;
        for (c_idx, c) in line.chars().enumerate() {
            if c != '.' {
                positions
                    .entry(c)
                    .or_insert_with(HashSet::new)
                    .insert((r_idx, c_idx));
            }
        }
    }

    Ok(positions)
}

fn is_in_bounds(anti_pos: (i32, i32), max_height: i32, max_width: i32) -> bool {
    return anti_pos.0 >= 0 && anti_pos.0 < max_width && anti_pos.1 >= 0 && anti_pos.1 < max_height;
}

// given a single antenna frequency (i.e., a char) find all of its antinode positions
fn find_antinodes(
    antennas: &HashSet<(usize, usize)>,
    max_height: i32,
    max_width: i32,
) -> HashSet<(i32, i32)> {
    let mut ret: HashSet<(i32, i32)> = HashSet::new();
    for (i, a1) in antennas.iter().enumerate() {
        for a2 in antennas.iter().skip(i + 1) {
            // see the typst writeup for a long explanation of this
            let a1_row = a1.0 as i32;
            let a1_col = a1.1 as i32;
            let a2_row = a2.0 as i32;
            let a2_col = a2.1 as i32;
            let dr = a1_row - a2_row;
            let dc = a1_col - a2_col;
            let anti1 = (a1_row + dr, a1_col + dc);
            let anti2 = (a2_row - dr, a2_col - dc);
            if is_in_bounds(anti1, max_height, max_width) {
                ret.insert(anti1);
            }
            if is_in_bounds(anti2, max_height, max_width) {
                ret.insert(anti2);
            }
        }
    }
    return ret;
}

fn find_linear_antinodes(
    antennas: &HashSet<(usize, usize)>,
    max_height: i32,
    max_width: i32,
) -> HashSet<(i32, i32)> {
    let mut ret: HashSet<(i32, i32)> = HashSet::new();
    for (i, a1) in antennas.iter().enumerate() {
        for a2 in antennas.iter().skip(i + 1) {
            let a1_row = a1.0 as i32;
            let a1_col = a1.1 as i32;
            let a2_row = a2.0 as i32;
            let a2_col = a2.1 as i32;
            let dr = a1_row - a2_row;
            let dc = a1_col - a2_col;
            let mut anti1 = (a1_row + dr, a1_col + dc);
            let mut anti2 = (a2_row - dr, a2_col - dc);
            // insert the initial antennae
            ret.insert((a1_row, a1_col));
            ret.insert((a2_row, a2_col));
            while is_in_bounds(anti1, max_height, max_width) {
                ret.insert(anti1);
                anti1 = (anti1.0 + dr, anti1.1 + dc);
            }
            while is_in_bounds(anti2, max_height, max_width) {
                ret.insert(anti2);
                anti2 = (anti2.0 - dr, anti2.1 - dc);
            }
        }
    }
    return ret;
}

fn main() -> io::Result<()> {
    let antenna_positions = read_input("eight.in")?;
    let max_height = 50;
    let max_width = 50;

    let deduped_antinodes: HashSet<(i32, i32)> = antenna_positions
        .iter()
        .map(|(_, antennas)| find_antinodes(&antennas, max_height, max_width))
        .into_iter()
        .flat_map(|set| set.into_iter())
        .collect();
    let ans_one = deduped_antinodes.len();

    let deduped_linear_antinodes: HashSet<(i32, i32)> = antenna_positions
        .iter()
        .map(|(_, antennas)| find_linear_antinodes(&antennas, max_height, max_width))
        .into_iter()
        .flat_map(|set| set.into_iter())
        .collect();
    let ans_two = deduped_linear_antinodes.len();

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
