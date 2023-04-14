# Week10 rust mini project
This project demonstrates a simple file encryption and decryption tool using Rust and the sodiumoxide crate.


This example reads data from an input file (input.txt), encrypts it using a randomly generated key and nonce, writes the encrypted data to a file (encrypted.bin), and then decrypts the encrypted data and writes the result to a new file (decrypted.txt). The decrypted data should be the same as the original input data.
## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Dependencies

- sodiumoxide
- anyhow

## Project Structure

The project consists of a single file:

- `src/main.rs`: Contains the main function and all the necessary functions to read, encrypt, decrypt, and write files.

## Running the File Encryption Tool

1. Clone the repository.
2. Navigate to the project directory.
3. Create a file named `input.txt` with some content to be encrypted.
4. Run `cargo build` to compile the project.
5. Run `cargo run` to execute the file encryption tool.

After running the program, you should see two new files in the project directory:

- `encrypted.bin`: The encrypted data from the input file.
- `decrypted.txt`: The decrypted data from the encrypted file, which should match the original input file.

The current implementation reads data from an `input.txt` file, encrypts it, writes the encrypted data to an `encrypted.bin` file, and then decrypts the encrypted data and writes the result to a `decrypted.txt` file. You can modify the paths and names
