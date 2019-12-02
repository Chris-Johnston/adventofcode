use std::env;
use std::fs;
use std::vec;

fn main() {
    let input_file = "/home/chris/Git/adventofcode/2019/2/day2/input.txt";

    let content = fs::read_to_string(input_file).expect("Failed to read the file.");
    let mut values: Vec<usize> = content
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    println!("input");
    println!("{:?}", values);

    // part 1
    // 12 and 2
    values[1] = 12;
    values[2] = 2;

    // let content = "1,0,0,0,99"; // works
    // let content = "2,3,0,3,99"; // works
    // let content = "1,1,1,4,99,5,6,0,99"; // works

    let mut index = 0; // instruction pointer

    while index < values.len() {
        let opcode = values[index];

        match opcode {
            1 => {
                println!("add");
                // add
                let operands = (values[index + 1], values[index + 2], values[index + 3]);
                values[operands.2] = values[operands.0] + values[operands.1];
            }
            2 => {
                println!("mul");
                // mul
                let operands = (values[index + 1], values[index + 2], values[index + 3]);
                values[operands.2] = values[operands.0] * values[operands.1];
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

        // increment index by 4
        // don't need to worry about 99
        index += 4;
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
