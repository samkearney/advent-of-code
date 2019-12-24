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

fn get_positional_param(pos: i32, program: &Vec<String>) -> i32 {
    assert!((pos as usize) < program.len(), "Index out of range!");
    program[pos as usize].trim().parse::<i32>().unwrap()
}

fn handle_add(opcode: &Opcode, a_pos: i32, b_pos: i32, result_pos: usize, program: &mut Vec<String>, current_pos: usize) -> usize {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let operand_1 = if opcode.param_1_mode { a_pos } else { get_positional_param(a_pos, program) };
    let operand_2 = if opcode.param_2_mode { b_pos } else { get_positional_param(b_pos, program) };
    program[result_pos] = (operand_1 + operand_2).to_string();
    current_pos + 4
}

fn handle_mult(opcode: &Opcode, a_pos: i32, b_pos: i32, result_pos: usize, program: &mut Vec<String>,
               current_pos: usize) -> usize {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let operand_1 = if opcode.param_1_mode { a_pos } else { get_positional_param(a_pos, program) };
    let operand_2 = if opcode.param_2_mode { b_pos } else { get_positional_param(b_pos, program) };
    program[result_pos] = (operand_1 * operand_2).to_string();
    current_pos + 4
}

fn handle_save(opcode: &Opcode, pos: usize, program: &mut Vec<String>, current_pos: usize) -> usize {
    assert!(opcode.param_1_mode == false, "Write location cannot be immediate");
    assert!(pos < program.len(), "Index out of range!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // parse() and to_string() to make sure we are getting a valid integer
    program[pos] = input.trim().parse::<i32>().unwrap().to_string();
    current_pos + 2
}

fn handle_output(opcode: &Opcode, param: i32, program: &mut Vec<String>, current_pos: usize) -> usize {
    let param = if opcode.param_1_mode { param } else { get_positional_param(param, program) };
    println!("{}", param);
    current_pos + 2
}

fn handle_jump_if_true(opcode: &Opcode, param: i32, pos: i32, program: &mut Vec<String>,
                       current_pos: usize) -> usize {
    let param = if opcode.param_1_mode { param } else { get_positional_param(param, program) };
    let pos = if opcode.param_2_mode { pos } else { get_positional_param(pos, program) };
    if param != 0 { pos as usize } else { current_pos + 3 }
}

fn handle_jump_if_false(opcode: &Opcode, param: i32, pos: i32, program: &mut Vec<String>,
                        current_pos: usize) -> usize {
    let param = if opcode.param_1_mode { param } else { get_positional_param(param, program) };
    let pos = if opcode.param_2_mode { pos } else { get_positional_param(pos, program) };
    if param == 0 { pos as usize } else { current_pos + 3 }
}

fn handle_less_than(opcode: &Opcode, param_1: i32, param_2: i32, result_pos: usize, program: &mut Vec<String>,
                    current_pos: usize) -> usize {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let param_1 = if opcode.param_1_mode { param_1 } else { get_positional_param(param_1, program) };
    let param_2 = if opcode.param_2_mode { param_2 } else { get_positional_param(param_2, program) };
    program[result_pos] = if param_1 < param_2 { "1".to_string() } else { "0".to_string() };
    current_pos + 4
}

fn handle_equals(opcode: &Opcode, param_1: i32, param_2: i32, result_pos: usize, program: &mut Vec<String>,
                 current_pos: usize) -> usize {
    assert!(opcode.param_3_mode == false, "Write location cannot be immediate");
    assert!(result_pos < program.len(), "Index out of range!");
    let param_1 = if opcode.param_1_mode { param_1 } else { get_positional_param(param_1, program) };
    let param_2 = if opcode.param_2_mode { param_2 } else { get_positional_param(param_2, program) };
    program[result_pos] = if param_1 == param_2 { "1".to_string() } else { "0".to_string() };
    current_pos + 4
}

pub fn run(input: &mut Vec<String>) {
    let mut pos: usize = 0;
    let mut iteration_num: u32 = 0;
    while pos < input.len() {
        let opcode = Opcode::from_string(&input[pos]);
        match opcode.code {
            1 => {
                let params : Vec<String> = input[pos + 1..pos + 4].to_vec();
                pos = handle_add(&opcode, params[0].parse().unwrap(), params[1].parse().unwrap(),
                                 params[2].parse().unwrap(), input, pos);
            },
            2 => {
                let params : Vec<String> = input[pos + 1..pos + 4].to_vec();
                pos = handle_mult(&opcode, params[0].parse().unwrap(), params[1].parse().unwrap(), 
                                  params[2].parse().unwrap(), input, pos);
            },
            3 => {
                pos = handle_save(&opcode, input[pos + 1].parse().unwrap(), input, pos);
            },
            4 => {
                pos = handle_output(&opcode, input[pos + 1].parse().unwrap(), input, pos);
            },
            5 => {
                pos = handle_jump_if_true(&opcode, input[pos + 1].parse().unwrap(), input[pos + 2].parse().unwrap(),
                                          input, pos);
            },
            6 => {
                pos = handle_jump_if_false(&opcode, input[pos + 1].parse().unwrap(), input[pos + 2].parse().unwrap(),
                                           input, pos);
            },
            7 => {
                pos = handle_less_than(&opcode, input[pos + 1].parse().unwrap(), input[pos + 2].parse().unwrap(),
                                       input[pos + 3].parse().unwrap(), input, pos)
            },
            8 => {
                pos = handle_equals(&opcode, input[pos + 1].parse().unwrap(), input[pos + 2].parse().unwrap(),
                                    input[pos + 3].parse().unwrap(), input, pos)
            },
            99 => return,
            _ => panic!("Bad opcode {} Pos: {} Str: {} Iteration: {}!", opcode.code, pos, input[pos], iteration_num)
        };
        iteration_num += 1;
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
