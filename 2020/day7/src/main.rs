use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
#[macro_use] extern crate lazy_static;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

// how many bag colors can eventually contain at least one shiny gold bag

// thinking I'll have to build a dict for this data structure
// and then dig through each of the contents (have a check to prevent recursion)
// for each of the keys to see if they can contain a shiny gold bag
// the number doesn't count right now but it seems like it will later on

const EXAMPLE_ANSWER_1: usize = 4;
const EXAMPLE_ANSWER_2: usize = 32;

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
    assert!(answer == 144);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer == 5956);
}

fn parse_bags(input: &str) -> HashMap<&str, HashMap<&str, usize>>
{
    // parse the input
    // maybe I can cheat this, and not match against X contain no other bags
    // and just say if it doesn't exist in the dict then assume it contains no other bags

    // going to be easier to use two regexes instead of one big one
    // unused: ^(\w+ \w+) bags contain ((\d+) (\w+ \w+) bags?,? ?)*.$
    lazy_static! {
        static ref bag_name: Regex = Regex::new(r"(\w+ \w+) bags").unwrap();
        static ref rule: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?,? ?").unwrap();
    }
    let mut bag_rules = HashMap::new();

    for rules in input.split("\n")
    {
        
        let parts : Vec<&str> = rules.split("contain").collect();
        if parts.len() < 2
        {
            println!("weird rule: {}", rules);
            continue;
        }
        let bag_name_part = parts[0];
        let rules_part = parts[1];

        let name = bag_name
            .captures(bag_name_part)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        if rule.is_match(rules_part)
        {
            // has matching rules
            let mut rules = HashMap::new();

            for caps in rule.captures_iter(rules_part)
            {
                let rule_name = caps.get(2).unwrap().as_str();
                let rule_amount = caps.get(1).unwrap().as_str()
                    .parse::<usize>().expect("failed to parse the amount");
                println!("{} bag has rule {} {}", name, rule_name, rule_amount);

                rules.insert(rule_name, rule_amount);
            }

            bag_rules.insert(name, rules);
        }
        // otherwise do not insert this key into the map, does not have any capacity for other bags
    }

    bag_rules
}

fn solution(input: &str) -> Option<usize>
{
    let bag_rules = parse_bags(input);

    // this approach is not right, do it the longer way
    // // now for all keys in the map, count how many can contain
    // let mut to_inspect_stack = Vec::new();
    // let mut can_contain_gold_bag = HashSet::new();


    // for key in bag_rules.keys()
    // {
    //     to_inspect_stack.push(key);
    // }

    // while !to_inspect_stack.is_empty()
    // {
    //     let name = to_inspect_stack.pop().expect("failed to pop value");

    //     println!("inspecting {}", name);
    //     if bag_rules.contains_key(name)
    //     {
    //         // rules exist for this key
    //         let sub_rules = bag_rules.get(name).expect("failed to get values for key");
    //         for key in sub_rules.keys()
    //         {
    //             if *key == "shiny gold"
    //             {
    //                 can_contain_gold_bag.insert(name);
    //             }
    //             else
    //             {
    //                 to_inspect_stack.push(key);
    //             }
    //         }
    //     }
    // }

    // now for all keys in the map, count how many can contain
    let mut can_contain_gold_bag = HashSet::new();

    for key in bag_rules.keys()
    {
        // println!("BAG ----- {}", key);
        let mut to_inspect_stack = Vec::new();
        to_inspect_stack.push(key);
        while !to_inspect_stack.is_empty()
        {
            let name = to_inspect_stack.pop().expect("failed to pop value");

            // println!("inspecting {}", name);
            if bag_rules.contains_key(name)
            {
                // rules exist for this key
                let sub_rules = bag_rules.get(name).expect("failed to get values for key");
                for sub_key in sub_rules.keys()
                {
                    // println!("\t sub key {}", *sub_key);
                    if *sub_key == "shiny gold" || can_contain_gold_bag.contains(sub_key)
                    {
                        // println!("------ has the shiny gold");
                        can_contain_gold_bag.insert(key);
                    }
                    else
                    {
                        to_inspect_stack.push(sub_key);
                    }
                }
            }
        }
    }

    Some(can_contain_gold_bag.len())
}

// this part was tricky just because I didn't read the question carefully enough
// I thought that this was a modification of part 1, but actually despite the methods being the same
// the question being asked was different
// so i could have done some copy paste but instead it was easier to just redo it using the same methods
fn solution_part2(input: &str) -> Option<usize>
{
    let bag_rules = parse_bags(input);
    let mut total = 0;

    let gold_bag_rule = bag_rules.get("shiny gold")
        .expect("What??? no shiny gold???");
    
    let mut inspect_stack = Vec::new();
    for k in gold_bag_rule.keys()
    {
        inspect_stack.push((k, gold_bag_rule[k]));
    }

    while !inspect_stack.is_empty()
    {
        let child_bag = inspect_stack.pop().expect("couldn't pop");
        if bag_rules.contains_key(child_bag.0)
        {
            let child_bag_rules = &bag_rules[child_bag.0];
            for k in child_bag_rules.keys()
            {
                inspect_stack.push((k, child_bag.1 * child_bag_rules[k]));
            }
        }
        
        // otherwise bag contains nothing else
        println!("adding {} {} bags to total {}", child_bag.1, child_bag.0, total);
        total += child_bag.1;
    }

    Some(total)
}