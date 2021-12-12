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

fn parse_input(input: &str) -> Vec<usize>
{
    let mut values = Vec::new();

    for l in input
    .trim()
    .lines()
    .map(str::trim)
    .map(|x| x.chars()
        .map(|c| c.to_digit(10)
        .unwrap() as usize))
    {
        values.extend(l);
    }

    values
}

fn print(energy: &Vec<usize>, dim: usize)
{
    for x in 0..dim
    {
        for idx in 0..dim
        {
            let v = energy[x * dim + idx];
            if v > 9
            {
                print!("#");
            }
            else
            {
                print!("{}", v);
            }
        }
        println!("");
    }
}

fn part1(input: &Vec<usize>) -> usize
{
    let mut energy = input.clone();

    // println!("start {:?}", energy);

    // let adjacentpoints = vec![
    //     -1, // left
    //     -10, // up
    //     1, // right
    //     10, // down
    //     -9, // NE
    //     -11, // NW
    //     9, // SW
    //     11, // SE
    // ];

    let dim : isize = 10;
    let size = 100;
    let steps = 100;

    // print(&energy, dim as usize);

    // let adjacentpoints : Vec<isize> = vec![
    //     -1, // left
    //     -dim, // up
    //     1, // right
    //     dim, // down
    //     -dim + 1, // NE
    //     -dim - 1, // NW
    //     dim - 1, // SW
    //     dim + 1, // SE
    // ];

    let diff_coord = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1)
    ];

    let mut flash_count = 0;

    for step in 0..steps
    {
        println!("step {}", step);
        print(&energy, dim as usize);

        let mut has_flash = false;

        // not sure if it possible for this to happen
        let mut flashed_this_step = HashSet::new();

        // increment energy by 1
        // for e in 0..100
        for e in 0..size
        {
            energy[e] += 1;
        }

        let mut flash_update = true;
        while flash_update
        {
            flash_update = false;

            println!("flash");
            print(&energy, dim as usize);

            for idx in 0..size
            {
                // println!("energy({}) {}", idx, energy[idx]);
                if energy[idx] > 9 && !flashed_this_step.contains(&idx)
                {
                    // flash
                    flash_count += 1;
                    flash_update = true;

                    // set this to to 0 after update
                    flashed_this_step.insert(idx);

                    // increment adjacent nodes
                    for (dx, dy) in &diff_coord
                    {
                        // convert the idx to coords
                        let y = (idx / 10) as isize;
                        let x = (idx % 10) as isize;

                        let neighbor_x = x + dx;
                        let neighbor_y = y + dy;

                        if (neighbor_x < 0 || neighbor_x >= dim) || (neighbor_y < 0 || neighbor_y >= dim)
                        {
                            continue;
                        }

                        let neighboridx = neighbor_x + dim * neighbor_y;
                        if neighboridx >= 0 && neighboridx < size as isize
                        {
                            energy[neighboridx as usize] += 1;
                        }
                    }
                }
            }
        }

        for idx in flashed_this_step
        {
            energy[idx] = 0;
        }
    }

    flash_count
}

fn part2(input: &Vec<usize>) -> usize
{
    let mut energy = input.clone();

    // println!("start {:?}", energy);

    // let adjacentpoints = vec![
    //     -1, // left
    //     -10, // up
    //     1, // right
    //     10, // down
    //     -9, // NE
    //     -11, // NW
    //     9, // SW
    //     11, // SE
    // ];

    let dim : isize = 10;
    let size = 100;
    let steps = 99999;

    let diff_coord = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1)
    ];

    let mut flash_count = 0;

    for step in 0..steps
    {
        println!("step {}", step);
        print(&energy, dim as usize);

        let mut has_flash = false;

        // not sure if it possible for this to happen
        let mut flashed_this_step = HashSet::new();

        // increment energy by 1
        // for e in 0..100
        for e in 0..size
        {
            energy[e] += 1;
        }

        let mut flash_update = true;
        while flash_update
        {
            flash_update = false;

            println!("flash");
            print(&energy, dim as usize);

            for idx in 0..size
            {
                // println!("energy({}) {}", idx, energy[idx]);
                if energy[idx] > 9 && !flashed_this_step.contains(&idx)
                {
                    // flash
                    flash_count += 1;
                    flash_update = true;

                    // set this to to 0 after update
                    flashed_this_step.insert(idx);

                    // increment adjacent nodes
                    for (dx, dy) in &diff_coord
                    {
                        // convert the idx to coords
                        let y = (idx / 10) as isize;
                        let x = (idx % 10) as isize;

                        let neighbor_x = x + dx;
                        let neighbor_y = y + dy;

                        if (neighbor_x < 0 || neighbor_x >= dim) || (neighbor_y < 0 || neighbor_y >= dim)
                        {
                            continue;
                        }

                        let neighboridx = neighbor_x + dim * neighbor_y;
                        if neighboridx >= 0 && neighboridx < size as isize
                        {
                            energy[neighboridx as usize] += 1;
                        }
                    }
                }
            }
        }

        for idx in flashed_this_step
        {
            energy[idx] = 0;
        }

        if energy.iter().filter(|&x| *x == 0).count() == size
        {
            return step + 1;
        }
    }

    0
}

#[test]
fn example()
{
    // let input =
    // "11111
    // 19991
    // 19191
    // 19991
    // 11111";
    let input =
    "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    // let input =
    // "9000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000
    // 0000000000";
    let input = parse_input(input);
    assert_eq!(part1(&input), 1656);
    assert_eq!(part2(&input), 195);
}