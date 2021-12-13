use std::fs;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

struct Input
{
    points: Vec<(isize, isize)>,
    folds: Vec<(char, isize)>
}

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);
    assert!(answer == 666);
    
    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Input
{
    let mut points = Vec::new();

    // parse all points
    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    for capture in re.captures_iter(input.trim())
    {
        let x = capture[1].parse::<isize>().unwrap();
        let y = capture[2].parse::<isize>().unwrap();

        points.push((x, y));
    }

    let mut folds = Vec::new();

    // parse all points
    let re = Regex::new(r"fold along ([xy])=(\d+)").unwrap();

    for capture in re.captures_iter(input.trim())
    {
        let dimension = capture.get(1).unwrap().as_str().chars().next().unwrap();
        let val = capture[2].parse::<isize>().unwrap();

        // might need to check of ordering is important here
        folds.push((dimension, val));
    }

    Input {
        points: points,
        folds: folds,
    }
}

fn print_grid(points: &HashSet<(isize, isize)>)
{
    // find max x and y
    let max_x = points
        .iter()
        .map(|(x, _)| *x)
        .max()
        .unwrap();
    let max_y = points
        .iter()
        .map(|(_, y)| *y)
        .max()
        .unwrap();

    for y in 0..(max_y + 1)
    {
        for x in 0..(max_x + 1)
        {
            if points.contains(&(x, y))
            {
                print!("#");
            }
            else
            {
                print!(".");
            }
        }
        println!("");
    }
}

fn part1(input: &Input) -> usize
{
    println!("{:?}", input.points);
    println!("{:?}", input.folds);

    let mut points : HashSet<(isize, isize)> = HashSet::new();
    for p in input.points.iter()
    {
        points.insert(*p);
    }

    print_grid(&points);

    // for (dir, value) in &input.folds
    {
        let (dir, value) = &input.folds.iter().next().unwrap();
        let mut newpoints = HashSet::new();
        for (x, y) in points.iter()
        {
            if *dir == 'x'
            {
                // fold horizontally
                if x <= value
                {
                    newpoints.insert((*x, *y));
                }
                else
                {
                    let diff_from_midpoint = (value - x).abs();
                    newpoints.insert((value - diff_from_midpoint, *y));
                }
            }
            else
            {
                if y <= value
                {
                    newpoints.insert((*x, *y));
                }
                else
                {
                    // newpoints.insert((*x, value - y));
                    let diff_from_midpoint = (value - y).abs();
                    newpoints.insert((*x, value - diff_from_midpoint));
                }
            }
        }

        println!("{:?}", newpoints);

        points = newpoints;

        print_grid(&points);
    }

    points.len()
}

fn part2(input: &Input) -> usize
{
    let mut points : HashSet<(isize, isize)> = HashSet::new();
    for p in input.points.iter()
    {
        points.insert(*p);
    }

    print_grid(&points);

    for (dir, value) in &input.folds
    {
        let mut newpoints = HashSet::new();
        for (x, y) in points.iter()
        {
            if *dir == 'x'
            {
                // fold horizontally
                if x <= value
                {
                    newpoints.insert((*x, *y));
                }
                else
                {
                    let diff_from_midpoint = (value - x).abs();
                    newpoints.insert((value - diff_from_midpoint, *y));
                }
            }
            else
            {
                if y <= value
                {
                    newpoints.insert((*x, *y));
                }
                else
                {
                    // newpoints.insert((*x, value - y));
                    let diff_from_midpoint = (value - y).abs();
                    newpoints.insert((*x, value - diff_from_midpoint));
                }
            }
        }

        println!("{:?}", newpoints);

        points = newpoints;

        print_grid(&points);
    }

    points.len()
}

#[test]
fn example()
{
    let input =
    "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";
    let input = parse_input(input);
    // assert_eq!(part1(&input), 17);
    assert_eq!(part1(&input), 17);
    assert_eq!(part2(&input), 0);
}