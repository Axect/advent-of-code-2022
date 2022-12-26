use std::fmt::Display;

pub use crate::problems::{
    p001::P001,
    p002::P002,
    p003::P003,
    p004::P004,
    p005::P005,
    p006::P006,
    p007::P007,
    p008::P008,
};
pub use crate::traits::Phase::{Phase1, Phase2};
pub use crate::traits::*;

pub fn get_ans(problem_num: usize, phase: Phase) -> String {
    match problem_num {
        1 => get_ans_by_problem(P001::new(), phase),
        2 => get_ans_by_problem(P002::new(), phase),
        3 => get_ans_by_problem(P003::new(), phase),
        4 => get_ans_by_problem(P004::new(), phase),
        5 => get_ans_by_problem(P005::new(), phase),
        6 => get_ans_by_problem(P006::new(), phase),
        7 => get_ans_by_problem(P007::new(), phase),
        8 => get_ans_by_problem(P008::new(), phase),
        _ => panic!("Problem {} not implemented", problem_num),
    }
}

fn get_ans_by_problem<T: Display, P: Problem<T>>(p: P, phase: Phase) -> String {
    p.ans_to_string(phase)
}

pub fn num_to_phase(num: usize) -> Phase {
    match num {
        1 => Phase1,
        2 => Phase2,
        _ => panic!("Phase {} not implemented", num),
    }
}
