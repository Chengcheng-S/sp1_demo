use serde::{Deserialize, Serialize};
use sp1_sdk::{utils, ProverClient, SP1Stdin};

// use serde_json::Result;
// use std::fs::File;
// use std::io::BufReader;

/// The ELF we want to execute inside the zkVM.
const JSON_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MyPointUnaligned {
    pub x: usize,
    pub y: usize,
    pub b: bool,
}

fn main() {
    // Setup a tracer for logging.
    utils::setup_tracer();

    // Create an input stream.
    let mut stdin = SP1Stdin::new();
    let p = MyPointUnaligned {
        x: 1,
        y: 2,
        b: true,
    };
    let q = MyPointUnaligned {
        x: 3,
        y: 4,
        b: false,
    };
    stdin.write(&p);
    stdin.write(&q);

    // Generate the proof for the given program.
    let client = ProverClient::new();
    let (pk,vk) = client.setup(JSON_ELF);

    let mut proof = client.prove(&pk, stdin).unwrap();

    // Read the output.
    let r = proof.public_values.read::<MyPointUnaligned>();
    println!("r: {:?}", r);

    // Verify proof.
    client.verify(&proof,&vk).expect("verification failed");
    /* 
    let file = File::open("proof-with-pis.json").expect("read failed");
    let reader = BufReader::new(file);
    let proof: SP1ProofWithIO<BabyBearPoseidon2> = serde_json::from_reader(reader).expect("deserialization failed");
    println!("{:?}",proof.proof);
    */

    // Save the proof.
    proof
        .save("proof-with-pis.json")
        .expect("saving proof failed");

    
    println!("successfully generated and verified proof for the program!")
}
