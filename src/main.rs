use nqueens::*;

fn main() {
    let board = Board::randomly_filled_board(8);
    println!("Initial board:");
    println!("{}", board);
    let (moves, result) = hill_climb(&board);
    println!("Final board:");
    println!("{}", result);
    println!("Moves = {}", moves);
    if result.is_goal() {
      println!("Goal reached!");
    } else {
      println!("Goal not reached!");
    }
}

fn hill_climb(initial_board: &Board) -> (i32, Board) {
  let mut board = initial_board.clone();
  let mut cnt = 0;
  loop {
    let next_board = board.best_neighbor();
    cnt += 1;
    if next_board.is_goal() {
      return (cnt, board);
    }
    if next_board == board {
      return (cnt, board);
    }
    board = next_board;
    // assert!(board.check_board());
    println!("{}", board);
    println!("Current heuristic: {}", board.attacking_heuristic());
  }
}
