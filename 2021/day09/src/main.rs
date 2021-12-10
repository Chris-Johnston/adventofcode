use std::fs;
use std::collections::HashSet;

static INPUT_FILE: &str = "input.txt";

struct PuzzleInput
{
    width: usize,
    height: usize,
    data: Vec<usize>,
}

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);
    assert!(answer != 509);
    assert!(answer != 1968);
    assert!(answer != 1698);
    assert!(answer != 1728);

    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> PuzzleInput
{
    let width =
        input
        .trim()
        .lines()
        .next()
        .unwrap()
        .len();

    let height =
        input
        .trim()
        .lines()
        .count();

    let mut data = Vec::new();

    for l in input.lines().map(str::trim)
    {
        let points_iter = l
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize);
        data.extend(points_iter);
    }

    PuzzleInput
    {
        width: width,
        height: height,
        data: data,
    }
}

fn part1(input: &PuzzleInput) -> usize
{
    let mut risk_level = Vec::new();

    fn coords_to_idx(x: isize, y: isize, width: usize, height: usize) -> Option<usize>
    {
        let max_idx = width * height;
        let point = x + (width as isize) * y;

        if x < 0 || x >= width as isize
        {
            return None;
        }
        if y < 0 || y >= height as isize
        {
            return None;
        }

        if point >= max_idx as isize
        {
            // println!("coords {} {} are OOB {}", x, y, point);
            return None;
        }
        if point < 0
        {
            // println!("coords {} {} are OOB {}", x, y, point);
            return None;
        }

        // println!("{} {} -> {}", x, y, point);

        return Some(point as usize);
    }

    println!("input {:?} x {}", input.width, input.height);

    for x in 0..input.width as isize
    {
        for y in 0..input.height as isize
        {
            let center = coords_to_idx(x, y, input.width, input.height).unwrap();
            let center_val = input.data[center];

            let mut is_lowest = true;

            // low point if value of center is less then the surrounding points
            if let Some(left) = coords_to_idx(x - 1, y, input.width, input.height)
            {
                if input.data[left] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(right) = coords_to_idx(x + 1, y, input.width, input.height)
            {
                if input.data[right] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(up) = coords_to_idx(x, y - 1, input.width, input.height)
            {
                if input.data[up] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(down) = coords_to_idx(x, y + 1, input.width, input.height)
            {
                if input.data[down] <= center_val
                {
                    is_lowest = false;
                }
            }

            if is_lowest
            {
                println!("low point {} {} of {}", x, y, center_val);

                risk_level.push(center_val + 1);
            }
        }
    }
    
    risk_level.iter().sum()
}

fn part2(input: &PuzzleInput) -> usize
{
    let mut lowpoints = Vec::new();

    fn coords_to_idx(x: isize, y: isize, width: usize, height: usize) -> Option<usize>
    {
        let max_idx = width * height;
        let point = x + (width as isize) * y;

        if x < 0 || x >= width as isize
        {
            return None;
        }
        if y < 0 || y >= height as isize
        {
            return None;
        }

        if point >= max_idx as isize
        {
            // println!("coords {} {} are OOB {}", x, y, point);
            return None;
        }
        if point < 0
        {
            // println!("coords {} {} are OOB {}", x, y, point);
            return None;
        }

        // println!("{} {} -> {}", x, y, point);

        return Some(point as usize);
    }

    println!("input {:?} x {}", input.width, input.height);

    for x in 0..input.width as isize
    {
        for y in 0..input.height as isize
        {
            let center = coords_to_idx(x, y, input.width, input.height).unwrap();
            let center_val = input.data[center];

            let mut is_lowest = true;

            // low point if value of center is less then the surrounding points
            if let Some(left) = coords_to_idx(x - 1, y, input.width, input.height)
            {
                if input.data[left] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(right) = coords_to_idx(x + 1, y, input.width, input.height)
            {
                if input.data[right] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(up) = coords_to_idx(x, y - 1, input.width, input.height)
            {
                if input.data[up] <= center_val
                {
                    is_lowest = false;
                }
            }

            if let Some(down) = coords_to_idx(x, y + 1, input.width, input.height)
            {
                if input.data[down] <= center_val
                {
                    is_lowest = false;
                }
            }

            if is_lowest
            {
                println!("low point {} {} of {}", x, y, center_val);

                // risk_level.push(center_val + 1);
                lowpoints.push((x, y));
            }
        }
    }

    let mut basin_sizes = Vec::new();

    for basin in lowpoints
    {
        let mut basin_points : HashSet<(isize, isize)> = HashSet::new();

        fn walk_points(x: isize, y: isize, input: &PuzzleInput, basin_points: &mut HashSet<(isize, isize)>)
        {
            // add current point if not exists
            basin_points.insert((x, y));

            // left
            if let Some(p) = coords_to_idx(x - 1, y, input.width, input.height)
            {
                // point exists
                if input.data[p] != 9 && !basin_points.contains(&(x - 1, y))
                {
                    // point is in basin
                    walk_points(x - 1, y, input, basin_points);
                }
            }

            // right
            if let Some(p) = coords_to_idx(x + 1, y, input.width, input.height)
            {
                // point exists
                if input.data[p] != 9 && !basin_points.contains(&(x + 1, y))
                {
                    // point is in basin
                    walk_points(x + 1, y, input, basin_points);
                }
            }

            // down
            if let Some(p) = coords_to_idx(x, y - 1, input.width, input.height)
            {
                // point exists
                if input.data[p] != 9 && !basin_points.contains(&(x, y - 1))
                {
                    // point is in basin
                    walk_points(x, y - 1, input, basin_points);
                }
            }

            // up
            if let Some(p) = coords_to_idx(x, y + 1, input.width, input.height)
            {
                // point exists
                if input.data[p] != 9 && !basin_points.contains(&(x, y + 1))
                {
                    // point is in basin
                    walk_points(x, y + 1, input, basin_points);
                }
            }
        }

        walk_points(basin.0, basin.1, input, &mut basin_points);

        println!("basin {:?} was size {}", basin, basin_points.len());

        // basin_size *= basin_points.len();
        basin_sizes.push(basin_points.len());
    }
    
    let mut answer = 1;
    basin_sizes.sort_by(|a, b| b.cmp(a));

    for size in basin_sizes.iter().take(3)
    {
        answer *= size;
    }

    answer
}

#[test]
fn example()
{
    let input =
    "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";
    let input = parse_input(input);
    assert_eq!(part1(&input), 15);
    assert_eq!(part2(&input), 1134);
}