use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yuxuan Yang",
    about = "A tool that returns the longest palindromic substring in a string"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Longestpalin>,
}

#[derive(Parser)]
enum Longestpalin {
    #[clap(
        version = "1.0",
        author = "Yuxuan Yang",
        about = "A tool that returns the longest palindromic substring in a string"
    )]
    Longestpalin {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();
    match args.command {
        Some(Longestpalin::Longestpalin { input }) => {
            let result = week2::longest_palindrome(input);
            println!(
                "The longest palindromic substring in the string is: {}",
                result
            );
        }
        None => println!("No command given"),
    }
}
