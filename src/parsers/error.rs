use colored::*;
use std::fmt::{self, Debug, Display};

pub enum ParserError {
    InvalidCharacter(InvalidChar),
}

impl std::error::Error for ParserError {}

pub struct InvalidChar {
    pub input: String,
    pub invalid_char: char,
    pub pos: usize,
}

impl std::error::Error for InvalidChar {}

// === Debug overriding ===
impl Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Debug for InvalidChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

// === Display Error messages ====
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(i) => write!(f, "{}", i),
        }
    }
}

impl Display for InvalidChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right): (String, String) = extract_regions(&self.input, self.pos);

        // Calculate offset based on the left-hand region only
        let offset: usize = "\t".len()                                 // Tab indent
            + (self.pos.saturating_sub(left.len())).to_string().len()  // Starting index width
            + "-->".len()                                              // Arrow
            + "...".len()                                              // Ellipsis
            + left.len() -1;                                           // Characters on the left

        let error_string = format!(
            "\t{}{}...{}{}{}...{}{}",
            (self.pos.saturating_sub(left.len())).to_string().blue(),
            "-->".blue(),
            left,
            self.invalid_char.to_string().red(),
            right,
            "<--".blue(),
            (self.pos + right.len()).to_string().blue()
        );

        writeln!(f, "{}:\n", "Error parsing FEN string".red())?;
        writeln!(f, "{}", error_string)?;
        writeln!(f, "\t{}{}", " ".repeat(offset), "^".red())?;
        writeln!(
            f,
            "\tInvalid character ({}) at position {}",
            self.invalid_char,
            self.pos
        )?;

        Ok(())
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
