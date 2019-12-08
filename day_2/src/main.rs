use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn handle_add(a_pos : usize, b_pos : usize, result_pos : usize, program : &mut Vec<String>) {
    assert!(a_pos < program.len(), "Index out of range!");
    assert!(b_pos < program.len(), "Index out of range!");
    assert!(result_pos < program.len() "Index out of range!");
    program[result_pos] = (program[a_pos].parse::<u32>().unwrap() + 
                           program[b_pos].parse::<u32>().unwrap())
                            .to_string();
}

fn handle_mult(a_pos : usize, b_pos : usize, result_pos : usize, program : &mut Vec<String>) {
    assert!(a_pos < program.len(), "Index out of range!");
    assert!(b_pos < program.len(), "Index out of range!");
    assert!(result_pos < program.len() "Index out of range!");
    program[result_pos] = (program[a_pos].parse::<u32>().unwrap() *
                           program[b_pos].parse::<u32>().unwrap())
                            .to_string();
}

fn run_computer(input: &mut Vec<String>) {
    let mut pos: usize = 0;
    while pos < input.len() {
        let opcode : Vec<String> = input[pos..pos + 4].to_vec();
        match opcode[0].parse::<u32>().unwrap() {
            1 => {
                assert!(opcode.len() == 4, "Ill-formed opcode!");
                handle_add(opcode[1].parse::<usize>().unwrap(),
                           opcode[2].parse::<usize>().unwrap(), 
                           opcode[3].parse::<usize>().unwrap(),
                           input);
            },
            2 => {
                assert!(opcode.len() == 4, "Ill-formed opcode!");
                handle_mult(opcode[1].parse::<usize>().unwrap(),
                           opcode[2].parse::<usize>().unwrap(), 
                           opcode[3].parse::<usize>().unwrap(),
                           input);
            }
            99 => return,
            _ => panic!("Bad opcode {}!", opcode[0])
        };
        pos += 4;
    }
}

fn try_input_pair(input_1: u32, input_2: u32, initial_program: &Vec<String>) -> u32 {
    let mut program_copy = initial_program.clone();
    program_copy[1] = input_1.to_string();
    program_copy[2] = input_2.to_string();
    run_computer(&mut program_copy);
    return program_copy[0].parse::<u32>().unwrap();
}

fn determine_inputs_for_output(output: u32, initial_program: &Vec<String>) -> Option<(u32, u32)> {
    for input_1 in 0..99 {
        for input_2 in 0..99 {
            if try_input_pair(input_1, input_2, initial_program) == output {
                return Some((input_1, input_2));
            }
        }
    }
    None
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
    let op_list : Vec<String> = contents.split(',').map(|s| s.to_string()).collect();

    match determine_inputs_for_output(19690720, &op_list) {
        Some((input_1, input_2)) => {
            println!("Input 1: {} Input 2: {}", input_1, input_2);
            println!("Solution: {}", 100 * input_1 + input_2);
        },
        None => println!("No valid inputs found for result {}", 19690720)
    };
}
