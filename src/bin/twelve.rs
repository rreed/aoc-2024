// https://adventofcode.com/2024/day/12
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
// get in loser, we're doing BFS again

// I got tired of typing `as usize` + `as isize` until my fingers fell off
// so let's try just using some sentinel values in the form of `.`
// and turning this into a vaguely-cursed 1d array
fn read_input(filepath: &str) -> io::Result<(Vec<char>, usize)> {
    let reader = BufReader::new(File::open(filepath).unwrap());
    let mut plots: Vec<char> = Vec::new();
    let mut width = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if width == 0 {
            width = line.len() + 2;
            plots.extend(iter::repeat_n('.', width));
        }
        plots.push('.');
        for c in line.chars() {
            plots.push(c);
        }
        plots.push('.');
    }
    plots.extend(iter::repeat_n('.', width));
    Ok((plots, width))
}

fn main() -> io::Result<()> {
    let (garden_map, width) = read_input("twelve.in")?;

    let mut visited = vec![false; garden_map.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut ans_one = 0;
    let mut ans_two = 0;

    for (index, &c) in garden_map.iter().enumerate() {
        if c == '.' || visited[index] {
            continue;
        }

        queue.push_back(index);
        visited[index] = true;

        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;

        while let Some(cur_pos) = queue.pop_front() {
            area += 1;
            let neighbors = [cur_pos - width, cur_pos + 1, cur_pos + width, cur_pos - 1];
            for (idx, neighbor) in neighbors.into_iter().enumerate() {
                if garden_map[neighbor] == c {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                } else {
                    perimeter += 1;
                }

                // we don't actually need to count "runs of fence":
                // the number of corners in the fenced zone *is* the number of sides
                // so let's find all of the corners
                let next = neighbors[(idx + 1) % neighbors.len()];
                if garden_map[neighbor] != c && garden_map[next] != c {
                    sides += 1;
                } else if garden_map[neighbor] == c
                    && garden_map[next] == c
                    && garden_map[neighbor + next - cur_pos] != c
                {
                    sides += 1;
                }
            }
        }
        ans_one += area * perimeter;
        ans_two += area * sides;
    }

    println!("Ans Part One: {ans_one}");
    println!("Ans Part Two: {ans_two}");

    Ok(())
}
