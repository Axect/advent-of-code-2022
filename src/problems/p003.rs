use crate::traits::Problem;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct P003 {
    input: String,
    score: Option<usize>,
}

impl P003 {
    pub fn new() -> Self {
        P003 {
            input: fs::read_to_string("input/p003.txt").unwrap(),
            score: None,
        }
    }
}

impl Problem<usize> for P003 {
    fn solve(&self) -> usize {
        match self.score {
            None => (),
            Some(score) => return score,
        }

        let input = &self.input;
        let mut score = 0;

        for line in input.lines() {
            let alphabet = find_shared_alphabet(
                split_half(line.trim())
            );
            dbg!(alphabet);
            score += alphabet_to_ascii(alphabet);
        }
        score
    }

    fn phase2(&self) -> Self {
        unimplemented!()
    }
}

fn split_half(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn alphabet_to_ascii(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}

fn find_shared_alphabet((a, b): (&str, &str)) -> char {
    let mut set_1 = HashSet::new();
    let mut set_2 = HashSet::new();

    for (c1, c2) in a.chars().zip(b.chars()) {
        if set_2.contains(&c1) {
            return c1;
        } else {
            set_1.insert(c1);
        }

        if set_1.contains(&c2) {
            return c2;
        } else {
            set_2.insert(c2);
        }
    }

    unreachable!("No shared alphabet found");
}
