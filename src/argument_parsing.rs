use core::fmt;

// TODO: Write a generic argument parsing library

/// Struct for error indicating that the argument is not recognized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentNotFoundError {
    /// The argument that was not successfully retrieved.
    pub argument: String,
}

/// An enumeration of errors that occur when reading arguments from the command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgReadingError {
    /// The argument is not recognized.
    ArgumentNotFoundError(ArgumentNotFoundError),
}

impl fmt::Display for ArgReadingError {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::ArgumentNotFoundError(ArgumentNotFoundError { argument }) => {
                write!(
                    formatter,
                    "Failed to parse arguments. Argument {argument:#?} was not found."
                )
            }
        }
    }
}

/// LS arguments
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Arguments {
    /// The path to search.
    pub path: String,
    /// Do not ignore entries starting with .
    pub all: bool,
}

impl Arguments {
    /// Parses arguments from the command line.
    ///
    /// ### Parameters
    /// * `raw_args`: The unparsed arguments to parse.
    ///
    /// ### Returns
    /// * A set of arguments with typing information.
    ///
    /// ### Errors
    /// If an argument is not recognized, returns an `ArgumentNotFoundError`.
    #[inline]
    pub fn new(raw_args: &Vec<String>) -> Result<Self, ArgReadingError> {
        let mut path_found = false;

        let mut path = String::new();
        let mut all = false;

        for arg in raw_args {
            match arg.as_str() {
                "-a" => {
                    all = true;
                }
                not_parsed_as_flag => {
                    if path_found {
                        return Err(ArgReadingError::ArgumentNotFoundError(
                            ArgumentNotFoundError {
                                argument: not_parsed_as_flag.to_owned(),
                            },
                        ));
                    }
                    not_parsed_as_flag.clone_into(&mut path);
                    path_found = true;
                }
            };
        }

        if path.is_empty() {
            "./".clone_into(&mut path);
        }

        Ok(Self { path, all })
    }
}
