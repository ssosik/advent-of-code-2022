use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Instruction {
    total: u8,
    from: u8,
    to: u8,
}

#[derive(Debug, Default, Clone)]
pub struct Input {
    stacks: HashMap<u8, Vec<char>>,
    instructions: Vec<Instruction>,
}

//impl Input {
//    pub fn new() -> Self {
//        Default::default()
//    }
//    pub fn process(self) -> String {
//        for instruction in &self.instructions {
//            dbg!(instruction);
//            let to = &mut self.stacks[&instruction.to];
//            let from = &mut self.stacks[&instruction.from];
//            for _ in 0..instruction.total {
//                to.push(from.pop().unwrap());
//                // let val = self.stacks.get(from).unwrap().pop().unwrap();
//                //let val = self.stacks.entry(from).and_modify(|l| { l.pop().unwrap();} ).or_default();
//                //self.stacks.entry(to).and_modify(|l| l.push(val));
//            }
//        }
//        return String::from("nope");
//    }
//}

enum ReadingInputSection {
    InitialStacks,
    StackNumbers,
    Moves,
}

//#[aoc_generator(day5)]
//pub fn input_generator(lines: &str) -> Input {
//    let mut reading_section = ReadingInputSection::InitialStacks;
//
//    lines.lines().fold(Input::new(), |mut input, line| {
//        match reading_section {
//            ReadingInputSection::InitialStacks => {
//                line.chars()
//                    .collect::<Vec<_>>()
//                    .chunks(4)
//                    .fold(1, |col, item| {
//                        match item[..] {
//                            [' ', ' ', ' ', ' '] | [' ', ' ', ' '] => {
//                                println!("empty col {col}");
//                            }
//                            ['[', c, ']', ' '] | ['[', c, ']'] => {
//                                dbg!(c, col);
//                                input
//                                    .stacks
//                                    .entry(col)
//                                    .and_modify(|l| l.push(c))
//                                    .or_insert(vec![c]);
//                            }
//                            _ => {
//                                println!("unparseable {item:?} {col}");
//                                reading_section = ReadingInputSection::StackNumbers;
//                            }
//                        }
//                        col + 1
//                    });
//            }
//            ReadingInputSection::StackNumbers => {
//                reading_section = ReadingInputSection::Moves;
//                //skip
//            }
//            ReadingInputSection::Moves => {
//                let line = line.split(' ').collect::<Vec<&str>>();
//                let instruction = Instruction {
//                    total: line[1].parse().unwrap(),
//                    from: line[3].parse().unwrap(),
//                    to: line[5].parse().unwrap(),
//                };
//                input.instructions.push(instruction);
//            }
//        };
//
//        input
//    })
//}

#[aoc(day5, part1)]
pub fn part1(lines: &str) -> String {
    let mut reading_section = ReadingInputSection::InitialStacks;

    let (mut stacks, mut instructions) = lines.lines().fold((HashMap::new(), Vec::new()), |(mut stacks, mut instructions): (HashMap<u8, Vec<char>>, Vec<Instruction>), line| {
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
                instructions.push(instruction);
            }
        };

        (stacks, instructions)
    });

    for instruction in &instructions {
        dbg!(instruction);
        let mut to = stacks[&instruction.to].clone();
        let mut from = stacks[&instruction.from].clone();
        for i in 0..instruction.total {
            let item = from.remove(0);
            dbg!(i, item);
            to.push(item);
        }
        stacks.insert(instruction.to, to);
        stacks.insert(instruction.from, from);
    }
    let mut s = String::from("");
    for i in 1..=9 {
        s.push(stacks[&i].clone().remove(0));
    }
    dbg!(stacks);
    dbg!(s);

    "nope".to_string()
}

//#[aoc(day5, part2)]
pub fn part2(_input: &Input) -> String {
    "nope".into()
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