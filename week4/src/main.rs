use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yuxuan Yang",
    about = "A tool that returns the BMR value "
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Bmrcalculator>,
}

#[derive(Parser)]
enum Bmrcalculator {
    #[clap(
        version = "1.0",
        author = "Yuxuan Yang",
        about = "A tool that returns the BMR value "
    )]
    BMR {
        #[clap(short, long)]
        weight: f32,
        #[clap(short, long)]
        tall: f32,
        #[clap(short, long)]
        age: f32,
        #[clap(short, long)]
        choice: i32,
    },
}

fn main() {
    let args: Cli = Cli::parse();
    println!("Please Enter your weight, height, age, choise : ");
    println!("Please note you have following choices: ");
    println!("1. Sedentary (little or no exercise, and desk job)");
    println!("2. Lightly Active(light workout/sports 1-3 days/week)");
    println!("3. Moderately Active(moderate workout/sports 3-5 days/week)");
    println!("4. Very Active(hard workout/sports 6-7 days a week)");
    println!("5. Extra Active(very hard workout/sports & physical job or 2x training)");

    // println!("1. Harris-Benedict Equation");
    // println!("2. Mifflin-St Jeor Equation");
    // println!("3. Katch-McArdle Equation");
    // println!("4. Revised Harris-Benedict Equation");
    match args.command {
        Some(Bmrcalculator::BMR {
            weight,
            tall,
            age,
            choice,
        }) => {
            week4::harris(weight, tall, age, choice);
        }
        None => println!("No command given"),
    }
}
