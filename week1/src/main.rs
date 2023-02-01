use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Yuxuan Yang", about = "A simple example")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Marco>,
}

#[derive(Parser)]
enum Marco {
    #[clap(version = "1.0", author = "Yuxuan Yang", about = "A simple example")]
    Marco {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();
    match args.command {
        Some(Marco::Marco { input }) => {
            let result = week1::marco_polo(&input);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
