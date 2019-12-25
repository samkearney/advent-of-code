use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

extern crate intcode_computer;

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
    let op_list: Vec<String> = contents.split(',').map(|s| s.to_string()).collect();

    intcode_computer::run(op_list);
}
