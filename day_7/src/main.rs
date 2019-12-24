extern crate intcode_computer;
extern crate permutohedron;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn test_permutation(program: &Vec<String>, phase_settings: &mut[u32]) -> u32 {
    let mut program_copy = program.to_owned();
    let mut output = 0;
    let mut next_output = 0;
    for phase in phase_settings {
        let mut gave_phase = false;
        intcode_computer::run_with_custom_io(&mut program_copy,
            &mut || { if !gave_phase { gave_phase = true; phase.to_string() } else { output.to_string() } },
            &mut |out_str| { next_output = out_str.parse().unwrap(); }
        );
        output = next_output;
    }
    output
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
    let original_program : Vec<String> = contents.split(',').map(|s| s.to_string()).collect();

    let mut largest_signal = 0;

    let mut data = [0, 1, 2, 3, 4];
    permutohedron::heap_recursive(&mut data, |permutation| {
        let result = test_permutation(&original_program, permutation);
        if result > largest_signal {
            largest_signal = result;
        }
    });

    println!("Largest signal: {}", largest_signal);
}
