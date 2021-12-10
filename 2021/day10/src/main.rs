// All of them?!

use std::fs;
use std::collections::HashSet;

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

fn parse_input(input: &str) -> Vec<Vec<char>>
{
    input
    .trim()
    .lines()
    .map(str::trim)
    .map(|x| x.chars().collect())
    .collect()
}

fn part1(input: &Vec<Vec<char>>) -> usize
{
    fn get_score(illegal_char: char) -> Option<usize>
    {
        match illegal_char {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None
        }
    }

    fn is_matching(opening: char, closing: char) -> bool
    {
        if let Some(expected) = match opening {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None
        } {
            return closing == expected;
        }
        
        return false;
    }

    let mut sum = 0;

    for line in input
    {
        let mut stack = Vec::new();

        for c in line
        {
            // this feels wrong
            if match c {
                '(' => true,
                '[' => true,
                '{' => true,
                '<' => true,
                _ => false
            } {
                // opening
                stack.push(c);
            }
            else if match c {
                ')' => true,
                ']' => true,
                '}' => true,
                '>' => true,
                _ => false
            } {
                let matching = stack.pop().unwrap();

                if !is_matching(*matching, *c)
                {
                    println!("expected match of {} found {}", matching, c);

                    sum += get_score(*c).unwrap();
                    break;
                }
            }
        }
    }

    sum
}

fn part2(input: &Vec<Vec<char>>) -> usize
{
    fn get_score(illegal_char: char) -> Option<usize>
    {
        match illegal_char {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None
        }
    }

    fn get_closing(opening: char) -> Option<char>
    {
        match opening {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None
        }
    }

    fn is_matching(opening: char, closing: char) -> bool
    {
        if let Some(expected) = get_closing(opening) {
            return closing == expected;
        }
        
        return false;
    }

    let mut sum = 0;
    let mut autocomplete_scores = Vec::new();

    for line in input
    {
        println!("line {:?}", line);

        let mut stack = Vec::new();
        let mut valid = true;

        for c in line
        {
            // this feels wrong
            if match c {
                '(' => true,
                '[' => true,
                '{' => true,
                '<' => true,
                _ => false
            } {
                // opening
                stack.push(c);
            }
            else if match c {
                ')' => true,
                ']' => true,
                '}' => true,
                '>' => true,
                _ => false
            } {
                let matching = stack.pop().unwrap();

                if !is_matching(*matching, *c)
                {
                    println!("expected match of {} found {}", matching, c);

                    sum += get_score(*c).unwrap();
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            continue;
        }

        println!("remaining stack {:?}", stack);

        let mut autocomplete_score = 0;

        while stack.len() > 0
        {
            autocomplete_score *= 5;

            let c = stack.pop().unwrap();

            let matching = get_closing(*c).unwrap();

            let points = match matching
            {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("bad input {}", matching),
            };

            autocomplete_score += points;
        }

        autocomplete_scores.push(autocomplete_score);
    }

    autocomplete_scores.sort();

    println!("{:?}", autocomplete_scores);

    autocomplete_scores[(autocomplete_scores.len() / 2)]
}

#[test]
fn example()
{
    let input =
    "[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]";
    let input = parse_input(input);
    assert_eq!(part1(&input), 26397);
    assert_eq!(part2(&input), 288957);
}