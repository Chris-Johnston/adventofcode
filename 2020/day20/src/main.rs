use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

const EXAMPLE_ANSWER_1: isize = 20899048083289;
const EXAMPLE_ANSWER_2: isize = 2;

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

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    // assert!(answer == 1355323200);
}

fn parse_all_input(input: &str)
-> std::collections::HashMap<usize, std::vec::Vec<usize>> {
    let mut all_data = HashMap::new();

    for section in input.split("\n\n")
    {
        let (tile, data) = parse_section(section);

        let edges = determine_edges(&data);

        println!("Tile: {}", tile);
        for edge in &edges
        {
            println!("edge: {:010b}", edge);
        }

        all_data.insert(tile, edges);
    }

    all_data
}

fn parse_section(input: &str)
-> (usize, std::vec::Vec<usize>) {
    let mut line_iter = input.lines();
    let first_line = line_iter.next()
        .unwrap();

    let line_number_regex = Regex::new(r"Tile (\d+):")
        .unwrap();
    let tile : usize = line_number_regex
        .captures(first_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    
    let mut idx = 0;

    // 10 * 10 grid of data, 1s are #, 0s are .
    let mut data = vec![0; 100];
    for line in line_iter
    {
        for c in line.chars()
        {
            data[idx] = match c {
                '#' => 1,
                '.' => 0,
                _ => 0,
            };

            idx += 1;
        }

        println!("{}", line);
    }

    (tile, data)
}

fn determine_edges(data: &Vec<usize>)
-> std::vec::Vec<usize> {
    // north, north flipped, south, south flipped
    // east, east flipped, west, west flipped
    let mut edges = Vec::new();
    let mut north = 0;
    let mut north_reverse = 0;
    for i in 0..10
    {
        north |= data[i] << i;
        north_reverse |= data[i] << (9 - i);
    }
    edges.push(north);
    edges.push(north_reverse);

    // south
    let mut edge = 0;
    let mut edge_reverse = 0;
    let mut i = 0;
    for idx in 90..100
    {
        edge |= data[idx] << i;
        edge_reverse |= data[idx] << (9 - i);

        i += 1;
    }
    edges.push(edge);
    edges.push(edge_reverse);

    // east
    let mut edge = 0;
    let mut edge_reverse = 0;
    let mut i = 0;
    for idx in (0..91).step_by(10)
    {
        edge |= data[idx] << i;
        edge_reverse |= data[idx] << (9 - i);
        i += 1;
    }
    edges.push(edge);
    edges.push(edge_reverse);

    // west
    let mut edge = 0;
    let mut edge_reverse = 0;
    let mut i = 0;
    for idx in (9..100).step_by(10)
    {
        edge |= data[idx] << i;
        edge_reverse |= data[idx] << (9 - i);
        i += 1;
    }
    edges.push(edge);
    edges.push(edge_reverse);

    edges
}

fn solution(input: &str) -> Option<isize>
{
    let mut data = parse_all_input(input);

    // pick one key, assume it's the center of a grid
    // and then find the first match
    // then go through the rest

    let mut remaining_keys = VecDeque::new();
    for key in data.keys()
    {
        remaining_keys.push_back(key);
    }

    let mut coordinates = HashMap::new();
    let mut key_to_coord = HashMap::new();

    // first key is a special case
    let first_key = remaining_keys.pop_front().unwrap();
    coordinates.insert((0, 0), first_key);
    key_to_coord.insert(first_key, (0, 0));

    // the "rotations" of each tile
    let mut rotations = HashMap::new();

    let mut solved_keys = HashSet::new();
    solved_keys.insert(first_key);
    rotations.insert(first_key, 0);

    let mut coord = (0, 0);

    while remaining_keys.len() > 0
    {
        let key = remaining_keys.pop_front().unwrap();
        let key_edges = &data[key];

        let mut has_insert = false;

        let solved_keys_iter = solved_keys.clone();
        for last_key in solved_keys_iter
        {
            let prev_edges = &data[last_key];

            // find the first intersection
            for prev_edge in prev_edges
            {
                for key_edge_index in 0..key_edges.len()
                {
                    let key_edge = key_edges[key_edge_index];

                    if *prev_edge == key_edge
                    {
                        println!("Found a match at idx {}", key_edge_index);
                        println!("{} => {} dir {}", last_key, key, key_edge_index);

                        let coord = key_to_coord[last_key];

                        let from_rotation : isize = rotations[last_key];

                        let dcoord = match (8 + from_rotation - key_edge_index as isize / 2) % 4
                        {
                            0 => (0, -1),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (-1, 0),
                            _ => panic!("aaaa!")
                        };

                        rotations.insert(key, key_edge_index as isize / 2);

                        println!("idx {} dcoord {:?}", key_edge_index, dcoord);

                        // coord = (coord.0 + 1, coord.1 + 1);
                        let coord = (coord.0 + dcoord.0, coord.1 + dcoord.1);
                        coordinates.insert(coord, key);
                        key_to_coord.insert(key, coord);
                        has_insert = true;

                        solved_keys.insert(key);

                        break;
                    }
                }

                if has_insert
                {
                    break;
                }
            }

            if has_insert
            {
                break;
            }
        }

        if !has_insert
        {
            println!("no match {}", key);
            remaining_keys.push_back(key);
        }
    }

    println!("coords: {:?}", coordinates);

    None
}

fn solution_part2(input: &str) -> Option<isize>
{
    None
}