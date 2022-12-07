use std::fs;

use RSP::{Rock, Scissors, Paper};
use crate::traits::Problem;

#[derive(Debug, Clone)]
pub struct P002 {
    input: String,
    score: Option<usize>,
}

impl P002 {
    pub fn new() -> Self {
        let input = fs::read_to_string("input/p002.txt").unwrap();
        Self { input, score: None }
    }

}

impl Problem<usize> for P002 {
    fn solve(&self) -> usize {
        match self.score {
            None => (),
            Some(score) => return score,
        }

        let input = &self.input;
        let mut score = 0usize;

        for line in input.lines() {
            let mut lines = line.trim().split_whitespace();
            let op = RSPOppo::parse(lines.next().unwrap());
            let pl = RSPPlayer::parse(lines.next().unwrap());

            score += battle(pl, op);
        }

        score
    }

    fn phase2(&self) -> Self {
        let input = &self.input;
        let mut score = 0;

        for line in input.lines() {
            let mut lines = line.trim().split_whitespace();
            let op = RSPOppo::parse(lines.next().unwrap());
            let pl = RSPPlayer::parse(lines.next().unwrap());
            let pl = pl.phase2_conv(&op);
            score += battle(pl, op);
        }

        Self { input: input.to_owned(), score: Some(score) }
    }
}

fn battle<S: RSPTrait, T: RSPTrait>(pl: S, op: T) -> usize {
    let pl = pl.to_rsp();
    let op = op.to_rsp();

    let score = match (pl, op) {
        (x, y) if x == y => 3,
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
        (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => 0,
        _ => unreachable!(),
    };

    score + pl.calc_shape()
}

// =========================================================================
// Rock Scissors Paper (Backend)
// =========================================================================
#[derive(Debug, Clone, Copy)]
enum RSPOppo {
    A,
    B,
    C
}

#[derive(Debug, Clone, Copy)]
enum RSPPlayer {
    X,
    Y,
    Z
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RSP {
    Rock,
    Scissors,
    Paper
}

trait RSPTrait {
    fn parse(s: &str) -> Self;
    fn to_rsp(&self) -> RSP;
    fn calc_shape(&self) -> usize {
        self.to_rsp().calc_shape()
    }
}

impl RSPTrait for RSP {
    fn parse(_s: &str) -> Self {
        unimplemented!()
    }
    fn to_rsp(&self) -> RSP {
        *self
    }
    fn calc_shape(&self) -> usize {
        match self {
            RSP::Rock => 1,
            RSP::Scissors => 3,
            RSP::Paper => 2,
        }
    }
}

impl RSPTrait for RSPOppo {
    fn parse(s: &str) -> Self {
        match s {
            "A" => RSPOppo::A,
            "B" => RSPOppo::B,
            "C" => RSPOppo::C,
            _ => panic!("Invalid Opponent: {}", s),
        }
    }
    fn to_rsp(&self) -> RSP {
        match self {
            RSPOppo::A => RSP::Rock,
            RSPOppo::B => RSP::Paper,
            RSPOppo::C => RSP::Scissors,
        }
    }
}

impl RSPTrait for RSPPlayer {
    fn parse(s: &str) -> Self {
        match s {
            "X" => RSPPlayer::X,
            "Y" => RSPPlayer::Y,
            "Z" => RSPPlayer::Z,
            _ => unreachable!(),
        }
    }
    fn to_rsp(&self) -> RSP {
        match self {
            RSPPlayer::X => RSP::Rock,
            RSPPlayer::Y => RSP::Paper,
            RSPPlayer::Z => RSP::Scissors,
        }
    }
}

impl RSPPlayer {
    fn phase2_conv(&self, op: &RSPOppo) -> RSP {
        let op = op.to_rsp();
        match (self, op) {
            (RSPPlayer::X, Rock) => Scissors,
            (RSPPlayer::X, Scissors) => Paper,
            (RSPPlayer::X, Paper) => Rock,
            (RSPPlayer::Y, rsp) => rsp,
            (RSPPlayer::Z, Rock) => Paper,
            (RSPPlayer::Z, Scissors) => Rock,
            (RSPPlayer::Z, Paper) => Scissors,
        }
    }
}
