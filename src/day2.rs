// A - Rock, 1
// B - Paper, 2
// C - Scissor, 3
// X - Rock, 1
// Y - Paper, 2
// Z - Scissor, 3
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const LOSS: i32 = 0;
const TIE: i32 = 3;
const WIN: i32 = 6;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + match line.split(' ').collect::<Vec<&str>>()[..] {
            [a, b] => match (a, b) {
                ("A", "X") => TIE + ROCK,
                ("A", "Y") => WIN + PAPER,
                ("A", "Z") => LOSS + SCISSORS,
                ("B", "X") => LOSS + ROCK,
                ("B", "Y") => TIE + PAPER,
                ("B", "Z") => WIN + SCISSORS,
                ("C", "X") => WIN + ROCK,
                ("C", "Y") => LOSS + PAPER,
                ("C", "Z") => TIE + SCISSORS,
                _ => panic!("Invalid play"),
            },
            _ => panic!("Invalid line"),
        }
    })
}

// X - Loss, 1
// Y - Tie, 2
// Z - Win, 3
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + match line.split(' ').collect::<Vec<&str>>()[..] {
            [a, b] => match (a, b) {
                ("A", "X") => LOSS + SCISSORS,
                ("A", "Y") => TIE + ROCK,
                ("A", "Z") => WIN + PAPER,
                ("B", "X") => LOSS + ROCK,
                ("B", "Y") => TIE + PAPER,
                ("B", "Z") => WIN + SCISSORS,
                ("C", "X") => LOSS + PAPER,
                ("C", "Y") => TIE + SCISSORS,
                ("C", "Z") => WIN + ROCK,
                _ => panic!("Invalid play"),
            },
            _ => panic!("Invalid line"),
        }
    })
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
