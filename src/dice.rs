use rand::Rng;
use colored::*;

#[derive(Debug)]
pub enum DiceError {
    InvalidSides,
    InvalidCount,
}

fn format_roll(roll: u32, sides: u32) -> String {
    if roll == 1 {
        roll.to_string().red().to_string()
    } else if roll == sides {
        roll.to_string().green().to_string()
    } else {
        roll.to_string()
    }
}

pub fn roll_dice(
    sides: u32,
    count: u32,
    advantage: bool,
    disadvantage: bool,
) -> Result<String, DiceError> {
    if sides < 2 {
        return Err(DiceError::InvalidSides);
    }

    if count < 1 {
        return Err(DiceError::InvalidCount);
    }

    let mut rolls = Vec::new();

    if advantage || disadvantage {
        let mut rng = rand::thread_rng();
        let roll1 = rng.gen_range(1..=sides);
        let roll2 = rng.gen_range(1..=sides);

        let formatted_roll1 = format_roll(roll1, sides);
        let formatted_roll2 = format_roll(roll2, sides);

        let roll1_is_winner = if advantage { roll1 >= roll2 } else { roll1 <= roll2 };

        let result_str = if roll1_is_winner {
            format!("{} \x1B[9m{}\x1B[0m", formatted_roll1, formatted_roll2)
        } else {
            format!("\x1B[9m{}\x1B[0m {}", formatted_roll1, formatted_roll2)
        };
        rolls.push(result_str);

    } else {
        for _ in 0..count {
            let roll = rand::thread_rng().gen_range(1..=sides);
            rolls.push(format_roll(roll, sides));
        }
    }

    Ok(rolls.join(", "))
}