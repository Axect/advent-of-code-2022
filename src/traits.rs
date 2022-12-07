use std::fmt::Display;
use Phase::{Phase1, Phase2};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Phase {
    Phase1,
    Phase2,
}

pub trait Problem<T: Sized + Display>: Sized {
    fn solve(&self) -> T;
    fn phase2(&self) -> Self;
    fn ans_to_string(&self, phase: Phase) -> String {
        match phase {
            Phase1 => format!("{}", self.solve()),
            Phase2 => format!("{}", self.phase2().solve()),
        }
    }

}
