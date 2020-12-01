use std::fs;

fn main() {
    let test_data = vec![1721, 979, 366, 299, 675];
    let result = entries(&test_data, 2020);
    assert!(result == 514579);
    println!("rest data result: {}", result);
    
    let input_file = "/home/chris/Git/adventofcode/2020/day1/input_part1.txt";
    let data = fs::read_to_string(input_file)
        .expect("failed to read input file");
    let input: Vec<u32> = data
        .trim()
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
    let answer = entries(&input, 2020);
    println!("ANSWER: {}", answer); // 542619

    // part 2
    let part2_test = entries_part2(&test_data, 2020);
    println!("part 2 test: {}", part2_test);
    // assert!(result == 241861950);

    let answer2 = entries_part2(&input, 2020);
    println!("ANSWER P2: {}", answer2);
}

fn entries(entries: &Vec<u32>, total: u32) -> u32
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

fn entries_part2(entries: &Vec<u32>, total: u32) -> u32
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