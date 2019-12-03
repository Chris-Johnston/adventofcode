use std::collections::HashSet;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::vec;

fn main() {
    // input will be a list of directions
    // for each wire, will need to traverse the directions
    // 
    // each wire will update each of the cells in the table with the wire #
    // if it is 0 (unset)
    // if there is already a wire on that cell, then there is an intersection
    // and the intersection coords will be logged

    // make the empty grid
    // let mut wire_grid = [[0u8, ..]]

    // actually, I do not know the upper bound

    // so instead, make a set of coordinates that are covered by each of the wires
    // the union of those sets will be all the intersection points
    // then find the set with the smallest intersection point

    // read the file
    let input_file = "/home/chris/Git/adventofcode/2019/3/day3/input.txt";

    let file = fs::File::open(input_file)
        .expect("File could not be opened.");
    let file = BufReader::new(file);

    let mut sets : Vec<HashSet<(isize, isize)>> = vec::Vec::new();

    for line in file.lines()
    // for line in vec!["R8,U5,L5,D3", "U7,R6,D4,L4"].iter()
    {
        let result = line.expect("couldn't read the line");
        let set_for_wire = get_set(result);
        sets.push(set_for_wire);
    }

    // after getting all of the sets
    // find the intersection of each of the sets, but not the intersection of all sets
    let mut intersection : HashSet<(isize, isize)> = HashSet::new();

    for (index, a) in sets.iter().enumerate()
    {
        println!("finding all interesction of index {} {:?}", index, a);
        for (i2, b) in sets.iter().enumerate()
        {
            // skip union of self
            if index <= i2
            {
                continue;
            }
            let sub_intersection = a.intersection(b);
            println!("duplicate points: {:?}", sub_intersection);
            intersection.extend(sub_intersection);
        }
    }

    println!("found {} crossed wires", intersection.len());

    // for each of the duplicate points, find the shortest distance from the origin
    // TODO

    // let min_dist = intersection
    //     .iter()
    //     .enumerate()
    //     .min_by(|x, y| (x.0 + x.1).cmp(y.0 + y.1)).unwrap();
    let mut min = -1;
    for coord in intersection
    {
        let dist = coord.0.abs() + coord.1.abs();
        if min == -1 || dist < min
        {
            min = dist;
        }
    }

    println!("min dist is: {}", min);
}

fn get_set(input_str: String) -> HashSet<(isize, isize)>
{
    // let w1_instructions = "R8,U5,L5,D3";
    let w1_instructions = input_str.split(",");

    let mut cur = (0, 0);
    let mut set = HashSet::new();
    // set.insert((0, 0));

    for instruction in w1_instructions
    {
        let mut chars = instruction.chars();
        let direction = chars.next().unwrap();
        let distance: isize = chars.as_str()
                        .parse()
                        .expect("Couldn't parse the substring as an int.");

        println!("dir {} dist {}", direction, distance);

        for _ in 0..distance
        {
            match direction
            {
                'R' => {
                    cur.0 += 1;
                },
                'U' => {
                    cur.1 += 1;
                },
                'L' => {
                    cur.0 -= 1;
                },
                'D' =>
                {
                    cur.1 -=1;
                },
                _ => {
                    println!("invalid dir");
                    break;
                },
            }
            println!("Adding {:?}", cur);
            set.insert(cur);
        }
    }
    return set;
}