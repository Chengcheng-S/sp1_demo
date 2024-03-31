use sp1_sdk::{utils,SP1Prover, SP1Stdin, SP1Verifier};

const WITHDRAWLS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let a = proof.stdout.read::<u128>();
    let b = proof.stdout.read::<u128>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
