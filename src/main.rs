use aoc2022::prelude::*;
use std::env::args;
use took::took;
//use aoc2022_proc::num_to_prob;

fn main() {
    let prob_num = args().nth(1).unwrap().parse::<usize>().unwrap();
    let phase_num = args().nth(2).unwrap().parse::<usize>().unwrap();
    let phase = num_to_phase(phase_num);

    let (took, ans) = took(|| {
        get_ans(prob_num, phase)
    });

    println!("Problem {}-{}: {}\t{}", prob_num, phase_num, ans, took);
}
