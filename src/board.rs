use crate::utils::*;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Board {
  size: usize,
  board: Vec<Vec<bool>>,
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

  fn all_queens(&self) -> Vec<Position> {
    let mut positions = Vec::new();
    for i in 0..self.size {
      for j in 0..self.size {
        if self.board[i][j] {
          positions.push((i, j));
        }
      }
    }
    return positions;
  }

  fn columnwise_queens(&self) -> Vec<usize> {
    let mut queens = Vec::new();
    for column in 0..self.size {
      for row in 0..self.size {
        if self.board[row][column] {
          queens.push(row);
          break;
        }
      }
    }
    return queens;
  }

  pub fn is_goal(&self) -> bool {
    let queens = self.all_queens();
    for i in 0..self.size {
      for j in (i + 1)..self.size {
        if is_in_line(queens[i], queens[j]) {
          return false;
        }
      }
    }
    return true;
  }

  pub fn attacking_heuristic(&self) -> i32 {
    let mut cnt = 0;
    let queens = self.all_queens();
    for i in 0..self.size {
      for j in (i + 1)..self.size {
        if is_in_line(queens[i], queens[j]) {
          cnt += 1;
        }
      }
    }
    return cnt;
  }

  pub fn best_neighbor(self) -> Self {
    let size = self.size;
    let current_queens = self.columnwise_queens();
    let old_heuristic = self.attacking_heuristic();

    let mut min_heuristic = (2 * size * size) as i32;
    let mut position = (current_queens[0], 0);
    let mut next_board = self;

    for column in 0..size {
      let current = current_queens[column];
      next_board.board[current][column] = false;
      for row in 0..size {
        if row != current {
          next_board.board[row][column] = true;
          let heuristic = next_board.attacking_heuristic();
          if heuristic < min_heuristic {
            position = (row, column);
            min_heuristic = heuristic;
          }
          next_board.board[row][column] = false;
        }
      }
      next_board.board[current][column] = true;
    }

    let current = current_queens[position.1];
    next_board.board[current][position.1] = false;
    next_board.board[position.0][position.1] = true;

    if next_board.attacking_heuristic() >= old_heuristic {
      next_board.board[current][position.1] = true;
      next_board.board[position.0][position.1] = false;
    }
    return next_board;
  }

  pub fn check_board(&self) -> bool {
    let queens = self.all_queens();
    if queens.len() != self.size {
      return false;
    } else {
      for column in 0..self.size {
        let mut cnt = 0;
        for queen in &queens {
          if queen.1 == column {
            cnt += 1;
          }
        }
        if cnt != 1 {
          return false;
        }
      }
      return true;
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

#[test]
fn goal_check() {
  let mut solved = Board::new(8);

  solved.board[0][5] = true;
  solved.board[1][3] = true;
  solved.board[2][6] = true;
  solved.board[3][0] = true;
  solved.board[4][7] = true;
  solved.board[5][1] = true;
  solved.board[6][4] = true;
  solved.board[7][2] = true;

  assert!(solved.is_goal());
  assert_eq!(solved.attacking_heuristic(), 0);
}
