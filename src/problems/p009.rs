use crate::traits::Problem;
use std::fs;
use std::str::FromStr;
use std::ops::Sub;
use std::collections::HashSet;

pub struct P009 {
    input: String,
    counts: Option<usize>,
}

impl Problem<usize> for P009 {
    fn new() -> Self {
        P009 {
            input: fs::read_to_string("input/p009.txt").unwrap(),
            counts: None,
        }
    }

    fn solve(&self) -> usize {
        match self.counts {
            Some(c) => return c,
            None => (),
        }

        let mut counts = 0;
        let mut head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);
        let mut tail_pos = HashSet::new();

        for line in self.input.lines() {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            let direction = Direction::from_str(split[0]).unwrap();
            let times = split[1].parse::<usize>().unwrap();

            for _ in 0..times {
                head.move_mut(direction);
                let dir = determine_direction(&head, &tail);
                tail.move_mut(dir);
                let tuple = tail.to_tuple();
                if !tail_pos.contains(&tuple) {
                    tail_pos.insert(tuple);
                    counts += 1;
                }
            }
        }

        counts
    }
    
    fn phase2(&self) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn move_mut(&mut self, direction: Direction) {
        match direction {
            Direction::N => { },
            Direction::U => { self.y += 1 },
            Direction::D => { self.y -= 1 },
            Direction::L => { self.x -= 1 },
            Direction::R => { self.x += 1 },
            Direction::RU => { self.x += 1; self.y += 1 },
            Direction::RD => { self.x += 1; self.y -= 1 },
            Direction::LU => { self.x -= 1; self.y += 1 },
            Direction::LD => { self.x -= 1; self.y -= 1 },
        }
    }

    #[allow(dead_code)]
    fn move_copy(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => self.clone(),
            Direction::U => Position::new(self.x, self.y + 1),
            Direction::D => Position::new(self.x, self.y - 1),
            Direction::L => Position::new(self.x - 1, self.y),
            Direction::R => Position::new(self.x + 1, self.y),
            Direction::RU => Position::new(self.x + 1, self.y + 1),
            Direction::RD => Position::new(self.x + 1, self.y - 1),
            Direction::LU => Position::new(self.x - 1, self.y + 1),
            Direction::LD => Position::new(self.x - 1, self.y - 1),
        }
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[allow(non_snake_case)]
fn determine_direction(H: &Position, T: &Position) -> Direction {
    let diff = H - T;
    match diff.to_tuple() {
        (2, 0) => Direction::R,
        (0, 2) => Direction::U,
        (-2, 0) => Direction::L,
        (0, -2) => Direction::D,
        (1, 2) | (2, 1) => Direction::RU,
        (1, -2) | (2, -1) => Direction::RD,
        (-1, 2) | (-2, 1) => Direction::LU,
        (-1, -2) | (-2, -1) => Direction::LD,
        _ => Direction::N,
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &'b Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    U,
    D,
    L,
    R,
    RU,
    RD,
    LU,
    LD,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Direction::N),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "RU" => Ok(Direction::RU),
            "RD" => Ok(Direction::RD),
            "LU" => Ok(Direction::LU),
            "LD" => Ok(Direction::LD),
            _ => Err(()),
        }
    }
}
