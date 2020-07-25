use nqueens::Board;

fn main() {
    let board = Board::randomly_filled_board(8);
    println!("{}", board);
}
