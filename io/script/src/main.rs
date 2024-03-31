use sp1_sdk::{utils,SP1Prover, SP1Stdin, SP1Verifier};
use serde::{Deserialize,Serialize};

const IO_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");


#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct MyPointUnaligned{
    pub x : usize,
    pub y : usize,
    pub b : bool,
}


fn main() {
    utils::setup_tracer();
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let p = MyPointUnaligned{
        x: 1,
        y: 2,
        b: true,
    };
    let q = MyPointUnaligned{
        x: 3,
        y: 4,
        b: false,
    };
    stdin.write(&p);
    stdin.write(&q);

    let mut proof = SP1Prover::prove(IO_ELF, stdin).expect("proving failed");

    // Read output.
    let r = proof.stdout.read::<MyPointUnaligned>();
    
    println!("r: {:?}", r);
    
    // Verify proof.
    SP1Verifier::verify(IO_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}