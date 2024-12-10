// https://adventofcode.com/2024/day/9
use std::fs;
use std::io;

fn read_input(filepath: &str) -> io::Result<String> {
    return Ok(fs::read_to_string(filepath)?);
}

fn map_drive(drive: String) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();
    let mut is_block = true; // switch between blocks and free spaces
    let mut id = 0;
    for c in drive.chars() {
        let mut c_digit = c.to_digit(10).unwrap();
        if is_block {
            while c_digit > 0 {
                ret.push(id);
                c_digit -= 1;
            }
            id += 1;
            is_block = false;
        } else {
            // free space
            while c_digit > 0 {
                ret.push(-1);
                c_digit -= 1;
            }
            is_block = true;
        }
    }
    return ret;
}

fn reorder_drive_one(drive: &mut Vec<i64>) {
    for i in (0..drive.len()).rev() {
        if drive[i] != -1 {
            let first_neg_idx = drive.iter().position(|&i| i == -1).unwrap();
            if i < first_neg_idx {
                break;
            }
            drive.swap(i, first_neg_idx);
        }
    }
}

fn reorder_drive_two(drive: &mut Vec<i64>) {
    let mut r_pos = drive.len() - 1;

    while r_pos > 0 {
        if drive[r_pos] != -1 {
            let mut blk_size = 1;
            let cur_block_id = drive[r_pos];

            while blk_size <= r_pos && drive[r_pos - blk_size] == cur_block_id {
                blk_size += 1;
            }

            if blk_size >= r_pos {
                break;
            }

            // Now, find blk_size -1s in a row
            let mut found = false; // Reset the found flag on each iteration
            let mut l_pos = 0;
            while !found && l_pos < drive.len() - blk_size {
                if drive[l_pos..l_pos + blk_size].iter().all(|&x| x == -1) {
                    // Found a free block that can accept the block we want to swap
                    if l_pos < r_pos {
                        let mut swaps_remaining = blk_size;
                        while swaps_remaining > 0 {
                            drive.swap(
                                r_pos - (swaps_remaining - 1), // r_pos - 2, r_pos - 1, r_pos...
                                l_pos + (swaps_remaining - 1), // l_pos + 2, l_pos + 1, l_pos...
                            );
                            swaps_remaining -= 1;
                        }
                        found = true;
                        break;
                    }
                }
                l_pos += 1;
            }

            if !found {
                // skip ahead so we aren't moving part of a block
                r_pos -= blk_size - 1;
            }
        }
        r_pos -= 1;
    }
}

fn main() -> io::Result<()> {
    let drive_str = read_input("nine.in")?;

    let drive_map = map_drive(drive_str);
    let mut drive_cpy_one = drive_map.clone();
    reorder_drive_one(&mut drive_cpy_one);

    let ans_one = drive_cpy_one
        .iter()
        .take_while(|&&x| x != -1)
        .enumerate()
        .map(|(idx, el)| (el) * (idx as i64))
        .fold(0, |acc, x| acc + x);

    println!("Ans Part One: {ans_one}");

    let mut drive_cpy_two = drive_map.clone();
    reorder_drive_two(&mut drive_cpy_two);

    let ans_two = drive_cpy_two
        .iter()
        .enumerate()
        .map(|(idx, el)| {
            if *el >= 0 {
                return (el) * (idx as i64);
            } else {
                return 0;
            }
        })
        .fold(0, |acc, x| acc + x);

    println!("Ans Part Two: {ans_two}");

    Ok(())
}
