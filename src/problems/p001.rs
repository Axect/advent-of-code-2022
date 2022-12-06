use std::fs;
use crate::traits::Problem;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct P001 {
    input: String,
}

impl P001 {
    pub fn new() -> Self {
        let input = fs::read_to_string("input/p001.txt").unwrap();
        Self { input }
    }
}

impl Problem<usize> for P001 {
    fn solve(&self) -> usize {
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
}
