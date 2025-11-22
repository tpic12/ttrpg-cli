use clap::Parser;
mod dice;
use std::process;

#[derive(Parser)]
#[command(name = "ttrpg", version, about = "A TTRPG CLI tool", long_about=None)]
struct Args {
    #[arg(short, long)]
    /// Number of sides on die in format "XdY" (e.g., 2d20).
    roll: String,

    #[arg(short, long)]
    /// Roll with advantage
    advantage: bool,

    #[arg(short, long)]
    /// roll with disadvantage
    disadvantage: bool,
}

fn main() {
    let args = Args::parse();

    let (count, sides) = match dice::parse_roll(&args.roll) {
        Ok((count, sides)) => (count, sides),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        }
    };

    match dice::roll_dice(sides, count, args.advantage, args.disadvantage) {
        Ok(result) => println!("Roll Result: {}", result),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        }
    }
}