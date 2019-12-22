use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Default)]
struct Opcode {
    code: u32,
    param_1_mode: bool,
    param_2_mode: bool,
    param_3_mode: bool
}

impl Opcode {
    fn from_string(string: &str) -> Opcode {
        let code_val = if string.len() == 1 { string.parse().unwrap() } else { string[string.len()-2..].parse().unwrap() };
        let mut result = Opcode {
            code: code_val, ..Default::default()
        };
        if string.len() > 2 {
            let mut iter = string.chars().rev().skip(2);
            match iter.next() {
                Some(mode) => result.param_1_mode = if mode.to_digit(10).unwrap() == 1 { true } else { false },
                None => return result
            }
            match iter.next() {
                Some(mode) => result.param_2_mode = if mode.to_digit(10).unwrap() == 1 { true } else { false },
                None => return result
            }
            match iter.next() {
                Some(mode) => result.param_3_mode = if mode.to_digit(10).unwrap() == 1 { true } else { false },
                None => return result
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_code_parsed_successfully() {
        let opcode = Opcode::from_string("11101");
        assert_eq!(opcode.code, 1);
        assert_eq!(opcode.param_1_mode, true);
        assert_eq!(opcode.param_2_mode, true);
        assert_eq!(opcode.param_3_mode, true);

        let opcode = Opcode::from_string("11002");
        assert_eq!(opcode.code, 2);
        assert_eq!(opcode.param_1_mode, false);
        assert_eq!(opcode.param_2_mode, true);
        assert_eq!(opcode.param_3_mode, true);
    }

    #[test]
    fn test_leading_zero_suppression() {
        let opcode = Opcode::from_string("03");
        assert_eq!(opcode.code, 3);
        assert_eq!(opcode.param_1_mode, false);
        assert_eq!(opcode.param_2_mode, false);
        assert_eq!(opcode.param_3_mode, false);
    }

    #[test]
    fn test_single_digit() {
        let opcode = Opcode::from_string("3");
        assert_eq!(opcode.code, 3);
        assert_eq!(opcode.param_1_mode, false);
        assert_eq!(opcode.param_2_mode, false);
        assert_eq!(opcode.param_3_mode, false);
    }
}

fn handle_add(opcode: &Opcode, a_pos : i32, b_pos : i32, result_pos : usize, program : &mut Vec<String>) {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let operand_1 = if opcode.param_1_mode { a_pos } else {
        assert!((a_pos as usize) < program.len(), "Index out of range!");
        program[a_pos as usize].parse::<i32>().unwrap()
    };
    let operand_2 = if opcode.param_2_mode { b_pos } else {
        assert!((b_pos as usize) < program.len(), "Index out of range!");
        program[b_pos as usize].parse::<i32>().unwrap()
    };
    program[result_pos] = (operand_1 + operand_2).to_string();
}

fn handle_mult(opcode: &Opcode, a_pos : i32, b_pos : i32, result_pos : usize, program : &mut Vec<String>) {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let operand_1 = if opcode.param_1_mode { a_pos } else { 
        assert!((a_pos as usize) < program.len(), "Index out of range!");
        program[a_pos as usize].parse::<i32>().unwrap() 
    };
    let operand_2 = if opcode.param_2_mode { b_pos } else {
        assert!((b_pos as usize) < program.len(), "Index out of range!");
        program[b_pos as usize].parse::<i32>().unwrap()
    };
    program[result_pos] = (operand_1 * operand_2).to_string();
}

fn handle_save(opcode: &Opcode, pos: usize, program: &mut Vec<String>) {
    assert!(opcode.param_1_mode == false, "Write location cannot be immediate");
    assert!(pos < program.len(), "Index out of range!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // parse() and to_string() to make sure we are getting a valid integer
    program[pos] = input.trim().parse::<i32>().unwrap().to_string();
}

fn handle_output(opcode: &Opcode, param: i32, program: &mut Vec<String>) {
    let param = if opcode.param_1_mode { param } else {
        assert!((param as usize) < program.len(), "Index out of range!");
        program[param as usize].parse::<i32>().unwrap()
    };
    println!("{}", param);
}

fn run_computer(input: &mut Vec<String>) {
    let mut pos: usize = 0;
    while pos < input.len() {
        let opcode = Opcode::from_string(&input[pos]);
        match opcode.code {
            1 => {
                let params : Vec<String> = input[pos + 1..pos + 4].to_vec();
                handle_add(&opcode,
                           params[0].parse::<i32>().unwrap(),
                           params[1].parse::<i32>().unwrap(), 
                           params[2].parse::<usize>().unwrap(),
                           input);
                pos += 4;
            },
            2 => {
                let params : Vec<String> = input[pos + 1..pos + 4].to_vec();
                handle_mult(&opcode,
                            params[0].parse::<i32>().unwrap(),
                            params[1].parse::<i32>().unwrap(), 
                            params[2].parse::<usize>().unwrap(),
                            input);
                pos += 4;
            },
            3 => {
                handle_save(&opcode,
                            input[pos + 1].parse::<usize>().unwrap(),
                            input);
                pos += 2;
            }
            4 => {
                handle_output(&opcode, input[pos + 1].parse::<i32>().unwrap(), input);
                pos += 2;
            }
            99 => return,
            _ => panic!("Bad opcode {}!", opcode.code)
        };
    }
}

fn main() {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open("input.txt") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open input.txt: {}", why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut op_list : Vec<String> = contents.split(',').map(|s| s.to_string()).collect();

    run_computer(&mut op_list);
}
