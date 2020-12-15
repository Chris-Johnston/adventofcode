use std::fs;
use std::time::{Duration, Instant};
use std::collections::HashMap;

static REAL_INPUT: &str = 
"0,8,15,2,12,1,4";

const TARGET_NUMBER: usize = 2020;
const BIG_TARGET_NUMBER: usize = 30000000;

static EXAMPLE_INPUT: &str = 
"0,3,6";

const EXAMPLE_ANSWER_1: usize = 436;
const EXAMPLE_ANSWER_2: usize = 2;

fn main() {
    let input = REAL_INPUT;

    // let example_solution = solution(EXAMPLE_INPUT)
    //     .expect("no result");
    
    // println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    // assert!(EXAMPLE_ANSWER_1 == example_solution);

    // let answer = solution(&input)
    //     .expect("no result");
    
    // println!("Answer Part 1 {}", answer);
    // assert!(answer == 289);

    // part 2

    let start = Instant::now();
    let answer = algo(input, BIG_TARGET_NUMBER);
    let duration = start.elapsed();
    println!("answer {} in {:?}", answer, duration);

    let test_cases = vec![
        ("0,3,6", 2020, 436),
        ("1,3,2", 2020, 1),
        ("2,1,3", 2020, 10),
        ("1,2,3", 2020, 27),
        ("2,3,1", 2020, 78),
        ("3,2,1", 2020, 438),
        ("3,1,2", 2020, 1836),
        ("0,3,6", 30000000, 175594),
        ("1,3,2", 30000000, 2578),
        ("2,1,3", 30000000, 3544142),
        ("1,2,3", 30000000, 261214),
        ("2,3,1", 30000000, 6895259),
        ("3,2,1", 30000000, 18),
        ("3,1,2", 30000000, 362),
    ];
    for (input, iter, expected) in test_cases
    {
        println!("running {}", input);
        let start = Instant::now();
        let real = algo(input, iter);
        let duration = start.elapsed();
        println!("{} {} == {} (expected: {}) in {:?}", iter, input, real, expected, duration);
        assert!(expected == real);
    }


    // let example_solution = solution_part2(EXAMPLE_INPUT)
    //     .expect("no result");

    // println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    // assert!(EXAMPLE_ANSWER_2 == example_solution);

    // let answer = solution_part2(&input)
    //     .expect("no result");
    
    // println!("Answer Part 2 {}", answer);
    // // assert!(answer == 1355323200);
}

fn solution(input: &str) -> Option<usize>
{
    let input : Vec<usize> = input.split(",")
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();

    // works just as long as it isn't the same as the first number in the input
    let mut last_number_spoken = 0;
    let mut number_index = 0;
    let mut first_time = false;
    let mut last_spoken_time = 0;

    let mut number_memory = HashMap::new();

    // starting numbers
    for (i, x) in input.iter().enumerate()
    {
        number_memory.insert(*x, i + 1);

        last_number_spoken = *x;
        first_time = true;
    }

    println!("starting {:?}", number_memory);
    println!("last number {} first_time {}", last_number_spoken, first_time);
    
    for turn in input.len() + 1..BIG_TARGET_NUMBER + 1
    {
        // println!("last number {}", last_number_spoken);
        if !first_time
        {
            // the last number spoken was already spoken
            let delta = turn - 1 - last_spoken_time;
            // println!("\tdelta {}", delta);
            // println!("\t{} - {}", turn, number_memory[&last_number_spoken]);
            last_number_spoken = delta;
        }
        else
        {
            // first time, zero
            last_number_spoken = 0;
        }

        if turn % (BIG_TARGET_NUMBER / 100) == 0
        {
            println!("turn {} {}", turn, last_number_spoken);
        }
        // update last number spoken
        if let Some(x) = number_memory.insert(last_number_spoken, turn)
        {
            last_spoken_time = x;
            first_time = true;
        }
        else
        {
            first_time = false;
        }
    }

    
    Some(last_number_spoken)
}

fn solution_part2(input: &str) -> Option<usize>
{
    None
}

fn algo(input: &str, target: usize) -> usize
{
    let input : Vec<usize> = input.split(",")
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();
    
    // works just as long as it isn't the same as the first number in the input
    let mut last_number_spoken = 0;
    let mut first_time = false;
    let mut last_spoken_time = 0;

    let mut number_memory = HashMap::new();

    // starting numbers
    for (i, x) in input.iter().enumerate()
    {
        number_memory.insert(*x, i + 1);

        last_number_spoken = *x;
        first_time = true;
    }

    for turn in input.len() + 1..target + 1
    {
        // println!("last number {}", last_number_spoken);
        if !first_time
        {
            // the last number spoken was already spoken
            let delta = turn - 1 - last_spoken_time;
            // println!("\tdelta {}", delta);
            // println!("\t{} - {}", turn, number_memory[&last_number_spoken]);
            last_number_spoken = delta;
        }
        else
        {
            // first time, zero
            last_number_spoken = 0;
        }

        // update last number spoken
        first_time = true;
        if let Some(x) = number_memory.insert(last_number_spoken, turn)
        {
            last_spoken_time = x;
            first_time = false;
        }
    }

    return last_number_spoken;
}