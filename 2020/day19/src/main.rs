use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

const EXAMPLE_ANSWER_1: isize = 2; // 2 rules match rule 0
const EXAMPLE_ANSWER_2: isize = 2;

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    // let example_solution = solution(EXAMPLE_INPUT)
    //     .expect("no result");
    
    //println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    // assert!(EXAMPLE_ANSWER_1 == example_solution);

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    // assert!(answer == 42069);

    // part 1 is 213
    // part 2 is more than 213

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

fn parse_rules(input: &str) // assumed split by blank line
-> (HashMap<usize, char>, HashMap<usize, Vec<Vec<usize>>>) {
    // rules which map to a literal character
    let mut literal_rules = HashMap::new();
    // rules which map to other rules
    let mut nested_rules = HashMap::new();

    for line in input.lines()
    {
        let mut parts = line.split(":");
        let number = parts.next().unwrap().parse::<usize>().unwrap();
        let rules = parts.next().unwrap();

        if rules.contains("\"")
        {
            // match against a single literal character
            let char_idx = rules.find('"').unwrap() + 1;
            let m = rules.chars().nth(char_idx).unwrap();

            println!("{}: '{}'", number, m);
            literal_rules.insert(number, m);
        }
        else
        {
            let mut rule_definitions = Vec::new();
            // nested pattern
            for subrules in rules.split("|")
            {
                // parse each rule
                let subrule_definition : Vec<usize> = subrules.split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                rule_definitions.push(subrule_definition);
            }

            println!("{}: {:?}", number, rule_definitions);
            nested_rules.insert(number, rule_definitions);
        }
    }

    (literal_rules, nested_rules)
}

// check to see if it matches expression with the given rule number
fn eval_rule_number(expression: &str, rule_num: usize, literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>) -> bool
{
    // starting index is where in the str to start from, for nested rules
    // and num match is the number of chars to match against in the case of a nested rule being followed by another
    fn eval_rule_number_inner(expression: &str, rule_num: usize, starting_index: usize, num_match: usize, literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>)
        -> bool
    {
        let mut char_iter = expression.trim()
            .get(starting_index..num_match)
            .unwrap()
            .chars();

        if literal_rules.contains_key(&rule_num)
        {
            // check that first char is the literal char
            let next_char = char_iter.next().unwrap();
            let match_char = literal_rules[&rule_num];

            return next_char == match_char;
        }
        else
        {
            // match against possible rules
            let nested_rules = &nested_rules[&rule_num];

            // might have to worry about the branching scenario in which two nested rules match
            // but only one actually applies
            // in which case it would be expression tree time!!!11!!
            // actually that seems most likely to be the solution
            for sub_rule in nested_rules
            {
                let mut idx_counter = starting_index;
                for criteria in sub_rule
                {
                    // eval_rule_number(expression: &str, rule_num: usize, literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>)
                }
            }
        }

        false
    }

    false
}

struct ExpressionTreeNode
{
    literal: Option<char>,
    valid_end: bool,
    children: HashMap<char, ExpressionTreeNode>,
}

impl Clone for ExpressionTreeNode
{
    fn clone(&self) -> ExpressionTreeNode
    {
        ExpressionTreeNode
        {
            literal: self.literal,
            valid_end: self.valid_end,
            children: self.children.clone()
        }
    }
}

impl ExpressionTreeNode
{
    fn literal(c: char) -> ExpressionTreeNode
    {
        ExpressionTreeNode
        {
            literal: Some(c),
            valid_end: false,
            children: HashMap::new(),
        }
    }

    fn deep_clone(&self) -> ExpressionTreeNode
    {
        let mut children : HashMap<char, ExpressionTreeNode> = HashMap::new();
        for (k, v) in self.children.clone()
        {
            let cloned_v = v.deep_clone();
            children.insert(k, cloned_v);
        }


        return ExpressionTreeNode
        {
            literal: self.literal,
            valid_end: self.valid_end,
            children: children,
        };
    }
}

fn build_expression_tree(literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>)
{
    // trying to do too much in a single function
    // need to have a function which can handle a rule like
    // 1 3 | 3 1
    // and expand these into the literals
    // a b | b a
    // and then reutnr the hashmap of children for another node

    // let mut current_rule = nested_rules.get(&0).unwrap();

    // this should maintain each of the rules for which we have generated the expression tree
    // so will start with just the ones which are literals
    let mut treeheads = HashMap::new();

    for (&key, &value) in literal_rules
    {
        let mut literal_node = ExpressionTreeNode::literal(value);
        treeheads.insert(key, literal_node);
    }

    // this is the stack of the dependent trees that have to be determined next before the current one can be
    // for example, if key 3 depends on 4, then we will stop creating 3 (not ideal probably), push 3 and push 4
    // I do not have a way of determining infinite loops but I assume that these will not exist in the dataset
    // to start, init this with just 0
    let mut dependency_stack = VecDeque::new();

    dependency_stack.push_front(0);

    while dependency_stack.len() > 0
    {
        let key = dependency_stack.pop_front().unwrap();

        let mut current_rule = nested_rules.get(&key).unwrap();
        for sub_rule in current_rule
        {
            for part in sub_rule
            {
                // building this very very big tree might get expeeeensive
                // need to be able to append to the tails
                

                // in hindsight just had another idea
            }
        }
    }
}


// this other idea is like the tree idea, but instead of a bunch of nodes, generate lists of all possible
// permutations of strings

// so "a" | "ba" | aaa
// expands into 3 strings - a and ba and aaa

