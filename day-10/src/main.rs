extern crate core;

use common::read_lines;

fn main() {
    let mut lines = read_lines("input.txt")
        .unwrap()
        .map(|line| line.unwrap())
        .into_iter();

    let mut signal_strength_total = 0;

    let mut cycle = 1;
    let mut register_x = 1;

    let mut current_instruction: Box<dyn Instruction> = Box::new(NoopInstruction::new());
    current_instruction.do_work(&mut register_x);

    let mut end_of_program = false;
    while !end_of_program {
        if current_instruction.do_work(&mut register_x) {
            let next_line = lines.next();
            if let Some(instruction) = next_line {
                match &instruction[0..4] {
                    "noop" => current_instruction = Box::new(NoopInstruction::new()),
                    "addx" => {
                        current_instruction = Box::new(AddXInstruction::new(
                            instruction[5..].parse::<i32>().unwrap(),
                        ))
                    }
                    _ => panic!("unknown instruction {}", instruction),
                }

                continue;
            } else {
                end_of_program = true;
            }
        }

        if (cycle - 20) % 40 == 0 {
            signal_strength_total += cycle * register_x;
        }

        cycle += 1;
    }

    println!("part 1: {}", signal_strength_total);
}

trait Instruction {
    fn do_work(&mut self, register_x: &mut i32) -> bool;
}

struct NoopInstruction {
    cycles_remaining: i32,
}

impl NoopInstruction {
    fn new() -> Self {
        NoopInstruction {
            cycles_remaining: 1,
        }
    }
}

impl Instruction for NoopInstruction {
    fn do_work(&mut self, _: &mut i32) -> bool {
        if self.cycles_remaining == 0 {
            true
        } else {
            self.cycles_remaining -= 1;
            false
        }
    }
}

struct AddXInstruction {
    cycles_remaining: i32,
    v: i32,
}

impl AddXInstruction {
    fn new(v: i32) -> Self {
        AddXInstruction {
            cycles_remaining: 2,
            v,
        }
    }
}

impl Instruction for AddXInstruction {
    fn do_work(&mut self, register_x: &mut i32) -> bool {
        if self.cycles_remaining == 0 {
            *register_x = *register_x + self.v;
            true
        } else {
            self.cycles_remaining -= 1;
            false
        }
    }
}
