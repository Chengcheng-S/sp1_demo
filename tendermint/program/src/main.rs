#![no_main]
sp1_zkvm::entrypoint!(main);

use core::time::Duration;
use tendermint_light_client_verifier::{
    options::Options, types::LightBlock, ProdVerifier, Verdict, Verifier,
};

pub fn main() {
    println!("cycle-tracker-start: io");
    println!("cycle-tracker-start: reading bytes");

    let encode1 = sp1_zkvm::io::read::<Vec<u8>>();
    let encode2 = sp1_zkvm::io::read::<Vec<u8>>();
    println!("cycle-tracker-end: reading bytes");

    println!("cycle-tracker-start: serde");
    let block1: LightBlock = serde_cbor::from_slice(&encode1).unwrap();
    let block2: LightBlock = serde_cbor::from_slice(&encode2).unwrap();
    println!("cycle-tracker-end: serde");
    println!("cycle-tracker-end: io");

    println!(
        "
        LightBlock1 number of Vaildators:{}
    ",
        block1.validators.validators().len()
    );

    println!(
        "
        LightBlock2 number of Vaildators:{}
    ",
        block2.validators.validators().len()
    );

    println!("cycle-tracker-start: verify");
    let vp = ProdVerifier::default();
    let opt = Options {
        trust_threshold: Default::default(),
        trusting_period: Duration::from_secs(500),
        clock_drift: Default::default(),
    };

    let verify_time = block2.time() + Duration::from_secs(20);
    let verdict = vp.verify_update_header(
        block2.as_untrusted_state(),
        block1.as_trusted_state(),
        &opt,
        verify_time.unwrap(),
    );

    println!("cycle-tracker-end: verify");

    match verdict {
        Verdict::Success => println!("cycle-tracker-end: Valid"),
        _ => panic!("cycle-tracker-end: Invalid"),
    }
}
