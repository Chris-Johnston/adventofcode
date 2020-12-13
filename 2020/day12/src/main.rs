use std::fs;
use std::f64::consts::PI;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"F10
N3
F7
R90
F11";

#[derive(PartialEq, Debug)]
enum Action
{
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

// expressed as degrees on a compass
// will need to be converted to radians
const COMPASS_N : usize = 0;
const COMPASS_E : usize = 90;
const COMPASS_S : usize = 180;
const COMPASS_W : usize = 270;

const EXAMPLE_ANSWER_1: isize = 25;
const EXAMPLE_ANSWER_2: isize = 286;

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
    assert!(answer == 998);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer > 20557); // too low
    assert!(answer > 47024); // too low, rounding errors
    assert!(answer == 71586);
}

fn parse_instruction(instruction: &str) -> (Action, isize)
{
    let c : char = instruction.chars().nth(0)
        .expect("failed to get char");
    let (_, amount) = instruction.split_at(1);
    let amount = amount.parse::<isize>()
        .expect("Failed to parse amount");

    let action = match c {
        'N' => Action::N,
        'S' => Action::S,
        'E' => Action::E,
        'W' => Action::W,
        'L' => Action::L,
        'R' => Action::R,
        'F' => Action::F,
        _ => panic!("failed to match letter to the action")
    };

    (action, amount)
}

fn solution(input: &str) -> Option<isize>
{
    let instructions : Vec<(Action, isize)> = input.lines()
        .map(|x| parse_instruction(&x))
        .collect();

    let mut direction : isize = COMPASS_E as isize;
    let mut coords : (isize, isize) = (0, 0);

    for instruction in instructions
    {
        let mut action = instruction.0;

        println!("action {:?} {}", action, instruction.1);

        // aww, can't do this, gotta use trig
        // if action == Action::F
        // {
        //     // determine which direction
        //     action = match direction as usize {
        //         COMPASS_N => Action::N,
        //         COMPASS_E => Action::E,
        //         COMPASS_S => Action::S,
        //         COMPASS_W => Action::W,
        //         _ => panic!("invalid direction"),
        //     };
        // }

        match action
        {
            Action::N => {
                coords = (coords.0 + instruction.1, coords.1);
            },
            Action::E => {
                coords = (coords.0, coords.1 + instruction.1);
            },
            Action::S => {
                coords = (coords.0 - instruction.1, coords.1);
            },
            Action::W => {
                coords = (coords.0, coords.1 - instruction.1);
            },
            Action::L => {
                direction = (direction - instruction.1) % 360;
            },
            Action::R => {
                direction = (direction + instruction.1) % 360;
            },
            Action::F => {
                // convert compass direction to radians
                let radians = PI * (-direction - 90) as f64 / 180.0;

                // I have to use... trig!
                let dx = instruction.1 as f64 * radians.cos();
                let dy = instruction.1 as f64 * radians.sin();

                println!("dir {} -> rad {} -> dx {} dy {}", direction, radians, dx, dy);

                coords = (coords.0 + dx as isize, coords.1 + dy as isize);
            }
            _ => panic!("failed to match action"),
        };

        println!("coords: {:?}", coords);
    }

    // return manhattan distance
    Some(coords.0.abs() + coords.1.abs())
}

fn solution_part2(input: &str) -> Option<isize>
{
    let instructions : Vec<(Action, isize)> = input.lines()
        .map(|x| parse_instruction(&x))
        .collect();

    let mut direction : isize = COMPASS_E as isize;

    let mut waypoint_coords : (isize, isize) = (10, 1);
    let mut ship_coords : (isize, isize) = (0, 0);

    for instruction in instructions
    {
        let mut action = instruction.0;

        println!("action {:?} {}", action, instruction.1);
        match action
        {
            Action::N => {
                waypoint_coords = (waypoint_coords.0, waypoint_coords.1 + instruction.1);
            },
            Action::E => {
                waypoint_coords = (waypoint_coords.0 + instruction.1, waypoint_coords.1);
            },
            Action::S => {
                waypoint_coords = (waypoint_coords.0, waypoint_coords.1 - instruction.1);
            },
            Action::W => {
                waypoint_coords = (waypoint_coords.0 - instruction.1, waypoint_coords.1);
            },
            Action::L => {
                // rotate the waypoint around the ship left given # of degrees

                // going to get the diff between ship and waypoint
                let dx = (ship_coords.0 - waypoint_coords.0) as f64;
                let dy = (ship_coords.1 - waypoint_coords.1) as f64;

                let dx = (waypoint_coords.0) as f64;
                let dy = (waypoint_coords.1) as f64;

                let mag = (dx * dx + dy * dy).sqrt() as f64;

                let mut rad = f64::atan(dy / dx); // need to handle the case when this is negative

                // feels like a hack but it worked so who cares
                if dx < 0f64
                {
                    rad = PI + rad;
                }

                let drad = PI * instruction.1 as f64 / 180.0;

                println!("deg {} drad {}", instruction.1, drad);

                // where the actual rotation happens
                let rad = rad + drad;

                println!("new rad {}", rad);

                waypoint_coords = ((mag * rad.cos()).round() as isize, (mag * rad.sin()).round() as isize);
                println!("mag {} rad {} => {:?}", mag, rad, waypoint_coords);
            },
            Action::R => {
                // rotate the waypoint around the ship right given # of degrees

                // going to get the diff between ship and waypoint
                let dx = (ship_coords.0 - waypoint_coords.0) as f64;
                let dy = (ship_coords.1 - waypoint_coords.1) as f64;

                let dx = (waypoint_coords.0) as f64;
                let dy = (waypoint_coords.1) as f64;

                let mag = (dx * dx + dy * dy).sqrt() as f64;
                let mut rad = f64::atan(dy / dx); // need to handle the case when this is negative

                if dx < 0f64
                {
                    rad = PI + rad;
                }

                let drad = PI * -instruction.1 as f64 / 180.0;

                println!("deg {} drad {}", instruction.1, drad);

                // where the actual rotation happens
                let rad = rad + drad;

                println!("new rad {}", rad);
                waypoint_coords = ((mag * rad.cos()).round() as isize, (mag * rad.sin()).round() as isize);
                println!("mag {} rad {} => {:?}", mag, rad, waypoint_coords);
            },
            Action::F => {
                // convert compass direction to radians
                let mag = instruction.1;
                let (dx, dy) = (waypoint_coords.0 * mag, waypoint_coords.1 * mag);
                ship_coords = (ship_coords.0 + dx, ship_coords.1 + dy);
            }
            _ => panic!("failed to match action"),
        };

        println!("coords: {:?}, waypoint coords: {:?}", ship_coords, waypoint_coords);
    }

    // return manhattan distance
    Some(ship_coords.0.abs() + ship_coords.1.abs())
}