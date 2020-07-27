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
    for _ in 0..(2 * size) {
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

  pub fn positive_value(&self, column: usize) -> i32 {
    self.positive[column + self.queens[column]]
  }

  pub fn negative_value(&self, column: usize) -> i32 {
    let row = self.queens[column] as i32;
    let column = column as i32;

    self.negative[(row - column + self.size as i32) as usize]
  }

  pub fn total_value(&self, column: usize) -> i32 {
    self.positive_value(column) + self.negative_value(column)
  }

  pub fn swap(&mut self, column1: usize, column2: usize) {
    let row1 = self.queens[column1] as i32;
    let row2 = self.queens[column2] as i32;
    let column1 = column1 as i32;
    let column2 = column2 as i32;
    let size = self.size as i32;

    self.positive[(row1 + column1) as usize] -= 1;
    self.positive[(row2 + column2) as usize] -= 1;
    self.negative[(row1 - column1 + size) as usize] -= 1;  
    self.negative[(row2 - column2 + size) as usize] -= 1;

    self.queens[column1 as usize] = row2 as usize;
    self.queens[column2 as usize] = row1 as usize;

    self.positive[(row2 + column1) as usize] += 1;
    self.positive[(row1 + column2) as usize] += 1;
    self.negative[(row2 - column1 + size) as usize] += 1;  
    self.negative[(row1 - column2 + size) as usize] += 1;
  }
  
  pub fn is_goal(&self) -> bool {
    for value in &self.positive {
      if *value > 1 {
        return false;
      }
    }
    for value in &self.negative {
      if *value > 1 {
        return false;
      }
    }

    return true;
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