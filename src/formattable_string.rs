use crate::color::Color;
use core::fmt;
use core::fmt::Display;

/// A string that supports color printing out of the box.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct FormattableString {
    /// The text to output
    pub input: String,
    /// The RGB color to output
    pub color: Color,
    /// Whether to output text in bold
    pub bold: bool,
}

impl Display for FormattableString {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_string = format!(
            "\x1B[38;2;{};{};{}m{}\x1B[0m",
            self.color.rgb[0], self.color.rgb[1], self.color.rgb[2], self.input
        );
        let bold_colored_string = format!(
            "{}{}{}",
            if self.bold { "\x1b[1m" } else { "" },
            colored_string,
            if self.bold { "\x1b[22m" } else { "" },
        );
        write!(formatter, "{bold_colored_string}")
    }
}
