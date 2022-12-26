use crate::traits::Problem;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub struct P003 {
    input: String,
    score: Option<usize>,
}

impl Problem<usize> for P003 {
    fn new() -> Self {
        P003 {
            input: fs::read_to_string("input/p003.txt").unwrap(),
            score: None,
        }
    }

    fn solve(&self) -> usize {
        match self.score {
            None => (),
            Some(score) => return score,
        }

        let input = &self.input;
        let mut score = 0;

        for line in input.lines() {
            let alphabet = find_shared_alphabet(split_half(line.trim()));
            score += alphabet_to_ascii(alphabet);
        }
        score
    }

    fn phase2(&self) -> Self {
        let input = self.input.trim();
        let input = input.lines().map(|s| s.trim()).collect::<Vec<_>>();

        let mut score = 0;

        input.chunks(3).for_each(|chunk| {
            let mut lens = chunk
                .iter()
                .enumerate()
                .map(|(i, line)| (i, line.len()))
                .collect::<Vec<_>>();
            lens.sort_by(|a, b| a.1.cmp(&b.1));

            let line_1 = chunk[lens[0].0].chars().collect::<Vec<_>>();
            let line_2 = chunk[lens[1].0].chars().collect::<Vec<_>>();
            let line_3 = chunk[lens[2].0].chars().collect::<Vec<_>>();

            let alphabet = find_badge((&line_1, &line_2, &line_3));
            score += alphabet_to_ascii(alphabet);
        });

        P003 {
            input: self.input.clone(),
            score: Some(score),
        }
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

fn find_badge((line_1, line_2, line_3): (&[char], &[char], &[char])) -> char {
    let l_1 = line_1.len();

    let mut set_1 = HashSet::new();
    let mut set_2 = HashSet::new();
    let mut set_3 = HashSet::new();

    for ((&a, &b), &c) in line_1.iter().zip(line_2.iter()).zip(line_3.iter()) {
        if set_2.contains(&a) && set_3.contains(&a) {
            return a;
        } else {
            set_1.insert(a);
        }

        if set_1.contains(&b) && set_3.contains(&b) {
            return b;
        } else {
            set_2.insert(b);
        }

        if set_1.contains(&c) && set_2.contains(&c) {
            return c;
        } else {
            set_3.insert(c);
        }
    }

    let line_2s = line_2.iter().skip(l_1).collect::<Vec<_>>();
    let line_3s = line_3.iter().skip(l_1).collect::<Vec<_>>();

    let l_2 = line_2s.len();

    for (&&b, &&c) in line_2s.iter().zip(line_3s.iter()) {
        if set_1.contains(&b) && set_3.contains(&b) {
            return b;
        } else {
            set_2.insert(b);
        }

        if set_1.contains(&c) && set_2.contains(&c) {
            return c;
        } else {
            set_3.insert(c);
        }
    }

    let line_3s = line_3s.into_iter().skip(l_2).collect::<Vec<_>>();

    for &c in line_3s.into_iter() {
        if set_1.contains(&c) && set_2.contains(&c) {
            return c;
        }
    }

    unreachable!("No badge found");
}
