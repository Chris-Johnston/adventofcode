use std::fs;
use std::collections;

fn main() {
    // example_one();
    example_two();
    //example_three();
    // solution_part1();
    solution_part2();
}

fn example_two()
{
    let map =
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    let mut coords = Vec::new();
    set_asteroid_coords(String::from(map),&mut coords);

    let (best_point, best_count) = get_results(&coords);

    println!("best: {:?} {}", best_point, best_count);
    // offset the coords by best point
    let mut offset_coords = Vec::new();

    for coord in &coords
    {
        let new_coord = (coord.0 - best_point.0, coord.1 - best_point.1);
        if new_coord.0 != 0 || new_coord.1 != 0
        {
            offset_coords.push(new_coord);
        }
    }

    vaporize(&mut offset_coords, best_point);
}

fn example_three()
{
    let map =
".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

    let mut coords = Vec::new();
    set_asteroid_coords(String::from(map),&mut coords);

    let (best_point, best_count) = get_results(&coords);

    println!("best: {:?} {}", best_point, best_count);
    // offset the coords by best point
    let mut offset_coords = Vec::new();

    for coord in &coords
    {
        let new_coord = (best_point.0 - coord.0, best_point.1 - coord.1);
        if new_coord.0 != 0 || new_coord.1 != 0
        {
            offset_coords.push(new_coord);
        }
    }
    vaporize(&mut offset_coords, best_point);
}

fn solution_part1()
{
    let input_file = "/home/chris/Git/adventofcode/2019/10/input.txt";
    let map = fs::read_to_string(input_file)
        .expect("failed to read file");
    let mut coords = Vec::new();
    set_asteroid_coords(map,&mut coords);

    let (best_point, best_count) = get_results(&coords);

    println!("solution: best: {:?} {}", best_point, best_count);
}

fn solution_part2()
{
    let input_file = "/home/chris/Git/adventofcode/2019/10/input.txt";
    let map = fs::read_to_string(input_file)
        .expect("failed to read file");
    let mut coords = Vec::new();
    set_asteroid_coords(map,&mut coords);

    let (best_point, best_count) = get_results(&coords);

    println!("best: {:?} {}", best_point, best_count);
    // offset the coords by best point
    let mut offset_coords = Vec::new();

    for coord in &coords
    {
        let new_coord = (coord.0 - best_point.0, coord.1 - best_point.1);
        if new_coord.0 != 0 || new_coord.1 != 0
        {
            offset_coords.push(new_coord);
        }
    }

    vaporize(&mut offset_coords, best_point);
}

