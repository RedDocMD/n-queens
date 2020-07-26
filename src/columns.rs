use crate::utils::*;
use rand::prelude::random;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ColumnBoard {
  size: usize,
  queens: Vec<usize>,
  heuristic: i32,
}

impl ColumnBoard {
  pub fn new(size: usize) -> Self {
    let mut queens = Vec::new();
    for _ in 0..size {
      let row: usize = random();
      queens.push(row % size);
    }
    let heuristic = attacking_heuristic(&queens);
    Self {
      size,
      queens,
      heuristic,
    }
  }

  pub fn is_goal(&self) -> bool {
    self.heuristic == 0
  }

  // Call after changing queen in 'column' from 'old_row'
  fn recalculate_heuristic(&mut self, column: usize, old_row: usize) {
    let mut old_contrib = 0;
    let mut new_contrib = 0;

    for i in 0..self.size {
      if i != column {
        if is_in_line((old_row, column), (self.queens[i], i)) {
          old_contrib += 1;
        }
        if is_in_line((self.queens[column], column), (self.queens[i], i)) {
          new_contrib += 1;
        }
      }
    }

    self.heuristic += new_contrib - old_contrib;
  }

  pub fn best_neighbor(self) -> Self {
    let size = self.size;
    let current_queens = self.queens.clone();
    let old_heuristic = self.heuristic;

    let mut min_heuristic = (2 * size * size) as i32;
    let mut position = (current_queens[0], 0);
    let mut next_board = self;

    for column in 0..size {
      let current = current_queens[column];
      for row in 0..size {
        if row != current {
          let old_row = next_board.queens[column];
          next_board.queens[column] = row;
          next_board.recalculate_heuristic(column, old_row);
          if next_board.heuristic < min_heuristic {
            position = (row, column);
            min_heuristic = next_board.heuristic;
          }
          next_board.queens[column] = old_row;
          next_board.recalculate_heuristic(column, row);
        }
      }
    }

    let current = current_queens[position.1];
    next_board.queens[position.1] = position.0;
    next_board.recalculate_heuristic(position.1, current);

    if next_board.heuristic >= old_heuristic {
      next_board.queens[position.1] = current;
      next_board.recalculate_heuristic(position.1, position.0);
    }

    return next_board;
  }
}

impl Display for ColumnBoard {
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
