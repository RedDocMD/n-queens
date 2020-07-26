use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Eq, PartialEq, Debug)]
pub struct Board {
  size: usize,
  board: Vec<Vec<bool>>,
}

pub type Position = (usize, usize);

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
        if is_in_line(&queens[i], &queens[j]) {
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
        if is_in_line(&queens[i], &queens[j]) {
          cnt += 1;
        }
      }
    }
    return cnt;
  }

  pub fn best_neighbor(&self) -> Self {
    let mut next_board = self.clone();
    let mut min_heuristic = self.attacking_heuristic();
    let current_queens = self.columnwise_queens();
    let mut position = (0, current_queens[0]);
    for column in 0..self.size {
      let current = current_queens[column];
      next_board.board[current][column] = false;
      for row in 0..self.size {
        if row != current {
          next_board.board[row][column] = true;
          let heuristic = next_board.attacking_heuristic(); 
          if  heuristic < min_heuristic {
            position = (row, column);
            min_heuristic = heuristic;
          }
          next_board.board[row][column] = false;
        }
      }
      next_board.board[current][column] = true;
    }
    if !self.board[position.0][position.1] {
      next_board.board[position.0][position.1] = true;
    }
    return next_board;
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

impl Clone for Board {
  fn clone(&self) -> Self {
    let mut new_board = Board::new(self.size);
    let queens = self.all_queens();
    for queen in queens {
      new_board.board[queen.0][queen.1] = true;
    }
    return new_board;
  }
}

fn is_in_line(pos1: &Position, pos2: &Position) -> bool {
  let x1 = pos1.0 as isize;
  let x2 = pos2.0 as isize;
  let y1 = pos1.1 as isize;
  let y2 = pos2.1 as isize;

  x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
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

#[test]
fn eq_check() {
    let board = Board::randomly_filled_board(10);
    let cloned_board = board.clone();
    assert_eq!(board, cloned_board);
}
