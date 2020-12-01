use std::fs;
use std::collections::HashSet;

fn main() {
    let test_data = vec![1721, 979, 366, 299, 675];
    let result = entries(&test_data, 2020);
    assert!(result == 514579);
    println!("rest data result: {}", result);
    
    let input_file = "/home/chris/Git/adventofcode/2020/day1/input_part1.txt";
    let data = fs::read_to_string(input_file)
        .expect("failed to read input file");
    let input: Vec<usize> = data
        .trim()
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let answer = entries(&input, 2020);
    println!("ANSWER: {}", answer); // 542619

    // part 2
    let part2_test = entries_part2(&test_data, 2020);
    println!("part 2 test: {}", part2_test);
    assert!(part2_test == 241861950);

    let answer2 = entries_part2(&input, 2020);
    println!("ANSWER P2: {}", answer2);


    // better impl
    let better_example = better(&test_data, 2020, 0)
        .expect("nada");
    println!("better worked {}", better_example);
    assert!(better_example == 514579);

    let part2_test = better_part2(&test_data, 2020);
    println!("better 2 test: {}", part2_test);
    assert!(part2_test == 241861950);

    // better with real input
    let answer = better(&input, 2020, 0)
        .expect("nada");
    println!("ANSWER: {}", answer); // 542619
    assert!(answer == 542619);

    let answer2 = better_part2(&input, 2020);
    println!("ANSWER P2: {}", answer2);
    assert!(answer2 == 32858450);
}

fn better(entries: &Vec<usize>, total: usize, start_index: usize) -> Option<usize>
{
    let mut sorted = entries
        .to_vec();
    sorted.sort();
    // but also create a set
    let mut set = HashSet::new();
    for x in entries
    {
        set.insert(x);
    }
    
    for i_left in (0..(entries.len() - start_index)).rev()
    {
        let left = sorted[i_left];
        if left > total {
            continue;
        }

        // check to see if the answer exists 
        if set.contains(&(total - left))
        {
            return Some((total - left) * left);
        }
    }
    None
}

fn better_part2(entries: &Vec<usize>, total: usize) -> usize
{
    let mut sorted = entries
        .to_vec();
    sorted.sort();
    // but also create a set
    let mut set = HashSet::new();
    for x in entries
    {
        set.insert(x);
    }
    
    for i_left in (0..entries.len()).rev()
    {
        let left = sorted[i_left];
        if left > total {
            continue;
        }

        // get the list from i_left..0
        // return left * better(&sorted, total - left, 0);
        match better(&sorted, total - left, 0) {
            None => continue,
            Some(x) => return left * x,
        }
    }

    return 1;
}

fn entries(entries: &Vec<usize>, total: usize) -> usize
{
    // find the two entries in the list which sum to the total
    // then multiply them
    for first in entries {
        for second in entries {
            if first + second == total
            {
                return first * second;
            }
        }
    }

    return 1
}

fn entries_part2(entries: &Vec<usize>, total: usize) -> usize
{
    // find the two entries in the list which sum to the total
    // then multiply them
    for first in entries {
        for second in entries { // o(n^3) lets gooo
            if first == second {
                continue;
            }
            for third in entries {
                if first == third {
                    continue;
                }
                if second == third {
                    continue;
                }
            if first + second + third == total
            {
                return first * second * third;
            }
            }
        }
    }

    return 1
}