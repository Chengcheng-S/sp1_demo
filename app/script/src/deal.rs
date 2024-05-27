pub mod Deal_ZKP {

    use sp1_core::{stark::ShardProof, utils::BabyBearPoseidon2};
    use sp1_sdk::{utils, ProverClient, SP1ProofWithMetadata, SP1Stdin};
    use std::fs::File;
    use std::io::BufReader;

    const APP_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

    pub async fn deal_zkp() {

        utils::setup_tracer();
        // Generate proof.
        let mut stdin = SP1Stdin::new();
        let n = 186u32;
        stdin.write(&n);
        let client = ProverClient::new();
        let (_, vk) = client.setup(APP_ELF);

        // let mut proof = client.prove(&pk, stdin).expect("proving failed");

        // Verify proof.
        // client.verify(&proof, &vk).expect("verification failed");

        // // Save proof.
        // proof
        // .save("proof-with-pis.json")
        // .expect("saving proof failed");

        // let proof_for_sp1 = serde_json::to_string(&proof).expect("Unable to serialize data");
        // std::fs::write("proof.json", proof_for_sp1).expect("Unable to write file");

        let file = File::open("proof.json").expect("read failed");
        let reader = BufReader::new(file);
        let proof: SP1ProofWithMetadata<Vec<ShardProof<BabyBearPoseidon2>>> =
            serde_json::from_reader(reader).expect("deserialization failed");

        client.verify(&proof, &vk).expect("verification failed");

        println!("successfully generated and verified proof for the program!")
    }
}
