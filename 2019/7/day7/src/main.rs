// day 7
// amp a - e, each has a phase setting 0-4
// each phase setting used exactly once
// program will call another input instructio to get the amps input signal
// compute the output, supply back
// get the highest signal

use std::env;
use std::fs;
use std::vec;
use std::io;
use std::collections::VecDeque;
use std::slice;
use std::collections::HashMap;
use std::process;

fn main()
{
    assert!(get_parameter_modes(10100, 0) == vec!(1, 0, 1));
    assert!(get_parameter_modes(0, 3) == vec!(0, 0, 0));

    let input_file = "/home/chris/Git/adventofcode/2019/7/day7/input.txt";
    let mut values = parse_input_file(input_file);
    // add a lot of empty memory
    // values.append(&mut vec![0; 8000]);
    let mut memory : HashMap<usize, isize> = HashMap::new();

    for (index, value) in values.iter().enumerate()
    {
        memory.insert(index, *value);
    }

    //println!("{:?}", memory);

    run(&mut memory);
}

fn parse_input_file(input_file: &str) -> Vec<isize>
{
    let content = fs::read_to_string(input_file).expect("Failed to read the file.");
    let values: Vec<isize> = content
        .split(",")
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect();
    values
}

fn run(values: &mut HashMap<usize, isize>)
{   
    let mut index : usize = 0; // instruction pointer
    let mut relative_base = 0;

    while index < values.len() {
        let opcode = values[&index] % 100;
        // pad with more digits than required for all use cases
        let modes = get_parameter_modes(values[&index], 4);

        // print!("{} ", values[&index]);

        match opcode {
            1 => {
                ////println!("add");
                // add
                // let location = values[&(index + 3)] as usize; // special case
                let location = get_parameter(values, index + 3, modes[2], relative_base, true) as usize;
                let a = get_parameter(values, index + 1, modes[0], relative_base, false);
                let b = get_parameter(values, index + 2, modes[1], relative_base, false);
                //println!("add {} + {} => {}", a, b, location);
                // values[&location] = a + b;
                values.insert(location, a + b);

                // increment index for all parameters
                index += 4;
            },
            2 => {
                ////println!("mul");
                // mul
                let location = get_parameter(values, index + 3, modes[2], relative_base, true) as usize;
                let a = get_parameter(values, index + 1, modes[0], relative_base, false);
                let b = get_parameter(values, index + 2, modes[1], relative_base, false);
                //println!("mul {} * {} => {}", a, b, location);
                // values[&location] = a * b;
                values.insert(location, a * b);
                
                // increment index for all parameters
                index += 4;
            },
            3 => {
                // store single int from the console input
                let location = get_parameter(values, index + 1, modes[0], relative_base, true) as usize;
                //println!("op {}, {} location is {} (base {} )", values[&index], values[&(index + 1)], location, relative_base);
                //println!("INPUT: ");
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .ok()
                    .expect("failed to read from stdin");
                let value : isize = input_text
                            .trim()
                            .parse()
                            .expect("couldn't parse the input");
                // values[&location] = value;
                values.insert(location, value);
                //println!("store {} => {} (mode {})", value, location, modes[0]);

                index += 2;
            },
            4 => {
                // output
                let value = get_parameter(values, index + 1, modes[0], relative_base, false);
                //println!("OUTPUT: {}", value);
                println!("{}", value);

                index += 2;
            },
            5 => // jump non zero 
            {
                // if first param non zero
                // set instruction pointer to the value from the second parameter
                let condition = get_parameter(values, index + 1, modes[0], relative_base, false);
                //println!("jnz {} {}", condition, condition != 0);
                if condition != 0
                {
                    // index = values[&(index + 2)] as usize;
                    index = get_parameter(values, index + 2, modes[1], relative_base, false) as usize;
                }
                else
                {
                    index += 3;
                }
            },
            6 => // jump zero
            {
                // if first param zero
                // set instruction pointer to the value from the second parameter
                let condition = get_parameter(values, index + 1, modes[0], relative_base, false);
                //println!("jz {} {}", condition, condition == 0);
                if condition == 0
                {
                    // index = values[&(index + 2)] as usize;
                    index = get_parameter(values, index + 2, modes[1], relative_base, false) as usize;
                }
                else
                {
                    index += 3;
                }
            },
            7 => // less than
            {
                let a = get_parameter(values, index + 1, modes[0], relative_base, false);
                let b = get_parameter(values, index + 2, modes[1], relative_base, false);
                let loc = get_parameter(values, index + 3, modes[2], relative_base, true) as usize;
                //println!("LT {} < {} {} => {}", a, b, a < b, loc);

                // values[&loc] = match a < b {
                //     true => 1,
                //     false => 0
                // };
                values.insert(loc, match a < b {
                    true => 1,
                    false => 0,
                });

                index += 4;
            },
            8 => // eq
            {
                let a = get_parameter(values, index + 1, modes[0], relative_base, false);
                let b = get_parameter(values, index + 2, modes[1], relative_base, false);
                let loc = get_parameter(values, index + 3, modes[2], relative_base, true) as usize;
                //println!("EQ {} == {} {} => {}", a, b, a == b, loc);

                // values[&loc] = match a == b {
                //     true => 1,
                //     false => 0
                // };
                values.insert(loc, match a == b {
                    true => 1,
                    false => 0,
                });

                index += 4;
            },
            9 =>
            {
                // adjusts the relative base by the value of the only parameter
                let adjust = get_parameter(values, index + 1, modes[0], relative_base, false);
                //println!("OP 9 rel base + {} ({})", adjust, relative_base + adjust);
                relative_base += adjust;

                index += 2;
            }
            99 => {
                // println!("EXIT");
                // process::exit(123);
                // exit
                break;
            },
            _ => {
                println!("Unknown opcode at index {}.", index);
                break;
            }
        }
    }
    println!("EXIT");
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

    // //println!("opcode {} -> param {}", opcode, param_val);
    while param_val > 0 {
        // //println!("param val {}", param_val);
        // either 1 or 0
        let digit = param_val % 10;
        param_val /= 10;
        // //println!("adding digit {}", digit);
        parameter_modes.push(digit as usize);
    }

    while parameter_modes.len() < min_len
    {
        // //println!("padding 0");
        // pad with zeros
        parameter_modes.push(0);
    }

    // //println!("modes: {:?}", parameter_modes);
    return parameter_modes;
}

