use sp1_sdk::{utils, SP1Stdin, ProverClient};

const WITHDRAWLS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();
    let client = ProverClient::new();

    let mut proof = SP1Prover::prove(WITHDRAWLS_ELF, stdin).expect("proving failed");


    // Verify proof.
    SP1Verifier::verify(WITHDRAWLS_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
