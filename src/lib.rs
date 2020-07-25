use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

pub struct Board {
  size: usize,
  board: Vec<Vec<bool>>
}

impl Board {
  pub fn new(size: usize) -> Self {
    let mut board = Vec::new();
    for _ in 0..size {
      let mut row = Vec::new();
      for _ in 0..size {
        row.push(false);
      }
      board.push(row);
    }
    Board { size, board }
  }

  pub fn queens_in_row(self: &Self, row: usize) -> Vec<(usize, usize)> {
    let mut queens = Vec::new();
    for column in 0..self.size {
      if self.board[row][column] {
        queens.push((row, column));
      }
    }
    return queens;
  }

  pub fn queens_in_columns(self: &Self, column: usize) -> Vec<(usize, usize)> {
    let mut queens = Vec::new();
    for row in 0..self.size {
      if self.board[row][column] {
        queens.push((row, column));
      }
    }
    return queens;
  }

  pub fn randomly_filled_board(size: usize) -> Self {
    let mut board = Self::new(size);
    board.randomly_fill();
    return board;
  }

  pub fn randomly_fill(self: &mut Self) {
    for j in 0..self.size {
      let i: usize = random();
      self.board[i % self.size][j] = true;
    }
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut output = String::new();
    for row in &self.board {
      for cell in row {
        let mut val = '.';
        if *cell {
          val = 'Q';
        }
        output.push_str(&format!("{} ", val));
      }
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}