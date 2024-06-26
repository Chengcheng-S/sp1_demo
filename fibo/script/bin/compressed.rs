use sp1_sdk::{utils, ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '500' to it.
    let n = 500u32;
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Generate the constant-sized proof for the given program and input.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove_compressed(&pk, stdin).unwrap();

    println!("generated proof");
    // Read and verify the output.
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();
    println!("a: {}, b: {}", a, b);

    // Verify proof and public values
    client
        .verify_compressed(&proof, &vk)
        .expect("verification failed");


    let proof_for_sp1 = serde_json::to_string(&proof).expect("Unable to serialize data");
    std::fs::write("proof-compressed.json", proof_for_sp1).expect("Unable to write file");
    // // Save the proof.
    // proof
    //     .save("compressed-proof-with-pis.json")
    //     .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
