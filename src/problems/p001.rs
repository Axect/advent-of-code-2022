use crate::traits::Problem;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct P001 {
    input: String,
    score: Option<usize>,
}

impl Problem<usize> for P001 {
    fn new() -> Self {
        let input = fs::read_to_string("input/p001.txt").unwrap();
        Self { input, score: None }
    }

    fn solve(&self) -> usize {
        match self.score {
            None => (),
            Some(score) => return score,
        }

        let mut max = 0;
        let mut temp = 0;

        let input = &self.input;
        for line in input.lines() {
            match line.trim() {
                empty if empty.is_empty() => {
                    if temp > max {
                        max = temp;
                    }
                    temp = 0;
                }
                num => {
                    temp += usize::from_str(num).unwrap();
                }
            }
        }

        max
    }

    fn phase2(&self) -> Self {
        let mut max = (0, 0, 0);
        let mut temp = 0;

        let input = &self.input;
        for line in input.lines() {
            match line.trim() {
                empty if empty.is_empty() => {
                    if temp > max.0 {
                        max = (temp, max.0, max.1);
                    } else if temp > max.1 {
                        max = (max.0, temp, max.1);
                    } else if temp > max.2 {
                        max = (max.0, max.1, temp);
                    }
                    temp = 0;
                }
                num => {
                    temp += usize::from_str(num).unwrap();
                }
            }
        }

        Self {
            input: input.to_owned(),
            score: Some(max.0 + max.1 + max.2),
        }
    }
}
