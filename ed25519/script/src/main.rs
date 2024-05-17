use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ED25519_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // generate proof
    utils::setup_logger();
    let stdin = SP1Stdin::new();
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ED25519_ELF);
    let proof = client.prove(&pk, stdin).expect("failed to generate proof");

    // verify proof
    client.verify(&proof, &vk).expect("failed to verify proof");

    // save proof
    proof
        .save("proof-with-pis.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
