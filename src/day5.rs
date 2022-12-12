use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
    total: u8,
    from: u8,
    to: u8,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.total, self.from, self.to)
    }
}

enum ReadingInputSection {
    InitialStacks,
    StackNumbers,
    Moves,
}

pub fn print_stacks(stacks: &HashMap<u8, Vec<char>>) {
    for i in 1..=9 {
        let mut line = format!("{i}: ");
        for c in &stacks[&i] {
            line.push_str(&format!("[{c}] "));
        }
        println!("{line}")
    }
    println!();
}

#[aoc(day5, part1)]
pub fn part1(lines: &str) -> String {
    let mut reading_section = ReadingInputSection::InitialStacks;

    let (mut stacks, instructions) = lines.lines().fold(
        (HashMap::new(), Vec::new()),
        |(mut stacks, mut instructions): (HashMap<u8, Vec<char>>, Vec<Instruction>), line| {
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
                                    stacks
                                        .entry(col)
                                        .and_modify(|l| l.insert(0, c))
                                        .or_insert_with(|| vec![c]);
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
                    instructions.push(instruction);
                }
            };

            (stacks, instructions)
        },
    );

    //dbg!(stacks);
    println!("Initial");
    print_stacks(&stacks);

    for instruction in &instructions {
        println!("Instruction {instruction}");
        let mut to = stacks[&instruction.to].clone();
        let mut from = stacks[&instruction.from].clone();
        for _i in 0..instruction.total {
            let item = from.pop().unwrap();
            //dbg!(i, item);
            to.push(item);
        }
        stacks.insert(instruction.to, to);
        stacks.insert(instruction.from, from);
        print_stacks(&stacks);
    }
    let mut s = String::from("");
    for i in 1..=9 {
        //s.push(stacks[&i].clone().remove(0));
        s.push(stacks[&i].clone().pop().unwrap());
    }
    s
}

#[aoc(day5, part2)]
pub fn part2(lines: &str) -> String {
    let mut reading_section = ReadingInputSection::InitialStacks;

    let (mut stacks, instructions) = lines.lines().fold(
        (HashMap::new(), Vec::new()),
        |(mut stacks, mut instructions): (HashMap<u8, Vec<char>>, Vec<Instruction>), line| {
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
                                    stacks
                                        .entry(col)
                                        .and_modify(|l| l.insert(0, c))
                                        .or_insert_with(|| vec![c]);
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
                    instructions.push(instruction);
                }
            };

            (stacks, instructions)
        },
    );

    println!("Initial");
    print_stacks(&stacks);

    for instruction in &instructions {
        println!("Instruction {instruction}");
        let mut to = stacks[&instruction.to].clone();
        let mut from = stacks[&instruction.from].clone();
        let x = from.len() - instruction.total as usize;
        for _i in 0..instruction.total {
            let item = from.remove(x);
            to.push(item);
        }
        stacks.insert(instruction.to, to);
        stacks.insert(instruction.from, from);
        print_stacks(&stacks);
    }
    let mut s = String::from("");
    for i in 1..=9 {
        s.push(stacks[&i].clone().pop().unwrap());
    }
    s
}

#[cfg(test)]
mod tests {

    //// (()) and ()() both result in floor 0.
    //#[test]
    //fn sample1() {
    //    assert_eq!(part1("(())"), 0);
    //    assert_eq!(part1("()()"), 0);
    //}
}
