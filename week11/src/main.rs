use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    Add {
        description: String,
        amount: f64,
    },
    History,
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, Deserialize)]
struct Expense {
    timestamp: DateTime<Local>,
    description: String,
    amount: f64,
}

const EXPENSE_FILE: &str = "expenses.json";

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Add { description, amount } => {
            add_expense(&description, amount)?;
            println!("Expense added successfully.");
        }
        Command::History => {
            let expenses = read_expenses()?;
            print_expenses(&expenses);
        }
    }

    Ok(())
}

fn add_expense(description: &str, amount: f64) -> Result<()> {
    let expense = Expense {
        timestamp: Local::now(),
        description: description.to_owned(),
        amount,
    };

    let mut expenses = read_expenses()?;
    expenses.push(expense);

    write_expenses(&expenses)?;
    Ok(())
}

fn read_expenses() -> Result<Vec<Expense>> {
    if Path::new(EXPENSE_FILE).exists() {
        let file = File::open(EXPENSE_FILE)?;
        let reader = BufReader::new(file);
        let expenses = serde_json::from_reader(reader)?;
        Ok(expenses)
    } else {
        Ok(vec![])
    }
}

fn write_expenses(expenses: &[Expense]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(EXPENSE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, expenses)?;
    Ok(())
}

fn print_expenses(expenses: &[Expense]) {
    if expenses.is_empty() {
        println!("No expenses found.");
    } else {
        for expense in expenses {
            println!(
                "{}, {}, {}",
                expense.timestamp.format("%Y-%m-%d %H:%M:%S"),
                expense.description,
                expense.amount
            );
        }
    }
}
