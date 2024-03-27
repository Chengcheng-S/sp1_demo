use sp1_core::{utils,SP1Prover, SP1Stdin, SP1Verifier};

const REGEX_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_tracer();

    // generate proof
    let mut stdin = SP1Stdin::new();

    let pattern = "a+".to_string();

    let target_string = "an era of truth, not trust".to_string();

    stdin.write(&target_string);
    stdin.write(&pattern);


    let mut proof = SP1Prover::prove(REGEX_ELF, stdin).expect("proving failed");

    // Read output.
    let res = proof.stdout.read::<bool>();

    println!("result: {}", res);

    // Verify proof.
    SP1Verifier::verify(REGEX_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
