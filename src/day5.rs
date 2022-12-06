use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Instruction {
    total: u8,
    from: u8,
    to: u8,
}

#[derive(Debug, Default)]
pub struct Input {
    stacks: HashMap<u8, Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }
}

enum ReadingInputSection {
    InitialStacks,
    StackNumbers,
    Moves,
}

#[aoc_generator(day5)]
pub fn input_generator(lines: &str) -> Input {
    let mut reading_section = ReadingInputSection::InitialStacks;

    lines.lines().fold(Input::new(), |mut input, line| {
        match reading_section {
            ReadingInputSection::InitialStacks => {
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .fold(1, |col, item| {
                        match item[..] {
                            [' ', ' ', ' ', ' '] | [' ', ' ', ' '] => {
                                println!("empty col {col}");
                            }
                            ['[', c, ']', ' '] | ['[', c, ']'] => {
                                dbg!(c, col);
                                input
                                    .stacks
                                    .entry(col)
                                    .and_modify(|l| l.push(c))
                                    .or_insert(vec![c]);
                            }
                            _ => {
                                println!("unparseable {item:?} {col}");
                                reading_section = ReadingInputSection::StackNumbers;
                            }
                        }
                        col + 1
                    });
            }
            ReadingInputSection::StackNumbers => {
                reading_section = ReadingInputSection::Moves;
                //skip
            }
            ReadingInputSection::Moves => {
                let line = line.split(' ').collect::<Vec<&str>>();
                let instruction = Instruction {
                    total: line[1].parse().unwrap(),
                    from: line[3].parse().unwrap(),
                    to: line[5].parse().unwrap(),
                };
                input.instructions.push(instruction);
            }
        };

        input
    })
}

#[aoc(day5, part1)]
pub fn part1(lines: &Input) -> String {
    //dbg!(lines);
    for instruction in &lines.instructions {
        dbg!(instruction);
    }

    "nope".into()
}

//#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    "nope".into()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    //// (()) and ()() both result in floor 0.
    //#[test]
    //fn sample1() {
    //    assert_eq!(part1("(())"), 0);
    //    assert_eq!(part1("()()"), 0);
    //}
}
