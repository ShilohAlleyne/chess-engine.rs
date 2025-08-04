use colored::*;
use std::fmt::{self, Debug, Display};

pub enum Error {
    Deserialization {input: String, invalid_char: char, pos: usize},
    Serialization(String)
}

impl std::error::Error for Error {}


// === Debug overriding ===
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

// === Display Error messages ====
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deserialization{input, invalid_char, pos} => {
                let (left, right): (String, String) = extract_regions(&input, *pos);

                // Calculate offset based on the left-hand region only
                let offset: usize = "\t".len()                                 // Tab indent
                    + (pos.saturating_sub(left.len())).to_string().len()  // Starting index width
                    + "-->".len()                                              // Arrow
                    + "...".len()                                              // Ellipsis
                    + left.len() -1;                                           // Characters on the left

                let error_string = format!(
                    "\t{}{}...{}{}{}...{}{}",
                    (pos.saturating_sub(left.len())).to_string().blue(),
                    "-->".blue(),
                    left,
                    invalid_char.to_string().red(),
                    right,
                    "<--".blue(),
                    (pos + right.len()).to_string().blue()
                );

                writeln!(f, "{}:\n", "Error parsing FEN string".red())?;
                writeln!(f, "{}", error_string)?;
                writeln!(f, "\t{}{}", " ".repeat(offset), "^".red())?;
                writeln!(
                    f,
                    "\tInvalid character ({}) at position {}",
                    invalid_char,
                    pos
                )?;

                Ok(())
            },
            Self::Serialization(msg) => writeln!(f, "{}", msg),
        }
    }
}

fn extract_regions(s: &str, pos: usize) -> (String, String) {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let left_start = pos.saturating_sub(5);
    let left_end = pos.min(len);

    let right_start = pos.saturating_add(1);
    let right_end = (pos + 6).min(len); // up to 5 chars after pos

    let left: String = if left_start < left_end {
        chars[left_start..left_end].iter().collect()
    } else {
        String::new()
    };

    let right: String = if right_start < right_end {
        chars[right_start..right_end].iter().collect()
    } else {
        String::new()
    };

    (left, right)
}
