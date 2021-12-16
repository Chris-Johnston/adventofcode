use std::fs;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

struct Input
{
    map: Vec<usize>,
    dimension: usize,
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

fn parse_input(input: &str) -> Input
{
    // first line is always the template
    let width = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .len();

    let mut map = Vec::new();
    for l in input.trim().lines()
    {
        map.extend(l.chars().map(|x| x.to_digit(10).unwrap() as usize));
    }

    Input 
    {
        map: map,
        dimension: width
    }
}


fn part1(input: &Input) -> usize
{
    // I do not feel like solving dijkstra's right about now
    // because I know what this is
    0
}

fn part2(input: &Input) -> usize
{
    0
}

#[test]
fn example()
{
    let input =
    "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";
    let input = parse_input(input);
    assert_eq!(part1(&input), 40);
    assert_eq!(part2(&input), 2188189693529);
}