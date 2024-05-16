use sp1_sdk::{utils, ProverClient, SP1Stdin};

const FIBO_ELF : &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main(){
    utils::setup_logger();

    let stdin = SP1Stdin::new();
    let client = ProverClient::new();

    let (pk, vk) = client.setup(FIBO_ELF);

    // not implemented
    let mut proof = client.prove_plonk(&pk,stdin).expect("failed to generate plonk prove");

    // Read and verify the output.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();
    println!("a: {}", a);
    println!("b: {}", b);

    client.verify_plonk(&proof, &vk)
    .expect("failed to verify proof");

     // Save the proof.
     proof
     .save("proof-with-pis.json")
     .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")

}