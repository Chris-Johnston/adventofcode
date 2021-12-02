use std::fs;

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 1 {}", answer);
}

fn parse_input(input: &str) -> Vec<(&str, usize)>
{
    input
    .lines()
    .map(str::trim)
    .map(|x|
        {
            let mut parts = x.split(' ');
            let action = parts.next().unwrap();
            let num = parts
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            (action, num)
        } )
    .collect()
}

fn part1(input: &[(&str, usize)]) -> usize
{
    let mut depth = 0;
    let mut x = 0;
    for (action, distance) in input
    {
        if *action == "up"
        {
            depth -= distance;
        }
        if *action == "down"
        {
            depth += distance;
        }
        if *action == "forward"
        {
            x += distance;
        }
    }

    depth * x
}

fn part2(input: &[(&str, usize)]) -> usize
{
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for (action, distance) in input
    {
        if *action == "up"
        {
            aim -= distance;
        }
        if *action == "down"
        {
            aim += distance;
        }
        if *action == "forward"
        {
            x += distance;
            depth += aim * distance;
        }
    }

    depth * x
}



#[test]
fn example()
{
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";
    let input = parse_input(input);
    assert_eq!(part1(&input), 150);
    assert_eq!(part2(&input), 900);
}