fn vaporize(coords: &mut Vec<(isize, isize)>, best: (isize, isize, isize))
{
    // sort the coordinates by tan(x, y), then by d

    // println!("coords {:?}", coords);
    let tan_offset = std::f64::consts::PI / 2.0f64;
    let two_pi = std::f64::consts::PI * 2f64;
    coords.sort_by(|a, b| {
        let tan_a = (a.0 as f64).atan2(a.1 as f64);
        let tan_a = (tan_a + std::f64::consts::PI) % two_pi;
        let tan_b = (b.0 as f64).atan2(b.1 as f64);
        let tan_b = (tan_b + std::f64::consts::PI) % two_pi;
        // if a.0 == 0
        // {
        //     if a.1 > 0
        //     {
        //         tan_a = 0f64;
        //     }
        //     else
        //     {
        //         tan_a = std::f64::consts::PI;
        //     }
        // }
        // else
        // {
        //     tan_a  = a.1 as f64 / (a.0 as f64 + 0.001f64);
        //     tan_a = tan_a.atan2() - tan_offset;

        //     if tan_a < 0f64
        //     {
        //         tan_a += two_pi;
        //         tan_a %= two_pi;
        //     }
        // }
        // println!("tan a {:?} -> {}", a, tan_a);
        // if b.0 == 0
        // {
        //     if b.1 > 0
        //     {
        //         tan_b = 0f64;
        //     }
        //     else
        //     {
        //         tan_b = std::f64::consts::PI;
        //     }
        // }
        // else
        // {
        //     tan_b = b.1 as f64 / (b.0 as f64 + 0.001f64);
        //     tan_b = tan_b.atan() - tan_offset;

        //     if tan_b < 0f64
        //     {
        //         tan_b += two_pi;
        //         tan_b %= two_pi;
        //     }
        // }
        
        let gcd_a = a.0 * a.0 + a.1 * a.1;
        // println!("dist a {}", gcd_a);
        let gcd_b = b.0 * b.0 + b.1 * b.1;
        tan_b.partial_cmp(&tan_a).unwrap().then_with(|| gcd_a.cmp(&gcd_b))

        // gcd_a.cmp(&gcd_b).then_with(|| tan_a.partial_cmp(&tan_a).unwrap())
    });

    println!("coords {:?}", coords);
    // println!("coord 200 {:?}", coords[199]);

    // iterate through the set of coords
    // if the current coord atan2 is the same as previous one, then continue
    // when it resets, then restart

    let mut last_atan = -1f64;
    let mut visited = collections::HashSet::new();
    let mut counter = 0;

    for (index, c) in coords.iter().enumerate()
    {
        let atan = (c.0 as f64).atan2(c.1 as f64);
        let atan = (atan + two_pi) % two_pi;
        if last_atan == atan
        {
            println!("skipping {:?} atan {}", c, atan);
            continue;
        }
        else
        {
            last_atan = atan;
            visited.insert(index);
            counter += 1;
        }

        let unoffset = (c.0 + best.0, c.1 + best.1);
        let solution = unoffset.0 * 100 + unoffset.1;
        println!("{} c {:?} -> {:?} ({}) atan {}", counter, c, unoffset, solution, atan);
    }
}

fn get_results(coords: &Vec<(isize, isize, isize)>) -> ((isize, isize, isize), usize)
{
    let mut best_point = (0, 0, 0);
    let mut best_count = 0;
    for (start_idx, start_coord) in coords.iter().enumerate()
    {
        let mut asteroid_set = collections::HashSet::new();
        for (end_idx, end_coord) in coords.iter().enumerate()
        {
            if start_idx == end_idx
            {
                continue;
            }
            
            // push x / y on to the set
            // y + 1 to avoid divide by zero

            let delta_coord : (isize, isize) = (start_coord.0 - end_coord.0 , start_coord.1 - end_coord.1 );

            let d = gcd(delta_coord.0, delta_coord.1);

            asteroid_set.insert((delta_coord.0 / d, delta_coord.1 / d));

            // asteroid_set.insert((end_coord.0 as f64) / (end_coord.1 as f64 + 1f64));
            // asteroid_set.insert((end_coord.0 as f64) / (end_coord.1 as f64 + 1f64));
        }

        // println!("Point {} {:?} had {} visible ({:?})", start_idx, start_coord, asteroid_set.len(), asteroid_set);

        if asteroid_set.len() > best_count
        {
            best_count = asteroid_set.len();
            best_point = (start_coord.0, start_coord.1, start_coord.2);
        }
    }
    (best_point, best_count)
}

fn example_one()
{
    let map = ".#..#
.....
#####
....#
...##";

    let mut coords = Vec::new();
    set_asteroid_coords(String::from(map), &mut coords);
    
    let (best_point, best_count) = get_results(&coords);

    println!("best: {:?} {}", best_point, best_count);
}

fn set_asteroid_coords(map: String, coord_list: &mut Vec<(isize, isize, isize)>)
{
    for (y, line) in map.lines().enumerate()
    {
        for (x, chr) in line.chars().enumerate()
        {
            match chr
            {
                '#' =>
                {
                    let gcd = gcd(x as isize, y as isize);
                    coord_list.push((x as isize, y as isize, gcd));
                },
                _ => continue,
            }
        }
    }
}

fn gcd(mut m: isize, mut n: isize) -> isize
{
    while m != 0
    {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}