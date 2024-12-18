use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input(filepath: &str) -> io::Result<Vec<(usize, usize)>> {
    let reader = BufReader::new(File::open(filepath).unwrap());

    let ret: Vec<(usize, usize)> = reader
        .lines()
        .map(|line| {
            let s = line.unwrap();
            let mut parts = s.split(',');
            let first = parts.next().unwrap().parse::<usize>().unwrap();
            let second = parts.next().unwrap().parse::<usize>().unwrap();
            (first, second)
        })
        .collect();

    Ok(ret)
}

fn dijkstra(grid: &HashSet<(i32, i32)>, start: (i32, i32), goal: (i32, i32)) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    heap.push(Reverse((0, start))); // (distance, position)
    distances.insert(start, 0);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Reverse((cost, (x, y)))) = heap.pop() {
        if (x, y) == goal {
            return Some(cost);
        }

        if let Some(&current_distance) = distances.get(&(x, y)) {
            if current_distance < cost {
                continue;
            }
        }

        for &(dx, dy) in &directions {
            let neighbor = (x + dx, y + dy);
            if grid.contains(&neighbor) {
                let new_cost = cost + 1;

                if new_cost < *distances.get(&neighbor).unwrap_or(&i32::MAX) {
                    distances.insert(neighbor, new_cost);
                    heap.push(Reverse((new_cost, neighbor)));
                }
            }
        }
    }

    None
}

fn map_corruption(map: &Vec<(usize, usize)>, to_take: usize) -> Vec<Vec<char>> {
    // the coordinate grid is from zero to seventy INCLUSIVE
    let mut maze: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    map.iter()
        .take(to_take)
        .for_each(|(r, c)| maze[*r][*c] = '#');

    return maze;
}

fn free_spaces(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut indices = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '.' {
                indices.insert((row_idx as i32, col_idx as i32));
            }
        }
    }

    indices
}

fn first_unsolvable_index(corrupt_positions: &Vec<(usize, usize)>) -> usize {
    // we know that everything less than this is solvable from part one
    for i in 1025..corrupt_positions.len() {
        let corrupted_map = map_corruption(&corrupt_positions, i);
        let free_spaces = free_spaces(&corrupted_map);
        let maybe_ans = dijkstra(&free_spaces, (0, 0), (70, 70));
        if maybe_ans.is_none() {
            // `take`ing n things means the last relevant one was at i-1
            return i - 1;
        }
    }
    unreachable!()
}

fn main() {
    let corrupt_positions = read_input("eighteen.in").unwrap();

    let corrupted_map = map_corruption(&corrupt_positions, 1024);
    let free_spaces = free_spaces(&corrupted_map);
    let ans_one = dijkstra(&free_spaces, (0, 0), (70, 70));
    assert!(ans_one.is_some());
    println!("Ans Part One: {}", ans_one.unwrap());

    let ans_two_idx = first_unsolvable_index(&corrupt_positions);
    let ans_two = corrupt_positions[ans_two_idx];
    println!("Ans Part Two: {:?}", ans_two);
}
