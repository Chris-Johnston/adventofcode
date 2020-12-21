use std::fs;
use std::collections::VecDeque;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"<input goes here>";

const EXAMPLE_ANSWER_1: usize = 1;
const EXAMPLE_ANSWER_2: usize = 2;

fn main() {

    let test_cases = vec![
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("1 + 2 * 3 + 4 * 5 + 6", 71),
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];

    for (case, result) in test_cases
    {
        println!("testing case: {} == {}", case, result);
        let actual = eval(case);
        println!("actual: {} ", actual);
        assert!(actual == result);
    }


    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    // let example_solution = solution(EXAMPLE_INPUT)
    //     .expect("no result");
    
    // println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    // assert!(EXAMPLE_ANSWER_1 == example_solution);

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

// struct TreeNode
// {
//     operation: char,
//     value: usize,
//     is_value: bool,
//     left: Option<Box<TreeNode>>,
//     right:  Option<Box<TreeNode>>,
// }

fn eval(input: &str) -> usize
{
    fn eval_inner(input: &str, start: usize) -> (usize, usize)
    {
        println!("inner: {} {}", input, start);

        let mut result = 0;

        let mut op_stack = VecDeque::new();
        let mut val_stack = VecDeque::new();

        let mut i = start;
        
        loop
        {
            if i >= input.len()
            {
                println!("end of input");
                break;
            }


            let chr = input.chars().nth(i)
                .unwrap();

            // if is numeric, walk ahead to get the rest of the digits
            if chr.is_digit(10)
            {
                let mut value = chr.to_digit(10).unwrap() as usize;

                // need to include bounds check here as well
                while i + 1 < input.len() && input.chars().nth(i + 1).unwrap().is_digit(10)
                {
                    value *= 10;
                    i += 1;
                    value += input.chars().nth(i).unwrap() as usize;
                }

                // val_stack.insert(val_stack.len(), value); 
                val_stack.push_back(value);

                // val_stack.insert(val_stack.len() - 1, value);
                // val_stack.push(value);
            }
            else if chr == '('
            {
                // start recursion
                let (paren_result, new_i) = eval_inner(input, i + 1);
                val_stack.push_back(paren_result);
                // val_stack.insert(val_stack.len(), paren_result); 

                i = new_i;
            }
            else if chr == ')'
            {
                println!("end paren");
                // compute existing values
                // and return the result
                break;
            }
            else if chr == '*'
            {
                // op_stack.insert(op_stack.len() - 1, '*');
                op_stack.push_back('*');
            }
            else if chr == '+'
            {
                // op_stack.insert(op_stack.len() - 1, '+');
                op_stack.push_back('+');
            }
            i += 1;
        }

        println!("val {:?}", val_stack);
        println!("op {:?}", op_stack);

        // compute the result from the op stack
        if val_stack.len() == 0
        {
            println!("empty val stack");
            return (0, 0);
        }

        while val_stack.len() > 1
        {
            let a = val_stack.pop_front().unwrap();
            let b = val_stack.pop_front().unwrap();

            let op = op_stack.pop_front().unwrap();

            let result = match op
            {
                '+' => a + b,
                '*' => a * b,
                _ => 0,
            };

            println!("{} {} {} = {}", a, op, b, result);

            val_stack.push_front(result);
        };

        let answer = val_stack.pop_front().unwrap();
        println!("answer {}", answer);
    
        (answer, i)
    }

    eval_inner(input, 0).0
}

fn solution(input: &str) -> Option<usize>
{
    let x = input
        .lines()
        .fold(0, |acc, x| acc +eval(x));
    Some(x)
}

fn solution_part2(input: &str) -> Option<usize>
{
    None
}