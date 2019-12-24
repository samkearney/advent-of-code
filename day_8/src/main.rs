use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

    const LAYER_SIZE : usize = 25 * 6;
    let num_layers : usize = contents.len() / LAYER_SIZE;

    let mut resolved_image : [char; LAYER_SIZE] = ['2'; LAYER_SIZE];
    for i in 0..num_layers {
        let mut pos = 0;
        for character in contents[LAYER_SIZE*i..LAYER_SIZE*i + LAYER_SIZE].chars() {
            if resolved_image[pos] == '2' {
                match character {
                    '0' => resolved_image[pos] = '0',
                    '1' => resolved_image[pos] = '1',
                    _ => ()
                };
            }
            pos += 1;
        }
    }

    for row in 0..6 {
        let row_data : String = resolved_image[row*25..(row + 1)*25].iter().collect();
        println!("{}", row_data);
    }
}
