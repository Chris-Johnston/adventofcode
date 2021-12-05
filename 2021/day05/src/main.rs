use std::fs;
use std::cmp::{min, max};
use regex::Regex;
use std::collections::HashMap;

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 2 {}", answer);
    // NOT 19153
    // NOT 15961
    assert!(answer != 19153);
    assert!(answer != 15961);
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize, usize)>
{
    let input_re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    input
    .lines()
    .map(str::trim)
    .map(|x|
        {
            let captures = input_re.captures(x).unwrap();

            let x1 = captures[1].parse::<usize>().unwrap();
            let y1 = captures[2].parse::<usize>().unwrap();
            let x2 = captures[3].parse::<usize>().unwrap();
            let y2 = captures[4].parse::<usize>().unwrap();

            (x1, y1, x2, y2)
        } )
    .collect()
}

fn part1(input: &[(usize, usize, usize, usize)]) -> usize
{
    let mut coords : HashMap<(usize, usize), usize> = HashMap::new();

    // add all of the points
    for (x1, y1, x2, y2) in input
    {
        // define all of the points in the line
        let mut line_points : Vec<(usize, usize)> = Vec::new();
        if x1 == x2
        {
            let mut range = *y1..*y2 + 1;
            if y1 > y2
            {
                range = *y2..*y1 + 1;
            }

            for y in range
            {
                line_points.push((*x1, y));
            }
        }
        else if y1 == y2
        {
            let mut range = *x1..*x2 + 1;
            if x1 > x2
            {
                range = *x2..*x1 + 1;
            }

            for x in range
            {
                line_points.push((x, *y1));
            }
        }
        else
        {
            println!("skipping diagonal {},{} -> {},{}", x1, y1, x2, y2);
            continue;
        }

        for point in line_points
        {
            let count = coords.entry(point).or_insert(0);
            *count += 1;

            if *count > 1
            {
                // println!("POINT {:?}", point);
            }
        }
    }

    coords
        .values()
        .filter(|x| **x > 1)
        .count()
}

fn part2(input: &[(usize, usize, usize, usize)]) -> usize
{
    let mut coords : HashMap<(usize, usize), usize> = HashMap::new();

    // add all of the points
    for (x1, y1, x2, y2) in input
    {
        // define all of the points in the line
        let mut line_points : Vec<(usize, usize)> = Vec::new();
        if x1 == x2
        {
            // note: could have just used min max here lol
            let mut range = *y1..*y2 + 1;
            if y1 > y2
            {
                range = *y2..*y1 + 1;
            }

            for y in range
            {
                line_points.push((*x1, y));
            }
        }
        else if y1 == y2
        {
            let mut range = *x1..*x2 + 1;
            if x1 > x2
            {
                range = *x2..*x1 + 1;
            }

            for x in range
            {
                line_points.push((x, *y1));
            }
        }
        else
        {
            // this code is ugly
            let slope : isize;
            if x1 < x2
            {
                slope = match y1 < y2 {
                    true => 1,
                    false => -1,
                };
            }
            else
            {
                slope = match y1 < y2 {
                    true => -1,
                    false => 1,
                };
            }

            let start_y;
            let xrange;

            if x1 < x2
            {
                start_y = y1;
                xrange = *x1..*x2 + 1;
            }
            else
            {
                start_y = y2;

                xrange = *x2..*x1 + 1;
            }

            // println!("diag {} {} -> {} {} -- start y {} range {:?}", x1, y1, x2, y2, start_y, xrange);

            //let start_y = min(y1, y2);
            for (idx, x) in xrange.enumerate()
            {
                let p = (x, (idx as isize * slope + *start_y as isize) as usize);
                // println!("diag {:?} slope {} y {}", p, slope, start_y);
                line_points.push(p);
            }
        }

        for point in line_points
        {
            let count = coords.entry(point).or_insert(0);
            *count += 1;

            if *count > 1
            {
                // println!("POINT {:?}", point);
            }
        }
    }

    coords
        .values()
        .filter(|x| **x > 1)
        .count()
}



#[test]
fn example()
{
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let input = parse_input(input);
    assert_eq!(part1(&input), 5);
    assert_eq!(part2(&input), 12);
}