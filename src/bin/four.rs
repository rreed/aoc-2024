// https://adventofcode.com/2024/day/4
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

fn find_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let directions: [(i32, i32); 8] = [
        (-1, 0),  // N
        (-1, 1),  // NE
        (0, 1),   // E
        (1, 1),   // SE
        (1, 0),   // S
        (1, -1),  // SW
        (0, -1),  // W
        (-1, -1), // NW
    ];

    grid.iter()
        .enumerate()
        .flat_map(|(row, v)| {
            v.iter().enumerate().filter_map(move |(col, &c)| {
                if c == 'X' {
                    Some(
                        directions
                            .iter()
                            .filter_map(|&(dr, dc)| {
                                let mod_row = row as i32 + dr * 3;
                                let mod_col = col as i32 + dc * 3;
                                let in_bounds_row = mod_row >= 0 && mod_row < grid.len() as i32;
                                let in_bounds_col = mod_col >= 0 && mod_col < v.len() as i32;

                                if in_bounds_row && in_bounds_col {
                                    let c1 = grid[(row as i32 + dr * 1) as usize]
                                        [(col as i32 + dc * 1) as usize];
                                    let c2 = grid[(row as i32 + dr * 2) as usize]
                                        [(col as i32 + dc * 2) as usize];
                                    let c3 = grid[mod_row as usize][mod_col as usize];

                                    if c1 == 'M' && c2 == 'A' && c3 == 'S' {
                                        return Some(1);
                                    }
                                }
                                None
                            })
                            .sum::<i32>(),
                    )
                } else {
                    None
                }
            })
        })
        .sum()
}

fn find_x_mas(grid: &Vec<Vec<char>>) -> i32 {
    grid.iter()
        .enumerate()
        .flat_map(|(row, v)| {
            v.iter().enumerate().filter_map(move |(col, &c)| {
                if c == 'A' && row >= 1 && row < grid.len() - 1 && col >= 1 && col < v.len() - 1 {
                    let patterns = [
                        (('M', 'S'), ('S', 'M')),
                        (('M', 'M'), ('S', 'S')),
                        (('S', 'M'), ('M', 'S')),
                        (('S', 'S'), ('M', 'M')),
                    ];
                    for &((tl, tr), (br, bl)) in &patterns {
                        if grid[row - 1][col - 1] == tl
                            && grid[row - 1][col + 1] == tr
                            && grid[row + 1][col + 1] == br
                            && grid[row + 1][col - 1] == bl
                        {
                            return Some(1);
                        }
                    }
                }
                None
            })
        })
        .sum()
}

fn main() -> io::Result<()> {
    let grid = read_input("four.in")?;
    let ans_one = find_xmas(&grid);
    let ans_two = find_x_mas(&grid);

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
