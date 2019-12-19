// day 13

use std::env;
use std::fs;
use std::vec;
use std::io;
use std::collections::VecDeque;
use std::slice;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::thread::sleep_ms;
use std::char;

#[derive(Clone, Debug)]
pub struct IntcodeComputer
{
    memory: HashMap<usize, isize>,
    instruction_pointer: usize,
    relative_base: isize,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
    stdin_input: bool,
    stdin_to_byte: bool,
    stdout_output: bool,
    halt: bool,
    output_index: usize,
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

    pub fn push_input(&mut self, input: isize)
    {
        self.inputs.push_back(input);
    }

    fn get_input(&mut self) -> isize
    {
        if self.stdin_input
        {
            println!("{} INPUT: ", self.instruction_pointer);
            // read input from stdin
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            if self.stdin_to_byte
            {
                let text = input_text;
                if text.len() == 0
                {
                    return 10 // newline
                }
                text.as_bytes()[0] as isize
            }
            else
            {
                let value : isize = input_text
                    .trim()
                    .parse()
                    .expect("couldn't parse the input");
                value
            }
        }
        else
        {
            let v = self.inputs.pop_front()
                .expect("input stack was empty");
            v
        }
    }

    pub fn run_until_output(&mut self) -> Option<isize>
    {
        //  && self.outputs.len() < self.output_index + 1
        // let start_len = self.outputs.len();
        while !self.halt
        {
            // println!("halt {} output len {} out index {}", self.halt, self.outputs.len(), self.output_index);
            self.run_single_command();

            // if self.outputs.len() > start_len
            // {
            //     self.output_index = self.outputs.len() - 1;
            //     return Some(self.outputs[self.output_index]);
            // }

            if !self.outputs.is_empty()
            {
                return self.outputs.pop();
            }
        }
        None
    }

    pub fn run_single_command(&mut self)
    {
        let instruction = self.memory[&self.instruction_pointer];
        let opcode = instruction % 100;
        let modes = IntcodeComputer::get_parameter_modes(instruction, 4);

        match opcode
        {
            1 => {
                // add
                let location = self.get_parameter(self.instruction_pointer + 3, modes[2], self.relative_base, true) as usize;
                let a = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                let b = self.get_parameter(self.instruction_pointer + 2, modes[1], self.relative_base, false);
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
                self.instruction_pointer += 4;
            },
            3 => {
                // store single int from the console input
                let location = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, true) as usize;
                let value = self.get_input();
                
                self.memory.insert(location, value);
                
                self.instruction_pointer += 2;
            },
            4 => {
                // output
                let value = self.get_parameter(self.instruction_pointer + 1, modes[0], self.relative_base, false);
                self.outputs.push(value);

                if self.stdout_output
                {
                    println!("OUTPUT: {}", value);
                }

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
            }
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

                if self.memory.contains_key(loc)
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
    part_1();
}

fn part_1()
{
    let input_file = "/home/chris/Git/adventofcode/2019/19/day19/input.txt";

    // set up the initial state of the computer
    let mut original_cpu = IntcodeComputer {
        memory: HashMap::new(),
        instruction_pointer: 0,
        relative_base: 0,
        inputs: VecDeque::new(),
        outputs: Vec::new(),
        stdin_input: false,
        stdin_to_byte: false,
        stdout_output: false,
        halt: false,
        output_index: 0,
    };
    original_cpu.load_input_file(input_file);


    // the program uses two input instructions to request the
    // X Y, which are >= 0
    // will output 0 if stationary, 1 = pulled

    // scan 50 x 50 area, how many points are affected

    let mut map = Vec::new();
    let mut count = 0;

    let upper_bound = 2000;
    // let find_dimension = 100;
    let find_dimension = 99;
    // let min = 980;
    let min = 930;
    let scale = 20;
    
    for x in min..upper_bound
    {
        for y in min..upper_bound
        {
            // skip over the first 500
            if x < min || y < min
            {
                map.push('X');
                if x % scale == 0 && y % scale == 0
                {
                    // print!("X");
                }
                continue;
            }

            // skip below slope
            let slope = x as f32 / y as f32;
            // println!("slope {}", slope);
            if slope < 0.65 || slope > 0.85
            {
                map.push('_');
                if x % scale == 0 && y % scale == 0
                {
                    print!("_");
                }
                continue;
            }

            // can I keep resetting it? so that it doesn't halt?
            // cpu.load_input_file(input_file);
            // cpu.instruction_pointer = 0;
            let mut cpu = original_cpu.clone();

            cpu.inputs.push_back(x);
            cpu.inputs.push_back(y);
            // println!("x: {} y: {}", x, y);

            let out = cpu.run_until_output()
                .expect("couldn't get output") as u32;
            match out {
                0 => {
                    // stationary
                    if x % scale == 0 && y % scale == 0
                    {
                        print!(".");
                    }
                    map.push('.');
                },
                1 => {
                    if x % scale == 0 && y % scale == 0
                    {
                        print!("#");
                    }
                    map.push('#');
                    count += 1;

                    let mut cpu = original_cpu.clone();

                    // part 2
                    // let right_index = (x + find_dimension + y * upper_bound) as usize;
                    // let bot_index = (x + (y + find_dimension) * upper_bound) as usize;
                    cpu.inputs.push_back(x + find_dimension);
                    cpu.inputs.push_back(y);
                    let right = cpu.run_until_output()
                        .expect("couldn't get output right") as u32;
                    if right == 0
                    {
                        continue;
                    }

                    let mut cpu = original_cpu.clone();

                    cpu.inputs.push_back(x);
                    cpu.inputs.push_back(y + find_dimension);
                    let down = cpu.run_until_output()
                        .expect("couldn't get output down") as u32;
                    if down == 0
                    {
                        continue;
                    }

                    // POINT FOUND 979 1328 = 9791328
                    println!("\nPOINT FOUND {} {} = {}", x, y, x * 10000 + y);
                    return;
                },
                _ => {},
            };
        }
        if x % scale == 0
        {
            println!("");
        }
    }

    // start from large to small

    println!("count: {}", count);

    // part 2
    for x in min..(upper_bound - find_dimension)
    {
        for y in min..(upper_bound - find_dimension)
        {
            let index = (x + y * upper_bound) as usize;
            print!("{}", map[index]);
            if map[index] != '#'
            {
                continue;
            }

            // current index is '#'
            // get the index of the point find_dimension to the right
            let right_index = (x + find_dimension + y * upper_bound) as usize;
            let bot_index = (x + (y + find_dimension) * upper_bound) as usize;

            if map[right_index] == '#' && map[bot_index] == '#'
            {
                println!("x {} y {}", x, y);
                let result = x * 10000 + y;
                println!("result {}", result);
                return;
            }
        }
        println!();
    }
}