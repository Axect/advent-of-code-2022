use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use crate::traits::Problem;
use regex::Regex;
use Crane::*;

#[derive(Debug, Clone)]
pub struct P005 {
    input: String,
    message: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Crane {
    Old,
    New,
}

impl P005 {
    fn move_with_crane(&self, crane: Crane) -> Crates {
        let mut crates_str = String::new();
        let mut orders_str = String::new();
        let mut orders = false;

        for line in self.input.lines() {
            if !orders {
                if line.trim().is_empty() {
                    orders = true;
                } else {
                    crates_str.push_str(line);
                    crates_str.push_str("\n");
                }
            } else {
                orders_str.push_str(line);
                orders_str.push_str("\n");
            }
        }

        let mut crates = Crates::from_str(&crates_str).unwrap();
        let orders = Orders::from_str(&orders_str).unwrap();

        for inst in orders.into_iter() {
            match crane {
                Old => inst.move_crate(&mut crates),
                New => inst.move_crate_9001(&mut crates),
            }
        }

        crates
    }
}

impl Problem<String> for P005 {
    fn new() -> Self {
        P005 {
            input: fs::read_to_string("input/p005.txt").unwrap(),
            message: None,
        }
    }

    fn solve(&self) -> String {
        match &self.message {
            None => (),
            Some(msg) => return msg.to_string(),
        }

        let cr = self.move_with_crane(Old).crates;

        let mut ans = String::new();
        
        for i in 1usize .. cr.len() + 1 {
            ans.push(cr.get(&i).unwrap().last().unwrap().clone());
        }

        ans
    }

    fn phase2(&self) -> Self {
        let cr = self.move_with_crane(New).crates;
        let mut ans = String::new();

        for i in 1usize .. cr.len() + 1 {
            ans.push(cr.get(&i).unwrap().last().unwrap().clone());
        }

        P005 {
            input: self.input.clone(),
            message: Some(ans),
        }
    }
}

type Stack<T> = Vec<T>;

#[derive(Debug, Clone)]
struct Crates {
    crates: HashMap<usize, Stack<char>>
}

impl std::fmt::Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for i in 1 .. self.crates.len() + 1 {
            result.push_str(&format!("{}: ", i));
            for c in self.crates.get(&i).unwrap().iter() {
                result.push('[');
                result.push(*c);
                result.push(']');
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
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

        let mut from_stack = cr.crates.get(&from).unwrap().clone();
        let mut to_stack = cr.crates.get(&to).unwrap().clone();

        for _ in 0..num {
            let c = match from_stack.pop() {
                Some(c) => c,
                None => {
                    dbg!(&self);
                    println!("{}", cr);
                    panic!("Not enough crates to move");
                },
            };
            to_stack.push(c);
        }

        cr.crates.get_mut(&from).unwrap().clone_from(&from_stack);
        cr.crates.get_mut(&to).unwrap().clone_from(&to_stack);
    }

    fn move_crate_9001(&self, cr: &mut Crates) {
        let num = self.num;
        let from = self.from;
        let to = self.to;

        let mut from_stack = cr.crates.get(&from).unwrap().clone();
        let mut to_stack = cr.crates.get(&to).unwrap().clone();
        let mut temp_stack = vec![];

        for _ in 0..num {
            let c = match from_stack.pop() {
                Some(c) => c,
                None => {
                    dbg!(&self);
                    println!("{}", cr);
                    panic!("Not enough crates to move");
                },
            };
            temp_stack.push(c);
        }
        for _ in 0 .. num {
            to_stack.push(temp_stack.pop().unwrap());
        }

        cr.crates.get_mut(&from).unwrap().clone_from(&from_stack);
        cr.crates.get_mut(&to).unwrap().clone_from(&to_stack);
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
        orders.reverse();
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

impl FromStr for Crates {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = HashMap::new();

        for line in s.lines() {
            line.chars().collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(i, c)| {
                    let stack = crates.entry(i+1).or_insert(Vec::new());
                    let c = c.iter().collect::<String>();
                    let c = c.trim();
                    let re = Regex::new(r"\[(\w)\]").unwrap();
                    let caps = re.captures(c);
                    if let Some(caps) = caps {
                        let c = caps.get(1).unwrap().as_str().parse::<char>().unwrap();
                        stack.push(c);
                    }
                });
        }

        crates.iter_mut().for_each(|(_, v)| {
            v.reverse();
        });

        Ok(
            Crates { crates }
        )
    }
}
