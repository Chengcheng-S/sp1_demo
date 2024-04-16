use sp1_sdk::{ProverClient, SP1Stdin};

const CHESS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let mut stdin = SP1Stdin::new();

    // FEN representation of a chessboard in its initial state
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    stdin.write(&fen);

    // SAN representation Queen's pawn opening
    let san = "d4".to_string();
    stdin.write(&san);

    let client = ProverClient::new();
    let mut proof = client.prove(CHESS_ELF, stdin).unwrap();

    // Read output.
    let is_valid_move = proof.public_values.read::<bool>();
    println!("is_valid_move: {}", is_valid_move);

    // Verify proof.
    client.verify(CHESS_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
