use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Addition of two integers")]
    Add { number_one: i32, number_two: i32 },
    #[clap(about = "Subtraction of two integers")]
    Sub { number_one: i32, number_two: i32 },
    #[clap(about = "Multiplication of two integers")]
    Mul { number_one: i32, number_two: i32 },
    #[clap(about = "Division of two integers")]
    Div { number_one: i32, number_two: i32 },
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Add { number_one, number_two } => {
           println!("Addition result is: {:?}", number_one + number_two)
        },
        Commands::Sub { number_one, number_two } => {
           println!("Subtraction result is: {:?}", number_one - number_two)
        },
        Commands::Mul { number_one, number_two } => {
           println!("Multiplication result is: {:?}", number_one * number_two)
        },
        Commands::Div { number_one, number_two } => {
           println!("Division result is: {:?}", number_one / number_two)
        },
    }
}
