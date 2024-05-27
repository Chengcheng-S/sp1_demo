use sp1_sdk::{utils, ProverClient, SP1Proof, SP1Stdin};

// use bincode;

const FIBO_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // Generate proof.

    let n = 500u32;
    let expected_a = 1926u32;
    let expected_b: u32 = 3194u32;

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(FIBO_ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");
    println!("generated proof");

    // Read and verify the output.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();
    assert_eq!(a, expected_a);
    assert_eq!(b, expected_b);

    // println!("a: {}", a);
    // println!("b: {}", b);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // // Save proof.
    // // proof
    // //     .save("proof-with-io")
    // //     .expect("saving proof failed");

    let proof_for_sp1 = serde_json::to_string(&proof).expect("Unable to serialize data");
    std::fs::write("proof.json", proof_for_sp1).expect("Unable to write file");

    // let data = std::fs::read("./proof-with-io").expect("Unable to read file");

    // let proof_for_sp1: SP1Proof = bincode::deserialize(&data).expect("Unable to deserialize data");

    // let json = serde_json::to_string(&proof_for_sp1).expect("Unable to serialize data");
    // std::fs::write("proof.json", json).expect("Unable to write file");

    println!("succesfully generated and verified proof for the program!")
}
