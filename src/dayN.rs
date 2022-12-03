#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + match line.split(' ').collect::<Vec<&str>>()[..] {
            [a, b] => match (a, b) {
                ("A", "X") => 0,
                _ => panic!("Invalid play"),
            },
            _ => panic!("Invalid line"),
        }
    })
}

#[aoc(day3, part2)]
pub fn part2(_input: &str) -> i32 {
    0
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
