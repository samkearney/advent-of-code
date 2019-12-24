use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
struct Layer {
    zero_count : usize,
    one_count : usize,
    two_count: usize
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

    const LAYER_SIZE : usize = 25 * 6;
    let num_layers : usize = contents.len() / LAYER_SIZE;

    let mut fewest_zeroes = Layer {
        zero_count: std::usize::MAX,
        ..Default::default()
    };
    for i in 0..num_layers {
        let mut layer : Layer = Default::default();
        println!("New layer, Counts: {} {} {}", layer.zero_count, layer.one_count, layer.two_count);
        for character in contents[LAYER_SIZE*i..LAYER_SIZE*i + LAYER_SIZE].chars() {
            if character == '0' {
                layer.zero_count += 1;
            } else if character == '1' {
                layer.one_count += 1;
            } else if character == '2' {
                layer.two_count += 1;
            }
        }
        println!("This layer counts: {} {} {}", layer.zero_count, layer.one_count, layer.two_count);
        if layer.zero_count < fewest_zeroes.zero_count {
            fewest_zeroes = layer;
            println!("Fewest zeroes is now: {} {} {}", fewest_zeroes.zero_count, fewest_zeroes.one_count, fewest_zeroes.two_count);
        }
    }

    println!("Answer: {}", fewest_zeroes.one_count * fewest_zeroes.two_count);
}
