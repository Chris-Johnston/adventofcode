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
use rand;

#[derive(Clone, Debug)]
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

fn coord_to_index(coord: (isize, isize)) -> usize
{
    ((coord.0 + 250) + 500 * (coord.1 + 250)) as usize
}

fn add_tile(map: &mut Vec<isize>, x: isize, y: isize, tile_id: isize)
{
    let index = coord_to_index((x, y));
    map[index] = tile_id;
}

fn add_direction(direction: isize, x: isize, y: isize) -> (isize, isize)
{
    match direction
    {
        1 =>
        {
            // north
            (x, y + 1)
        },
        2 =>
        {
            // south
            (x, y - 1)
        },
        3 =>
        {
            // west
            (x - 1, y)
        },
        4 =>
        {
            // east
            (x + 1, y)
        },
        _ => (-1, -1),
    }
}

fn get_next_directions(dir: isize, next : &mut Vec<isize>)
{
    next.clear();
    match dir
    {
        1 =>
        {
            if rand::random()
            {
                next.push(3);
                next.push(4);
            }
            else
            {
                next.push(3);
                next.push(4);
            }
            next.push(2);
        },
        2 =>
        {
            if rand::random()
            {
                next.push(3);
                next.push(4);
            }
            else
            {
                next.push(3);
                next.push(4);
            }
            next.push(1);
        },
        3 =>
        {
            if rand::random()
            {
                next.push(1);
                next.push(2);
            }
            else
            {
                next.push(1);
                next.push(2);
            }
            next.push(4);
        },
        4 =>
        {
            if rand::random()
            {
                next.push(1);
                next.push(2);
            }
            else
            {
                next.push(1);
                next.push(2);
            }
            next.push(3);
        },
        _ => {}
    };
}

#[derive(Debug)]
struct Node
{
    // children: Vec<Node>,
    point_x: isize,
    point_y: isize,
    depth: isize,
    direction: isize,
    computer: IntcodeComputer,
}

// impl Node
// {
//     pub fn new() -> Node
//     {
//         Node {
//             point_x: 0,
//             point_y: 0,
//             depth: 0,
//             direction: 0,

//         }
//     }
// }

