use std::collections::HashMap;
// https://adventofcode.com/2024/day/14
use std::fs::File;
use std::io::{self, BufRead, BufReader};
struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

fn read_input(filepath: &str) -> io::Result<Vec<Robot>> {
    let reader = BufReader::new(File::open(filepath).unwrap());
    let mut ret: Vec<Robot> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let stuff: String = line
            .chars()
            .filter(|&c| c.is_numeric() || c == '-' || c == ' ' || c == ',')
            .collect();
        let two_halves: Vec<&str> = stuff.split_whitespace().collect();
        assert!(two_halves.len() == 2);
        let mut pos_str = two_halves[0].split(',');
        let pos_x = pos_str.next().unwrap().parse::<i64>().unwrap();
        let pos_y = pos_str.next().unwrap().parse::<i64>().unwrap();
        let mut vel_str = two_halves[1].split(',');
        let vel_x = vel_str.next().unwrap().parse::<i64>().unwrap();
        let vel_y = vel_str.next().unwrap().parse::<i64>().unwrap();
        ret.push(Robot {
            pos: (pos_x, pos_y),
            velocity: (vel_x, vel_y),
        })
    }
    Ok(ret)
}

fn move_robots(robots: &mut Vec<Robot>, width: i64, height: i64, seconds: i32) {
    for _ in 0..seconds {
        robots.iter_mut().for_each(|r| {
            r.pos.0 = (r.pos.0 + r.velocity.0).rem_euclid(width);
            r.pos.1 = (r.pos.1 + r.velocity.1).rem_euclid(height);
        })
    }
}

fn safety_score(robots: &Vec<Robot>, width: i64, height: i64) -> i32 {
    let half_x = width / 2;
    let half_y = height / 2;
    let mut saf1 = 0;
    let mut saf2 = 0;
    let mut saf3 = 0;
    let mut saf4 = 0;
    for robot in robots {
        if robot.pos.0 < half_x && robot.pos.1 < half_y {
            saf1 += 1;
        } else if robot.pos.0 > half_x && robot.pos.1 < half_y {
            saf2 += 1;
        } else if robot.pos.0 < half_x && robot.pos.1 > half_y {
            saf3 += 1;
        } else if robot.pos.0 > half_x && robot.pos.1 > half_y {
            saf4 += 1;
        }
    }
    return saf1 * saf2 * saf3 * saf4;
}

fn all_safety_scores(robots: &mut Vec<Robot>, width: i64, height: i64) -> HashMap<i32, i32> {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    for i in 1..(103 * 101) {
        move_robots(robots, width, height, 1);
        let score = safety_score(robots, width, height);
        scores.insert(i, score);
    }
    return scores;
}

fn main() -> io::Result<()> {
    // hardcoding these because they're just magic numbers without an actual array
    let width = 101;
    let height = 103;

    let mut robots = read_input("fourteen.in")?;
    move_robots(&mut robots, width, height, 100);
    let ans_one = safety_score(&robots, width, height);
    println!("Ans Part One: {ans_one}");

    let mut also_robots = read_input("fourteen.in")?;
    let scores = all_safety_scores(&mut also_robots, width, height);

    let ans_two = scores.iter().min_by_key(|entry| entry.1).unwrap();
    println!("Ans Part Two: {}", ans_two.0);

    Ok(())
}
