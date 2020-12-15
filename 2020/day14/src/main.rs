#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

use std::fs;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

static EXAMPLE_PART2: &str =
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

const EXAMPLE_ANSWER_1: usize = 165;
const EXAMPLE_ANSWER_2: usize = 208;

const LEN: isize = 35;

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let example_solution = solution(EXAMPLE_INPUT)
        .expect("no result");
    
    println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    assert!(EXAMPLE_ANSWER_1 == example_solution);

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    assert!(answer == 7440382076205);
    assert!(answer < 26838140637731);

    // part 2
    let example_solution = solution_part2(EXAMPLE_PART2)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    // assert!(answer == 1355323200);
    assert!(answer < 1591421495707984);
}

fn solution(input: &str) -> Option<usize>
{
    lazy_static! {
        static ref regex_mem: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    let mut overlay = 0;
    let mut mask = 0;

    let mut memory = HashMap::new();

    for line in input.lines()
    {
        println!("line {}", line);
        if regex_mem.is_match(line)
        {
            // mem(e) line
            let caps = regex_mem.captures(line).unwrap();
            // println!("caps {:?}", caps);
            let location = caps.get(1).unwrap().as_str()
                .parse::<usize>()
                .expect("failed to parse location");
            let data = caps.get(2).unwrap().as_str()
                .parse::<usize>()
                .expect("failed to parse data");

            let applied_data = (data & mask) | overlay;

            memory.insert(location, applied_data);
        }
        else
        {
            mask = 0;
            overlay = 0;

            let mask_text = line.split(' ').nth(2).expect("failed to get mask value");

            for (i, x) in mask_text.chars().enumerate()
            {
                let bit = 35 - i;
                if x == 'X'
                {
                    // add another bit to the real mask
                    mask |= 1 << bit;
                }
                else
                {
                    // add another bit to the overlap
                    let x = x.to_digit(10).unwrap() as usize;
                    overlay |= x << bit;
                }
            }
        }
    }

    println!("generated bit mask {:0b} and overlay {:0b}", mask, overlay);
    
    // for each of the X's use this to form the real bitmask
    // and then for the non X's, construct the overlay which will be ORed

    // sum all values in memory
    let sum : usize = memory.values().sum();

    Some(sum)
}

fn solution_part2(input: &str) -> Option<usize>
{
    lazy_static! {
        static ref regex_mem: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    let mut overlay = 0;
    let mut mask = 0;
    let mut mask_weight = 0;

    let mut memory = HashMap::new();

    for line in input.lines()
    {
        println!("line {}", line);
        if regex_mem.is_match(line)
        {
            // mem(e) line
            let caps = regex_mem.captures(line).unwrap();
            // println!("caps {:?}", caps);
            let location = caps.get(1).unwrap().as_str()
                .parse::<usize>()
                .expect("failed to parse location");
            let data = caps.get(2).unwrap().as_str()
                .parse::<usize>()
                .expect("failed to parse data");

            let applied_data = (data & mask) | overlay;

            // each bit in the bitmask modifies the corresponding bit
            // of dest address in the following way

            // if 0, unchanged, 1 overwritten with 1
            let location = location | overlay;

            // get the hamming weight
            let ham = mask_weight;

            println!("ham {}", ham);

            let mut floaty_bits = Vec::new();

            for bit in 0..36
            {
                // if bit is 1 in mask, add 0 and 1 version
                if 1 << bit & mask > 0
                {
                    println!("bit {}", bit);
                    floaty_bits.push(bit);
                }
            }

            let kmax = floaty_bits.clone().len();
            for k in 0..kmax + 1
            {
                for permutation in floaty_bits 
                    .clone()
                    .into_iter().permutations(k)
                {
                    println!("permutation {:?}", permutation);
                    // flip those bits
                    let mut floaty_location = location;
                    for b in &permutation
                    {
                        floaty_location |= 1 << b;
                    }

                    println!("inserting {:0b} into {:0b}", applied_data, floaty_location);

                    memory.insert(floaty_location, applied_data);

                    let mut remove_mask = 0;
                    for b in &permutation
                    {
                        remove_mask |= 1 << b;
                    }

                    println!("remove mask {:0b}", remove_mask);
                    println!("aaa {:037b}", 0b111_11111111_11111111_11111111_11111111 ^ remove_mask);
                    println!("fff {:037b}", floaty_location);
                    
                    floaty_location &= 0b111_11111111_11111111_11111111_11111111 ^ remove_mask;

                    println!("fff {:037b}", floaty_location);

                    println!("inserting {:0b} into {:0b}", applied_data, floaty_location);


                    // maybe it has to consider the current value in this as well
                    memory.insert(floaty_location, applied_data);
                }
            }

            // brute force it, but just use a better way to do it
            

            // memory.insert(location, (applied_data, ham));

            // just the hamming weight does not work because parts will overwrite each other

            // just loop through all values from 0..inverted_mask to get combinations
            // let inverted_mask = 0b111_11111111_11111111_11111111_11111111 ^ mask;
            // println!("inverted mask {:0b}", inverted_mask);
            // for x in 0..inverted_mask
            // {
            //     let floating_location = location | (x & mask);
            //     memory.insert(floating_location, applied_data);
            // }
        }
        else
        {
            mask = 0;
            overlay = 0;
            mask_weight = 0;

            let mask_text = line.split(' ').nth(2).expect("failed to get mask value");

            for (i, x) in mask_text.chars().enumerate()
            {
                let bit = 35 - i;
                if x == 'X'
                {
                    // add another bit to the real mask
                    mask |= 1 << bit;
                    mask_weight += 1;
                }
                else
                {
                    // add another bit to the overlap
                    let x = x.to_digit(10).unwrap() as usize;
                    overlay |= x << bit;
                }
            }
        }
    }

    println!("generated bit mask {:0b} and overlay {:0b}", mask, overlay);
    
    // for each of the X's use this to form the real bitmask
    // and then for the non X's, construct the overlay which will be ORed

    println!("memory {:?}", memory);

    // sum all values in memory
    //let mut sum = 0;
    // for (val, weight) in memory.values()
    // {
    //     if *val != 0
    //     {
    //         println!("v, w {} {}", val, weight);
    //     }
    //     let v = val * 2usize.pow(*weight);
    //     sum += v;
    // }
    let sum = memory.values().sum();

    Some(sum)
}