use std::io;

#[derive(PartialEq, Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

struct Opcode {
    code: u32,
    param_1_mode: ParameterMode,
    param_2_mode: ParameterMode,
    param_3_mode: ParameterMode,
}

impl Default for Opcode {
    fn default() -> Self {
        Self {
            code: 0,
            param_1_mode: ParameterMode::Position,
            param_2_mode: ParameterMode::Position,
            param_3_mode: ParameterMode::Position,
        }
    }
}

fn decode_parameter_mode(encoded: char) -> ParameterMode {
    match encoded.to_digit(10).unwrap() {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        _ => panic!("Bad parameter mode {}!", encoded),
    }
}

impl Opcode {
    fn from_string(string: &str) -> Opcode {
        let code_val = if string.len() == 1 {
            string.parse().unwrap()
        } else {
            string[string.len() - 2..].parse().unwrap()
        };
        let mut result = Opcode {
            code: code_val,
            ..Default::default()
        };
        if string.len() > 2 {
            let mut iter = string.chars().rev().skip(2);
            match iter.next() {
                Some(mode) => result.param_1_mode = decode_parameter_mode(mode),
                None => return result,
            }
            match iter.next() {
                Some(mode) => result.param_2_mode = decode_parameter_mode(mode),
                None => return result,
            }
            match iter.next() {
                Some(mode) => result.param_3_mode = decode_parameter_mode(mode),
                None => return result,
            }
        }
        result
    }
}

#[derive(Default)]
struct Program {
    text: Vec<String>,
    current_pos: usize,
    relative_base: isize,
}

fn get_param_value(mode: ParameterMode, param: i32, program: &Program) -> i32 {
    match mode {
        ParameterMode::Immediate => param,
        ParameterMode::Position => {
            assert!((param as usize) < program.text.len(), "Index {} out of program range!", param);
            program.text[param as usize].trim().parse().unwrap()
        }
        ParameterMode::Relative => {
            let pos = (param as isize + program.relative_base) as usize;
            assert!(pos < program.text.len(), "Index {} out of program range!", param);
            program.text[pos].trim().parse().unwrap()
        }
    }
}

fn handle_add(opcode: &Opcode, a_pos: i32, b_pos: i32, result_pos: usize, program: &mut Program) {
    assert!(opcode.param_3_mode != ParameterMode::Immediate, "Write location cannot be immediate");
    assert!(result_pos < program.text.len(), "Index out of range!");
    let operand_1 = get_param_value(opcode.param_1_mode, a_pos, program);
    let operand_2 = get_param_value(opcode.param_2_mode, b_pos, program);
    program.text[result_pos] = (operand_1 + operand_2).to_string();
    program.current_pos += 4;
}

fn handle_mult(opcode: &Opcode, a_pos: i32, b_pos: i32, result_pos: usize, program: &mut Program) {
    assert!(opcode.param_3_mode != ParameterMode::Immediate, "Write location cannot be immediate");
    assert!(result_pos < program.text.len(), "Index out of range!");
    let operand_1 = get_param_value(opcode.param_1_mode, a_pos, program);
    let operand_2 = get_param_value(opcode.param_2_mode, b_pos, program);
    program.text[result_pos] = (operand_1 * operand_2).to_string();
    program.current_pos += 4;
}

fn handle_input(
    opcode: &Opcode,
    pos: usize,
    program: &mut Program,
    input_fn: &mut impl FnMut() -> String,
) {
    assert!(opcode.param_1_mode != ParameterMode::Immediate, "Write location cannot be immediate");
    assert!(pos < program.text.len(), "Index out of range!");
    // parse() and to_string() to make sure we are getting a valid integer
    program.text[pos] = input_fn().parse::<i32>().unwrap().to_string();
    program.current_pos += 2;
}

fn handle_output(
    opcode: &Opcode,
    param: i32,
    program: &mut Program,
    output_fn: &mut impl FnMut(&str),
) {
    let param = get_param_value(opcode.param_1_mode, param, program);
    output_fn(&param.to_string());
    program.current_pos += 2;
}

fn handle_jump_if_true(opcode: &Opcode, param: i32, pos: i32, program: &mut Program) {
    let param = get_param_value(opcode.param_1_mode, param, program);
    let pos = get_param_value(opcode.param_2_mode, pos, program);
    if param != 0 {
        program.current_pos = pos as usize;
    } else {
        program.current_pos += 3;
    }
}

fn handle_jump_if_false(opcode: &Opcode, param: i32, pos: i32, program: &mut Program) {
    let param = get_param_value(opcode.param_1_mode, param, program);
    let pos = get_param_value(opcode.param_2_mode, pos, program);
    if param == 0 {
        program.current_pos = pos as usize;
    } else {
        program.current_pos += 3;
    }
}

