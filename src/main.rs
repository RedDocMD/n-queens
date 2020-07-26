use nqueens::*;
use std::env::args;

fn main() {
    let size: usize;
    if let Some(num) = args().nth(1) {
      size = num.parse().unwrap();
       
    } else {
      size = 8;
    }
    let (moves, tries, solution) = restart_hill_climb(size);
    assert!(solution.is_goal());
    println!("{}", solution);
    println!("In {} moves", moves);
    println!("In {} restarts", tries);
}

fn restart_hill_climb(size: usize) -> (i32, i64, Board) {
  let mut board = Board::randomly_filled_board(size);
  let mut tries = 0;
  loop {
    let (moves, solution) = hill_climb(board);
    tries += 1;
    if let Some(solution) = solution {
      return (moves, tries, solution);
    }
    board = Board::randomly_filled_board(size);
  }
}

fn hill_climb(board: Board) -> (i32, Option<Board>) {
  let mut cnt = 0;
  let mut board = board;
  loop {
    let current_heuristic = board.attacking_heuristic();
    let next_board = board.best_neighbor();
    cnt += 1;
    if next_board.is_goal() {
      return (cnt, Some(next_board));
    }
    if next_board.attacking_heuristic() == current_heuristic {
      return (cnt, None);
    }
    board = next_board;
    // assert!(board.check_board());
  }
}
