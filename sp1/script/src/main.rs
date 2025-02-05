//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FRACTAL_ELF: &[u8] = include_elf!("sketch");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "16")]
    size: u64,

    #[clap(long, default_value = "7714059436743860000")]
    x: u64,

    #[clap(long, default_value = "9940888892175165000")]
    y: u64,

    #[clap(long, default_value = "11458566504308191000")]
    zoom: u64,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&[args.size, args.x, args.y, args.zoom]);

    println!(
        "size: {}, x: {}, y: {}, zoom: {}",
        args.size, args.x, args.y, args.zoom
    );

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FRACTAL_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        println!("buffer: {}", output.raw());

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FRACTAL_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");
        proof.save("./proof.bin").expect("failed to save proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
