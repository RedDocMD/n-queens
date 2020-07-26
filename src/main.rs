use nqueens::*;

fn main() {
    let board = Board::randomly_filled_board(8);
    println!("{}", board);
    println!("Solved?: {}", board.is_goal());
    println!("Attacking heuristic: {}", board.attacking_heuristic());
}