fn main()
{
    let input_file = "/home/chris/Git/adventofcode/2019/15/input.txt";

    // set up the initial state of the computer
    let mut cpu = IntcodeComputer {
        memory: HashMap::new(),
        instruction_pointer: 0,
        relative_base: 0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        stdin_input: false,
        stdout_output: false,
        halt: false,
        output_index: 0,
    };
    cpu.load_input_file(input_file);

    // initial settings
    // let mut output_index = 0;
    let mut map = vec![-1; 500 * 500];
    // let mut x = 0;
    // let mut y = 0;
    // let mut direction_attempt = 1;
    // let mut attempt_coord = (0, 0);
    // let mut start = Node::new();
    // start.point_x = 0;
    // start.point_y = 0;

    let mut step_queue = VecDeque::new();

    // add initial state to the queue
    step_queue.push_back(Node { point_x: 0, point_y: 0, depth: 1, direction: 1, computer: cpu.clone() });

    while !step_queue.is_empty()
    {
        let mut item = step_queue.pop_front().expect("empty");
        println!("x {} y {} depth {} dir {} out {}", item.point_x, item.point_y, item.depth, item.direction, item.computer.output_index);
        // println!("item {}    {}    {}", item.depth, item.point_x, item.point_y);

        // add direction to input
        item.computer.inputs.push(item.direction);

        let mut attempt_coord = add_direction(item.direction, item.point_x, item.point_y);
        // let mut attempt_coord = (item.point_x, item.point_y);
        // println!("dir {} from {} {} is {} {}", item.direction, item.point_x, item.point_y, attempt_coord.0, attempt_coord.1);

        let right_dir = match item.direction
        {
            1 => 4,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => -1,
        };
        let mut right_coord = add_direction(right_dir, attempt_coord.0, attempt_coord.1);
        let mut right_index = coord_to_index(right_coord);

        let mut ahead_coord = add_direction(item.direction, attempt_coord.0, attempt_coord.1);
        let mut ahead_index = coord_to_index(ahead_coord);

        let left_dir = match item.direction
        {
            4 => 1,
            3 => 2,
            1 => 3,
            2 => 4,
            _ => -1,
        };
        let mut left_coord = add_direction(left_dir, attempt_coord.0, attempt_coord.1);
        let mut left_index = coord_to_index(left_coord);

        let right_val = map[right_index];
        let left_val = map[left_index];
        let ahead_val = map[ahead_index];

        // run until the next output
        let response = item.computer.run_until_output();
        match response
        {
            0 => {
                // wall
                let index = coord_to_index(attempt_coord);
                map[index] = 0;
                print!("{:?} index {} is wall", attempt_coord, index);


                // add left and right to the queue
                // if right_val == -1
                // {
                //     step_queue.push_back(Node { point_x: right_coord.0, point_y: right_coord.1, depth: item.depth + 1, direction: right_dir, computer: cpu.clone() });
                // }

                // if left_val == -1
                // {
                //     step_queue.push_back(Node { point_x: left_coord.0, point_y: left_coord.1, depth: item.depth + 1, direction: left_dir, computer: cpu.clone() });
                // }
            },
            1 => {
                // move OK
                let index = coord_to_index(attempt_coord);
                map[index] = 1;

                println!("{:?} index {} is OK", attempt_coord, index);

                // add left right and behind to the queue
                // if right_val == -1
                // {
                //     step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: right_dir, computer: cpu.clone() });
                // }
                step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: right_dir, computer: item.computer.clone() });

                // if left_val == -1
                // {
                //     step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: left_dir, computer: cpu.clone() });
                // }
                step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: left_dir, computer: item.computer.clone() });

                // if ahead_val == -1
                // {
                //     step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: item.direction, computer: cpu.clone() });
                // }
                step_queue.push_back(Node { point_x: attempt_coord.0, point_y: attempt_coord.1, depth: item.depth + 1, direction: item.direction, computer: item.computer.clone() });
            },
            2 => {
                // found it
                let index = coord_to_index(attempt_coord);
                map[index] = 2;

                println!("found at point {:?}", attempt_coord);
                print!("FOUND {:?}", item);

                // 293 is wrong
                // 294
                // break;
            },
            _ => {},
        }

        // sleep_ms(500);
        for yi in -10..10
    {
        for xi in -10..10
        {
            let index = coord_to_index((xi, yi));
            let value = map[index];
                if xi == item.point_x && yi == item.point_y {
                    print!("D");
                }
                else if xi == 0 && yi == 0 {
                    print!("o")
                }
                else
                {
                    match value
                    {
                        0 => print!("#"),
                        1 => print!("."),
                        2 => print!("X"),
                        -1 => print!(" "),
                        _ => {},
                    };
                }
        }
        println!("");
    }
    }

    // part 2
        let mut count_no_oxygen = 0;
        for iter in 1..599
        {
            count_no_oxygen = 0;
            println!("iter {}", iter);
            let mut to_update_this_iter = Vec::new();

            for x in -30..30
            {
                for y in -30..30
                {
                    let index = coord_to_index((x, y));
                    let value = map[index];

                    match value
                    {
                        2 =>
                        {
                            let index_n = coord_to_index((x, y + 1));
                            if map[index_n] == 1
                            {
                                to_update_this_iter.push(index_n);
                            }
                            let index_e = coord_to_index((x + 1, y));
                            if map[index_e] == 1
                            {
                                to_update_this_iter.push(index_e);
                            }
                            let index_s = coord_to_index((x, y - 1));
                            if map[index_s] == 1
                            {
                                to_update_this_iter.push(index_s);
                            }
                            let index_w = coord_to_index((x - 1, y));
                            if map[index_w] == 1
                            {
                                to_update_this_iter.push(index_w);
                            }
                        },
                        1 =>
                        {
                            count_no_oxygen += 1;
                        }
                        _ => {},
                    }
                }
            }

            println!("count no oxy {} to update {}", count_no_oxygen, &to_update_this_iter.len());
            for idx in to_update_this_iter
            {
                map[idx] = 2;
                println!("updating index {}", idx);
            }

            if count_no_oxygen == 0
            {
                println!("iter {}", iter);
                break;
            }


            // print it
            for yi in -20..20
            {
                for xi in -20..20
                {
                    let index = coord_to_index((xi, yi));
                    let value = map[index];
                        if xi == 0 && yi == 0 {
                            print!("o")
                        }
                        else
                        {
                            match value
                            {
                                0 => print!("#"),
                                1 => print!("."),
                                2 => print!("X"),
                                -1 => print!(" "),
                                _ => {},
                            };
                        }
                }
                println!("");
            }

        }

        // 388 correct
        // 389 is too high

    // let mut current = start;

    // go south first
    // cpu.push_input(direction_attempt);

    // while !cpu.halt
    // {
    //     cpu.run_single_command();

    //     attempt_coord = add_direction(direction_attempt, x, y);

    //     // clear screen
    //     // print!("{}[2J", 27 as char);
    //     for yi in -10..10
    //     {
    //         for xi in -10..10
    //         {
    //             let index = coord_to_index((xi, yi));
    //             let value = map[index];
    //             if x == xi && y == yi
    //             {
    //                 print!("D")
    //             }
    //             else if xi == 0 && yi == 0 {
    //                 print!("o")
    //             }
    //             else
    //             {
    //                 match value
    //                 {
    //                     0 => print!("#"),
    //                     1 => print!("."),
    //                     2 => print!("X"),
    //                     -1 => print!(" "),
    //                     _ => {},
    //                 };
    //             }
    //         }
    //         println!("");
    //     }

    //     if cpu.outputs.len() >= output_index + 1
    //     {
    //         match cpu.outputs[output_index]
    //         {
    //             0 => {
    //                 // wall
    //                 let index = coord_to_index(attempt_coord);
    //                 map[index] = 0;

    //                 // use the next dir to the right
    //                 direction_attempt = match direction_attempt
    //                 {
    //                     1 => 4,
    //                     2 => 3,
    //                     3 => 1,
    //                     4 => 2,
    //                     _ => -1,
    //                 };

    //                 println!("dir {}", direction_attempt);
    //                 cpu.push_input(direction_attempt);
    //             },
    //             1 => {
    //                 let index = coord_to_index(attempt_coord);
    //                 map[index] = 1;

    //                 // moved OK
    //                 x = attempt_coord.0;
    //                 y = attempt_coord.1;// change dir
    //                 // if rand::random()
    //                 // {
    //                 //     let mut next_dirs = Vec::new();
    //                 //     get_next_directions(direction_attempt, &mut next_dirs);
    //                 //     for new_dir in next_dirs
    //                 //     {
    //                 //         let next_point = add_direction(new_dir, x, y);
    //                 //         let next_index = coord_to_index(next_point);
    //                 //         if map[next_index] != 0
    //                 //         {
    //                 //             //println!("using direction {}", new_dir);
    //                 //             direction_attempt = new_dir;
    //                 //             break;
    //                 //         }
    //                 //         else
    //                 //         {
    //                 //             // println!("wall");
    //                 //         }
    //                 //     }
    //                 // }

    //                 // use the next dir to the right
    //                 direction_attempt = match direction_attempt
    //                 {
    //                     4 => 1,
    //                     3 => 2,
    //                     1 => 3,
    //                     2 => 4,
    //                     _ => -1,
    //                 };

    //                 println!("direction {}", direction_attempt);
    //                 cpu.push_input(direction_attempt);
    //             },
    //             2 =>
    //             {
    //                 let index = coord_to_index(attempt_coord);
    //                 map[index] = 2;
    //                 // found
    //                 print!("Found at point {:?}", attempt_coord);
    //                 break;
    //             }
    //             _ => {},
    //         };
    //         output_index += 1;
    //         // sleep_ms(10);
    //     }
    // }

    for yi in -50..50
    {
        for xi in -50..50
        {
            let index = coord_to_index((xi, yi));
            let value = map[index];
                if xi == 0 && yi == 0 {
                    print!("o")
                }
                else
                {
                    match value
                    {
                        0 => print!("#"),
                        1 => print!("."),
                        2 => print!("X"),
                        -1 => print!(" "),
                        _ => {},
                    };
                }
        }
        println!("");
    }
}