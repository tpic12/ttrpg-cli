use rand::Rng;
use colored::*;

#[derive(Debug, PartialEq)]
pub enum DiceError {
    InvalidSides,
    InvalidCount,
    InvalidFormat,
}

pub fn parse_roll(roll_str: &str) -> Result<(u32, u32), DiceError> {
    let parts: Vec<&str> = roll_str.split('d').collect();
    match parts.as_slice() {
        [count_str, sides_str] => {
            let count = if count_str.is_empty() {
                1
            } else {
                count_str.parse().map_err(|_| DiceError::InvalidFormat)?
            };
            let sides = sides_str.parse().map_err(|_| DiceError::InvalidFormat)?;
            Ok((count, sides))
        }
        [sides_str] => {
            let sides = sides_str.parse().map_err(|_| DiceError::InvalidFormat)?;
            Ok((1, sides))
        }
        _ => Err(DiceError::InvalidFormat),
    }
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
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        if advantage || disadvantage {
            let roll1 = rng.gen_range(1..=sides);
            let roll2 = rng.gen_range(1..=sides);

            let formatted_roll1 = format_roll(roll1, sides);
            let formatted_roll2 = format_roll(roll2, sides);

            let roll1_is_winner = if advantage { roll1 >= roll2 } else { roll1 <= roll2 };

            let result_str = if roll1_is_winner {
                format!("({} \x1B[9m{}\x1B[0m)", formatted_roll1, formatted_roll2)
            } else {
                format!("(\x1B[9m{}\x1B[0m {})", formatted_roll1, formatted_roll2)
            };
            rolls.push(result_str);
        } else {
            let roll = rng.gen_range(1..=sides);
            rolls.push(format_roll(roll, sides));
        }
    }

    Ok(rolls.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\x1B' {
                // Skip until 'm'
                for char in chars.by_ref() {
                    if char == 'm' {
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    #[test]
    fn test_parse_roll() {
        assert_eq!(parse_roll("2d20"), Ok((2, 20)));
        assert_eq!(parse_roll("d20"), Ok((1, 20)));
        assert_eq!(parse_roll("20"), Ok((1, 20)));
        assert_eq!(parse_roll("1d6"), Ok((1, 6)));
    }

    #[test]
    fn test_parse_roll_invalid() {
        assert_eq!(parse_roll("2d"), Err(DiceError::InvalidFormat));
        assert_eq!(parse_roll("d"), Err(DiceError::InvalidFormat));
        assert_eq!(parse_roll("abc"), Err(DiceError::InvalidFormat));
        assert_eq!(parse_roll("2d2d2"), Err(DiceError::InvalidFormat));
    }

    #[test]
    fn test_roll_dice_errors() {
        assert!(matches!(roll_dice(1, 1, false, false), Err(DiceError::InvalidSides)));
        assert!(matches!(roll_dice(6, 0, false, false), Err(DiceError::InvalidCount)));
    }

    #[test]
    fn test_roll_dice_single() {
        let result = roll_dice(6, 1, false, false).unwrap();
        let stripped = strip_ansi(&result);
        let roll: u32 = stripped.parse().unwrap();
        assert!(roll >= 1 && roll <= 6);
    }

    #[test]
    fn test_roll_dice_multiple() {
        let result = roll_dice(6, 3, false, false).unwrap();
        let rolls: Vec<u32> = result.split(", ").map(|s| strip_ansi(s).trim().parse().unwrap()).collect();
        assert_eq!(rolls.len(), 3);
        for roll in rolls {
            assert!(roll >= 1 && roll <= 6);
        }
    }
    
    #[test]
    fn test_roll_dice_advantage() {
        let result = roll_dice(20, 1, true, false).unwrap();
        assert!(result.starts_with('(') && result.ends_with(')'));
        assert!(result.contains("\x1B[9m"));
    }

    #[test]
    fn test_roll_dice_disadvantage() {
        let result = roll_dice(20, 1, false, true).unwrap();
        assert!(result.starts_with('(') && result.ends_with(')'));
        assert!(result.contains("\x1B[9m"));
    }

    #[test]
    fn test_roll_dice_multiple_advantage() {
        let result = roll_dice(20, 2, true, false).unwrap();
        let rolls: Vec<&str> = result.split(", ").collect();
        assert_eq!(rolls.len(), 2);
        for roll in rolls {
            assert!(roll.starts_with('(') && roll.ends_with(')'));
            assert!(roll.contains("\x1B[9m"));
        }
    }
}