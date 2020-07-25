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

  pub fn randomly_filled_board(size: usize) -> Self {
    let mut board = Self::new(size);
    board.randomly_fill();
    return board;
  }

  pub fn randomly_fill(&mut self) {
    for j in 0..self.size {
      let i: usize = random();
      self.board[i % self.size][j] = true;
    }
  }

  pub fn is_goal(&self) -> bool {
    // Row-wise
    for i in 0..self.size {
      let mut cnt = 0;
      for j in 0..self.size {
        if self.board[i][j] {
          cnt += 1
        }
        if cnt > 1 {
          return false;
        }
      }
    }

    // Column-wise
    for i in 0..self.size {
      let mut cnt = 0;
      for j in 0..self.size {
        if self.board[j][i] {
          cnt += 1
        }
        if cnt > 1 {
          return false;
        }
      }
    }

    // Diagonals
    for i in 0..self.size {
      let mut ii = i;
      let mut jj = 0;
      let mut cnt = 0;

      while ii < self.size && jj < self.size {
        if self.board[ii][jj] {
          cnt += 1;
        }
        if cnt > 1 {
          return false;
        }
        ii += 1; jj += 1;
      }

      while jj < self.size {
        if self.board[ii][jj] {
          cnt += 1;
        }
        if cnt > 1 {
          return false;
        }
        if ii == 0 {
          break;
        }
        ii -= 1; jj += 1;
      }
    }

    for j in 0..self.size {
      let mut ii = 0;
      let mut jj = j;
      let mut cnt = 0;

      while ii < self.size && jj < self.size {
        if self.board[ii][jj] {
          cnt += 1;
        }
        if cnt > 1 {
          return false;
        }
        ii += 1; jj += 1;
      }

      while jj < self.size {
        if self.board[ii][jj] {
          cnt += 1;
        }
        if cnt > 1 {
          return false;
        }
        if ii == 0 {
          break;
        }
        ii -= 1; jj += 1;
      }
    }

    return true;
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