fn handle_less_than(
    opcode: &Opcode,
    param_1: i32,
    param_2: i32,
    result_pos: usize,
    program: &mut Program,
) {
    assert!(opcode.param_3_mode != ParameterMode::Immediate, "Write location cannot be immediate");
    assert!(result_pos < program.text.len(), "Index out of range!");
    let param_1 = get_param_value(opcode.param_1_mode, param_1, program);
    let param_2 = get_param_value(opcode.param_2_mode, param_2, program);
    program.text[result_pos] = if param_1 < param_2 {
        "1".to_string()
    } else {
        "0".to_string()
    };
    program.current_pos += 4
}

fn handle_equals(
    opcode: &Opcode,
    param_1: i32,
    param_2: i32,
    result_pos: usize,
    program: &mut Program,
) {
    assert!(opcode.param_3_mode != ParameterMode::Immediate, "Write location cannot be immediate");
    assert!(result_pos < program.text.len(), "Index out of range!");
    let param_1 = get_param_value(opcode.param_1_mode, param_1, program);
    let param_2 = get_param_value(opcode.param_2_mode, param_2, program);
    program.text[result_pos] = if param_1 == param_2 {
        "1".to_string()
    } else {
        "0".to_string()
    };
    program.current_pos += 4
}

fn default_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn default_output(output: &str) {
    println!("{}", output);
}

pub fn run(input: Vec<String>) {
    run_with_custom_io(input, &mut default_input, &mut default_output);
}

pub fn run_with_custom_io(
    input: Vec<String>,
    input_fn: &mut impl FnMut() -> String,
    output_fn: &mut impl FnMut(&str),
) {
    let mut program = Program {
        text: input,
        ..Default::default()
    };
    let mut iteration_num: u32 = 0;
    loop {
        let opcode = Opcode::from_string(&program.text[program.current_pos].trim());
        match opcode.code {
            1 => {
                let params: Vec<String> =
                    program.text[program.current_pos + 1..program.current_pos + 4].to_vec();
                handle_add(
                    &opcode,
                    params[0].parse().unwrap(),
                    params[1].parse().unwrap(),
                    params[2].parse().unwrap(),
                    &mut program,
                );
            }
            2 => {
                let params: Vec<String> =
                    program.text[program.current_pos + 1..program.current_pos + 4].to_vec();
                handle_mult(
                    &opcode,
                    params[0].parse().unwrap(),
                    params[1].parse().unwrap(),
                    params[2].parse().unwrap(),
                    &mut program,
                );
            }
            3 => {
                handle_input(
                    &opcode,
                    program.text[program.current_pos + 1].parse().unwrap(),
                    &mut program,
                    input_fn,
                );
            }
            4 => {
                handle_output(
                    &opcode,
                    program.text[program.current_pos + 1].parse().unwrap(),
                    &mut program,
                    output_fn,
                );
            }
            5 => {
                handle_jump_if_true(
                    &opcode,
                    program.text[program.current_pos + 1].parse().unwrap(),
                    program.text[program.current_pos + 2].parse().unwrap(),
                    &mut program,
                );
            }
            6 => {
                handle_jump_if_false(
                    &opcode,
                    program.text[program.current_pos + 1].parse().unwrap(),
                    program.text[program.current_pos + 2].parse().unwrap(),
                    &mut program,
                );
            }
            7 => handle_less_than(
                &opcode,
                program.text[program.current_pos + 1].parse().unwrap(),
                program.text[program.current_pos + 2].parse().unwrap(),
                program.text[program.current_pos + 3].parse().unwrap(),
                &mut program,
            ),
            8 => handle_equals(
                &opcode,
                program.text[program.current_pos + 1].parse().unwrap(),
                program.text[program.current_pos + 2].parse().unwrap(),
                program.text[program.current_pos + 3].parse().unwrap(),
                &mut program,
            ),
            99 => return,
            _ => panic!(
                "Bad opcode {} Pos: {} Str: {} Iteration: {}!",
                opcode.code, program.current_pos, program.text[program.current_pos], iteration_num
            ),
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
        assert_eq!(opcode.param_1_mode, ParameterMode::Immediate);
        assert_eq!(opcode.param_2_mode, ParameterMode::Immediate);
        assert_eq!(opcode.param_3_mode, ParameterMode::Immediate);

        let opcode = Opcode::from_string("21002");
        assert_eq!(opcode.code, 2);
        assert_eq!(opcode.param_1_mode, ParameterMode::Position);
        assert_eq!(opcode.param_2_mode, ParameterMode::Immediate);
        assert_eq!(opcode.param_3_mode, ParameterMode::Relative);
    }

    #[test]
    fn test_leading_zero_suppression() {
        let opcode = Opcode::from_string("03");
        assert_eq!(opcode.code, 3);
        assert_eq!(opcode.param_1_mode, ParameterMode::Position);
        assert_eq!(opcode.param_2_mode, ParameterMode::Position);
        assert_eq!(opcode.param_3_mode, ParameterMode::Position);
    }

    #[test]
    fn test_single_digit() {
        let opcode = Opcode::from_string("3");
        assert_eq!(opcode.code, 3);
        assert_eq!(opcode.param_1_mode, ParameterMode::Position);
        assert_eq!(opcode.param_2_mode, ParameterMode::Position);
        assert_eq!(opcode.param_3_mode, ParameterMode::Position);
    }
}
