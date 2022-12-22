use aoc2022::prelude::*;
use aoc2022_proc::num_to_prob;

fn main() {
    let prob1 = num_to_prob!(1);
    println!("Problem 1-1: {}", prob1.ans_to_string(Phase1));
    println!("Problem 1-2: {}", prob1.ans_to_string(Phase2));

    let prob2 = num_to_prob!(2);
    println!("Problem 2-1: {}", prob2.ans_to_string(Phase1));
    println!("Problem 2-2: {}", prob2.ans_to_string(Phase2));

    let prob3 = num_to_prob!(3);
    println!("Problem 3-1: {}", prob3.ans_to_string(Phase1));
    println!("Problem 3-2: {}", prob3.ans_to_string(Phase2));

    let prob4 = num_to_prob!(4);
    println!("Problem 4-1: {}", prob4.ans_to_string(Phase1));
    println!("Problem 4-2: {}", prob4.ans_to_string(Phase2));

    let prob5 = num_to_prob!(5);
    println!("Problem 5-1: {}", prob5.ans_to_string(Phase1));
    println!("Problem 5-2: {}", prob5.ans_to_string(Phase2));

    let prob6 = num_to_prob!(6);
    println!("Problem 6-1: {}", prob6.ans_to_string(Phase1));
    println!("Problem 6-2: {}", prob6.ans_to_string(Phase2));

    let prob7 = num_to_prob!(7);
    println!("Problem 7-1: {}", prob7.ans_to_string(Phase1));
    println!("Problem 7-2: {}", prob7.ans_to_string(Phase2));
}
