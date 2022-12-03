use phf::phf_map;
use std::collections::HashSet;

static PRIORITIES: phf::Map<char, usize> = phf_map! {
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
pub fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let mut comp1_map = HashSet::new();
        let mut comp2_map = HashSet::new();
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        for c in compartment1.chars() {
            comp1_map.insert(c);
        }
        for c in compartment2.chars() {
            comp2_map.insert(c);
        }
        let diff: Vec<&char> = comp1_map.intersection(&comp2_map).collect();
        if diff.len() != 1 {
            panic!("incorrect difference found");
        }
        acc + PRIORITIES.get(diff[0]).unwrap()
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