fn generate_expressions(literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>)
-> std::collections::HashMap<usize, HashSet<std::string::String>> {
    let mut treeheads = HashMap::new();
    for (&key, &value) in literal_rules
    {
        let mut literal = HashSet::new();
        // literal.push(value.to_string().as_str());
        let v = String::from(value);
        literal.insert(v);
        treeheads.insert(key, literal);
    }

    // this is the stack of the dependent trees that have to be determined next before the current one can be
    // for example, if key 3 depends on 4, then we will stop creating 3 (not ideal probably), push 3 and push 4
    // I do not have a way of determining infinite loops but I assume that these will not exist in the dataset
    // to start, init this with just 0
    let mut dependency_stack = VecDeque::new();

    dependency_stack.push_front(0);
    dependency_stack.push_front(31);
    dependency_stack.push_front(42);
    dependency_stack.push_front(8);

    while dependency_stack.len() > 0
    {
        let key = dependency_stack.pop_front().unwrap();
        println!("Checking {}", key);

        let current_rule = &nested_rules[&key];
        let mut all_defined = true;

        // if already in the treeheads, then skip
        if treeheads.contains_key(&key)
        {
            println!("Skipping {}, already eval'ed", key);
            continue;
        }

        let mut current = HashSet::new();

        if key == 11
        {
            println!("RULE 11");
            // append pairs of 42's and 31's
            for pairs in 1..10
            {
                println!("pairs: {}", pairs);
                let fourtytwo = &treeheads[&42];
                let thirtyone = &treeheads[&31];

                    for append42 in fourtytwo
                    {
                        for append31 in thirtyone
                        {
                            let mut si = String::new();
                            for _ in 0..pairs
                    {
                            si.push_str(append42);
                    }
                    for _ in 0..pairs
                    {
                        
                            si.push_str(append31);
                    }

                    // println!("pushing {}", si);

                    if si.len() < 100
                    {
                        break;
                    }
                    else{
                        current.insert(si);
                    }
                        }
                    }
            }

            treeheads.insert(key, current);
            continue;
        }
        // else if key == 8
        // {
        //     let mut current = HashSet::new();
        //     let fourtytwo = &treeheads[&42];
        //     for _ 0..100
        //     {
        //         let mut si = String::new();

        //         for a in fourtytwo
        //         {
        //             si.push_str(a);
        //         }
        //         si.push_str(fourtytwo);
        //     }

        //     treeheads.insert(key, current);
        //     continue;
        // }
        for sub_rule in current_rule
        {
            println!("\tsub rule {:?}", sub_rule);
            let mut s : Vec<String> = Vec::new();
            for part in sub_rule
            {
                println!("\tpart {:?}", part);
                // handle possibilities for this rule
                if treeheads.contains_key(&part)
                {
                    let mut combinations = Vec::new();

                    let mut first_pass = s.len() == 0;

                    // the data for this rule exists
                    // add to s every item in treeheads
                    for append in &treeheads[&part]
                    {
                        // println!("append {}", append);
                        for si in &s
                        {
                            let mut si = String::from(si);
                            si.push_str(append.as_str());
                            // println!("si: {}", si);
                            combinations.push(si.clone());

                            // println!("si len {}", si.len());

                            if si.len() > 100
                            {
                                break;
                            }

                            // special cases for part 2
                            // this is such a hack lol
                            // if key == 8
                            // {
                            //     // this part fails because there are different cobinations for 42
                            //     // println!("special case for 8");
                            //     for x in 0..100
                            //     {
                            //         si.push_str(append.as_str());
                            //         println!("{}", si);
                            //         combinations.push(si.clone());
                            //     }
                            // }
                        }

                        if first_pass
                        {
                            // just append without modification
                            combinations.push(append.clone());
                        }
                    }
                    println!("\tpart {:?} combinations {:?}", part, combinations.len());
                    s = combinations;
                }
                else
                {
                    println!("\tpart {:?} was not defined", part);
                    // doesn't exist for this rule, put back on the stack
                    dependency_stack.push_front(key);
                    dependency_stack.push_front(*part);
                    all_defined = false;
                    break;
                }
            }

            // current.append(&mut s);
            for x in s
            {
                if x.len() < 100
                {
                current.insert(x);}
            }
        }

        if all_defined
        {
            println!("Set tree {} == {:?}", key, current.len());
            treeheads.insert(key, current);
        }
        else
        {
            println!("tried to get through {} but there was a dependency missing", key);
        }
    }

    treeheads
}


fn solution(input: &str) -> Option<isize>
{
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let expressions = split.next().unwrap();

    let (literal_rules, nested_rules) = parse_rules(rules);

    // need to build an expression tree from this that matches against the literal chars
    // and accounts for the loops
    // where the nodes can track for which ones are the potential "ends"
    // also interesting to note, is that the characters are only either a or b
    // so that might be something I can design for, but could also be something that breaks horribly in part 2

    println!("literal rules: {:?}", literal_rules);
    println!("nested rules: {:?}", nested_rules);

    let g = generate_expressions(&literal_rules, &nested_rules);
    println!("x: {:?}", g);

    let mut count = 0;
    for expression in expressions.lines()
    {
        let possible = g.get(&0).unwrap();

        if possible.contains(expression)
        {
            count += 1;
        }

        // for x in possible
        // {
        //     if x == expression
        //     {
        //         count += 1;
        //         break;
        //     }
        // }
    }

    Some(count)
}

fn solution_part2(input: &str) -> Option<isize>
{
    None
}