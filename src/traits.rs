use std::fmt::Display;

pub trait Problem<T: Sized + Display> {
    fn solve(&self) -> T;
    fn ans_to_string(&self) -> String {
        format!("{}", self.solve())
    }
}
