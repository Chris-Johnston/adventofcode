use std::fs;
use std::collections::HashMap;

static INPUT_FILE: &str = "input.txt";

static EZ_EXAMPLE_INPUT: &str = "1
2
3
4
7";

// input = 1 2
// 0 1 2 5
// 0 2 5

// input = 1 2 3
// 0 1 2 3 6
// 0 1 3 6
// 0 2 3 6
// 0 3 6

// 0 1 2 3 4 7 10
// 0 1 3 4 7 10
// 0 2 3 4 7 10 
// 0 3 4 7 10
// 0 1 4 7 10
// 0 2 4 7 10


static EXAMPLE_INPUT: &str = 
"16
10
15
5
1
11
7
19
6
12
4";

static LARGE_EXAMPLE_INPUT: &str = 
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

const EXAMPLE_ANSWER_1: usize = 35;
const EXAMPLE_ANSWER_2: usize = 19208;

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let example_solution = solution(EXAMPLE_INPUT)
        .expect("no result");
    
    println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    assert!(EXAMPLE_ANSWER_1 == example_solution);

    let example_solution = solution(LARGE_EXAMPLE_INPUT)    
        .expect("no result");
    assert!(220 == example_solution);

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    assert!(answer == 1856);


    // part 2

    let example_solution = solution_part2(EZ_EXAMPLE_INPUT)
        .expect("no result");

    println!("EZ EXAMPLE Part 2 {}", example_solution);
    println!("EZ example should have 2 possible solutions == {}", example_solution);
    assert!(7 == example_solution);


    let example_solution = solution_part2(LARGE_EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer != 578509309952);
    assert!(answer > 1000000000000); // 1 trillion
    assert!(answer == 2314037239808);
    // assert!(answer == 1355323200);
}

fn solution(input: &str) -> Option<usize>
{
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();

    // need to go in ascending order
    data.sort();
    
    // count the number of deltas between each number
    let mut output = vec![0, 0, 1]; // built in diff of 3
    let mut prev = 0; // starting value is 0

    for x in data
    {
        let delta = x - prev;
        println!("{} -> {} delta: {}", prev, x, delta);
        if delta <= 3
        {
            output[delta - 1] += 1;
        }

        prev = x;
    }

    println!("Count of deltas {:?}", output);

    Some(output[0] * output[2])
}

fn clean_part2(input: &str) -> Option<usize>
{
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();
    data.sort();
    // insert min and max
    data.insert(0, 0);
    let max = data[data.len() - 1];
    data.push(max + 3);
    let mut body : usize = 1;
    let mut consecutive_ones = 0;
    for (i, x) in data.iter().enumerate()
    {
        if i < 1 { continue; }
        let delta = x - data[i - 1];
        if delta == 1
        {
            consecutive_ones += 1;
        }
        else if delta == 2
        {
            panic!("this does not happen?");
        }
        else if delta == 3 && consecutive_ones != 0
        {
            let p = match consecutive_ones
            {
                5 => 13, // had to fiddle around with these answers a bit to find what was right
                4 => 7,
                3 => 4,
                2 => 2,
                1 => 1,
                0 => 1,
                _ => panic!("aahh!!"),
            };
            body *= p;

            consecutive_ones = 0;
        }
    }
    println!("sum {}", body);

    Some(body) // once told me
}

