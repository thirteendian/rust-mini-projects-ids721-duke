# Rust Expense Tracker

This project demonstrates a simple command-line expense tracker using Rust and the chrono crate. It allows users to log their expenses and track their spending.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Dependencies

- chrono
- serde
- serde_derive
- serde_json
- structopt
- anyhow

## Project Structure

The project consists of a single file:
- `src/main.rs`: Contains the main function and all the necessary functions to add expenses, read expenses, write expenses, and print expenses.
## Running the Expense Tracker

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build` to compile the project.
4. Run the following commands to interact with the expense tracker:

- To add an expense, use: `cargo run -- add --description "Expense description" --amount 123.45`
- To view the expense history, use: `cargo run -- history`

Expenses are stored in a JSON file named `expenses.json` in the project directory.
