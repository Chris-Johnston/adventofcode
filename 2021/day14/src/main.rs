use std::fs;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

struct Input
{
    template: Vec<char>,
    // rules: Vec<(char, char, char)>,
    rules: HashMap<(char, char), char>
}

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);
    
    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Input
{
    // first line is always the template
    let template = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .collect();

    let mut rules = HashMap::new();
    let re = Regex::new(r"([A-Za-z])([A-Za-z]) -> ([A-Za-z])").unwrap();

    for capture in re.captures_iter(input.trim())
    {
        let first = capture.get(1).unwrap().as_str().chars().next().unwrap();
        let second = capture.get(2).unwrap().as_str().chars().next().unwrap();
        let insert = capture.get(3).unwrap().as_str().chars().next().unwrap();

        // rules.push((first, second, insert));
        rules.insert((first, second), insert);
    }

    Input 
    {
        template: template,
        rules: rules,
    }
}


fn part1(input: &Input) -> usize
{
    println!("template {:?}", input.template);

    let mut polymer = input.template.clone();
    
    for step in 0..10
    {
        // (position, value)
        let mut to_insert = Vec::new();

        for c_index in 0..polymer.len() - 1
        {
            let current = polymer[c_index];
            let next = polymer[c_index + 1];

            let lookup = (current, next);

            if let Some(insert) = input.rules.get(&lookup)
            {
                to_insert.push((c_index + 1 + to_insert.len(), insert));
            }
        }

        // apply the insertions to the current polymer
        for (idx, insert) in to_insert
        {
            polymer.insert(idx, *insert);
        }

        println!("step {} : {:?}", step, polymer);
        // println!("step {} : len {}", step, polymer.len());
    }

    // get the unique elements
    let mut counts = HashMap::new();
    let mut unique = polymer.clone();
    unique.dedup();
    for element in unique.iter()
    {
        let count = polymer
            .iter()
            .filter(|x| *x == element)
            .count();
        counts.insert(element, count);
    }

    println!("counts {:?}", counts);

    let (_, most_common) = counts
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    let (_, least_common) = counts
        .iter()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    println!("most {} least {}", most_common, least_common);

    most_common - least_common
}

fn solution_next(input: &Input, steps: usize) -> usize
{
    println!("template {:?}", input.template);

    let mut groupings = HashMap::new();

    // tracks the last char, since otherwise the count would be off by one
    let mut last_char = 'z';
    for c_index in 0..input.template.len() - 1
    {
        let current = input.template[c_index];
        let next = input.template[c_index + 1];

        last_char = next;

        groupings.insert((current, next), 1);
    }

    for step in 0..steps
    {
        let mut new_groupings = HashMap::new();

        // for all of the grouping keys, perform the action required
        for key in groupings.clone().keys()
        {
            if input.rules.contains_key(key)
            {
                let middle = input.rules.get(key).unwrap();

                // get the count of the current groupings, since we'll multiply current count
                // when re-adding back to groupings
                let current_grouping_count = groupings.get(key).unwrap();

                let front_pair = (key.0, *middle);
                let back_pair = (*middle, key.1);

                let frontpair = new_groupings.entry(front_pair).or_insert(0);
                *frontpair += current_grouping_count;

                let backpair = new_groupings.entry(back_pair).or_insert(0);
                *backpair += current_grouping_count;
            }
        }

        groupings = new_groupings;

        println!("step {} groupings {:?}", step, groupings);
    }

    // count letters
    let mut letters = HashMap::new();

    for ((letter, _), val) in groupings.iter()
    {
        let count = letters.entry(letter).or_insert(0);
        *count += val;
    }

    // account for off by one
    let c = letters.entry(&last_char).or_insert(0);
    *c += 1;

    println!("counts {:?}", letters);

    // just had this idea in the shower
    // instead of treating this as a whole set of strings
    // intead make a dict<(char, char), usize> which tracks pairs of chars
    // and tracks the count
    // as new points are inserted A, B -> C,
    // add to this dict AC CB and increment if not yet exists
    // then sum up by taking the first char of each of these, but also the last char
    // and ba-da bing
    // use the count of each pair to determine the count by multiplication

    let (_, most_common) = letters
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    let (_, least_common) = letters
        .iter()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    println!("most {} least {}", most_common, least_common);

    most_common - least_common
}

fn part2(input: &Input) -> usize
{
    return solution_next(&input, 40);

    // idea before I go to bed
    // we can take each of the input combinations and solve for the combined sum
    // so ABC turns into AB and BC
    // these might expand into AXB and BYC
    // which will continue to expand, we can solve these for 40 iterations
    // and then add the sums together at the end, excluding the shared 'b' between them

    // just had this idea in the shower
    // instead of treating this as a whole set of strings
    // intead make a dict<(char, char), usize> which tracks pairs of chars
    // and tracks the count
    // as new points are inserted A, B -> C,
    // add to this dict AC CB and increment if not yet exists
    // then sum up by taking the first char of each of these, but also the last char
    // and ba-da bing
    // use the count of each pair to determine the count by multiplication
    println!("template {:?}", input.template);

    let mut polymer = input.template.clone();
    
    for step in 0..40 // part 2
    {
        // (position, value)
        let mut to_insert = Vec::new();

        for c_index in 0..polymer.len() - 1
        {
            let current = polymer[c_index];
            let next = polymer[c_index + 1];

            let lookup = (current, next);

            if let Some(insert) = input.rules.get(&lookup)
            {
                to_insert.push((c_index + 1 + to_insert.len(), insert));
            }
        }

        // apply the insertions to the current polymer
        for (idx, insert) in to_insert
        {
            polymer.insert(idx, *insert);
        }

        let mut counts = HashMap::new();
    let mut unique = polymer.clone();
    unique.dedup();
    for element in unique.iter()
    {
        let count = polymer
            .iter()
            .filter(|x| *x == element)
            .count();
        counts.insert(element, count);
    }

    println!("{} counts {:?}", step, counts);

        // println!("step {} : {:?}", step, polymer);
        // println!("step {} : len {}", step, polymer.len());
    }

    // get the unique elements
    let mut counts = HashMap::new();
    let mut unique = polymer.clone();
    unique.dedup();
    for element in unique.iter()
    {
        let count = polymer
            .iter()
            .filter(|x| *x == element)
            .count();
        counts.insert(element, count);
    }

    println!("counts {:?}", counts);

    let (_, most_common) = counts
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    let (_, least_common) = counts
        .iter()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    println!("most {} least {}", most_common, least_common);

    most_common - least_common
}

#[test]
fn example()
{
    let input =
    "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";
    let input = parse_input(input);
    assert_eq!(part1(&input), 1588);

    assert_eq!(solution_next(&input, 10), 1588);
    assert_eq!(solution_next(&input, 40), 2188189693529);
    // assert_eq!(part2(&input), 2188189693529);
}