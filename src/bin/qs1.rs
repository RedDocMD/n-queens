use nqueens::qs1::*;
use std::env::args;

fn main() {
  let size: usize;
  if let Some(num) = args().nth(1) {
    size = num.parse().unwrap();
  } else {
    size = 8;
  }
  let solution = find_solution(size);
  println!("{}", solution);
}

fn local_search(size: usize) -> Option<QS1Board> {
  let mut board = QS1Board::new(size);
  loop {
    let mut swap_count = 0;
    for i in 0..size {
      for j in 0..size {
        let value1 = board.total_value(i);
        let value2 = board.total_value(j);

        if value1 > 1 || value2 > 1 {
          let before = value1 + value2;
          board.swap(i, j);
          let after = board.total_value(i) + board.total_value(j);
          if after < before {
            swap_count += 1;
          } else {
            board.swap(i, j);
          }
        }
      }
    }
    if swap_count == 0 {
      if board.is_goal() {
        return Some(board);
      } else {
        return None;
      }
    }
  }
}

fn find_solution(size: usize) -> QS1Board {
  loop {
    let result = local_search(size);
    if let Some(result) = result {
      return result;
    }
  }
}