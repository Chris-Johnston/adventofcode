// day 5

use std::env;
use std::fs;
use std::vec;
use std::io;

fn main() {
    assert!(get_parameter_modes(10100, 0) == vec!(1, 0, 1));
    assert!(get_parameter_modes(0, 3) == vec!(0, 0, 0));

    let input_file = "/home/chris/Git/adventofcode/2019/5/day5/input.txt";

    let content = fs::read_to_string(input_file).expect("Failed to read the file.");
    let mut values: Vec<isize> = content
        .split(",")
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect();
    println!("input");
    println!("{:?}", values);

    let mut index = 0; // instruction pointer

    while index < values.len() {
        let opcode = values[index] % 100;
        // pad with more digits than required for all use cases
        let modes = get_parameter_modes(values[index], 4);

        match opcode {
            1 => {
                //println!("add");
                // add
                let location = values[index + 3] as usize; // special case
                let a = get_parameter(&values, index + 1, modes[0]);
                let b = get_parameter(&values, index + 2, modes[1]);
                values[location] = a + b;

                // increment index for all parameters
                index += 4;
            }
            2 => {
                //println!("mul");
                // mul
                let location = values[index + 3] as usize; // special case
                let a = get_parameter(&values, index + 1, modes[0]);
                let b = get_parameter(&values, index + 2, modes[1]);
                values[location] = a * b;
                
                // increment index for all parameters
                index += 4;
            }
            3 => {
                // read single int from the console input
                let location = values[index + 1];
                print!("INPUT: ");
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");
                let value : isize = input_text
                            .trim()
                            .parse()
                            .expect("couldn't parse the input");
                values[location as usize] = value;

                index += 2;
            }
            4 => {
                // output
                let value = get_parameter(&values, index + 1, modes[0]);
                println!("OUTPUT: {}", value);

                index += 2;
            }
            99 => {
                println!("exit");
                // exit
                break;
            }
            _ => {
                print!("Unknown opcode.");
                break;
            }
        }
    }

    // finally get the value at position zero
    println!("{:?}", values);
    let output = values[0];
    println!("output: {}", output);
}

// opcode
// 1 - adds togethers numbers and stores in third
// first two indicate positions, third indicates position where it is stored
// 1, 10, 20, 30
// val at 10 and 20 added = 30
// 2 - multiplies
// each opcode is 4 csv
// 99 - program finished

fn get_parameter_modes(opcode: isize, min_len: usize) -> Vec<usize> {
    let mut parameter_modes = Vec::new();
    let mut param_val = opcode / 100;

    // println!("opcode {} -> param {}", opcode, param_val);
    while param_val > 0 {
        // println!("param val {}", param_val);
        // either 1 or 0
        let digit = param_val % 10;
        param_val /= 10;
        // println!("adding digit {}", digit);
        parameter_modes.push(digit as usize);
    }

    while parameter_modes.len() < min_len
    {
        // println!("padding 0");
        // pad with zeros
        parameter_modes.push(0);
    }

    // println!("modes: {:?}", parameter_modes);
    return parameter_modes;
}

fn get_parameter(values: &Vec<isize>, parameter_index: usize, parameter_mode: usize) -> isize
{
    // gets the value of a parameter based on if it is' immediate or position parameter_mode
    match parameter_mode
    {
        0 => {
            //println!("position mode");
            // position parameter mode
            // use the value at that index as the location
            // then return the value at that location
            return values[values[parameter_index] as usize];
        },
        1 => {
            //println!("imm");
            // immediate
            // use the value at that index
            return values[parameter_index];
        },
        _ => {
            println!("invald mode");
            return -1;
        }
    }
}