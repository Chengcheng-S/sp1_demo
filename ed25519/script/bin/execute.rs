use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ED25519_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    let stdin = SP1Stdin::new();

    let client = ProverClient::new();

    let _ = client.execute(ED25519_ELF, stdin);

    println!("execute success");
}
