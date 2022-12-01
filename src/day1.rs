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
    input.iter().fold(std::i32::MIN, |a,b| a.max(*b))
}

//#[aoc(day1, part2)]
//pub fn part2(input: Vec<i32>) -> usize {
//    let mut sum: u32 = 0;
//
//    for (i, c) in input.as_bytes().iter().enumerate() {
//        match c {
//            b'(' => sum += 1,
//            b')' => if let Some(s) = sum.checked_sub(1) {
//                sum = s;
//            } else {
//                return i + 1;
//            },
//            _ => unreachable!(),
//        }
//    }
//
//    unreachable!()
//}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}

