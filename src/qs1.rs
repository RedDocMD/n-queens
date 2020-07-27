use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct QS1Board {
  size: usize,
  queens: Vec<usize>,
  positive: Vec<i32>,
  negative: Vec<i32>,
}

impl QS1Board {
  pub fn new(size: usize) -> Self {
    let mut queens: Vec<usize> = (0..size).collect();
    queens.shuffle(&mut thread_rng());
    let mut positive = Vec::new();
    let mut negative = Vec::new();
    for _ in 0..size {
      positive.push(0);
      negative.push(0);
    }
    for column in 0..size {
      let row = queens[column] as i32;
      let column = column as i32;
      positive[(row + column) as usize] += 1;
      negative[(row - column + size as i32) as usize] += 1;
    }
    Self{ queens, size, positive, negative }
  }
}

impl Display for QS1Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut output = String::new();
    for row in 0..self.size {
      for column in 0..self.size {
        let val: char;
        if self.queens[column] == row {
          val = 'Q';
        } else {
          val = '.';
        }
        output.push_str(&format!("{} ", val));
      }
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}