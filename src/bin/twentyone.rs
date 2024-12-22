use memoize::memoize;
use std::{fs, io};

fn read_input(filepath: &str) -> io::Result<Vec<String>> {
    let input = fs::read_to_string(&filepath)?;
    let sequences: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    return Ok(sequences);
}

fn numpad(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn arrowpad(key: char) -> (i32, i32) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    }
}

#[memoize]
fn prepare_sequence(r: i32, c: i32, steps: usize, h_first: bool) -> usize {
    let (r_abs, c_abs) = (r.unsigned_abs() as usize, c.unsigned_abs() as usize);
    let mut chunk = vec![if r > 0 { '^' } else { 'v' }; r_abs];
    chunk.extend(vec![if c > 0 { '<' } else { '>' }; c_abs]);

    if h_first {
        chunk.reverse();
    }

    chunk.push('A');

    if steps == 0 {
        chunk.len()
    } else {
        let mut loc = arrowpad('A');

        chunk
            .into_iter()
            .map(|c| {
                let n = arrowpad(c);
                let p = loc;
                loc = n;
                let d = (p.0 - n.0, p.1 - n.1);
                if d.0 == 0 || d.1 == 0 {
                    prepare_sequence(d.0, d.1, steps - 1, false)
                } else if n == (1, 0) && p.0 == 0 {
                    prepare_sequence(d.0, d.1, steps - 1, false)
                } else if p == (1, 0) && n.0 == 0 {
                    prepare_sequence(d.0, d.1, steps - 1, true)
                } else {
                    std::cmp::min(
                        prepare_sequence(d.0, d.1, steps - 1, false),
                        prepare_sequence(d.0, d.1, steps - 1, true),
                    )
                }
            })
            .sum()
    }
}

fn beep_boop(sequence: &str, num_robots: usize) -> usize {
    let mut loc = numpad('A');

    sequence
        .chars()
        .map(|c| {
            let n = numpad(c);
            let p = loc;
            let d = (loc.0 - n.0, loc.1 - n.1);
            loc = n;
            if p.0 == 3 && n.1 == 0 {
                // up~
                prepare_sequence(d.0, d.1, num_robots, false)
            } else if p.1 == 0 && n.0 == 3 {
                // right~
                prepare_sequence(d.0, d.1, num_robots, true)
            } else {
                std::cmp::min(
                    prepare_sequence(d.0, d.1, num_robots, true),
                    prepare_sequence(d.0, d.1, num_robots, false),
                )
            }
        })
        .sum::<usize>()
        * sequence[0..3].parse::<usize>().unwrap()
}

fn main() {
    let sequences = read_input("twentyone.in").unwrap();

    memoized_flush_prepare_sequence();
    let ans_one: usize = sequences.iter().map(|s| beep_boop(s, 2)).sum();
    println!("Ans Part One: {ans_one}");

    let ans_two: usize = sequences.iter().map(|s| beep_boop(s, 25)).sum();
    println!("Ans Part Two: {ans_two}");
}
