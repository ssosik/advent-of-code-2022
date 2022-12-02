#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            result.push(curr);
            curr = 0;
            continue;
        }
        curr += line.parse::<i32>().unwrap();
    }
    result
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().fold(std::i32::MIN, |a, b| a.max(*b))
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut first = &std::i32::MIN;
    let mut second = &std::i32::MIN;
    let mut third = &std::i32::MIN;

    for x in input {
        if x > first {
            third = second;
            second = first;
            first = x;
        } else if x > second {
            third = second;
            second = x;
        } else if x > third {
            third = x;
        }
    }
    first + second + third
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
