use std::fs;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
".#.
..#
###";

const EXAMPLE_ANSWER_1: usize = 112;
const EXAMPLE_ANSWER_2: usize = 2;

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

fn get_neighbors_for_coord(x: &isize, y: &isize, z: &isize) -> Vec<(isize, isize, isize)>
{
    let mut coordinates = Vec::new();
    // let permutations = vec![0, 0, 1, 1, 1, -1, -1, -1];
    let permutations = vec![
        // 7
        (0, 0, 1),
        (0, 1, 0),
        (0, 1, 1),
        (1, 0, 0),
        (1, 0, 1),
        (1, 1, 0),
        (1, 1, 1),

        // 11
        (0, 0, -1),
        (0, 1, -1),
        (1, 0, -1),
        (1, 1, -1),

        // 15
        (0, -1, 0),
        (0, -1, 1),
        (1, -1, 0),
        (1, -1, 1),

        // 19
        (-1, 0, 0),
        (-1, 0, 1),
        (-1, 1, 0),
        (-1, 1, 1),
        
        // 26
        (0, 0, -1),
        (0, -1, 0),
        (0, -1, -1),
        (-1, 0, 0),
        (-1, 0, -1),
        (-1, -1, 0),
        (-1, -1, -1),
    ];
    for (dx, dy, dz) in permutations
    {
        // println!("perm ({} {} {})", dx, dy, dz);

        coordinates.push((x + dx, y + dy, z + dz));
    }

    // println!("coordinates {} {:?}", coordinates.len(), coordinates);

    coordinates
}

fn solution(input: &str) -> Option<usize>
{
    // get the current active cubes
    // where contents are a 3tuple of the xyz coords
    let mut active_cube_coordinates = HashSet::new();
    let mut inactive_cube_coordinates = HashSet::new();

    for (li, line) in input.lines().enumerate()
    {
        for (ci, c) in line.chars().enumerate()
        {
            if c == '#'
            {
                active_cube_coordinates.insert((ci as isize, li as isize, 0isize));
            }
            else
            {
                inactive_cube_coordinates.insert((ci as isize, li as isize, 0isize));
            }
        }
    }

    for cycle in 0..6
    {
        println!("cycle {}", cycle);

        // each cycle will go into this and is copied
        let mut update_active = HashSet::new();
        let mut update_inactive = HashSet::new();

        for (x, y, z) in &active_cube_coordinates
        {
            let is_current_active = active_cube_coordinates.contains(&(*x, *y, *z));
            let is_current_inactive = inactive_cube_coordinates.contains(&(*x, *y, *z));

            // get neighbors
            let neighbors =
                get_neighbors_for_coord(x, y, z);

            let mut active_neighbor_count = 0;
            let mut inactive_neighbor_count = 0;

            // all neighbors that are reached and not in the map should be added to inactive
            for n in &neighbors
            {
                // println!("neighbor {:?}", n);
                if active_cube_coordinates.contains(&n)
                {
                    active_neighbor_count += 1;
                }
                else if inactive_cube_coordinates.contains(&n)
                {
                    inactive_neighbor_count += 1;
                }
                else
                {
                    inactive_neighbor_count += 1;
                    // not in anything, add to update_inactive
                    update_inactive.insert(n.clone());
                }
            }

            println!("active: {} inactive {}", active_neighbor_count, inactive_neighbor_count);
            
            if is_current_active && (active_neighbor_count == 2 || active_neighbor_count == 3)
            {
                println!("remain active ({}, {}, {})", x, y, z);
                // cube remains active
                update_active.insert((*x, *y, *z));
            }
            else
            {
                println!("deactivating ({}, {}, {})", x, y, z);
                // cube is inactive, remove it
                update_inactive.insert((*x, *y, *z));
            }

            if is_current_inactive && active_neighbor_count == 3
            {
                println!("activating ({}, {}, {})", x, y, z);
                // becomes active
                update_active.insert((*x, *y, *z));
            }
            else
            {
                println!("remain deactive ({}, {}, {})", x, y, z);
                update_inactive.insert((*x, *y, *z));
            }
        }

        println!("active count {}", active_cube_coordinates.len());
        println!("inactive count {}", inactive_cube_coordinates.len());

        active_cube_coordinates = update_active;
        inactive_cube_coordinates = update_inactive;

        println!("active count {}", active_cube_coordinates.len());
        println!("inactive count {}", inactive_cube_coordinates.len());
    }


    Some(active_cube_coordinates.len())
}

fn solution_part2(input: &str) -> Option<usize>
{
    None
}