fn solution_part2(input: &str) -> Option<usize>
{
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();
    data.sort();
    data.insert(0, 0);
    let max = data[data.len() - 1];
    data.push(max + 3);
    // one thing I noticed from brute forcing part 2 was that
    // the brute force doesn't scale
    // but also that the first index that is removed
    // can be factored to 2^3 * 3^1 * 7^3
    // so that's the count of the possible results when index 2 is removed

    // 5488 factors to 2^4 * 7^3
    // 2744 factors to 2^3 * 7^3
    // 1176 2^3 * 3 * 7^2
    // 784 2^4 * 7^2
    // 392 2^3 * 7^2

    // so it's definitely the case that I need to sum multiplications of things

    // I just realized, it's not a bug, there are no diffs of +2
    // this could play to my advantage

    let mut sum : usize = 1;

    let mut consecutive_count = HashMap::new();

    let mut consecutive_ones = 0;
    for (i, x) in data.iter().enumerate()
    {
        if i < 1
        {
            // consecutive_ones = 1;
            continue;
        }

        let delta = x - data[i - 1];
        if delta == 1
        {
            consecutive_ones += 1;
        }
        else if delta == 2
        {
            panic!("this does not happen?");
        }
        else if delta == 3 && consecutive_ones != 0
        {
            println!("span of consecutive ones: {}", consecutive_ones);
            // sum += consecutive_ones;

            // i think this just kinda breaks down
            // don't think it's the right answer
            let p = match consecutive_ones
            {
                // 3 1 1 1
                // 1 1 1
                // 1 1
                // 1   1
                //   1
                //   1 1
                //     1
                5 => 13, // had to fiddle around with these answers a bit to find what was right
                4 => 7,
                // 1 1 1
                // 1 
                //   1
                //     1
                // 1 1
                //   1 1
                // 1   1
                3 => 4, // this value was wrong which tripped me up
                2 => 2,
                1 => 1,
                0 => 1,
                // _ => 0,
                _ => panic!("aa"),
            };
            // let p = match consecutive_ones
            // {
            //     0 => 1,
            //     1 => 1,
            //     _ => 2usize.pow(consecutive_ones - 1) - 1,
            // }; 
            if !consecutive_count.contains_key(&p)
            {
                consecutive_count.insert(p, 1);
            }
            else
            {
                let new = consecutive_count[&p] + 1;
                consecutive_count.insert(p, new);
            }
            sum *= p;
            println!("sum {} (* {})", sum, p);

            consecutive_ones = 0;
        }
        else
        {
            // println!("invalid delta {}", delta);
        }
    }
    println!("count {:?}", consecutive_count);
    println!("sum {}", sum);

    Some(sum)
    
    // outer now at i 1, count is 0
    // outer now at i 2, count is 8232
    // outer now at i 3, count is 13720
    // outer now at i 4, count is 16464
    // outer now at i 5, count is 16464
    // outer now at i 6, count is 16464
}

fn brute_solution_part2(input: &str) -> Option<usize>
{
    solution(input);

    // going to do a brute force approach and see if I can simplify from there
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();
    data.sort();
    data.insert(0, 0);
    let max = data[data.len() - 1];
    data.push(max + 3);

    println!("input {:?}", data);

    fn inner(data: &Vec<usize>, starting: usize, outer: bool) -> usize
    {
        let mut prev_count = 0;
        let mut count = 0;
        // brute force
        for (i, x) in data.iter().enumerate()
        {
            if i < starting || i == data.len() - 1
            {
                continue;
            }
            // for each index, clone a copy with it removed and not removed
            // so long as it is valid
            if outer
            {
                println!("outer now at i {}, count is {} (+ {})", i, count, count - prev_count);
                prev_count = count;
            }
            
            fn is_valid(data: &Vec<usize>, start_index: usize) -> bool
            {
                let mut prev = 0;
                if start_index > 1
                {
                    prev = data[start_index - 1];
                }

                // for (i, x) in data.iter().enumerate()
                // for i in start_index..data.len()
                for i in start_index..start_index + 3
                {   
                    let x = data[i];
                    let diff = x - prev;
                    if diff > 3
                    {

                        return false;
                    }

                    prev = x;
                }
                return true;
            }

            // so this works but does not scale to fit
            // the answer
            // so what if instead of doing this we maintain a set
            // of numbers that can be toggled on and off
            // and use that for is_valid
            // this would be faster than cloning a bunch
            // actually this would not work because not all combinations
            // are valid
            // let same = data.clone();
            let mut removed = data.clone();
            removed.remove(i);

            let removed_valid = is_valid(&removed, i - 1);
            // println!("valid to remove idx {}? {}", i, removed_valid);

            // if is_valid(&same)
            // {
            //     // println!("valid {:?} same", same);
            //     // count += 1 + inner(&same, i);
            // }
            if removed_valid
            {
                 // println!("{:?} {} removed", removed, i);
                count += 1 + inner(&removed, i, false);
            }
        }
        return count;
    }

    let result = inner(&data, 1, true) + 1; // 1 for unmodified
    println!("result = {}", result);

    // None
    Some(result)

    // fn inner(index: usize, data: &Vec<usize>) -> usize
    // {
    //     if index >= data.len()
    //     {
    //         panic!("out of bounds");
    //     }

    //     let current = data[index];
    //     println!("at index {} ({})", index, current);
    //     let mut sum = 1;
    //     for offset in 1..4
    //     {
    //         let offset_index = index + offset;
    //         if offset_index == data.len() - 1
    //         {
    //             // let max = data[data.len() - 1];
    //             // if (max + 3 - current) > 3
    //             // {
    //             //     sum += 1;
    //             // }
    //             break;
    //         }
    //         if offset_index >= data.len()
    //         {
    //             break;
    //         }
    //         let delta = data[offset_index] - current;
    //         println!("delta of {} - {} (idx {} and {}) = {}", data[offset_index], current, offset_index, index, delta);
    //         if delta < 3
    //         {
    //             let additional = inner(offset_index, data);
    //             println!("adding {} permutations from index {}", additional, offset_index);
    //             sum += additional;
    //         }
    //     }

    //     return sum;
    // }

    // let result = inner(1, &data);

    // Some(result)
}

