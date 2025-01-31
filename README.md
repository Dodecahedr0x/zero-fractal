# Zero Fractal NFT

This project started in the context of the [Turbin3 Advanced SVM cohort](https://www.turbin3.com/) of Q1 2025.
The original idea is to use Succint SP1 zkVM and verify it on Solana.

## High Level Idea

We want to create an NFT collection where each NFT has the image of some portion of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set), and where holders can update independently the image of their NFT to any portion of the set.

The problem with a naive approach is that the Mandelbrot set is to complex to compute on-chain and so an on-chain program cannot verify that the new image provided by a holder is correct. There are also too many possible values, such that it is impossible to pre-compute each of them and verify that it corresponds to what a user submitted.

Thanks to ZK proofs, we are able to run the compute-intensive task of generating the image, and let an on-chain program verify that the provided image is valid without recomputing it.

## Project structure

1. `programs/test-sketch` is just a playground to generate the image and look for nice results.
2. `programs/sketch` is the actual verifiable program used to generate the proof.
3. `programs/zero-fractal` is the on-chain program that will mint NFTs and update them by interacting with the on-chain verifier.
4. `app` contains a simple front-end to interact with the program.

## Usage

1. `cd sp1/program`
2. `cargo prove build`
3. `cd ../script`
4. `SP1_ELF_sketch=../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/sketch cargo run --release -- --prove` (use `--size` to adjust the resource usage)