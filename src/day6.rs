use std::collections::HashSet;

#[derive(Default)]
pub struct Quad(Option<char>, Option<char>, Option<char>, Option<char>);

impl Quad {
    fn new() -> Self {
        Quad(None, None, None, None)
    }
    fn start_of_packet(&self) -> bool {
        match self {
            Quad(None, _, _, _) | Quad(_, None, _, _) | Quad(_, _, None, _) | Quad(_, _, _, None) => false,
            Quad(a, b, c, d) if a == b || b == c || c == d || a == c || a == d || b == d => false,
            _ => true,
        }
    }
    fn add(&mut self, c: char) {
        self.3 = self.2;
        self.2 = self.1;
        self.1 = self.0;
        self.0 = Some(c);
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    //input.chars().for_each(|x| println!("{x}"));
    let (i, _) = input.chars().fold((1, Quad::new()), |(i, quad), next| {
        println!("{i}: {next}");
        (i + 1, quad)
    });
    i
}

//#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    0
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
