use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

static INPUT_FILE: &str = "input.txt";

struct Inputs {
    numbers: Vec<usize>,
    boards: Vec<Vec<usize>>
}

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Inputs
{
    // first line is the list of inputs
    // then new line
    // then 5 lines of 5 inputs for each grid

    // thinking I should leave this alone when I parse it

    let mut iter = input.lines();

    let numbers : Vec<usize> = iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut boards : Vec<Vec<usize>> = Vec::new();

    let mut line = iter.next();

    while !line.is_none()
    {
        let mut board : Vec<usize> = Vec::new();

        for row in 0..5
        {
            line = iter.next();
            // println!("row {:?}", line);

            let values = line
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<usize>().unwrap());

            board.extend(values);
        }

        boards.push(board);

        // skip empty line
        line = iter.next();
    }

    Inputs {
        numbers: numbers,
        boards: boards
    }
}

fn part1(input: &Inputs) -> usize
{
    // idea when I solve it, I can build a collection of sets of 5 numbers which represent the numbers which have to be called
    // and then overlap the number of called numbers with each of those sets to determine if there is a pass
    // then build a collection of all numbers in all of the grids, remove all of the called numbers from it to get the sum

    let mut sets : Vec<HashSet<usize>> = Vec::new();

    // determine the valid sets
    for board in &input.boards
    {
        // add row solutions
        for row in 0..5
        {
            let mut set : HashSet<usize> = HashSet::new();

            for col in 0..5
            {
                let idx = row * 5 + col;
                set.insert(board[idx]);
            }

            sets.push(set);
        }

        // col solutions
        for col in 0..5
        {
            let mut set : HashSet<usize> = HashSet::new();

            for row in 0..5
            {
                let idx = row * 5 + col;
                set.insert(board[idx]);
            }

            sets.push(set);
        }

        // add diag solutions -- nevermind, no diagnols
        // let mut diag : HashSet<usize> = HashSet::new();
        // diag.insert(board[0]); // 0, 0
        // diag.insert(board[6]); // 1, 1
        // diag.insert(board[12]); // 2, 2
        // diag.insert(board[18]); // 3, 3
        // diag.insert(board[24]); // 4, 4

        // println!("diag 1 {:?}", diag);

        // sets.push(diag);

        // let mut diag : HashSet<usize> = HashSet::new();
        // diag.insert(board[4]); // 4, 0
        // diag.insert(board[8]); // 3, 1
        // diag.insert(board[12]); // 2, 2
        // diag.insert(board[16]); // 1, 3
        // diag.insert(board[20]); // 0, 4

        // println!("diag 2 {:?}", diag);

        // sets.push(diag);
    }

    // debug
    for set in &sets
    {
        println!("{:?}", set);
    }

    for picked in 2..input.numbers.len() + 1
    {
        let slice = input.numbers
                    .iter()
                    .take(picked)
                    .map(|x| *x);
        let picked_numbers : HashSet<usize> = HashSet::from_iter(slice);

        // println!("picked numbers: {:?}", picked_numbers);

        // for all sets, check if there are any matches

        // for set in &sets
        for (idx, set) in sets.iter().enumerate()
        {
            let intersect : Vec<usize> = set
                .intersection(&picked_numbers)
                .map(|x| *x)
                .collect();
            if intersect.len() == 5
            {
                let board_number = idx / 10;
                let last_add = input.numbers[picked - 1];

                let board_sum : usize = input.boards[board_number]
                        .iter()
                        .filter(|x| !picked_numbers.contains(x))
                        .sum();

                println!("last_added {} sum is {}", last_add, board_sum);

                return last_add * board_sum;
            }
        }
    }

    0
}

fn part2(input: &Inputs) -> usize
{
    // idea when I solve it, I can build a collection of sets of 5 numbers which represent the numbers which have to be called
    // and then overlap the number of called numbers with each of those sets to determine if there is a pass
    // then build a collection of all numbers in all of the grids, remove all of the called numbers from it to get the sum

    let mut sets : Vec<HashSet<usize>> = Vec::new();

    // determine the valid sets
    for board in &input.boards
    {
        // add row solutions
        for row in 0..5
        {
            let mut set : HashSet<usize> = HashSet::new();

            for col in 0..5
            {
                let idx = row * 5 + col;
                set.insert(board[idx]);
            }

            sets.push(set);
        }

        // col solutions
        for col in 0..5
        {
            let mut set : HashSet<usize> = HashSet::new();

            for row in 0..5
            {
                let idx = row * 5 + col;
                set.insert(board[idx]);
            }

            sets.push(set);
        }

        // add diag solutions -- nevermind, no diagnols
        // let mut diag : HashSet<usize> = HashSet::new();
        // diag.insert(board[0]); // 0, 0
        // diag.insert(board[6]); // 1, 1
        // diag.insert(board[12]); // 2, 2
        // diag.insert(board[18]); // 3, 3
        // diag.insert(board[24]); // 4, 4

        // println!("diag 1 {:?}", diag);

        // sets.push(diag);

        // let mut diag : HashSet<usize> = HashSet::new();
        // diag.insert(board[4]); // 4, 0
        // diag.insert(board[8]); // 3, 1
        // diag.insert(board[12]); // 2, 2
        // diag.insert(board[16]); // 1, 3
        // diag.insert(board[20]); // 0, 4

        // println!("diag 2 {:?}", diag);

        // sets.push(diag);
    }

    // debug
    for set in &sets
    {
        println!("{:?}", set);
    }

    let num_boards = sets.len() / 10;
    // boards that have won
    let mut winning_boards : HashSet<usize> = HashSet::new();

    for picked in 2..input.numbers.len() + 1
    {
        let slice = input.numbers
                    .iter()
                    .take(picked)
                    .map(|x| *x);
        let picked_numbers : HashSet<usize> = HashSet::from_iter(slice);

        // println!("picked numbers: {:?}", picked_numbers);

        // for all sets, check if there are any matches

        // for set in &sets
        for (idx, set) in sets.iter().enumerate()
        {
            let board_number = idx / 10;
            // skip boards that already won
            if winning_boards.contains(&board_number)
            {
                continue;
            }

            let intersect : Vec<usize> = set
                .intersection(&picked_numbers)
                .map(|x| *x)
                .collect();

            // part 2, which board will win last?
            if intersect.len() == 5
            {
                let last_add = input.numbers[picked - 1];

                // if this is the last board to win
                if num_boards - winning_boards.len() == 1
                {
                    let board_sum : usize = input.boards[board_number]
                        .iter()
                        .filter(|x| !picked_numbers.contains(x))
                        .sum();

                    println!("last_added {} sum is {}", last_add, board_sum);

                    return last_add * board_sum;   
                }
                else
                {
                    winning_boards.insert(board_number);
                }
            }
        }
    }

    0
}



#[test]
fn example()
{
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";
    let input = parse_input(input);
    assert_eq!(part1(&input), 4512);
    assert_eq!(part2(&input), 1924);
}