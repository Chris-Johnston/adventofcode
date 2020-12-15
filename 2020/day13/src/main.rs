use std::fs;
use gcd::Gcd;
use std::collections::HashSet;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"939
7,13,x,x,59,x,31,19";

// first line, earliest timestamp you could depart on a bus
// bus ids in service, ignore X for part 1
// need to find the number which has the smallest difference between a multiple and the target
// i can just use mod
// then mul the delta by the id

const EXAMPLE_ANSWER_1: usize = 295;
const EXAMPLE_ANSWER_2: usize = 1068781;

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


    // part 2 test cases
    let inputs = vec!["7,13,x,x,59,x,31,19", "17,x,13,19", "67,7,59,61", "67,x,7,59,61", "67,7,x,59,61", "1789,37,47,1889"];
    let answers = vec![1068781, 3417, 754018, 779210, 1261476, 1202161486];

    for test_case_i in 0..inputs.len()
    {
        let input = "discard\n".to_owned() + inputs[test_case_i];
        let input = input.as_str();

        let expected = answers[test_case_i];
        println!("input {} expected {}", input, expected);
        let answer = solution_part2(input).expect("no answer!?");
        println!("expected {} == {} ", expected, answer);
    }

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    //               1615033136751203
    assert!(answer > 100000000000000);
    assert!(answer < 1615033136751203);
    // assert!(answer == 1355323200);
    assert!(answer == 554865447501099);
}

fn solution(input: &str) -> Option<usize>
{
    let earliest_timestamp = input.lines().nth(0).expect("failed to get the first line")
        .parse::<usize>()
        .expect("failed to parse timestamp");
    let mut bus_times : Vec<usize> = input.lines().nth(1).expect("failed to get the second line")
        .split(",")
        .filter_map(|x| x.parse::<usize>().ok()) // ignores X
        .collect();

    

    // mod does not work, will have to be a bit smarter
    // sort by the diff
    // bus_times.sort_by(|b, a| (earliest_timestamp % a).cmp(&(earliest_timestamp % b)));

    fn find_next_multiple(mul: usize, min: usize) -> usize
    {
        (min as f64 / mul as f64).ceil() as usize * mul
    }


    bus_times.sort_by(
        |a, b| find_next_multiple(*a, earliest_timestamp)
            .cmp(
                &(find_next_multiple(*b, earliest_timestamp))
            ));
    let ideal = bus_times.iter().nth(0).expect("wat");
    let delta = find_next_multiple(*ideal, earliest_timestamp) - earliest_timestamp;
    println!("sorted times {:?}", bus_times);

    println!("ideal bus is {} with delta {}", ideal, delta);

    Some(ideal * delta)
}

fn solution_part2(input: &str) -> Option<usize>
{
    let first_line = input.lines().nth(0).expect("no first line").trim() == "1001612";
    let bus_times = input.lines().nth(1).expect("failed to get the second line")
        .split(",");

    let mut bus_data = Vec::new();

    for (i, x) in bus_times.enumerate()
    {
        let bus_id = x.parse::<usize>();

        if let Some(id) = bus_id.ok()
        {
            bus_data.push((id, i));
        }
    }

    // sort bus_data by id desc
    bus_data.sort_by(|b, a| a.0.cmp(&b.0));
    println!("bus data is {:?}", bus_data);

    let max = bus_data.iter().map(|(id, _)| id).max().expect("failed to find max");
    let t_for_max_id = bus_data.iter().filter(|(id, t)| max == id).nth(0).expect("aaa").1;

    // going to optimize this a bit, by skipping not by max but the LCM of all the Ids
    // cannot do this, all of the GCD is 1
    // and it also seems that all of these are prime, great
    /*fn gcd(data: &Vec<usize>) -> usize
    {
        let mut x = data[0];
        for i in 1..data.len()
        {
            let y = data[i];
            x = x.gcd(y);
        }

        x
    }

    let just_bus_id : Vec<usize> = bus_data.iter().map(|(id, _)| *id).collect();
    let gcd = gcd(&just_bus_id);

    println!("GCD of the set {:?} is {}", &just_bus_id, gcd);*/

    let just_bus_id : Vec<usize> = bus_data.iter().map(|(id, _)| *id).collect();
    let mut sum = 1;
    for x in &just_bus_id
    {
        sum *= x;
    }

    // so for any 2 numbers
    // it must be a repeating sequence for how frequently they "overlap" but not quite by 1
    // what if we select the top 2 from this list, and calculate the period in which they overlap by 1
    // and use that to optimize the start

    if first_line
    {
        let mut just_bus_id = just_bus_id;
        just_bus_id.sort_by(|a, b| b.cmp(a));

        let z1 = just_bus_id[0];
        let z2 = just_bus_id[1];
        let lim = z1 * z2;

        for x in 10..9999
        {
            if (x % z1 == 0 && x % z2 == (z2 - 1)) || (x % z1 == (z1 - 1) && x % z2 == 0)
            {
                // not sure if this will end up helping in the end
                println!("x {}, {}, {}", x, z1, z2);
            }
        }

        // x 79636, 821, 463
    }

    // so I cheated a bit and found a meme that mentioned this
    // I think it's a wall when I have to know XYZ theorem to solve a thing
    // anyways chinese remainder theorem, we can use it because numbers are prime

    // x = a (mod p)
    // x is the answer
    // a is the time param offset
    // and p is the bus id


    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    // solution belongs to the expression
    // a1,a1 + n1, a1 + 2n1
    // step by the largest number firt
    // until getting a number congruent to 3 mod 4, this is the new x value
    // and add 5x4 each time


    let start = 0;
    let mut step : usize = *max;

    let mut bus_data_idx = 1; // idx into bus data for which step is already accounted for

    let mut factors = HashSet::new();
    factors.insert(step);

    // brute force, step by max
    // in hindsight, I am actually pretty happy with how close I was
    // the big difference is that I had to sort all my keys and only eval the next largest one at a time
    // and then use that to multiply the current step value to optimize things a bit
    // for x in (start..1615033136751203usize).step_by(step) // step_by(*max) works great
    let mut x = start;
    while true // for some reason the step_by wasn't working as I expected, so just do it like this
    {
        x += step;
        if x > 1615033136751203usize
        {
            break;
        }
        // skip by the largest number
        // and verify conditions for each

        let mut conditions_met = false;

        // for (id, time) in &bus_data
        
            let (id, time) = &bus_data[bus_data_idx];
            println!("x {} {} finding match for {} {}", x, step, id, time);

            println!("x {} id {} time {}", x, id, time);
            println!("tmax {}", t_for_max_id);
            println!("relative t {}", *time - t_for_max_id);
            let relative_timestamp = x as isize + (*time as isize - t_for_max_id as isize);
            if relative_timestamp < 0
            {
                conditions_met = false;
                continue;
            }
            let relative_timestamp = relative_timestamp as usize;

            if relative_timestamp % id == 0
            {
                // condition is met
                if factors.insert(*id)
                {
                    // if not in, multiply the step by the bus id
                    step *= id;
                    println!("now step by {}", step);
                    println!("factors are {:?}", factors);

                    bus_data_idx += 1;
                }
            }
            else
            {
                conditions_met = false;
                // break;
                // do not optimize, maybe could speed this up a bit
            }
        

        if bus_data_idx == bus_data.len() {
            println!("idx {} {}", bus_data_idx, bus_data.len());
            return Some(x - t_for_max_id);
        }
    }

    None
}