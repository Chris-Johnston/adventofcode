// day 11
// robot needs to be able to move on grid of square panels
// detect current color
// paint panel black or white
// 0 - black
// 1 - white

// first will output a value indicating the color
// to paint the panel the robot is over
// 0 = black, 1 = white
// second outputs a value indicating the direction the robot
// should turn, 0 = left 90 deg, 1 = right 90 deg
// after turn, should always move forward exactly one panel, starts facing up

use std::env;
use std::fs;
use std::vec;
use std::io;
use std::collections::VecDeque;
use std::slice;
use std::collections::HashMap;

pub struct IntcodeComputer
{
    memory: HashMap<usize, isize>,
    instruction_pointer: usize,
    relative_base: isize,
    inputs: Vec<isize>,
    outputs: Vec<isize>,
    stdin_input: bool,
    halt: bool,
}

impl IntcodeComputer
{
    pub fn load_input_file(&mut self, input_file: &str)
    {
        let content = fs::read_to_string(input_file)
            .expect("Failed to read the file.");
        let values: Vec<isize> = content
            .split(",")
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect();
        self.memory = HashMap::new();
        for (index, value) in values.iter().enumerate()
        {
            self.memory.insert(index, *value);
        }
    }

    pub fn push_input(&mut self, isize input)
    {
        self.inputs.push(input);
    }

    fn get_input(&mut self) -> isize
    {
        if self.stdin_input
        {
            // read input from stdin
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            let value : isize = input_text
                .trim()
                .parse()
                .expect("couldn't parse the input");
            value
        }
        else
        {
            self.inputs.pop()
                .expect("input stack was empty")
        }
    }

    pub fn run_single_command(&mut self)
    {
        let opcode = self.memory[&self.instruction_pointer] % 100;
        let modes = self.get_parameter_modes(self.memory[&self.instruction_pointer], 4);

        match opcode
        {
            1 => {
                // add
                let location = self.get_parameter(index + 3, modes[2], self.relative_base, true) as usize;
                let a = self.get_parameter(index + 1, modes[0], self.relative_base, false);
                let b = self.get_parameter(index + 2, modes[1], self.relative_base, false);
                self.memory.insert(location, a + b);

                // increment index for all parameters
                self.instruction_pointer += 4;
            },
            2 => {
                // mul
                let location = self.get_parameter(self.instruction_pointer + 3, modes[2], self.relative_base, true) as usize;
                let a = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                let b = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false);
                // println!("mul {} * {} => {}", a, b, location);
                self.memory.insert(location, a * b);
                
                // increment index for all parameters
                index += 4;
            },
            3 => {
                // store single int from the console input
                let location = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, true) as usize;
                
                self.memory.insert(location, value);
                
                self.instruction_pointer += 2;
            },
            4 => {
                // output
                let value = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                self.outputs.push(value);
                println!("OUTPUT: {}", value);

                self.instruction_pointer += 2;
            },
            5 => // jump non zero 
            {
                // if first param non zero
                // set instruction pointer to the value from the second parameter
                let condition = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                // println!("jnz {} {}", condition, condition != 0);
                if condition != 0
                {
                    self.instruction_pointer = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false) as usize;
                }
                else
                {
                    self.instruction_pointer += 3;
                }
            },
            6 => // jump zero
            {
                // if first param zero
                // set instruction pointer to the value from the second parameter
                let condition = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                // println!("jz {} {}", condition, condition == 0);
                if condition == 0
                {
                    // index = values[&(index + 2)] as usize;
                    self.instruction_pointer = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false) as usize;
                }
                else
                {
                    self.instruction_pointer += 3;
                }
            },
            7 => // less than
            {
                let a = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                let b = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false);
                let loc = self.get_parameter(self.instruction_pointer + 3, modes[2], self.relative_base, true) as usize;
                // println!("LT {} < {} {} => {}", a, b, a < b, loc);

                self.memory.insert(loc, match a < b {
                    true => 1,
                    false => 0,
                });

                self.instruction_pointer += 4;
            },
            8 => // eq
            {
                let a = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                let b = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false);
                let loc = self.get_parameter(self.instruction_pointer + 3, modes[2], self.relative_base, true) as usize;
                self.memory.insert(loc, match a == b {
                    true => 1,
                    false => 0,
                });

                self.instruction_pointer += 4;
            },
            9 =>
            {
                // adjusts the relative base by the value of the only parameter
                let adjust = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                // println!("OP 9 rel base + {} ({})", adjust, self.relative_base + adjust);
                self.relative_base += adjust;

                self.instruction_pointer += 2;
            }
            99 => {
                println!("EXIT");
                self.halt = true;
                // exit
            },
            _ => {
                println!("Unknown opcode at index {}.", self.instruction_pointer);
                self.halt = true;
            }index
        }
    }

    fn get_parameter_modes(opcode: isize, min_len: usize) -> Vec<usize> {
        let mut parameter_modes = Vec::new();
        let mut param_val = opcode / 100;

        while param_val > 0 {
            let digit = param_val % 10;
            param_val /= 10;
            parameter_modes.push(digit as usize);
        }
    
        while parameter_modes.len() < min_len
        {
            // pad with zeros
            parameter_modes.push(0);
        }
    
        return parameter_modes;
    }

    fn get_parameter(&mut self, parameter_index: usize, parameter_mode: usize, mut relative_base: isize, is_write: bool) -> isize
    {
        // gets the value of a parameter based on if it is immediate or position parameter_mode
        match parameter_mode
        {
            0 => {
                // position parameter mode
                // use the value at that index as the location
                // then return the value at that location
                let loc = &(self.memory[&parameter_index] as usize);

                if is_write
                {
                    // println!("write: {}", loc);
                    return *loc as isize;
                }

                if values.contains_key(loc)
                {
                    // println!("pos ${} = {}", loc, values[loc]);
                    return self.memory[loc];
                }
                self.memory.insert(*loc, 0);
                // default value is 0
                return 0;
            },
            1 => {
                return self.memory[&parameter_index];
            },
            2 =>
            {
                // relative mode
                let index = &((relative_base + self.memory[&parameter_index]) as usize);

                if is_write
                {
                    return *index as isize;
                }

                if self.memory.contains_key(index)
                {
                    return self.memory[index];
                }
                self.memory.insert(*index, 0);
                // default value is 0
                return 0;
            }
            _ => {
                println!("invald mode");
                return -1;
            }
        }
    }
}



fn main()
{
    let input_file = "/home/chris/Git/adventofcode/2019/11/input.txt";
    let mut cpu = IntcodeComputer {
        memory: HashMap::new(),
        instruction_pointer: 0,
        relative_base: 0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        stdin_input: false,
        halt: false,
    };

    cpu.load_input_file(input_file);
    
    while !cpu.halt
    {
        cpu.run_single_command();
    }
}
