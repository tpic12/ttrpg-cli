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

fn parse_roll(roll_str: &str) -> Result<(u32, u32), &'static str> {
    let parts: Vec<&str> = roll_str.split('d').collect();
    match parts.as_slice() {
        [count_str, sides_str] => {
            let count = if count_str.is_empty() {
                1
            } else {
                count_str.parse().map_err(|_| "Invalid number of dice")?
            };
            let sides = sides_str.parse().map_err(|_| "Invalid number of sides")?;
            Ok((count, sides))
        }
        [sides_str] => {
            let sides = sides_str.parse().map_err(|_| "Invalid number of sides")?;
            Ok((1, sides))
        }
        _ => Err("Invalid roll format. Use 'XdY' or 'Y' (e.g., 2d20 or 20)."),
    }
}

fn main() {
    let args = Args::parse();

    let (count, sides) = match parse_roll(&args.roll) {
        Ok((count, sides)) => (count, sides),
        Err(e) => {
            eprintln!("Error: {}", e);
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

