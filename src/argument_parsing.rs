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
                write!(formatter, "Failed to parse arguments. Argument {argument:#?} was not found.")
            }
        }
    }
}

/// LS arguments
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Arguments {
    /// Enables long listing format, such as author, time created, and permissions.
    pub long_listing: bool,

    /// Enables reverse sorting.
    pub reverse_sort: bool,

    /// Sorts output by time.
    pub time_sort: bool,
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
        let mut long_listing = false;
        let mut reverse_sort = false;
        let mut time_sort = false;

        for arg in raw_args {
            match arg.as_str() {
                "-l" => {
                    long_listing = true;
                }
                "-r" => {
                    reverse_sort = true;
                }
                "-t" => {
                    time_sort = true;
                }
                not_found => {
                    return Err(ArgReadingError::ArgumentNotFoundError(
                        ArgumentNotFoundError {
                            argument: not_found.to_owned(),
                        },
                    ));
                }
            };
        }

        Ok(Self {
            long_listing,
            reverse_sort,
            time_sort,
        })
    }
}
