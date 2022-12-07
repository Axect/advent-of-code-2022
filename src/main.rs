use aoc2022::prelude::*;
use aoc2022_proc::num_to_prob;

fn main() {
    let prob1 = num_to_prob!(1);
    println!("Problem 1-1: {}", prob1.ans_to_string(Phase1));
    println!("Problem 1-2: {}", prob1.ans_to_string(Phase2));

    let prob2 = num_to_prob!(2);
    println!("Problem 2-1: {}", prob2.ans_to_string(Phase1));
    println!("Problem 2-2: {}", prob2.ans_to_string(Phase2));
}
