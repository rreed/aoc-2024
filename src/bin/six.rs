// https://adventofcode.com/2024/day/6
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        lines.push(chars);
    }

    Ok(lines)
}

fn find_guard(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_idx, row) in map.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&ch| ch == '^') {
            return Some((row_idx, col_idx));
        }
    }
    None
}

fn patrol(map: &Vec<Vec<char>>, init_row: usize, init_col: usize) -> (Vec<(usize, usize)>, bool) {
    let directions: [(i32, i32); 4] = [
        (-1, 0), // N
        (0, 1),  // E
        (1, 0),  // S
        (0, -1), // W
    ];
    // start facing north, always rotate 90° right
    let mut dir_idx = 0;
    let mut facing = directions[dir_idx];
    let mut row = init_row;
    let mut col = init_col;
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut revisited = 0;
    // let max_revisitable = map.len() * map[0].len();
    let max_revisitable = map.len() * 5;
    // include the starting position~
    visited.push((row, col));
    while let Some(row_vec) = map.get(((row as i32) + facing.0) as usize) {
        if let Some(next_step) = row_vec.get(((col as i32) + facing.1) as usize) {
            if *next_step == '#' {
                // if it's an obstacle, turn right
                dir_idx = (dir_idx + 1) % 4;
                facing = directions[dir_idx];
            } else {
                // otherwise move in that direction
                row = ((row as i32) + facing.0) as usize;
                col = ((col as i32) + facing.1) as usize;
                // and mark it visited
                let maybe_visited = (row, col);
                if !visited.contains(&maybe_visited) {
                    visited.push(maybe_visited);
                } else {
                    revisited += 1;
                    if revisited >= max_revisitable {
                        return (visited, true);
                    }
                }
            }
        } else {
            break;
        }
    }
    return (visited, false);
}

fn loop_possible(
    map: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    guard_row: usize,
    guard_col: usize,
) -> bool {
    // don't delete the guard...
    if row == guard_row && col == guard_col {
        return false;
    }
    // don't waste our time loop-checking something that was already an obstacle, either
    if map[row][col] == '#' {
        return false;
    }
    // potentially loopable
    let mut cpy = map.clone(); // spendy...
    cpy[row][col] = '#';
    let (_, looped) = patrol(&cpy, guard_row, guard_col);
    return looped;
}

fn main() -> io::Result<()> {
    let map = read_input("six.in")?;
    let (guard_row, guard_col) = find_guard(&map).unwrap();

    let (visited_p1, _) = patrol(&map, guard_row, guard_col);
    let ans_one = visited_p1.len();

    let map_height = map.len();
    let map_width = map[0].len();
    let ans_two = (0..map_height)
        .map(|r| {
            (0..map_width)
                .filter(|&c| loop_possible(&map, r, c, guard_row, guard_col))
                .count()
        })
        .sum::<usize>();

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
