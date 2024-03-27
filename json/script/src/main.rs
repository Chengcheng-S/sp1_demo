use lib::{Account, Transaction};
use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

const JSON_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_tracer();

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // generic sample Json (as a string input)
    let data_str = r#"{
        "name" :"Tim",
        "age":"30",
        "net_worth":"$100000"
    }"#
    .to_string();

    let key = "net_worth".to_string();

    // custom struct example.
    let init_account = Account {
        account_name: "Tim".to_string(),
        balance: 200,
    };

    let transactions = vec![
        Transaction {
            from: "Tim".to_string(),
            to: "John".to_string(),
            value: 100,
        },
        Transaction {
            from: "Jhon".to_string(),
            to: "Tim".to_string(),
            value: 50,
        },
    ];

    stdin.write(&data_str);
    stdin.write(&key);
    stdin.write(&init_account);
    stdin.write(&transactions);

    let mut proof = SP1Prover::prove(JSON_ELF, stdin).expect("proving failed");

    // Read output
    let val = proof.stdout.read::<String>();
    println! {"Value of {} is {}",key,val};

    let account_state = proof.stdout.read::<Account>();
    println! {"Balance of {} is {}",&account_state.account_name,&account_state.balance};
    println! {"final account state is {}",serde_json::to_string(&account_state).unwrap()};

    // Verify proof.
    SP1Verifier::verify(JSON_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
