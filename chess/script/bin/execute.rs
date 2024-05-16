use sp1_sdk::{utils, ProverClient, SP1Stdin};

const CHESS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    // FEN representation of a chessboard in its initial state
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    stdin.write(&fen);

    // SAN representation Queen's pawn opening
    let san = "d4".to_string();
    stdin.write(&san);

    let client = ProverClient::new();
    let mut public_values = client.execute(&CHESS_ELF, stdin).unwrap();

    let is_valid_move = public_values.read::<bool>();
    assert!(is_valid_move);

    println!("generated proof");

    println!("successfully generated and verified proof for the program!")
}
