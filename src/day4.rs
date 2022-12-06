#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        // Split lines like `8-13,10-65` on the comma
        acc + match line.split(',').collect::<Vec<&str>>()[..] {
            [r1, r2] => {
                // split the first range on '-', returning the two i32s
                let (s1, e1) = match r1.split('-').collect::<Vec<&str>>()[..] {
                    [s1, e1] => (s1.parse::<i32>().unwrap(), e1.parse::<i32>().unwrap()),
                    _ => panic!("invalid range 1"),
                };
                // split the second range on '-', returning the two i32s
                let (s2, e2) = match r2.split('-').collect::<Vec<&str>>()[..] {
                    [s2, e2] => (s2.parse::<i32>().unwrap(), e2.parse::<i32>().unwrap()),
                    _ => panic!("invalid range 2"),
                };
                dbg!(s1, e1, s2, e2);
                // Check if either range fully encompasses the other
                i32::from((s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
            }
            _ => panic!("Invalid line"),
        }
    })
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        // Split lines like `8-13,10-65` on the comma
        acc + match line.split(',').collect::<Vec<&str>>()[..] {
            [r1, r2] => {
                // Convert the first range into a std::ops::Range
                let r1 = match r1.split('-').collect::<Vec<&str>>()[..] {
                    [s1, e1] => s1.parse::<i32>().unwrap()..=e1.parse::<i32>().unwrap(),
                    _ => panic!("invalid range 1"),
                };
                // Convert the second range into a std::ops::Range
                let r2 = match r2.split('-').collect::<Vec<&str>>()[..] {
                    [s2, e2] => s2.parse::<i32>().unwrap()..=e2.parse::<i32>().unwrap(),
                    _ => panic!("invalid range 2"),
                };
                i32::from(
                    r1.contains(r2.start())
                        || r1.contains(r2.end())
                        || r2.contains(r1.start())
                        || r2.contains(r1.end()),
                )
            }
            _ => panic!("Invalid line"),
        }
    })
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
