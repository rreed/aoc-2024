use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io,
    io::BufRead,
    io::BufReader,
};

struct Maze {
    start: (i32, i32),
    end: (i32, i32),
    open_spaces: HashSet<(i32, i32)>,
}

fn read_input(filepath: &str) -> io::Result<Maze> {
    let reader = BufReader::new(File::open(filepath).unwrap());

    let maze: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut start = (-1, -1);
    let mut end = (-1, -1);
    let mut open_spaces: HashSet<(i32, i32)> = HashSet::new();

    for (r, row) in maze.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            match cell {
                'E' => {
                    end = (r as i32, c as i32);
                    open_spaces.insert((r as i32, c as i32));
                }
                'S' => {
                    start = (r as i32, c as i32);
                }
                '.' => {
                    open_spaces.insert((r as i32, c as i32));
                }
                _ => {}
            }
        }
    }
    Ok(Maze {
        start: start,
        end: end,
        open_spaces: open_spaces,
    })
}

fn dijkstra(
    start: (i32, i32),
    free_spaces: &HashSet<(i32, i32)>,
) -> HashMap<((i32, i32), char), i32> {
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    let rot = ['>', 'v', '<', '^'];

    let mut to_visit = BinaryHeap::new();
    let mut visited: HashMap<((i32, i32), char), i32> = HashMap::new();
    visited.insert((start, '>'), 0);

    to_visit.push((0, '>', start));

    while let Some((score, cd, (cx, cy))) = to_visit.pop() {
        let score = -score; // imogen min-heap send tweet

        if visited.get(&((cx, cy), cd)).map_or(false, |&v| v < score) {
            continue;
        }

        let (dx, dy) = directions[&cd];

        let np = (cx + dx, cy + dy);
        if free_spaces.contains(&np) && visited.get(&(np, cd)).map_or(true, |&v| v > score + 1) {
            visited.insert((np, cd), score + 1);
            to_visit.push((-(score + 1), cd, np));
        }

        for dr in [-1, 1] {
            let nd = rot[(((rot.iter().position(|&r| r == cd).unwrap() as i32) + dr).rem_euclid(4))
                as usize];
            if visited
                .get(&((cx, cy), nd))
                .map_or(true, |&v| v > score + 1000)
            {
                visited.insert(((cx, cy), nd), score + 1000);
                to_visit.push((-(score + 1000), nd, (cx, cy)));
            }
        }
    }

    visited
}

fn retrace(
    visited: &HashMap<((i32, i32), char), i32>,
    target_state: ((i32, i32), char),
) -> HashSet<(i32, i32)> {
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    let facings = ['>', 'v', '<', '^'];

    let mut to_visit = vec![target_state];
    let mut seen = HashSet::new();

    while let Some((cp, cd)) = to_visit.pop() {
        seen.insert(cp);

        let (dx, dy) = directions[&cd];
        let np = (cp.0 - dx, cp.1 - dy);

        if visited
            .get(&(np, cd))
            .map_or(false, |&v| v + 1 == visited[&(cp, cd)])
        {
            to_visit.push((np, cd));
        }

        let nd1 = facings
            [((facings.iter().position(|&r| r == cd).unwrap() as i32 + 1).rem_euclid(4)) as usize];
        let nd2 = facings
            [((facings.iter().position(|&r| r == cd).unwrap() as i32 - 1).rem_euclid(4)) as usize];

        if visited
            .get(&(cp, nd1))
            .map_or(false, |&v| v + 1000 == visited[&(cp, cd)])
        {
            to_visit.push((cp, nd1));
        }
        if visited
            .get(&(cp, nd2))
            .map_or(false, |&v| v + 1000 == visited[&(cp, cd)])
        {
            to_visit.push((cp, nd2));
        }
    }

    seen
}

fn main() {
    let maze = read_input("sixteen.in").unwrap();

    let visited = dijkstra(maze.start, &maze.open_spaces);
    let ans_one = visited
        .iter()
        .filter(|&(&(pos, _), _)| pos == maze.end)
        .map(|(_, &score)| score)
        .min()
        .unwrap();
    println!("Ans Part One: {ans_one}");

    let target_state = visited
        .iter()
        .find(|&(&(pos, _), &score)| pos == maze.end && score == ans_one)
        .map(|(&(pos, dir), _)| (pos, dir))
        .unwrap();
    let ans_two = retrace(&visited, target_state).len();
    println!("Ans Part Two: {ans_two}");
}
