use phf::phf_map;

static PRIORITIES: phf::Map<&'static char, u8> = phf_map! {
    'a' => 1,
    'b' => 2,
    'c' => 3,
    'd' => 4,
    'e' => 5,
    'f' => 6,
    'g' => 7,
    'h' => 8,
    'i' => 9,
    'j' => 10,
    'k' => 11,
    'l' => 12,
    'm' => 13,
    'n' => 14,
    'o' => 15,
    'p' => 16,
    'q' => 17,
    'r' => 18,
    's' => 19,
    't' => 20,
    'u' => 21,
    'v' => 22,
    'w' => 23,
    'x' => 24,
    'y' => 25,
    'z' => 26,
    'A' => 27,
    'B' => 28,
    'C' => 29,
    'D' => 30,
    'E' => 31,
    'F' => 32,
    'G' => 33,
    'H' => 34,
    'I' => 35,
    'J' => 36,
    'K' => 37,
    'L' => 38,
    'M' => 39,
    'N' => 40,
    'O' => 41,
    'P' => 42,
    'Q' => 43,
    'R' => 44,
    'S' => 45,
    'T' => 46,
    'U' => 47,
    'V' => 48,
    'W' => 49,
    'X' => 50,
    'Y' => 51,
    'Z' => 52,
};

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let len = input.length()
    //input.lines().fold(0, |acc, line| {
    //    acc + match line.split(' ').collect::<Vec<&str>>()[..] {
    //        [a, b] => match (a, b) {
    //            ("A", "X") => 0,
    //            _ => panic!("Invalid play"),
    //        },
    //        _ => panic!("Invalid line"),
    //    }
    //})
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
