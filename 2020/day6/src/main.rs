use std::fs;
use std::collections::HashSet;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"abc

a
b
c

ab
ac

a
a
a
a

b";

const EXAMPLE_ANSWER_1: usize = 11;
const EXAMPLE_ANSWER_2: usize = 6;

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
    // assert!(answer == 42069);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    // assert!(answer == 1355323200);
}

fn solution(input: &str) -> Option<usize>
{
    let mut sum = 0;

    // groups are split by blank lines
    for group in input.split("\n\n")
    {
        let mut yes_questions = HashSet::new();

        for c in group.chars()
        {
            if c >= 'a' && c <= 'z'
            {
                yes_questions.insert(c);
                println!("char {}", c);
            }
        }

        sum += yes_questions.len();
    }

    Some(sum)
}

fn solution_part2(input: &str) -> Option<usize>
{
    let mut sum = 0;

    // groups are split by blank lines
    for group in input.split("\n\n")
    {
        let mut yes_questions = HashSet::new();
        let mut first_person = true; // forgot to set this to true at first lol

        for person in group.split("\n")
        {
            let mut person_questions = HashSet::new();
            for c in person.chars()
            {
                if c >= 'a' && c <= 'z'
                {
                    person_questions.insert(c);
                    println!("char {}", c);
                }
            }

            if first_person
            {
                yes_questions = person_questions;
                first_person = false;

                println!("yes questions {:?}", yes_questions);
            }
            else 
            {
                println!("intersect of {:?} and {:?}", yes_questions, person_questions);
                yes_questions = yes_questions.intersection(&person_questions).cloned().collect();
                println!("is {:?}", yes_questions);
            }
        }

        sum += yes_questions.len();
    }

    Some(sum)
}