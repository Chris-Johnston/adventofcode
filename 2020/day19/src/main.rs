use std::fs;
use std::collections::HashMap;

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

    let example_solution = solution(EXAMPLE_INPUT)
        .expect("no result");
    
    println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    assert!(EXAMPLE_ANSWER_1 == example_solution);

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    // assert!(answer == 42069);

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
}

fn build_expression_tree(literal_rules: &HashMap<usize, char>, nested_rules: &HashMap<usize, Vec<Vec<usize>>>)
{
    // trying to do too much in a single function
    // need to have a function which can handle a rule like
    // 1 3 | 3 1
    // and expand these into the literals
    // a b | b a
    // and then reutnr the hashmap of children for another node

    let mut current_rule = nested_rules.get(&0).unwrap();
    let mut treeheads = HashMap::new();

    for (&key, &value) in literal_rules
    {
        let mut literal_node = ExpressionTreeNode::literal(value);
        treeheads.insert(key, literal_node);
    }

    for sub_rule in current_rule
    {
        for part in sub_rule
        {

        }
    }
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

    let mut count = 0;
    for expression in expressions.lines()
    {
        if eval_rule_number(expression, 0, &literal_rules, &nested_rules)
        {
            count += 1;
        }
    }

    Some(count)
}

fn solution_part2(input: &str) -> Option<isize>
{
    None
}