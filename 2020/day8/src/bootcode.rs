// I am so excited for this
// intcode last year was really fun, so I'm excited to see what comes out of this
// need to take things slow this year so that I have a workable design, last year was pretty bad
// Day 8 - Initial implementation

use std::fmt;

pub struct BootcodeComputer
{
    // where acc updates values
    pub accumulator: isize,
    // the current instruction number
    pub instruction_counter: isize,
    // the instructions to execute
    pub instructions: Vec<Instruction>,
    halted: bool,
}

#[derive(Clone)]
pub struct Instruction
{
    pub operation: String,
    pub argument: isize,
}

impl BootcodeComputer
{
    pub fn new() -> BootcodeComputer
    {
        BootcodeComputer
        {
            accumulator: 0,
            instruction_counter: 0,
            instructions: Vec::new(),
            halted: false,
        }
    }

    pub fn is_halted(&self) -> bool
    {
        self.halted
    }

    pub fn init(instructions: &str) -> BootcodeComputer
    {
        let mut computer = BootcodeComputer::new();
        computer.set_instructions(instructions);
        computer
    }

    pub fn step_instruction(&mut self) -> bool
    {
        // check if going oob, indicates halt
        if self.instruction_counter == self.instructions.len() as isize
        {
            self.halted = true;
            return false;
        }

        let i = self.get_current_instruction();
        self.execute_instruction(&i);

        return true;
    }

    pub fn get_current_instruction(&self) -> Instruction
    {
        let instruction_counter_u = self.instruction_counter as usize;

        let instruction = 
            self.instructions
            .get(instruction_counter_u)
            .expect("Failed to get instruction at current counter.");
        instruction.clone() // must clone for ownership reasons
    }

    pub fn set_instructions(&mut self, instructions: &str)
    {
        let instructions = BootcodeComputer::parse_instructions(instructions)
            .expect("Failed to parse instructions.");
        self.instructions = instructions;
    }

    pub fn parse_instructions(instructions: &str) -> Option<Vec<Instruction>>
    {
        let mut result = Vec::new();

        // one instruction per line of text
        for line in instructions.split("\n")
        {
            let instruction_parts : Vec<&str> = line.split(" ").collect();
            if instruction_parts.len() < 2
            {
                println!("line {} did not have 2 parts of the instruction, skipping", line);
                continue;
            }

            let opcode = instruction_parts[0];
            let arg = instruction_parts[1].parse::<isize>()
                .expect("Failed to parse the arg to an isize");

            let instruction = Instruction::new(opcode, arg);
            println!("Parsed instruction: {}", instruction);

            result.push(instruction);
        }

        if result.is_empty()
        {
            return None;
        }

        return Some(result);
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction)
    {
        match instruction.operation.as_str()
        {
            "nop" => self.nop(instruction),
            "acc" => self.acc(instruction),
            "jmp" => self.jmp(instruction),
            _ => panic!("Unknown instruction {}", instruction)
        };

        // increment instruction counter, this requires that jmp is offset by 1
        self.instruction_counter += 1;
    }

    fn nop(&mut self, _instruction: &Instruction)
    {
        println!("NOP");
        // does nothing
    }

    fn acc(&mut self, instruction: &Instruction)
    {
        println!("ACC {}", instruction.argument);

        self.accumulator += instruction.argument;
    }

    fn jmp(&mut self, instruction: &Instruction)
    {
        println!("JMP {} (-1)", instruction.argument);
        self.instruction_counter += instruction.argument - 1;
    }
}

impl Instruction
{
    pub fn new(opcode: &str, arg: isize) -> Instruction
    {
        Instruction
        {
            operation: String::from(opcode),
            argument: arg,
        }
    }
}

impl fmt::Display for Instruction
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "[{} {}]", self.operation, self.argument)
    }
}