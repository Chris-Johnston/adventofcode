use std::fs;
use std::cmp::{min, max};
use regex::Regex;
use std::collections::{HashMap, VecDeque};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    // let answer = part1(&input);
    // println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 2 {}", answer);
    // NOT 19153
    // NOT 15961
    assert!(answer != 19153);
    assert!(answer != 15961);
}

fn parse_input(input: &str) -> Vec<usize>
{
    // let input_re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    input
    .split(",")
    .map(str::trim)
    .map(|x| x.parse::<usize>().unwrap())
    .collect()
}

// fn part1(input: &[usize]) -> usize
// {
//     let mut numbers : Vec<usize> = input.to_vec();
//     // brute force
//     for day in 0..80
//     {
//         let mut newnumbers : Vec<usize> = Vec::new();
//         println!("day {} {:?}", day, numbers);

//         for x in numbers
//         {
//             if x == 0
//             {
//                 newnumbers.push(6);
//                 newnumbers.push(8);
//             }
//             else
//             {
//                 newnumbers.push(x - 1);
//             }
//         }

//         numbers = newnumbers;
//     }

//     numbers.len()
// }

fn part1(input: &[usize]) -> usize
{
    solution(input, 80)
}

// fn part2(input: &[usize]) -> usize
// {
//     let mut numbers : Vec<usize> = input.to_vec();
//     // brute force
//     for day in 0..256
//     {
//         let mut newnumbers : Vec<usize> = Vec::new();
//         println!("day {} {:?}", day, numbers);

//         for x in numbers
//         {
//             if x == 0
//             {
//                 newnumbers.push(6);
//                 newnumbers.push(8);
//             }
//             else
//             {
//                 newnumbers.push(x - 1);
//             }
//         }

//         numbers = newnumbers;
//     }

//     numbers.len()
// }

fn part2(input: &[usize]) -> usize
{
    solution(input, 256)
}

fn solution(input: &[usize], days: usize) -> usize
{
    // seeing a pattern here, once the inital numbers tick down after 0
    // there is a new fish added which will increase in 7 days
    // but also there will be a new fish added in 9 days

    // and we can treat each number in the input as a separate equation
    // to make it simpler
    // and re-use the answers, because an input of 3 will be the same each time

    // build a dict where the key is the input number
    // and the value is the count of occurances (when summing it back up)

    // for now: see if we can brute force the single number

    let mut inputcounts : HashMap<usize, usize> = HashMap::new();
    let mut answers : HashMap<usize, usize> = HashMap::new();

    for x in input
    {
        let count = inputcounts.entry(*x).or_insert(0);
        *count += 1;
    }

    println!("inputs {:?}", inputcounts);

    for key in inputcounts.keys()
    {
        println!("key {}", key);
        // let mut numbers : Vec<usize> = vec![*key];

        // let mut days_diff : Vec<usize> = vec![0; days];

        // maybe for each of the inputs, we can calculate the number of
        // fish that were added each day
        // so there is one added for the input
        // and then another added every 7 after that
        // for the number that are added

        // RECURSION!!
        // fn reset_fish_index(index: usize, days: usize, days_diff: &mut Vec<usize>)
        // {   
        //     if index >= days
        //     {
        //         return;
        //     }

        //     // add a fish at the given day, which means
        //     // a new fish

        //     // add a new fish at the given day
        //     new_fish_index(index, days, days_diff);

        //     reset_fish_index(index + 7, days, days_diff);

        // }

        // fn new_fish_index(index: usize, days: usize, days_diff: &mut Vec<usize>)
        // {
        //     if index >= days
        //     {
        //         return;
        //     }

        //     days_diff[index] += 1; // I think the issue is I'm adding this by hand, that's why it's so slow

        //     reset_fish_index(index + 9, days, days_diff);
        // }

        // reset_fish_index(*key, days, &mut days_diff);

        // println!("days diff {:?}", days_diff);
        // sum up days_diff
        //let answer : usize = days_diff.iter().sum();
        // answers.insert(*key, answer + 1);

        // this is not my solution, I stole it from discussion after solving pt 2
        // let mut numbers = vec![0; 9]; // position is the number of days until new spawn, value is the count
        let mut numbers = VecDeque::from(vec![0; 9]);
        // numbers.insert(*key, 1);
        let elem = numbers.get_mut(*key).unwrap();
        *elem = 1;

        for day in 0..days
        {
            // println!("{:?}", numbers);
            // rotate to the left
            let spawning_count = numbers.pop_front().unwrap();

            // spawning count has incremented
            numbers.push_back(spawning_count);

            // re-add the spawning count
            let elem = numbers.get_mut(6).unwrap();
            *elem += spawning_count;
        }

        let answer = numbers.iter().sum();

        // // brute force -- ok, will have to do this smarter
        // for day in 0..days
        // {
        //     println!("key {} day {}", key, day);

        //     let mut newnumbers : Vec<usize> = Vec::new();
        //     // println!("day {} {:?}", day, numbers);

        //     for x in numbers
        //     {
        //         if x == 0
        //         {
        //             newnumbers.push(6);
        //             newnumbers.push(8);
        //         }
        //         else
        //         {
        //             newnumbers.push(x - 1);
        //         }
        //     }

        //     numbers = newnumbers;
        // }

        answers.insert(*key, answer);
    }

    let mut sum = 0;

    println!("answers {:?}", answers);

    for (key, val) in answers.iter()
    {
        let mul = inputcounts[key];
        sum += val * mul;
    }

    sum
}



#[test]
fn example()
{
    let input = "3,4,3,1,2";
    let input = parse_input(input);
    assert_eq!(solution(&input, 18), 26);
    assert_eq!(part1(&input), 5934);
    assert_eq!(part2(&input), 26984457539);
}