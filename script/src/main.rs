//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute a b c
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove a b c
//! ```

//! A program that takes a number `a`, `b`, and `c` as inputs, and writes if `c` is a valid sum of `a` and `b`.
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::env::args;

const ELF: &[u8] = include_elf!("addition-program");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    let inputs: Vec<u32> = args()
        .skip(2)
        .map(|arg| arg.parse::<u32>().expect("Failed to parse input as u32"))
        .collect();

    stdin.write(&inputs[0]);
    stdin.write(&inputs[1]);
    stdin.write(&inputs[2]);

    // Generate and verify the proof.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).run().unwrap();

    let is_valid_addition = proof.public_values.read::<bool>();
    println!("Does a + b == c?: {}", is_valid_addition);

    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof
        .save("proof-addition.bin")
        .expect("saving proof failed");

    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-addition.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}
