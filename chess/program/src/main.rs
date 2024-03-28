#![no_main]
sp1_zkvm::entrypoint!(main);

use chess::{Board, ChessMove};
use std::str::FromStr;

pub fn main() {
    // read the board position in FEN , and move in SAN
    let fen = sp1_zkvm::io::read::<String>();
    let san = sp1_zkvm::io::read::<String>();
    
    // generate the chessboard from the fen output
    let b  = Board::from_str(&fen).expect("valid FEN board");
    
    // Try to parse the SAN as a legal chess move
    let is_valid_move = ChessMove::from_san(&b, &san).is_ok();

    sp1_zkvm::io::write(&is_valid_move);
}
