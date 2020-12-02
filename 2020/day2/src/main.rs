#[macro_use] extern crate lazy_static;

use std::fs;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

fn main() {
    // what!??
    let example_input = 
"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    // count number of valid passwords
    // where 1-3 is range of occurances for the letter
    // and a is the letter
    // and the string is the input

    let example_result = get_valid_password_count(&example_input);
    println!("example result {}", 2);
    assert!(example_result == 2);

    let lines = 
        fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let answer = get_valid_password_count(&lines);
    assert!(answer != 389);
    assert!(answer == 500); // lol
    println!("answer {}", answer);


    println!("part 2");
    let example_result = get_valid_password_count_2(&example_input);
    println!("example {}", example_result);
    assert!(example_result == 1);

    let answer = get_valid_password_count_2(&lines);
    println!("answer 2 {}", answer);
    assert!(answer == 313);
}

fn get_valid_password_count(input: &str) -> usize
{
    input.lines()
        .filter(|&x| is_valid_password(x))
        .count()
}

fn is_valid_password_2(input: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }

    let result = RE.captures(input)
        .expect("no match! :(");
    let first_index : usize = result
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("couldn't parse min");

    let second_index : usize = result
        .get(2)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("couldn't parse min");

    // println!("first {} second {}", first_index, second_index);

    let c : char = result
        .get(3)
        .unwrap()
        .as_str()
        .chars()
        .next()
        .unwrap();
    
    // get count of c in pw, that's all we need
    let pw = result.get(4).unwrap().as_str();
    // println!("pw: {}", pw);
    let first = pw
        .chars().nth(first_index - 1)
        .expect("out of range for first index") == c;
    let pw = result.get(4).unwrap().as_str();
    let second = pw
        .chars().nth(second_index - 1)
        .expect("out of range index 2") == c;
    
    // forgot to add parens around this logic oops
    (first || second) && (first != second)
}


fn get_valid_password_count_2(input: &str) -> usize
{
    input.lines()
        .filter(|&x| is_valid_password_2(x))
        .count()
}

fn is_valid_password(input: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }

    let result = RE.captures(input)
        .expect("no match! :(");
    let min : usize = result
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("couldn't parse min");

    let max : usize = result
        .get(2)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("couldn't parse min");

    let c : char = result
        .get(3)
        .unwrap()
        .as_str()
        .chars()
        .next()
        .unwrap();
    
    // get count of c in pw, that's all we need
    let count = result
        .get(4).unwrap()
        .as_str()
        .chars()
        .filter(|&x| x == c)
        .count();

    return min <= count && count <= max;
}