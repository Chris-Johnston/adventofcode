use std::fs;
use std::collections::HashSet;

static INPUT_FILE: &str = "input.txt";

fn main() {

    assert!(parse_boarding_pass("FBFBBFFRLR") == (44, 5, 357));
    assert!(parse_boarding_pass("BFFFBBFRRR") == (70, 7, 567));
    assert!(parse_boarding_pass("FFFBBBFRRR") == (14, 7, 119));
    assert!(parse_boarding_pass("BBFFBBFRLL") == (102, 4, 820));
    

    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    assert!(answer == 955);

    // part 2
    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer == 569);
}

// input string to row and col
fn parse_boarding_pass(input: &str) -> (usize, usize, usize)
{
    let mut row = 0;
    let mut col = 0;

    // first 7 chars are F or B
    for (pos, e) in input.chars().enumerate()
    {
        let bit = match e {
            'F' => 0,
            'B' => 1,
            'R' => 1,
            'L' => 0,
            _ => panic!("invalid char to match against {}", e),
        };

        if pos < 7
        {
            row |= bit << (6 - pos);
        }
        else
        {
            col |= bit << (2 - (pos - 7))
        }
    }
    println!("input {} row {:07b} col {:03b}", input, row, col);
    (row, col, row * 8 + col)
}

fn solution(input: &str) -> Option<usize>
{
    // find the highest seat Id on the boarding pass
    let mut max = 0;
    for line in input.lines()
    {
        let pass = parse_boarding_pass(line);
        if pass.2 > max
        {
            max = pass.2;
        }
    }

    Some(max)
}

fn solution_part2(input: &str) -> Option<usize>
{
    let mut seats = HashSet::new();
    let mut max = 0;

    for line in input.lines()
    {
        let pass = parse_boarding_pass(line);
        seats.insert(pass.2);
        if pass.2 > max
        {
            max = pass.2;
        }
    }

    for i in 1..(max - 1)
    {
        // need find seat such that i -1 and i + 1 exist, but i does not
        if seats.contains(&(i - 1)) && seats.contains(&(i + 1)) && !seats.contains(&i)
        {
            return Some(i);
        }
    }

    None
}