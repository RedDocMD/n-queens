use nqueens::*;

fn main() {
    let size = 30;
    let (moves, solution) = restart_hill_climb(size);
    assert!(solution.is_goal());
    println!("{}", solution);
    println!("In {} moves", moves);
}

fn restart_hill_climb(size: usize) -> (i32, Board) {
  let mut board = Board::randomly_filled_board(size);
  loop {
    let (moves, solution) = hill_climb(&board);
    if let Some(solution) = solution {
      return (moves, solution);
    }
    board = Board::randomly_filled_board(size);
  }
}

fn hill_climb(initial_board: &Board) -> (i32, Option<Board>) {
  let mut board = initial_board.clone();
  let mut cnt = 0;
  loop {
    let next_board = board.best_neighbor();
    cnt += 1;
    if next_board.is_goal() {
      return (cnt, Some(next_board));
    }
    if next_board.attacking_heuristic() == board.attacking_heuristic() {
      return (cnt, None);
    }
    board = next_board;
    // assert!(board.check_board());
  }
}
