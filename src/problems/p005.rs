use crate::traits::Problem;
use std::fs;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct P005 {
    input: String,
    message: Option<String>,
}

impl P005 {
    pub fn new() -> Self {
        P005 {
            input: fs::read_to_string("input/p005_test.txt").unwrap(),
            message: None,
        }
    }
}

type Stack<T> = Vec<T>;

#[derive(Debug, Clone)]
struct Crates {
    crates: Vec<Stack<char>>
}

#[derive(Debug, Clone)]
struct Orders {
    orders: Vec<Instructions>
}

#[derive(Debug, Clone)]
struct Instructions {
    num: usize,
    from: usize,
    to: usize,
}

impl Instructions {
    fn move_crate(&self, cr: &mut Crates) {
        let num = self.num;
        let from = self.from;
        let to = self.to;

        let mut stack = cr.crates[from].clone();
        let mut new_stack = cr.crates[to].clone();

        let mut i = 0;
        while i < num {
            let c = stack.pop().unwrap();
            new_stack.push(c);
            i += 1;
        }

        cr.crates[from] = stack;
        cr.crates[to] = new_stack;
    }
}

impl FromStr for Instructions {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Ok(
            Instructions { num, from, to }
        )
    }
}

impl FromStr for Orders {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut orders = Vec::new();
        for line in s.lines() {
            let order = line.parse::<Instructions>().unwrap();
            orders.push(order);
        }
        Ok(
            Orders { orders }
        )
    }
}

impl Iterator for Orders {
    type Item = Instructions;

    fn next(&mut self) -> Option<Self::Item> {
        self.orders.pop()
    }
}
