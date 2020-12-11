use std::fs;
use std::slice::Iter;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

#[derive(Debug, Copy, Clone, PartialEq)]
enum SeatType
{
    F, // Floor
    E, // Empty
    O // Occupied
}

const EXAMPLE_ANSWER_1: usize = 37;
const EXAMPLE_ANSWER_2: usize = 26;

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
    assert!(answer == 2281);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer == 2085);
}

fn print(floor_data: &Vec<SeatType>, width: usize)
{
    let len = floor_data.len() / width;
    for row in 0..len
    {
        for col in 0..width {
            let idx = col + row * width;
            let c = match floor_data[idx]
            {
                SeatType::E => 'L',
                SeatType::F => '.',
                SeatType::O => '#',
                _ => ' ',
            };
            print!("{}",c);
        }
        println!();
    }
}

fn get_adjacent_seats(position: (usize, usize), floor_data: &Vec<SeatType>, width: usize)
    -> Vec<SeatType>
{
    let len = floor_data.len() / width;
    fn in_bounds(position: (isize, isize), width: usize, len: usize) -> bool
    {
        position.0 >= 0 && position.0 < width as isize && position.1 >= 0 && position.1 < len as isize
    }

    // up down let right
    // NW NE SW SE
    let directions : Vec<(isize, isize)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    // tried and failed to return an iterator, just use a vec
    let result: Vec<SeatType> = directions
        .iter()
        .filter_map(|x| {
            let new_position = (position.0 as isize + x.0, position.1 as isize + x.1);
            // match in_bounds(new_position, width, len)
            // {
            //     True => Some(new_position),
            //     _ => None,
            // }
            if in_bounds(new_position, width, len)
            {
                let idx = new_position.0 + new_position.1 * width as isize;
                return Some(floor_data[idx as usize].clone());
            }
            return None;
        })
        .collect();
    return result;
}

fn get_adjacent_seats_part2(position: (usize, usize), floor_data: &Vec<SeatType>, width: usize)
    -> Vec<SeatType>
{
    let len = floor_data.len() / width;
    fn in_bounds(position: (isize, isize), width: usize, len: usize) -> bool
    {
        position.0 >= 0 && position.0 < width as isize && position.1 >= 0 && position.1 < len as isize
    }

    // up down let right
    // NW NE SW SE
    let directions : Vec<(isize, isize)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    // tried and failed to return an iterator, just use a vec
    let result: Vec<SeatType> = directions
        .iter()
        .filter_map(|x| {
            
            // println!("{:?} dir {:?}", position, x);
            // walk in each direction until out of bounds or the occupied spot is found
            let mut new_position = (position.0 as isize + x.0, position.1 as isize + x.1);
            for _ in 0..len+width
            {
                // println!("walking to {:?}", new_position);
                if in_bounds(new_position, width, len)
                {
                    let idx = new_position.0 + new_position.1 * width as isize;
                    let v = floor_data[idx as usize].clone();
                    if v == SeatType::O || v == SeatType::E
                    {
                        // println!("hit {:?}", v);
                        return Some(v);
                    }
                    new_position = (new_position.0 + x.0, new_position.1 + x.1);
                }
                else
                {
                    // println!("out of bounds!");
                    break;
                }                
            }
            None
        })
        .collect();
    return result;
}

fn solution(input: &str) -> Option<usize>
{
    // parse input into a big vec of SeatType
    let mut floor_data : Vec<SeatType> =
        input.lines()
        .map(|x| x.chars()
            .map(|c| match c {
                FLOOR => SeatType::F,
                EMPTY => SeatType::E,
                OCCUPIED => SeatType::O,
                _ => panic!("aaahhH!! invalid char {}", c),
            }))
        .flatten()
        .collect();

    let width = input.lines().nth(0)
            .expect("couldn't get width of line 0")
            .len();
    let len = input.lines().count();

    print(&floor_data, width);

    let mut has_changes = true;
    let mut iteration_counter = 0;
    while has_changes
    {
        iteration_counter += 1;
        let mut update_floor_data = floor_data.clone();
        has_changes = false;

        // so many lambdas this time
        update_floor_data = floor_data
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                let position = (i % width, i / width);
                let adjacent_seats = get_adjacent_seats(position, &floor_data, width);
                let count_adjacent_occupied_seats = adjacent_seats
                    .iter()
                    .filter(|x| **x == SeatType::O)
                    .count();
                if *x == SeatType::E && count_adjacent_occupied_seats == 0
                {
                    has_changes = true;
                    return Some(SeatType::O);
                }
                else if *x == SeatType::O && count_adjacent_occupied_seats >= 4
                {
                    has_changes = true;
                    return Some(SeatType::E);
                }

                return Some(*x);
            })
            .collect();

        println!("\niteration {}", iteration_counter);
        floor_data = update_floor_data;
        print(&floor_data, width);
    }
    
    print(&floor_data, width);
    // reminds me a bit of game of life

    let occupied_count = floor_data
        .iter()
        .filter(|x| **x == SeatType::O)
        .count();

    Some(occupied_count)
}

fn solution_part2(input: &str) -> Option<usize>
{
    // parse input into a big vec of SeatType
    let mut floor_data : Vec<SeatType> =
        input.lines()
        .map(|x| x.chars()
            .map(|c| match c {
                FLOOR => SeatType::F,
                EMPTY => SeatType::E,
                OCCUPIED => SeatType::O,
                _ => panic!("aaahhH!! invalid char {}", c),
            }))
        .flatten()
        .collect();

    let width = input.lines().nth(0)
            .expect("couldn't get width of line 0")
            .len();
    let len = input.lines().count();

    print(&floor_data, width);

    let mut has_changes = true;
    let mut iteration_counter = 0;
    while has_changes
    {
        iteration_counter += 1;
        let mut update_floor_data = floor_data.clone();
        has_changes = false;

        // so many lambdas this time
        update_floor_data = floor_data
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                let position = (i % width, i / width);
                let adjacent_seats = get_adjacent_seats_part2(position, &floor_data, width);
                let count_adjacent_occupied_seats = adjacent_seats
                    .iter()
                    .filter(|x| **x == SeatType::O)
                    .count();
                if *x == SeatType::E && count_adjacent_occupied_seats == 0
                {
                    has_changes = true;
                    return Some(SeatType::O);
                }
                else if *x == SeatType::O && count_adjacent_occupied_seats >= 5 
                // didn't read the rules, this was giving me trouble
                {
                    has_changes = true;
                    return Some(SeatType::E);
                }

                return Some(*x);
            })
            .collect();

        println!("\niteration {}", iteration_counter);
        floor_data = update_floor_data;
        print(&floor_data, width);
    }
    
    println!("done");
    print(&floor_data, width);
    // reminds me a bit of game of life

    let occupied_count = floor_data
        .iter()
        .filter(|x| **x == SeatType::O)
        .count();

    Some(occupied_count)
}