fn get_parameter(values: &mut HashMap<usize, isize>, parameter_index: usize, parameter_mode: usize, mut relative_base: isize, is_write: bool) -> isize
{
    // gets the value of a parameter based on if it is' immediate or position parameter_mode
    match parameter_mode
    {
        0 => {
            ////println!("pos");
            // position parameter mode
            // use the value at that index as the location
            // then return the value at that location

            let loc = &(values[&parameter_index] as usize);

            if is_write
            {
                //println!("write: {}", loc);
                return *loc as isize;
            }

            if values.contains_key(loc)
            {
                //println!("pos ${} = {}", loc, values[loc]);
                return values[loc];
            }
            //println!("pos missing {}", loc);
            values.insert(*loc, 0);
            //println!("added {:?}", values);
            // default value is 0
            return 0;
        },
        1 => {
            ////println!("imm");
            // immediate
            // use the value at that index
            return values[&parameter_index];
        },
        2 =>
        {
            // relative mode
            let index = &((relative_base + values[&parameter_index]) as usize);
            //println!("rel {} + {} = {}", relative_base, values[&parameter_index], index);

            if is_write
            {
                //println!("write: {}", index);
                return *index as isize;
            }

            if values.contains_key(index)
            {
                return values[index];
            }
            values.insert(*index, 0);
            // default value is 0
            return 0;
            // return values[];
            // return index;
        }
        _ => {
            //println!("invald mode");
            return -1;
        }
    }
}