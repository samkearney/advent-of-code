use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn calc_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + calc_fuel(fuel)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_fuel_large() {
        assert_eq!(calc_fuel(1969), 966);
    }

    #[test]
    fn test_calc_fuel_small() {
        assert_eq!(calc_fuel(14), 2);
    }
}

fn main() -> std::io::Result<()> {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let reader = BufReader::new(file);

    let mut sum : i32 = 0;
    for line in reader.lines() {
        let mass : i32 = line?.parse().unwrap();
        sum += calc_fuel(mass);
    }

    println!("Sum is {}", sum);

    // `file` goes out of scope, and the "input.txt" file gets closed
    Ok(())
}
