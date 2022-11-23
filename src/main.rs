use clap::Parser;

// Simple program to add numbers
#[derive(Parser)]
struct Value {
    // First number to add
    #[clap(short = 'o', long)]
    number_one: i32,
    // First number to add
    #[clap(short = 't', long)]
    number_two: i32,
}

fn main() {
    let value = Value::parse();

    let first_value = value.number_one;
    let second_value = value.number_two;

    let answer = first_value + second_value;

    println!("The answer is: {answer}!");
}
