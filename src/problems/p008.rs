use crate::traits::Problem;
use std::fs;

#[derive(Debug, Clone)]
pub struct P008 {
    input: String,
    visible: Option<u32>,
}

impl P008 {
    pub fn new() -> Self {
        let input = fs::read_to_string("input/p008.txt").unwrap();
        P008 {
            input,
            visible: None,
        }
    }
}

impl Problem<u32> for P008 {
    fn solve(&self) -> u32 {
        match self.visible {
            Some(vis) => return vis,
            None => (),
        }
        
        let input = &self.input;
        let input = input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();

        let forest = input.as_slice().iter().map(|l| l.as_slice()).collect::<Vec<&[u32]>>();

        let row = forest.len();
        let col = forest[0].len();

        let mut visible = 0;

        for i in 0 .. row {
            for j in 0 .. col {
                if is_visible((i, j), &forest) {
                    visible += 1;
                }
            }
        }

        visible
    }

    fn phase2(&self) -> Self {
        let input = &self.input;
        let input = input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();

        let forest = input.as_slice().iter().map(|l| l.as_slice()).collect::<Vec<&[u32]>>();

        let row = forest.len();
        let col = forest[0].len();

        let mut visible = 0;

        for i in 0 .. row {
            for j in 0 .. col {
                let count = count_trees((i, j), &forest);
                if count > visible {
                    visible = count;
                }
            }
        }

        Self {
            input: self.input.clone(),
            visible: Some(visible),
        }
    }
}

fn is_visible((i, j): (usize, usize), forest: &[&[u32]]) -> bool {
    match (i, j) {
        (0, _) => true,
        (_, 0) => true,
        (a, _) if a == forest.len()-1 => true,
        (_, b) if b == forest[0].len() - 1 => true,
        (i, j) => {
            check_top((i, j), forest)
                || check_bottom((i, j), forest)
                || check_left((i, j), forest)
                || check_right((i, j), forest)
        }
    }   
}

fn check_top((i, j): (usize, usize), forest: &[&[u32]]) -> bool {
    let height = forest[i][j];
    (0 .. i).all(|k| forest[k][j] < height)
}

fn check_left((i, j): (usize, usize), forest: &[&[u32]]) -> bool {
    let height = forest[i][j];
    (0 .. j).all(|k| forest[i][k] < height)
}

fn check_bottom((i, j): (usize, usize), forest: &[&[u32]]) -> bool {
    let height = forest[i][j];
    (i+1 .. forest.len()).all(|k| forest[k][j] < height)
}

fn check_right((i, j): (usize, usize), forest: &[&[u32]]) -> bool {
    let height = forest[i][j];
    (j+1 .. forest[0].len()).all(|k| forest[i][k] < height)
}

fn count_trees((i, j): (usize, usize), forest: &[&[u32]]) -> u32 {
    let top = count_top((i, j), forest);
    let bottom = count_bottom((i, j), forest);
    let left = count_left((i, j), forest);
    let right = count_right((i, j), forest);

    top * bottom * left * right
}

fn count_top((i, j): (usize, usize), forest: &[&[u32]]) -> u32 {
    let height = forest[i][j];
    let mut count = 0;
    for k in (0 .. i).rev() {
        if forest[k][j] < height {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn count_bottom((i, j): (usize, usize), forest: &[&[u32]]) -> u32 {
    let height = forest[i][j];
    let mut count = 0;
    for k in i+1 .. forest.len() {
        if forest[k][j] < height {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn count_left((i, j): (usize, usize), forest: &[&[u32]]) -> u32 {
    let height = forest[i][j];
    let mut count = 0;
    for k in (0 .. j).rev() {
        if forest[i][k] < height {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn count_right((i, j): (usize, usize), forest: &[&[u32]]) -> u32 {
    let height = forest[i][j];
    let mut count = 0;
    for k in j+1 .. forest[0].len() {
        if forest[i][k] < height {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}
