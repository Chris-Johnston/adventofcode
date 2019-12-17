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

    pub fn run_until_output(&mut self) -> isize
    {
        while !self.halt && self.outputs.len() < self.output_index + 1
        {
            self.run_single_command();
        }
        self.output_index = self.outputs.len();
        return self.outputs[self.outputs.len() - 1];
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

fn test_alignment_parameter()
{
    let map = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

    let mut intersections = Vec::new();
    let map_chars = &map.as_bytes();

    let width : usize = map.find('\n').expect("could not find end of line") + 1;
    // for (idx, c) in map_chars[width..map.len()-width].enumerate()
    for idx in width..map.len()-width
    {
        // let c = map_chars.nth(idx).expect("couldn't get index");
        let c = map_chars[idx];
        let idx = idx as usize;
        if c == b'#'
        {
            // compare left, right, up, down
            let left = map_chars[idx - 1] == b'#';
            let right = map_chars[idx + 1] == b'#';
            let up = map_chars[idx - width] == b'#';
            let down = map_chars[idx + width] == b'#';

            if left && right && up && down
            {
                intersections.push(idx);
                println!("intersection at {}", idx);
            }
        }
    }

    assert!(intersections.len() == 4);

    let mut total = 0;
    for idx in intersections
    {
        let x = idx % width;
        let y = idx / width;
        let val = x * y;
        println!("point idx {} -> {} {} = {}", idx, x, y, val);
        total += val;
    }

    assert!(76 == total);
}

fn get_alignment_parameter_sum(map: &String, map_chars: &[u8]) -> usize
{
    let mut intersections = Vec::new();

    let width : usize = map.find('\n').expect("could not find end of line") + 1;
    // for (idx, c) in map_chars[width..map.len()-width].enumerate()
    for idx in width..map.len()-width
    {
        // let c = map_chars.nth(idx).expect("couldn't get index");
        let c = map_chars[idx];
        let idx = idx as usize;
        if c == b'#'
        {
            // compare left, right, up, down
            let left = map_chars[idx - 1] == b'#';
            let right = map_chars[idx + 1] == b'#';
            let up = map_chars[idx - width] == b'#';
            let down = map_chars[idx + width] == b'#';

            if left && right && up && down
            {
                intersections.push(idx);
                println!("intersection at {}", idx);
            }
        }
    }

    let mut total = 0;
    for idx in intersections
    {
        let x = idx % width;
        let y = idx / width;
        let val = x * y;
        println!("point idx {} -> {} {} = {}", idx, x, y, val);
        total += val;
    }

    total
}

fn main()
{
    part_1();
    part_2();
}

fn part_2()
{
    let input_file = "/home/chris/Git/adventofcode/2019/17/input.txt";

    // set up the initial state of the computer
    let mut cpu = IntcodeComputer {
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
    cpu.load_input_file(input_file);

    // wake up robot
    cpu.memory.insert(0, 2);

    let all_input =
"A,B,B,C,B,C,B,C,A,A
L,6,R,8,L,4,R,8,L,12
L,12,R,10,L,4
L,12,L,6,L,4,L,4
n
".as_bytes();

    // for c in all_input.as_bytes()
    for idx in 0..all_input.len()
    {
        let c = all_input[idx] as isize;
        cpu.inputs.push_back(c);
        println!("input: {}", c);
    }

    let mut map_str = String::new();

    while !cpu.halt
    {
        let out = cpu.run_until_output() as u32;
        println!("o: {}", out);
        let c = char::from_u32(out)
            .expect("Couldn't parse the output to a char.");
        print!("{}", c);

        map_str.push(c);
    }
}

fn part_1()
{
    test_alignment_parameter();

    let input_file = "/home/chris/Git/adventofcode/2019/17/input.txt";

    // set up the initial state of the computer
    let mut cpu = IntcodeComputer {
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
    cpu.load_input_file(input_file);

    let mut map_str = String::new();

    while !cpu.halt
    {
        let out = cpu.run_until_output() as u32;
        let c = char::from_u32(out)
            .expect("Couldn't parse the output to a char.");
        print!("{}", c);
        map_str.push(c);
    }
    
    let result = get_alignment_parameter_sum(&map_str, map_str.as_bytes());
    println!("sum {}", result);
}