fn unused_solution_part2(input: &str) -> Option<usize>
{
    let mut data : Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().expect("failed to parse"))
        .collect();
    data.sort();

    let max = data[data.len() - 1];
    // data.insert(0, 0);
    data.push(max + 3);

    // 0 1 2 3 4
    // 0   2 3 4
    // 0     3 4
    // I think I can use integer division to divide into "steps"
    // 0 1 1 1 2 2 2

    // and then based on the number of permutations of that "step" I can make
    // 1 1 1
    // 1 1
    // 1   1
    //   1 1
    // 1
    //   1
    //     1
    // so 0b111 = 7
    // 0b11 = 3
    // 0b1 = 1

    // edit: this doesn't work, as you could have
    // 0 1 . . . . 2
    // which does not work

    // could do a look ahead type thing
    // if the sequence goes
    // 1 1 1 2 2 2
    // 1     2   
    //   1     2
    //     1     2
    // 1 1   2
    // 1 1     2 
    // 


    // lets just consider 1 2 3 4
    // 0 1 2 3 4 7
    // 0 1     4 7
    // 0 1 2   4 7
    // 0 1   3 4 7
    // 0   2 3 4 7
    // 0   2   4 7
    // 0     3 4 7

    // but if we insert 5 (this also means the max goes up with it)
    // then we actually cannot keep all of the same combinations
    // only the ones where 2 or 3 are included would work
    // so maybe that's the secret here

    // iterate for each x in data
    // but for each x determine the gaps in the data to determine the
    // possible combinations
    // like for gaps of 1 1 1 we can do 7 combinations as seen before
    // but for 1 2 1 
    // 0 1 3 4
    // 0 3 4
    // 0 1 4
    // there's only those combinations
    // and for 1 3 1
    // 0 1 4 5
    // there's only that one combination

    // 1 1 3:
    // 0 1 2 5
    // 0 2 5
    
    // so get the count of diffs where the sum is <= 3
    // and then use that to determine all of the combinations?


    // and then this number should be multiplied
    let mut sum = 1;
    let mut prev = 0;
    let mut prev_counter = 1;
    for (i, x) in data.iter().enumerate()
    {
        let mut lookahead_sum = 0;
        let mut lookahead_counter = 0;
        let mut lookahead_diffs = Vec::new();
        // look ahead by 3 or until the end of the vec
        for ahead_index in i+1..i+4
        {
            if ahead_index >= data.len()
            {
                break;
            }

            // println!("ahead index {} = {}", ahead_index, data[ahead_index]);
            // println!("ahead index {} (-1) = {}", ahead_index -1, data[ahead_index -1]);

            // get the delta of the current value to the next
            let lookahead_val = data[ahead_index] - data[ahead_index - 1];
            if lookahead_sum + lookahead_val > 3
            {
                break;
            }

            lookahead_diffs.push(lookahead_val);
            lookahead_sum += lookahead_val;
            lookahead_counter += 1;
        }
        println!("diffs {:?}", lookahead_diffs);
        println!("lookahead sum {} counter {} sum {}", lookahead_sum, lookahead_counter, sum);
        sum *= match lookahead_counter
            {
                3 => 7, // 3!
                2 => 4, // 2!
                1 => 1, // 1!
                0 => 1, // quietly ignore 0 case, means end of the search
                _ => panic!("aaahhH!!!")
            };
    }

    Some(sum)
}