use itertools::{
    any,
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::collections::HashSet;

#[derive(Default)]
pub struct Quad(Option<char>, Option<char>, Option<char>, Option<char>);

impl Quad {
    fn new() -> Self {
        Quad(None, None, None, None)
    }
    fn is_all_unique(&self) -> bool {
        match self {
            Quad(None, _, _, _)
            | Quad(_, None, _, _)
            | Quad(_, _, None, _)
            | Quad(_, _, _, None) => false,
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

pub struct StartMessageMarker([Option<char>; 14]);

impl StartMessageMarker {
    fn new() -> Self {
        Self([None; 14])
    }
    fn is_all_unique(&self) -> bool {
        // If any of the items are None we don't have enough input yet, FALSE
        if any(self.0, |item| item.is_none()) {
            return false;
        }
        match self.0.iter().fold_while(HashSet::new(), |mut hs, item| {
            if !hs.insert(item) {
                Done(hs)
            } else {
                Continue(hs)
            }
        }) {
            Continue(_) => true, // Ran off the end inserting, ALL UNIQUE
            _ => false,          // One of the inserts was a dupe, NOT ALL UNIQUE
        }
    }
    fn add(&mut self, c: char) {
        for i in 0..13 {
            self.0[i] = self.0[i + 1];
        }
        self.0[13] = Some(c);
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (i, _) = input
        .chars()
        .fold_while((1, Quad::new()), |(i, mut quad), next| {
            quad.add(next);
            if quad.is_all_unique() {
                Done((i, quad))
            } else {
                Continue((i + 1, quad))
            }
        })
        .into_inner();
    i
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (i, _) = input
        .chars()
        .fold_while((1, StartMessageMarker::new()), |(i, mut window), next| {
            window.add(next);
            if window.is_all_unique() {
                Done((i, window))
            } else {
                Continue((i + 1, window))
            }
        })
        .into_inner();
    i
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
