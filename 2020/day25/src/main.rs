use std::fs;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"5764801
17807724";

const EXAMPLE_ANSWER_1: usize = 14897079;
const EXAMPLE_ANSWER_2: usize = 2;

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

const DIVIDE : usize = 20201227;

fn generate_handshake(subject: usize, loop_size: usize, divide: usize) -> usize
{
    let mut result = 1;
    for _ in 0..loop_size
    {
        result *= subject;
        result %= divide;
    }

    result
}

fn determine_loop_size(subject: usize, target: usize) -> Option<usize>
{
    let mut result = 1;
    for l in 0..9999999999
    {
        result *= subject;
        result %= DIVIDE;

        if result == target
        {
            return Some(l + 1);
        }
    }
    None
}

// card transforms subject 7 w/ loop size = card pk
// door subject 7 w/ loop = doors pk
// card transform subject matter of the door pk according to loop size
// door transforms using card pk to determine door loop size
// encryption key that the card calculates

fn solution(input: &str) -> Option<usize>
{
    let mut input = input.lines()
        .map(|x| x.parse::<usize>().unwrap());
    let card_pk = input.next().unwrap();
    let door_pk = input.next().unwrap();

    // determine determine the first oop size
    let card_loop = determine_loop_size(7, card_pk).unwrap();
    let door_loop = determine_loop_size(7, door_pk).unwrap();

    println!("Card: {} Door: {}", card_loop, door_loop);

    // use the lesser of the two for optimization
    if card_loop < door_loop
    {
        // determine encryption key
        let result = generate_handshake(door_pk, card_loop, DIVIDE);
        return Some(result);
    }
    let result = generate_handshake(card_pk, door_loop, DIVIDE);
    return Some(result);
}

fn solution_part2(input: &str) -> Option<usize>
{
    None
}