pub fn main() {
    let source_code = get_source_code();
    let cycle_max = get_cycle_max();
    
    let program = parse_instructions(&source_code);
    let mut program_counter = 0;
    let mut program_complete_index = None;

    const HEAP_SIZE: usize = 30000;
    let mut heap = [0u8; HEAP_SIZE];
    let mut pointer = 0;

    for cycle_index in 0..cycle_max {
        if let Some(instruction) = program.get(program_counter) {
            match instruction {
                Instruction::IncreasePointer => pointer = (pointer + 1) % HEAP_SIZE,
                Instruction::DecrementPointer => pointer = pointer.checked_sub(1).unwrap_or(HEAP_SIZE - 1),
                Instruction::IncreaseValue => heap[pointer] = heap[pointer].wrapping_add(1),
                Instruction::DecreaseValue => heap[pointer] = heap[pointer].wrapping_sub(1),
                Instruction::Output => output(heap[pointer]),
                Instruction::Input => heap[pointer] = input(),
                Instruction::JumpAhead(jump_index) => {
                    if heap[pointer] == 0 {
                        program_counter = *jump_index;
                    }
                }
                Instruction::JumpBack(jump_index) => {
                    if heap[pointer] != 0 {
                        program_counter = *jump_index;
                    }
                }
            }
        } else {
            program_complete_index = Some(cycle_index);
            break;
        }
        program_counter += 1;
    }

    println!(
        "\nProgram {} to completion in {} cycles",
        if program_complete_index.is_some() { "executed" } else { "did not execute" },
        if let Some(index) = program_complete_index { index + 1 } else { cycle_max },
    );
}

#[derive(Debug)]
pub enum Instruction {
    IncreasePointer,
    DecrementPointer,
    IncreaseValue,
    DecreaseValue,
    Output,
    Input,
    JumpAhead(usize),
    JumpBack(usize),
}
impl Instruction {
    fn valid_char(character: &char) -> bool {
        match character {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        }
    }
}

fn get_source_code() -> String {
    std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("Missing source code file path"),
    )
    .expect("Failed to read source code file")
}

fn get_cycle_max() -> usize {
    const DEFAULT_CYCLE_MAX: &str = "1000";
    std::env::args()
        .nth(2)
        .unwrap_or_else(|| DEFAULT_CYCLE_MAX.to_string())
        .parse()
        .expect("First argument is invalid`")
}

fn parse_instructions(source_code: &str) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    let mut stack = Vec::<usize>::new();
    for (character_index, character) in source_code
        .chars()
        .filter(Instruction::valid_char)
        .enumerate()
    {
        let instruction = match character {
            '>' => Instruction::IncreasePointer,
            '<' => Instruction::DecrementPointer,
            '+' => Instruction::IncreaseValue,
            '-' => Instruction::DecreaseValue,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => {
                stack.push(character_index);
                Instruction::JumpAhead(0)
            }
            ']' => {
                let jump_back_index = stack.pop().expect("Missing open bracket(s)");
                instructions[jump_back_index] = Instruction::JumpAhead(character_index);
                Instruction::JumpBack(jump_back_index)
            }
            _ => continue,
        };

        instructions.push(instruction);
    }

    if !stack.is_empty() {
        panic!("Missing closing bracket(s)");
    }

    instructions
}

fn output(value: u8) {
    use std::io::{stdout, Write};
    stdout().write(&[value]).expect("Failed to write to stdout");
    stdout().flush().expect("Failed to flush stdout");
}

fn input() -> u8 {
    use std::io::{stdin, Read};
    let mut input = [0];
    stdin().read(&mut input).expect("Failed to read from stdin");
    input[0]
}
