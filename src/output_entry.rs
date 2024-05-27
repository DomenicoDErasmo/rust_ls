use crate::color::Color;
use core::fmt;
use core::fmt::Display;
use std::path::PathBuf;

/// A string that supports color printing out of the box.
#[derive(Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputEntry {
    /// The `PathBuf`
    pub file: PathBuf,
    /// Whether this represents the current directory.
    pub is_current_directory: bool,
    /// Whether this represents the parent directory.
    pub is_parent_directory: bool,
}

impl OutputEntry {
    #[inline]
    #[must_use]
    pub fn displayed_name(&self) -> String {
        if self.is_parent_directory {
            return "..".to_owned();
        };

        if self.is_current_directory {
            return ".".to_owned();
        }

        self.file
            .components()
            .last()
            .map_or_else(String::new, |filename| {
                filename.as_os_str().to_str().unwrap_or_default().to_owned()
            })
    }

    #[inline]
    #[must_use]
    pub fn color(&self) -> Color {
        if self.file.clone().is_dir() {
            Color::blue()
        } else {
            Color::white()
        }
    }

    #[inline]
    #[must_use]
    pub fn bold(&self) -> bool {
        self.file.clone().is_dir()
    }
}

impl Display for OutputEntry {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_string = format!(
            "\x1B[38;2;{};{};{}m{}\x1B[0m",
            self.color().rgb[0],
            self.color().rgb[1],
            self.color().rgb[2],
            self.displayed_name()
        );
        let bold_colored_string = format!(
            "{}{}{}",
            if self.bold() { "\x1b[1m" } else { "" },
            colored_string,
            if self.bold() { "\x1b[22m" } else { "" },
        );
        write!(formatter, "{bold_colored_string}")
    }
}
