use reqwest::Client;
use sp1_sdk::{utils, SP1Prover, SP1Stdin, SP1Verifier};
pub mod util;
use util::{fetch_latest_commit, fetch_light_block};

const TENDERMINT_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() {
    utils::setup_tracer();

    //Uniquely identify a peer in the network.
    let peer_id: [u8; 20] = [
        0x72, 0x6b, 0xc8, 0xd2, 0x60, 0x38, 0x7c, 0xf5, 0x6e, 0xcf, 0xad, 0x3a, 0x6b, 0xf6, 0xfe,
        0xcd, 0x90, 0x3e, 0x18, 0xa2,
    ];

    // Generate proof.
    const BASE_URL: &str = "https://celestia-mocha-rpc.publicnode.com:443";

    let client = Client::new();
    let url = format!("{}/commit", BASE_URL);
    let latest_commit = fetch_latest_commit(&client, &url).await.unwrap();
    let block: u64 = latest_commit.result.signed_header.header.height.into();
    println!("Latest block :{}", block);

    let light_block_1 = fetch_light_block(peer_id, BASE_URL, block - 20)
        .await
        .expect("Failed to fetch light block 1");
    let light_block_2 = fetch_light_block(peer_id, BASE_URL, block).await.unwrap();

    let mut stdin = SP1Stdin::new();

    let encode_block_1 = serde_cbor::to_vec(&light_block_1).unwrap();
    let encode_block_2 = serde_cbor::to_vec(&light_block_2).unwrap();

    stdin.write(&encode_block_1);
    stdin.write(&encode_block_2);

    let  proof = SP1Prover::prove(TENDERMINT_ELF, stdin).expect("proving failed");

    // Verify proof.
    SP1Verifier::verify(TENDERMINT_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
