use std::{collections::VecDeque, fs};

enum Tile {
    Space([Option<u16>; 2]),
    Wall,
}

fn calculate_distances(map: &mut Vec<Tile>, width: usize, index: usize, start: usize, end: usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if let Tile::Space(dists) = &mut map[pos] {
            if dists[index].is_none() {
                dists[index] = Some(dist);
                for new_pos in [pos + 1, pos - 1, pos + width, pos - width] {
                    queue.push_back((new_pos, dist + 1))
                }
            }
        }

        if pos == end {
            break;
        }
    }
}

// we're just going to do a 1d array here like back in problem uhhhh whatever
fn read_input(filepath: &str) -> (Vec<Tile>, usize, u16) {
    let input = fs::read_to_string(&filepath).unwrap();
    let width = input.chars().position(|c| c == '\n').unwrap();

    let mut start = 0;
    let mut end = 0;
    let mut map = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(n, c)| match c {
            '.' => Tile::Space([None; 2]),
            '#' => Tile::Wall,
            'S' => {
                start = n;
                Tile::Space([None; 2])
            }
            'E' => {
                end = n;
                Tile::Space([None; 2])
            }
            _ => unreachable!(),
        })
        .collect();

    calculate_distances(&mut map, width, 0, start, end);
    calculate_distances(&mut map, width, 1, end, start);

    let best_distance = match map[end] {
        Tile::Space([Some(dist), _]) => dist,
        _ => unreachable!(),
    };

    (map, width, best_distance)
}

fn main() {
    let (ref tiles, width, min_dist) = read_input("twenty.in");
    let height = tiles.len() / width;

    const TARGET_DIST: u16 = 100;
    const CHEAT_LEN: usize = 20;
    let ans_one: usize = (1..height - 1)
        .flat_map(|y| {
            (1..width - 1).filter_map(move |x| {
                let pos = y * width + x;
                if let Tile::Wall = tiles[pos] {
                    let surrounding = [
                        &tiles[pos - 1],
                        &tiles[pos + 1],
                        &tiles[pos - width],
                        &tiles[pos + width],
                    ];

                    Some(
                        surrounding
                            .iter()
                            .flat_map(|&a| {
                                surrounding.iter().filter(move |&&b| {
                                    if let (
                                        Tile::Space([Some(dist_from_start), _]),
                                        Tile::Space([_, Some(dist_from_end)]),
                                    ) = (a, b)
                                    {
                                        dist_from_start + dist_from_end + 2
                                            <= min_dist - TARGET_DIST
                                    } else {
                                        false
                                    }
                                })
                            })
                            .count(),
                    )
                } else {
                    None
                }
            })
        })
        .sum();
    println!("Ans Part One: {ans_one}");

    let ans_two: usize = (1..height - 1)
        .flat_map(|y| {
            (1..width - 1).filter_map(move |x| {
                let pos = y * width + x;
                if let Tile::Space([Some(dist_from_start), _]) = tiles[pos] {
                    Some(
                        ((std::cmp::max(x, CHEAT_LEN + 1) - CHEAT_LEN)
                            ..=(std::cmp::min(x + CHEAT_LEN, width - 2)))
                            .flat_map(|x2| {
                                let x_dist = x.abs_diff(x2);
                                let max_y_dist = CHEAT_LEN - x_dist;
                                ((std::cmp::max(y, max_y_dist + 1) - max_y_dist)
                                    ..=(std::cmp::min(y + max_y_dist, height - 2)))
                                    .filter(move |&y2| {
                                        let pos2 = y2 * width + x2;
                                        if let Tile::Space([_, Some(dist_from_end)]) = tiles[pos2] {
                                            dist_from_start
                                                + dist_from_end
                                                + x_dist as u16
                                                + y.abs_diff(y2) as u16
                                                <= min_dist - TARGET_DIST
                                        } else {
                                            false
                                        }
                                    })
                            })
                            .count(),
                    )
                } else {
                    None
                }
            })
        })
        .sum();
    println!("Ans Part Two: {ans_two}");
}
