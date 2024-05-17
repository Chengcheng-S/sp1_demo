use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ED25519_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    let stdin = SP1Stdin::new();

    let client = ProverClient::new();

    let (pk, vk) = client.setup(ED25519_ELF);
    let plonk_proof = client
        .prove_plonk(&pk, stdin)
        .expect("generate plonk proof failed");

    client
        .verify_plonk(&plonk_proof, &vk)
        .expect("verify plonk proff failed");

    println!("execute success");
}
