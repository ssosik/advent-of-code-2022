#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + match line.split(',').collect::<Vec<&str>>()[..] {
            [r1, r2] => {
                let (s1, e1) = match r1.split('-').collect::<Vec<&str>>()[..] {
                    [s1, e1] => (s1.parse::<i32>().unwrap(), e1.parse::<i32>().unwrap()),
                    _ => panic!("invalid range 1"),
                };
                let (s2, e2) = match r2.split('-').collect::<Vec<&str>>()[..] {
                    [s2, e2] => (s2.parse::<i32>().unwrap(), e2.parse::<i32>().unwrap()),
                    _ => panic!("invalid range 2"),
                };
                dbg!(s1, e1, s2, e2);
                if (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid line"),
        }
    })
}

#[aoc(day4, part2)]
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
