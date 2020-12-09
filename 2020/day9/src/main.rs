use std::fs;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

static INPUT_FILE: &str = "input.txt";


const EXAMPLE_PREAMBLE_LEN: usize = 5;
static EXAMPLE_INPUT: &str = 
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";


const PREAMBLE_LEN: usize = 25;
const EXAMPLE_ANSWER_1: usize = 127;
const EXAMPLE_ANSWER_2: usize = 62;

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let example_solution = solution(EXAMPLE_INPUT, EXAMPLE_PREAMBLE_LEN)
        .expect("no result");
    
    println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    assert!(EXAMPLE_ANSWER_1 == example_solution);

    let answer = solution(&input, PREAMBLE_LEN)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    assert!(answer != 29513374448792); // was using a stack when I needed a queue
    assert!(answer == 3199139634);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT, EXAMPLE_ANSWER_1)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input, answer)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    // assert!(answer == 1355323200);
}

fn solution(input: &str, preamble_len: usize) -> Option<usize>
{
    let mut data : VecDeque<usize> = input
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    // set for quick lookup, Vec for ordered search
    // let mut current_set = HashSet::new();
    let mut current_vals = VecDeque::new();

    // insert the preamble
    for _ in 0..preamble_len
    {
        let x = data.pop_front()
            .expect("Failed to pop data in preambles");
        current_vals.push_back(x);
    }

    println!("preamble {:?}", current_vals);
    
    // for remaining data
    while !data.is_empty()
    {
        let sum = data.pop_front().expect("failed to pop data");

        let mut has_matching = false;
        // find pairs of numbers such that they equal x
        // first one that breaks this is returned
        for x in 0..current_vals.len()
        {
            for y in 0..current_vals.len()
            {
                let val_x = current_vals[x];
                let val_y = current_vals[y];
                if (val_x + val_y) == sum
                {
                    has_matching = true;
                    break;
                }
            }

            if has_matching
            {
                break;
            }
        }

        if !has_matching
        {
            return Some(sum);
        }

        // this has a match
        // make room for the new value
        let removed = current_vals.pop_front()
            .expect("failed to pop off current values");
        // current_set.remove(&removed);
        // insert new value
        current_vals.push_back(sum);
        // current_set.insert(sum);
    }

    None
}

fn solution_part2(input: &str, target: usize) -> Option<usize>
{
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    // must fine the contiguous set of at least two numbers which sum to the invalid
    // number from step 1

    // plan is to iterate from 0..len
    // as the starting index
    // and start to sum the numbers until the sum is >= the target
    // if over the target, choose the next index
    // this considers the possibility of multiple sizes
    for x in 0..data.len()
    {
        let val_x = data[x];
        let mut sum = val_x;

        let mut i = x + 1;
        while sum < target
        {
            sum += data[i];
            i += 1;
        }

        if sum == target
        {
            // we found the range
            // and it's [x..i)
            // so copy that and find the min and max
            let slice = Vec::from_iter(data[x..i].iter().cloned());

            let min = slice.iter().min().expect("no min??");
            let max = slice.iter().max().expect("no max??");

            return Some(min + max);
        }
    }
    

    None
}