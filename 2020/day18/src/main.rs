use std::fs;
use std::collections::VecDeque;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"<input goes here>";

const EXAMPLE_ANSWER_1: usize = 1;
const EXAMPLE_ANSWER_2: usize = 2;

fn main() {

    let test_cases = vec![
        ("1 + 2 * 3 + 4 * 5 + 6", 231),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
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

    // let answer = solution(&input)
    //     .expect("no result");
    
    // println!("Answer Part 1 {}", answer);
    // assert!(answer == 42069);

    // part 2
    // let example_solution = solution_part2(EXAMPLE_INPUT)
    //     .expect("no result");

    // println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    // assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer == 360029542265462);
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
            // oops I forgot to copy paste this for part 2. oh well.

            // for part 2, instead of going L to R here, would need to walk through to solve all +'s, then *'s
            // let a = val_stack.pop_front().unwrap();
            // let b = val_stack.pop_front().unwrap();
            // let op = op_stack.pop_front().unwrap();

            // let result = match op
            // {
            //     '+' => a + b,
            //     '*' => a * b,
            //     _ => 0,
            // };
            // println!("{} {} {} = {}", a, op, b, result);
            // val_stack.push_front(result);

            // stack where * ops are stored after iterating through the op_stack
            let mut mul_stack = VecDeque::new();
            let mut mul_val_stack = VecDeque::new();

            // walk through the op stack to find all + operators
            // might need to adjust how this is done so that it respects when things change
            while op_stack.len() > 0
            {
                // this method feels kinda ugly, glad that there are only 2 levels of operator order (aside from paren)

                let op = op_stack.pop_front().unwrap();
                if op != '+'
                {
                    mul_stack.push_back(op);
                    // pop off the value associated with this non-+ op
                    let v = val_stack.pop_front().unwrap();
                    mul_val_stack.push_back(v);
                    continue;
                }

                // 3 * 2 + 1
                // * +
                // pop off 3 and stick on the mul stack
                // remaining is 2 + 1
                // stick result back on the val_stack
                // when op_stack is empty, add the remaining val to the other stack

                // get the operands
                let a = val_stack.pop_front().unwrap();
                let b = val_stack.pop_front().unwrap();
                let result = a + b;

                // remove b from the val stack and store the result in a
                println!("{} + {} = {}", a, b, result);
                
                val_stack.push_front(result);
            }

            while val_stack.len() > 0
            {
                println!("mitigate");
                let v = val_stack.pop_front().unwrap();
                mul_val_stack.push_back(v);
            }

            println!("stacks after + pass: {:?} {:?} {:?} {:?}", mul_stack, mul_val_stack, op_stack, val_stack);

            // iterate through the mul stack and compute that too
            while mul_stack.len() > 0
            {
                mul_stack.pop_front();

                let a = mul_val_stack.pop_front().unwrap();
                let b = mul_val_stack.pop_front().unwrap();

                let result = a * b;

                println!("{} * {} = {}", a, b, result);
                mul_val_stack.push_front(result);
            }

            // lazy
            val_stack.push_front(mul_val_stack.pop_front().unwrap());
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
    let x = input
        .lines()
        .fold(0, |acc, x| acc +eval(x));
    Some(x)
}