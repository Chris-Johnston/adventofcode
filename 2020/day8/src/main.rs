use std::fs;
use std::collections::HashSet;

mod bootcode;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

const EXAMPLE_ANSWER_1: isize = 5;
const EXAMPLE_ANSWER_2: isize = 8;

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

fn solution(input: &str) -> Option<isize>
{
    let mut cpu = bootcode::BootcodeComputer::init(input);

    for x in cpu.instructions.iter()
    {
        println!("{}", x);
    }

    let mut already_executed_instructions = HashSet::new();

    while true
    {
        if !already_executed_instructions.insert(cpu.instruction_counter)
        {
            println!("already contained this instruction");
            return Some(cpu.accumulator);
        }

        cpu.step_instruction();
    }


    None
}

fn solution_part2(input: &str) -> Option<isize>
{
    // let mut cpu = bootcode::BootcodeComputer::init(input);

    let mut instructions = bootcode::BootcodeComputer::parse_instructions(input)
        .expect("failed to parse instructions");

    // for each instruction that is either a nop or jmp
    // swap it to the other type
    // then run it to see if it loops or halts

    for (i, val) in instructions.iter().enumerate()
    {
        let mut modified_instructions = instructions.clone();
        let mut to_modify = &modified_instructions[i];
        println!("flipping instruction {}", i);
        match to_modify.operation.as_str() {
            "nop" => modified_instructions[i].operation = String::from("jmp"),
            "jmp" => modified_instructions[i].operation = String::from("nop"),
            _ => {
                println!("skipping {}", to_modify);
                continue;},
        };

        let mut cpu = bootcode::BootcodeComputer::new();
        cpu.instructions = modified_instructions;
        {
            let mut already_executed_instructions = HashSet::new();
    
            // function terminates by attempting to execute
            // instruction immediately after the last instruction in the file
    
            while !cpu.is_halted()
            {
                if !already_executed_instructions.insert(cpu.instruction_counter)
                {
                    println!("already contained this instruction");
                    break;
                }
                cpu.step_instruction();
            }

            if cpu.is_halted()
            {
                return Some(cpu.accumulator);
            }
        }
    }
    None
}
