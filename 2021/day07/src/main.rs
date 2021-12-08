use std::fs;
use std::collections::{HashSet, HashMap};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Vec<usize>
{
    input
    .trim()
    .split(",")
    .map(|x| x.parse::<usize>().unwrap())
    .collect()
}

fn part1(input: &[usize]) -> usize
{
    // brute force all outcomes and find the
    // least expensive out of all unique inputs?

    // don't think I could use the median
    // and I think the mode would be close but could be misleading

    // if i need to optimize this I can start by looking by the mode

    let mut inputs_distinct : HashSet<usize> = HashSet::new();

    for i in input
    {
        inputs_distinct.insert(*i);
    }

    let mut answers : HashMap<usize, isize> = HashMap::new();

    fn calc_fuel_cost(position: usize, input: &[usize]) -> isize
    {
        let mut sum = 0;
        for inp in input
        {
            let i = *inp as isize;
            sum += (position as isize - i).abs();
        }
        sum as isize
    }

    for inp in inputs_distinct
    {
        let a = calc_fuel_cost(inp, input);
        answers.insert(inp, a);
    }

    let (_k, v) = answers
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *v as usize
}

fn part2(input: &[usize]) -> usize
{
    // brute force all outcomes and find the
    // least expensive out of all unique inputs?

    // don't think I could use the median
    // and I think the mode would be close but could be misleading

    // if i need to optimize this I can start by looking by the mode

    // edit part 2: the middle point can also be a point that
    // isn't one of the numbers!

    let mut inputs_distinct : HashSet<usize> = HashSet::new();

    for i in input
    {
        inputs_distinct.insert(*i);
    }

    let min_input = inputs_distinct.iter().min().unwrap();
    let max_input = inputs_distinct.iter().max().unwrap();

    let max_delta = max_input - min_input;

    // look up table for fuel costs
    let mut fuel_cost = Vec::new();
    let mut last_cost = 0;
    for x in 0..max_delta + 1
    {
        let cost = last_cost + x;
        fuel_cost.push(cost);

        last_cost = cost;
    }

    let mut answers : HashMap<usize, usize> = HashMap::new();

    fn calc_fuel_cost(position: usize, input: &[usize], fuelcost: &[usize]) -> usize
    {
        let mut sum = 0;
        for inp in input
        {
            let i = *inp as isize;

            // part 2, now the distance between position
            // and input is non linear

            // delta -> num fuel
            // 1 -> 1
            // 2 -> 1 + 2 = 3
            // 3 -> 1 + 2 + 3 = 6
            // 4 -> 1 + 2 + 3 + 4 = 10

            // going to make a look up table for this outside
            // of this loop, using the min and max input

            let delta = (position as isize - i).abs() as usize;

            sum += fuelcost[delta];
        }
        sum
    }

    // for inp in inputs_distinct
    for inp in *min_input..*max_input
    {
        let a = calc_fuel_cost(inp, input, &fuel_cost);
        answers.insert(inp, a);
    }

    println!("answers {:?}", answers);

    let (_k, v) = answers
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *v as usize
}



#[test]
fn example()
{
    let input = "16,1,2,0,4,2,7,1,2,14";
    let input = parse_input(input);
    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 168);
}