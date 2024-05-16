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
    let (pk, vk) = client.setup(CHESS_ELF);
    let mut groth16_proof = client.prove_groth16(&pk, stdin).unwrap();

    client.verify_groth16(&groth16_proof, &vk).unwrap();

    let is_valid_move = groth16_proof.public_values.read::<bool>();
    assert!(is_valid_move);

    println!("generated proof");

    println!("successfully generated and verified proof for the program!")
}
