use clap::{CommandFactory, Parser};
mod dice;
mod open5e; // Import the new module
use std::process;

#[derive(Parser)]
#[command(name = "ttrpg", version, about = "A TTRPG CLI tool", long_about=None)]
struct Args {
    #[arg(short, long)]
    /// Number of sides on die in format "XdY" (e.g., 2d20).
    roll: Option<String>, // Make roll optional

    #[arg(short, long)]
    /// Roll with advantage
    advantage: bool,

    #[arg(short, long)]
    /// roll with disadvantage
    disadvantage: bool,

    #[arg(long)]
    /// Get information about a D&D class (e.g., "paladin" or "barbarian")
    class: Option<String>,

    #[arg(long, num_args = 1..)]
    /// Get information about a D&D spell (e.g., "fire shield" or "fireball")
    spell: Option<Vec<String>>,
}

#[tokio::main]
async fn main() {
    // Initialize the base URL for the Open5e API
    // Removed after open5e.rs was reverted, now not needed as BASE_URL is a const
    // open5e::init_base_url("https://api.open5e.com");

    let args = Args::parse();

    if let Some(class_name) = args.class {
        match open5e::get_class_by_slug(&class_name).await {
            Ok(Some(class)) => {
                println!("{}", serde_json::to_string_pretty(&class).unwrap());
            }
            Ok(None) => {
                eprintln!("Error: Class '{}' not found.", class_name);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error fetching class information: {:?}", e);
                process::exit(1);
            }
        }
    } else if let Some(spell_parts) = args.spell {
        let spell_name = spell_parts.join("-").to_lowercase();
        match open5e::get_spell_by_slug(&spell_name).await {
            Ok(Some(spell)) => {
                println!("{}", serde_json::to_string_pretty(&spell).unwrap());
            }
            Ok(None) => {
                eprintln!("Error: Spell '{}' not found.", spell_name);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error fetching spell information: {:?}", e);
                process::exit(1);
            }
        }
    } else if let Some(roll_str) = args.roll {
        let (count, sides) = match dice::parse_roll(&roll_str) {
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
    } else {
        let mut cmd = Args::command();
        cmd.print_help().unwrap();
        process::exit(1);
    }
}