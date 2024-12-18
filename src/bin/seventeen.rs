use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::BitXor,
};

#[derive(Clone)]
struct TimeComputer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

fn read_input(filepath: &str) -> io::Result<(TimeComputer, Vec<u8>)> {
    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut reg_a: u64 = 0;
    let mut reg_b: u64 = 0;
    let mut reg_c: u64 = 0;
    let mut instruction_set: Vec<u8> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Register A: ") {
            reg_a = line
                .strip_prefix("Register A: ")
                .unwrap()
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register B") {
            reg_b = line
                .strip_prefix("Register B: ")
                .unwrap()
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register C") {
            reg_c = line
                .strip_prefix("Register C: ")
                .unwrap()
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Program: ") {
            instruction_set = line
                .strip_prefix("Program: ")
                .unwrap()
                .split(",")
                .map(|i| i.parse::<u8>().unwrap())
                .collect();
        }
    }

    Ok((
        TimeComputer {
            reg_a: reg_a,
            reg_b: reg_b,
            reg_c: reg_c,
        },
        instruction_set,
    ))
}

fn combo_operand(computer: &TimeComputer, op: u8) -> u64 {
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.
    match op {
        0 => return 0,
        1 => return 1,
        2 => return 2,
        3 => return 3,
        4 => return computer.reg_a,
        5 => return computer.reg_b,
        6 => return computer.reg_c,
        7 => unreachable!(),
        _ => unreachable!(),
    }
}

// The adv instruction (opcode 0) performs division.
fn adv(computer: &mut TimeComputer, op: u8) {
    // The numerator is the value in the A register.
    let numerator = computer.reg_a;
    // The denominator is found by raising 2 to the power of the instruction's combo operand.
    // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    let combo_op = combo_operand(computer, op);
    let denominator = 2_u64.pow(combo_op as u32);
    assert!(denominator != 0);
    let res = numerator / denominator;
    // The result of the division operation is truncated to an integer and then written to the A register.
    println!("adv({op}) sets register a to {res}");
    computer.reg_a = res;
}

// The bxl instruction (opcode 1) calculates the bitwise XOR of register B
// and the instruction's literal operand, then stores the result in register B.
fn bxl(computer: &mut TimeComputer, op: u8) {
    let res = computer.reg_b.bitxor(op as u64);
    println!("bxl({op}) sets register b to {res}");
    computer.reg_b = res;
}

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
//  (thereby keeping only its lowest 3 bits), then writes that value to the B register.
fn bst(computer: &mut TimeComputer, op: u8) {
    let res = combo_operand(computer, op).rem_euclid(8);
    println!("bst({op}) sets register b to {res}");
    computer.reg_b = res;
}

fn jnz(computer: &TimeComputer, op: u8) -> Option<u8> {
    // The jnz instruction (opcode 3) does nothing if the A register is 0.
    if computer.reg_a == 0 {
        println!("jnz({op}) does not jump");
        return None;
    }
    // However, if the A register is not zero, it jumps by setting the instruction pointer
    // to the value of its literal operand; if this instruction jumps,
    // the instruction pointer is not increased by 2 after this instruction.
    println!("jnz({op}) jumps to {op}");
    return Some(op);
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
// then stores the result in register B.
// (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(computer: &mut TimeComputer, _: u8) {
    let res = computer.reg_b.bitxor(computer.reg_c);
    println!("bxc() sets register b to {res}");
    computer.reg_b = res;
}

// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
//  then outputs that value. (If a program outputs multiple values, they are separated by commas.)
fn out(computer: &TimeComputer, op: u8) -> String {
    let combo_op = combo_operand(computer, op).rem_euclid(8);
    let s = combo_op.to_string();
    println!("out({op}) returns {s}");
    return s;
}

// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
fn bdv(computer: &mut TimeComputer, op: u8) {
    let numerator = computer.reg_a;
    let combo_op = combo_operand(computer, op);
    let denominator = 2_u64.pow(combo_op as u32);
    assert!(denominator != 0);
    let res = numerator / denominator;
    println!("bdv({op}) sets register b to {res}");
    computer.reg_b = res;
}

// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
fn cdv(computer: &mut TimeComputer, op: u8) {
    let numerator = computer.reg_a;
    let combo_op = combo_operand(computer, op);
    let denominator = 2_u64.pow(combo_op as u32);
    assert!(denominator != 0);
    let res = numerator / denominator;
    println!("cdv({op}) sets register c to {res}");
    computer.reg_c = res;
}

fn process_instructions(computer: &mut TimeComputer, instruction_set: Vec<u8>) -> String {
    // A number called the instruction pointer identifies the position in the program
    // from which the next opcode will be read; it starts at 0,
    // pointing at the first 3-bit number in the program. Except for jump instructions,
    // the instruction pointer increases by 2 after each instruction is processed
    // (to move past the instruction's opcode and its operand).
    // If the computer tries to read an opcode past the end of the program, it instead halts.
    let mut output: String = String::new();
    let mut instruction_ptr = 0;
    loop {
        if instruction_ptr > instruction_set.len() - 1 {
            // this would read past the end
            break;
        }
        let next_inst = instruction_set[instruction_ptr];
        let next_op = instruction_set[instruction_ptr + 1];
        match next_inst {
            0 => {
                adv(computer, next_op);
                instruction_ptr += 2;
            }
            1 => {
                bxl(computer, next_op);
                instruction_ptr += 2;
            }
            2 => {
                bst(computer, next_op);
                instruction_ptr += 2;
            }
            3 => {
                if let Some(jump) = jnz(computer, next_op) {
                    instruction_ptr = jump as usize;
                } else {
                    instruction_ptr += 2;
                }
            }
            4 => {
                bxc(computer, next_op);
                instruction_ptr += 2
            }
            5 => {
                let out_str = out(computer, next_op);
                output.push_str(&format!("{},", out_str));
                instruction_ptr += 2
            }
            6 => {
                bdv(computer, next_op);
                instruction_ptr += 2;
            }
            7 => {
                cdv(computer, next_op);
                instruction_ptr += 2
            }
            _ => unreachable!(),
        }
    }

    output.pop();
    return output.to_string();
}

fn reverse_engineer(computer: &TimeComputer, instruction_set: Vec<u8>) -> u64 {
    let mut candidates = vec![(0, 0u64)];
    let instruction_count = instruction_set.len();
    let mut correct_answers: Vec<u64> = vec![];

    while let Some((digit, candidate)) = candidates.pop() {
        for x in 0..8 {
            let mut new_computer: TimeComputer = computer.clone();
            new_computer.reg_a = candidate + x;
            let output = process_instructions(&mut new_computer, instruction_set.clone());

            let out_vec: Vec<u8> = output
                .split(',')
                .filter_map(|s| s.parse::<u8>().ok())
                .collect();
            if instruction_set[instruction_count - digit - 1] == out_vec[0] {
                if instruction_set == out_vec {
                    correct_answers.push(candidate + x);
                } else {
                    candidates.push((digit + 1, (candidate + x) * 8));
                }
            }
        }
    }
    assert!(correct_answers.len() > 0);
    return *correct_answers.iter().min().unwrap();
}

fn main() {
    let (mut computer, instruction_set) = read_input("seventeen.in").unwrap();
    let ans_one = process_instructions(&mut computer, instruction_set);
    println!("Ans Part One: {ans_one}");

    // get a fresh copy of everything
    let (new_computer, new_instruction_set) = read_input("seventeen.in").unwrap();
    let ans_two = reverse_engineer(&new_computer, new_instruction_set);
    println!("Ans Part Two: {ans_two}");
}
