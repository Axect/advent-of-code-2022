use crate::traits::Problem;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct P006 {
    input: String,
    position: Option<usize>
}

impl Problem<usize> for P006 {
    fn new() -> Self {
        P006 {
            input: fs::read_to_string("input/p006.txt").unwrap(),
            position: None
        }
    }

    fn solve(&self) -> usize {
        match self.position {
            None => (),
            Some(num) => return num,
        }

        let input = &self.input;
        let mut pos = 4usize;

        let mut chars = input.chars();
        let mut three = {
            let mut vec = Vec::new();
            for _ in 0..3 {
                vec.push(chars.next().unwrap());
            }
            vec
        };

        loop {
            match chars.next() {
                None => break,
                Some(c) => {
                    three.push(c);
                    let set: HashSet<char> = HashSet::from_iter(three.iter().cloned());
                    if set.len() == 4 {
                        break;
                    } else {
                        three.remove(0);
                        pos += 1;
                    }
                }
            }
        }

        pos
    }

    fn phase2(&self) -> Self {
        let input = &self.input;
        let mut pos = 14usize;

        let mut chars = input.chars();
        let mut thirteen = {
            let mut vec = Vec::new();
            for _ in 0..13 {
                vec.push(chars.next().unwrap());
            }
            vec
        };

        loop {
            match chars.next() {
                None => break,
                Some(c) => {
                    thirteen.push(c);
                    let set: HashSet<char> = HashSet::from_iter(thirteen.iter().cloned());
                    if set.len() == 14 {
                        break;
                    } else {
                        thirteen.remove(0);
                        pos += 1;
                    }
                }
            }
        }

        P006 {
            input: input.to_string(),
            position: Some(pos)
        }
    }
}
