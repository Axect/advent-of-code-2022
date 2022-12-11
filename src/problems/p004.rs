use crate::traits::Problem;
use std::{fs, ops::Range};

#[derive(Debug, Clone)]
pub struct P004 {
    input: String,
    score: Option<usize>,
}

impl P004 {
    pub fn new() -> Self {
        P004 {
            input: fs::read_to_string("input/p004.txt").unwrap(),
            score: None,
        }
    }
}

impl Problem<usize> for P004 {
    fn solve(&self) -> usize {
        match self.score {
            None => (),
            Some(score) => return score,
        }

        let mut score = 0;

        for line in self.input.lines() {
            let (s1, s2) = string_to_ranges(line);
            if fully_contains(s1, s2) {
                score += 1;
            }
        }

        score
    }

    fn phase2(&self) -> Self {
        let mut score = 0;

        for line in self.input.lines() {
            let (s1, s2) = string_to_ranges(line);
            if overlaps(s1, s2) {
                score += 1;
            }
        }

        P004 {
            input: self.input.clone(),
            score: Some(score),
        }
    }
}

fn fully_contains(s1: Range<usize>, s2: Range<usize>) -> bool {
    (s1.start <= s2.start && s1.end >= s2.end) || (s2.start <= s1.start && s2.end >= s1.end)
}

fn overlaps(s1: Range<usize>, s2: Range<usize>) -> bool {
    let start = s1.start.max(s2.start);
    let end = s1.end.min(s2.end);

    start < end
}

fn string_to_ranges(s: &str) -> (Range<usize>, Range<usize>) {
    let (s1, s2) = s.split_once(',').unwrap();
    let (s1, s2) = (s1.trim(), s2.trim());
    let (s1_i, s1_f) = s1.split_once('-').unwrap();
    let (s2_i, s2_f) = s2.split_once('-').unwrap();
    let (s1_i, s1_f, s2_i, s2_f) = (
        s1_i.parse::<usize>().unwrap(),
        s1_f.parse::<usize>().unwrap(),
        s2_i.parse::<usize>().unwrap(),
        s2_f.parse::<usize>().unwrap(),
    );
    let s1_range = Range {
        start: s1_i,
        end: s1_f + 1,
    };
    let s2_range = Range {
        start: s2_i,
        end: s2_f + 1,
    };

    (s1_range, s2_range)
}
