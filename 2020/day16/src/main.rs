use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
#[macro_use] extern crate lazy_static;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

const EXAMPLE_ANSWER_1: usize = 71;
const EXAMPLE_ANSWER_2: usize = 12 * 11 * 13;

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
    // let example_solution = solution_part2(EXAMPLE_INPUT)
    //     .expect("no result");

    // solution_part2(EXAMPLE_INPUT);

    // println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    // assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    // assert!(answer == 1355323200);
}

fn parse_rules(input: &str) -> std::vec::Vec<(std::string::String, usize, usize, usize, usize)> {
    lazy_static! {
        static ref rule_regex: Regex = Regex::new(r"(?m)^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let mut results = Vec::new();

    for cap in rule_regex.captures_iter(input)
    {
        let rule_name = &cap[1];
        let rule_name = String::from(rule_name);
        let first_lower_bound = &cap[2].parse::<usize>().expect("failed to parse bound");
        let first_upper_bound = &cap[3].parse::<usize>().expect("failed to parse bound");
        let last_lower_bound = &cap[4].parse::<usize>().expect("failed to parse bound");
        let last_upper_bound = &cap[5].parse::<usize>().expect("failed to parse bound");

        results.push((rule_name, *first_lower_bound, *first_upper_bound, *last_lower_bound, *last_upper_bound));
    }
    results
}

fn get_nearby_tickets(input: &str)
-> std::vec::Vec<Vec<usize>> {
    let mut results = Vec::new();

    let nearby_tickets_line = input
        .lines()
        .enumerate()
        .filter(|&(_, x)| x == "nearby tickets:")
        .nth(0)
        .expect("failed to find line").0;

    for line in input
        .lines()
        .skip(nearby_tickets_line + 1)
        {
            // duplicate numbers satisfy the same requirements
            // so use that to optimize slightly?

            // part 2 depends on the order
            let split : Vec<usize> = line.split(",").map(|x| x.parse::<usize>().expect("falied to parse"))
                .collect();
            results.push(split);
        }

    results
}

fn get_my_ticket(input: &str)
-> Option<Vec<usize>> {

    let nearby_tickets_line = input
        .lines()
        .enumerate()
        .filter(|&(_, x)| x == "your ticket:")
        .nth(0)
        .expect("failed to find line").0;

    for line in input
        .lines()
        .skip(nearby_tickets_line + 1)
        {
            // duplicate numbers satisfy the same requirements
            // so use that to optimize slightly?

            // update: p2 depends on order, so go back to a set
            let split : Vec<usize> = line.split(",").map(|x| x.parse::<usize>().expect("falied to parse"))
                .collect();
            return Some(split);
        }
    None
}

fn solution(input: &str) -> Option<usize>
{
    let rules = parse_rules(input);
    let nearby_tickets = get_nearby_tickets(input);

    println!("rules: {:?}", rules);
    println!("nearby tickets: {:?}", nearby_tickets);

    // add all fields which are invalid for any rule
    let mut error_rate = 0;

    // find all tickets which aren't valid for any field
    for ticket in nearby_tickets
    {
        for field in ticket
        {
            let mut has_valid = false;
            for rule in &rules
            {
                // check validity of this rule
                let (_, first_lower, first_upper, last_lower, last_upper) = rule;

                let is_valid = (field >= *first_lower &&
                               field <= *first_upper) ||
                               (field >= *last_lower && field <= *last_upper);
                has_valid |= is_valid;
            }

            if !has_valid 
            {
                error_rate += field;
            }
        }
    }

    Some(error_rate)
}

fn solution_part2(input: &str) -> Option<usize>
{
    let rules = parse_rules(input);
    let nearby_tickets = get_nearby_tickets(input);
    let my_ticket = get_my_ticket(input).unwrap();

    println!("rules: {:?}", rules);
    println!("nearby tickets: {:?}", nearby_tickets);
    println!("my ticket {:?}", my_ticket);

    // for each valid ticket, match which indexes matched against
    // and union with each one to determine what is valid
    // then by the time its done it'll have the value for our ticket
    let mut rule_indexes = HashMap::new();

    for rule in &rules
    {
        let mut set : HashSet<usize> = HashSet::new();
        for x in 0..rules.len()
        {
            set.insert(x);
        }
        rule_indexes.insert(&rule.0, set);
    }

    // find all tickets which aren't valid for any field
    for ticket in nearby_tickets
    {
        for (idx, &field) in ticket.iter().enumerate()
        {
            let mut has_valid = false;
            for rule in &rules
            {
                // check validity of this rule
                let (name, first_lower, first_upper, last_lower, last_upper) = rule;

                let is_valid = (field >= *first_lower &&
                               field <= *first_upper) ||
                               (field >= *last_lower && field <= *last_upper);
                has_valid |= is_valid;
            }

            if has_valid
            {
                // only if this ticket is valid
                // get all of the rules where this matched
                for rule in &rules
                {
                    let (name, first_lower, first_upper, last_lower, last_upper) = &rule;

                    let is_valid = (field >= *first_lower &&
                                field <= *first_upper) ||
                                (field >= *last_lower && field <= *last_upper);

                    if !is_valid
                    {
                        println!("removing {} from {}", idx, name);

                        // if invalid, remove this index from rule_indexes if exists
                        // rule_indexes[&name].insert(idx);
                        rule_indexes.get_mut(name)
                            .expect("did not find rule in dict")
                            .remove(&idx);
                    }
                    else
                    {
                        // valid, insert into rule_indexes

                        // this should only insert if valid for everything
                        // rule_indexes.get_mut(name)
                        //     .expect("did not find rule in dict")
                        //     .insert(&idx);
                    }
                }

            }
        }
    }

    println!("result: {:?}", rule_indexes);

    // for example this returns
    // {"seat": {0, 2}, "row": {0, 1}, "class": {1}}

    // there is probably a way to determine this smarter, given that class is definitely 1
    // and that elimintates that from being the option for row, which then eliminates 0 as an 
    // option for seat (maybe that's the algo right there)
    // not sure if this is necssary to solve here, can just cheat it manually

    // edit nevermind, I totally have to implement this, even after smarter filtering it's too much to do by hand
    let mut solved_rules = HashMap::new();
    let len = rule_indexes.len();

    fn pprint<K: std::fmt::Debug, V: std::fmt::Debug>(map: &HashMap<K, V>)
    {
        for (k, v) in map.iter() {
            println!("{:?}: {:?},", k, v);
        }
    }

    while solved_rules.len() < len
    {
        println!("RULES:");
        pprint(&rule_indexes);

        println!("SOLVED:");
        pprint(&solved_rules);

        let count = rule_indexes.iter().clone()
        .filter(|&(k, v)| v.len() == 1 && !solved_rules.contains_key(k)) //  && k.contains("departure")
        .count();

        if count != 1
        {
            println!("found {} matches", count);
            break;
        }

        // find all of the ones in which only one index is valid
        let x = rule_indexes.iter().clone()
            .filter(|&(k, v)| v.len() == 1 && !solved_rules.contains_key(k)) //  && k.contains("departure")
            .nth(0);

        if x == None{
            break;
        }

        let (rule_name, rule_values) = x.unwrap();
        // from all of the other rules, remove the only match
        let to_remove = rule_values.iter().nth(0)
            .unwrap().clone();
        println!("rule {:?} => {}", rule_name, to_remove);
        solved_rules.insert(rule_name.clone(), to_remove.clone());

        for (k, v) in rule_indexes.iter_mut()
        {
            v.remove(&to_remove);
        }
    }
    println!("solved rules: {:?}", solved_rules);

    if rules.len() != 3
    {
        // determine the answer
        fn lookup(input: &str, solve: &HashMap<&String, usize>, ticket: &Vec<usize>) -> usize
        {
            // get the index from the name
            let input = String::from(input);
            let i = solve[&input];

            ticket[i]
        }

        let result = lookup("departure date", &solved_rules, &my_ticket) *
        lookup("departure time", &solved_rules, &my_ticket) * 
        lookup("departure platform", &solved_rules, &my_ticket) * 
        lookup("departure station", &solved_rules, &my_ticket) * 
        lookup("departure track", &solved_rules, &my_ticket) * 
        lookup("departure location", &solved_rules, &my_ticket);

        // 644 too low, wasn't multiplying
        // 1053686852011
        println!("ANSWER: {}", result);
    }

    None
}