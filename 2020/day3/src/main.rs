use std::fs;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

const OPEN: char = '.';
const TREE: char = '#';

fn main() {
    let example_expected = 7;
    
    let example_solution = solution(EXAMPLE_INPUT)
        .expect("no result");
    
    println!("example result {} = {}", example_expected, example_solution);
    assert!(example_expected == example_solution);

    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");
    let answer = solution(&input)
        .expect("no result");
    
    println!("answer {}", answer);
    assert!(answer == 193);

    // part 2
    let example_expected = 336;
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("example result #2 {} = {}", example_expected, example_solution);
    assert!(example_expected == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("answer {}", answer);
    assert!(answer == 1355323200);
}

static SLOPE: (usize, usize) = (3, 1);

fn solution(input: &str) -> Option<isize>
{
    let lines : Vec<&str> = input.lines().collect();
    let mut count = 0;

    let mut position = (0, 0);
    while position.1 < lines.len()
    {
        // check if current pos is tree
        let y_line : Vec<char> = lines.get(position.1)
            .expect("out of range")
            .chars()
            .collect();
        let tree = y_line.get(position.0 % y_line.len())
            .expect("out of range");

        if tree == &TREE { count += 1; }

        println!("position {:?} tree {}", position, tree);

        // increment the slope
        position.0 += SLOPE.0;
        position.1 += SLOPE.1;
    }

    Some(count)
}

fn solution_part2(input: &str) -> Option<isize>
{
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;

    for slope in slopes
    {
        let result = solution_part2_innter(input, slope)
            .expect("didn't get a result");
        product *= result;
    }

    Some(product)
}
fn solution_part2_innter(input: &str, slope: (usize, usize)) -> Option<isize>
{
    let lines : Vec<&str> = input.lines().collect();
    let mut count = 0;

    let mut position = (0, 0);
    while position.1 < lines.len()
    {
        // check if current pos is tree
        let y_line : Vec<char> = lines.get(position.1)
            .expect("out of range")
            .chars()
            .collect();
        let tree = y_line.get(position.0 % y_line.len())
            .expect("out of range");

        if tree == &TREE { count += 1; }

        println!("position {:?} tree {}", position, tree);

        // increment the slope
        position.0 += slope.0;
        position.1 += slope.1;
    }

    Some(count)
}
