extern crate intcode_computer;
extern crate permutohedron;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;

fn test_permutation(program: &Vec<String>, phase_settings: &mut [u32]) -> u32 {
    let mut recv_channels = Vec::new();
    let mut send_channels = Vec::new();
    for i in 0..5 {
        let (tx, rx) = mpsc::channel();
        recv_channels.push(rx);
        if i == 4 {
            send_channels.insert(0, tx);
        } else {
            send_channels.push(tx);
        }
    }

    let mut join_handles = Vec::new();

    let (last_output_tx, last_output_rx) = mpsc::channel();

    // Special closure for the first amp
    let program_copy = program.to_owned();
    let phase = phase_settings[0];
    let send_channel = send_channels.pop().unwrap();
    let recv_channel = recv_channels.pop().unwrap();
    join_handles.push(thread::spawn(move || {
        let mut gave_phase = false;
        let mut gave_ampl = false;
        intcode_computer::run_with_custom_io(
            program_copy,
            &mut || {
                if !gave_phase {
                    // Feed the phase to each amp
                    gave_phase = true;
                    phase.to_string()
                } else if !gave_ampl {
                    // Feed the initial value to the first amp
                    gave_ampl = true;
                    "0".to_string()
                } else {
                    recv_channel.recv().unwrap()
                }
            },
            &mut |out_str| {
                send_channel.send(out_str.to_string()).unwrap();
            },
        );
        last_output_tx.send(recv_channel.recv().unwrap()).unwrap();
    }));

    // Closures for the other 4 amps
    for index in 1..5 {
        let program_copy = program.to_owned();
        let phase = phase_settings[index];
        let send_channel = send_channels.pop().unwrap();
        let recv_channel = recv_channels.pop().unwrap();
        join_handles.push(thread::spawn(move || {
            let mut gave_phase = false;
            intcode_computer::run_with_custom_io(
                program_copy,
                &mut || {
                    if !gave_phase {
                        // Feed the phase to each amp
                        gave_phase = true;
                        phase.to_string()
                    } else {
                        recv_channel.recv().unwrap()
                    }
                },
                &mut |out_str| {
                    send_channel.send(out_str.to_string()).unwrap();
                },
            );
        }));
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
    last_output_rx.recv().unwrap().parse().unwrap()
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
    let original_program: Vec<String> = contents.split(',').map(|s| s.to_string()).collect();

    let mut largest_signal = 0;

    let mut data = [5, 6, 7, 8, 9];
    permutohedron::heap_recursive(&mut data, |permutation| {
        let result = test_permutation(&original_program, permutation);
        if result > largest_signal {
            largest_signal = result;
        }
    });

    println!("Largest signal: {}", largest_signal);
}
