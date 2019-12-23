use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_indirect_orbits(orbitee: &str, objects: &HashMap<String, String>) -> u32 {
    let mut orbits = 0;
    let mut new_orbitee = objects.get(orbitee);
    loop {
        match new_orbitee {
            Some(val) => {
                orbits += 1;
                new_orbitee = objects.get(val);
            },
            None => return orbits
        }
    }
}

fn get_santa_path(objects: &HashMap<String, String>) -> HashMap<String, u32> {
    let mut santa_path : HashMap<String, u32> = HashMap::new();
    let mut num_traversals = 0;
    let mut orbitee = "SAN";
    loop {
        match objects.get(orbitee) {
            Some(val) => {
                santa_path.insert(val.to_string(), num_traversals);
                num_traversals += 1;
                orbitee = val;
            },
            None => return santa_path
        }
    }
}

fn get_traversals(objects: &HashMap<String, String>, santa_path: &HashMap<String, u32>) -> u32 {
    let mut num_traversals = 0;
    let mut orbitee = "YOU";
    loop {
        match objects.get(orbitee) {
            Some(next_orbitee) => {
                match santa_path.get(next_orbitee) {
                    Some(intersection) => return num_traversals + intersection,
                    None => {
                        num_traversals += 1;
                        orbitee = next_orbitee;
                    }
                }
            }
            None => panic!("Didn't find a path between YOU and SAN!")
        }
    }
}

fn main() {
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open("input.txt") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open input.txt: {}", why.description()),
        Ok(file) => file,
    };

    let mut objects = HashMap::new();
    let mut iteration = 0;

    // Use a BufReader to read the file line-by-line.
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Err(why) => panic!("Error reading line from input file: {}", why.description()),
            Ok(line) => line
        };
        iteration += 1;

        let orbit_pair : Vec<&str> = line.trim().split(')').collect();
        assert_eq!(orbit_pair.len(), 2, "Invalid line!");

        let insert_result = objects.insert(orbit_pair[1].to_string(), orbit_pair[0].to_string());
        assert_eq!(insert_result, None, "Duplicate value {}, iteration {}!", orbit_pair[1], iteration);
    }

    let mut direct_orbits = 0;
    let mut indirect_orbits = 0;
    for (_, orbitee) in objects.iter() {
        direct_orbits += 1;
        indirect_orbits += get_indirect_orbits(&orbitee, &objects);
    }

    println!("Direct: {} Indirect: {} Total: {}", direct_orbits, indirect_orbits, direct_orbits + indirect_orbits);

    // Grab the path from Santa to COM
    let santa_path = get_santa_path(&objects);
    println!("Traversals: {}", get_traversals(&objects, &santa_path));
}
