use sp1_sdk::{utils, ProverClient, SP1Stdin};

const FIBO_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    let client = ProverClient::new();
    let (pk,vk) = client.setup(FIBO_ELF);
    let  proof = client.prove(&pk,stdin).expect("proving failed");

    // Verify proof.
    client.verify( &proof,&vk).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
