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

pub struct IntcodeComputer
{
    memory: HashMap<usize, isize>,
    instruction_pointer: usize,
    relative_base: isize,
    inputs: Vec<isize>,
    outputs: Vec<isize>,
    stdin_input: bool,
    stdout_output: bool,
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

    pub fn push_input(&mut self, input: isize)
    {
        self.inputs.push(input);
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
        let instruction = self.memory[&self.instruction_pointer];
        let opcode = instruction % 100;
        let modes = IntcodeComputer::get_parameter_modes(instruction, 4);

        // println!("instruction {}", instruction);

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

fn coord_to_index(coord: (isize, isize)) -> usize
{
    ((coord.0 + 250) + 500 * (coord.1 + 250)) as usize
}

fn add_tile(map: &mut Vec<isize>, x: isize, y: isize, tile_id: isize)
{
    let index = coord_to_index((x, y));
    map[index] = tile_id;
}

fn main()
{
    let input_file = "/home/chris/Git/adventofcode/2019/13/input.txt";
    let mut cpu = IntcodeComputer {
        memory: HashMap::new(),
        instruction_pointer: 0,
        relative_base: 0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        stdin_input: false,
        stdout_output: true,
        halt: false,
    };

    cpu.load_input_file(input_file);

    // load 2 quarters
    cpu.memory.insert(0, 2);

    let mut output_index = 0;
    let mut x = 0;
    let mut y = 0;
    let mut tile_id = 0;

    let mut map = vec![0; (500 * 500)];
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut delay_speed = 0;

    while !cpu.halt
    {
        cpu.run_single_command();

        if cpu.outputs.len() >= output_index + 1
        {
            match output_index % 3
            {
                0 =>
                {
                    x = cpu.outputs[output_index];
                    println!("x: {}", x);
                },
                1 =>
                {
                    y = cpu.outputs[output_index];
                    println!("y: {}", y);
                },
                2 =>
                {
                    tile_id = cpu.outputs[output_index];
                    println!("tile_id: {}", tile_id);

                    if x == -1 && y == 0
                    {
                        // new score to show in the display
                        println!("score: {}", tile_id);
                        delay_speed = 20;
                    }
                    else
                    {
                        // add the tile
                        add_tile(&mut map, x, y, tile_id);
                    }

                    // clear screen
                    print!("{}[2J", 27 as char);
                    for yi in 0..23
                    {
                        for xi in 0..40
                        {
                            let index = coord_to_index((xi, yi));
                            let value = map[index];
                            match value
                            {
                                1 => print!("X"),
                                2 => print!("o"),
                                3 =>
                                {
                                    print!("=");
                                    paddle_x = xi;
                                },
                                4 => 
                                {
                                    print!("B");
                                    ball_x = xi;
                                },
                                _ => print!(" "),
                            }
                        }
                        println!("");
                    }

                    // add input
                    cpu.inputs.push(
                        match ball_x.cmp(&paddle_x)
                        {
                            Ordering::Less => -1,
                            Ordering::Greater => 1,
                            Ordering::Equal => 0,
                        }
                    );

                    sleep_ms(delay_speed);
                },
                _ => {},
            }
            output_index += 1;
        }
    }

    let mut count = 0;
    for x in map
    {
        if x == 2
        {
            count += 1;
        }
    }
    println!("count: {}", count);
}