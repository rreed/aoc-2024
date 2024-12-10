// https://adventofcode.com/2024/day/10
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn read_input(filepath: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(&filepath)?;
    let reader = io::BufReader::new(file);
    let mut ret: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut inner_vec: Vec<i32> = Vec::new();
        for c in line.chars() {
            inner_vec.push(c.to_digit(10).unwrap() as i32);
        }
        ret.push(inner_vec);
    }

    Ok(ret)
}

fn find_trailheads(map: &Vec<Vec<i32>>) -> HashSet<(usize, usize)> {
    let mut ret: HashSet<(usize, usize)> = HashSet::new();
    for (r_idx, row) in map.iter().enumerate() {
        for (c_idx, col) in row.iter().enumerate() {
            if *col == 0 {
                ret.insert((r_idx, c_idx));
            }
        }
    }
    ret
}

fn bfs(map: &Vec<Vec<i32>>, trailhead: (usize, usize)) -> (i32, i32) {
    let directions: [(i32, i32); 4] = [
        (-1, 0), // N
        (0, 1),  // E
        (1, 0),  // S
        (0, -1), // W
    ];

    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    deque.push_back(trailhead);
    let mut num_paths = 0;
    let mut visited_peaks: HashSet<(usize, usize)> = HashSet::new();
    while let Some((row, col)) = deque.pop_front() {
        let cur_height = map[row][col];
        if cur_height == 9 {
            num_paths += 1;
            visited_peaks.insert((row, col));
            continue;
        }
        for &(dr, dc) in &directions {
            let next_row = row as isize + dr as isize;
            let next_col = col as isize + dc as isize;

            if next_row < 0
                || next_col < 0
                || next_row >= map.len() as isize
                || next_col >= map[0].len() as isize
            {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            if map[next_row][next_col] == cur_height + 1 {
                deque.push_back((next_row, next_col));
            }
        }
    }
    (visited_peaks.len() as i32, num_paths)
}

fn main() -> io::Result<()> {
    let trail_map = read_input("ten.in")?;
    let trailheads = find_trailheads(&trail_map);
    let ans_one = trailheads
        .iter()
        .map(|idx| bfs(&trail_map, *idx).0)
        .fold(0, |acc, x| acc + x);

    let ans_two = trailheads
        .iter()
        .map(|idx| bfs(&trail_map, *idx).1)
        .fold(0, |acc, x| acc + x);

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
