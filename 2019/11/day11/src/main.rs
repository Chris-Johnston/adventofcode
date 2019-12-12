// day 11
// robot needs to be able to move on grid of square panels
// detect current color
// paint panel black or white
// 0 - black
// 1 - white

// first will output a value indicating the color
// to paint the panel the robot is over
// 0 = black, 1 = white
// second outputs a value indicating the direction_instruction the robot
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
        stdout_output: true,
        halt: false,
    };

    cpu.load_input_file(input_file);

    // create 500 x 500 empty grid of all black
    let mut grid = vec![-1; (500 * 500)];

    // first instruction is 0
    cpu.inputs.push(1);
    let mut output_index = 0;
    let mut direction_instruction = 0;
    let mut paint_instruction = 0;
    let mut coordinate = (0, 0);
    let mut facing_direction = 0;
    // 0 = up, 1 = right, 2 = down, 3 = left
    
    while !cpu.halt
    {
        cpu.run_single_command();

        // println!("len {} index is {}", cpu.outputs.len(), output_index);
        if cpu.outputs.len() >= output_index + 1
        {
            // first is value indicates the color to paint
            if output_index % 2 == 0
            {
                paint_instruction = cpu.outputs[output_index];
                // println!("paint: {}", paint_instruction);
                
                output_index += 1;
            }
            // second is the direction
            else
            {
                direction_instruction = cpu.outputs[output_index];
                // println!("dir: {}", direction_instruction);

                output_index += 1;

                // paint and move the robot

                // color the current cell
                let index = coord_to_index(coordinate);
                println!("coordinate {:?}", coordinate);
                grid[index] = paint_instruction;

                // move the robot
                // this can be done in a better way
                match facing_direction
                {
                    0 =>
                    {
                        // up
                        match direction_instruction
                        {
                            0 =>
                            {
                                // turn left, move forward
                                facing_direction = 3;
                                coordinate.0 -= 1;
                            },
                            1 =>
                            {
                                // turn right, move forward
                                facing_direction = 1;
                                coordinate.0 += 1;
                            },
                            _ => {},
                        }
                    },
                    1 =>
                    {
                        // right
                        match direction_instruction
                        {
                            0 =>
                            {
                                // turn up, move forward
                                facing_direction = 0;
                                coordinate.1 += 1;
                            },
                            1 =>
                            {
                                // turn down, move forward
                                facing_direction = 2;
                                coordinate.1 -= 1;
                            },
                            _ => {},
                        }
                    },
                    2 =>
                    {
                        // down
                        match direction_instruction
                        {
                            0 =>
                            {
                                // turn left, move forward
                                facing_direction = 1;
                                coordinate.0 += 1;
                            },
                            1 =>
                            {
                                // turn right, move forward
                                facing_direction = 3;
                                coordinate.0 -= 1;
                            },
                            _ => {},
                        }
                    },
                    3 =>
                    {
                        // left
                        match direction_instruction
                        {
                            0 =>
                            {
                                // turn left, move forward
                                facing_direction = 2;
                                coordinate.1 -= 1;
                            },
                            1 =>
                            {
                                // turn right, move forward
                                facing_direction = 0;
                                coordinate.1 += 1;
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
                println!("-> coordinate {:?}", coordinate);
                // add the current color
                let index = coord_to_index(coordinate);
                let mut color = grid[index];
                if color == -1
                {
                    color = 0;
                }
                cpu.inputs.push(color);
            }
        }
    }

    // get the number of panels painted
    let mut count = 0;
    // for x in grid
    // {
    //     if x != -1
    //     {
    //         count += 1;
    //     }
    // }

    for x in -50..50
    {
        for y in -50..50
        {
            let index = coord_to_index((x, y));
            let value = grid[index];
            if value == -1 || value == 0
            {
                print!(" ");
            }
            else
            {
                print!("{}", value);
            }
        }
        println!("");
    }


    println!("count of painted: {}", count);
}
