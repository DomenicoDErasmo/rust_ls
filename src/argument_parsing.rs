use core::fmt;
use std::env::args;

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
/// TODO: can I remove this allow and refactor?
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Arguments {
    /// The path to search.
    pub path: String,
    /// Do not ignore entries starting with .
    ///
    /// ### Example
    ///
    /// <ins> Without -a </ins>
    /// ```shell
    /// $ ls
    /// foo.txt
    /// ```
    /// <ins> With -a </ins>
    /// ```shell
    /// $ ls -a
    /// . .. .hidden foo.txt
    /// ```
    pub all: bool,
    /// Do not list implied . and ..
    ///
    /// ### Example
    ///
    /// <ins> Without -A </ins>
    /// ```shell
    /// $ ls
    /// foo.txt
    /// ```
    /// <ins> With -A </ins>
    /// ```shell
    /// $ ls -A
    /// .hidden foo.txt
    /// ```
    pub almost_all: bool,
    /// Do not list implied entries ending with ~
    ///
    /// ### Example
    ///
    /// <ins> Without -B </ins>
    /// ```shell
    /// $ ls
    /// foo.txt foo.text~
    /// ```
    /// <ins> With -B </ins>
    /// ```shell
    /// $ ls -B
    /// foo.txt
    /// ```
    pub ignore_backups: bool,
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
        // Handling the path arg
        let mut path_found = false;
        let mut path = String::new();

        // Handling flags
        let mut all = false;
        let mut almost_all = false;
        let mut ignore_backups = false;

        for arg in raw_args {
            match arg.as_str() {
                "-a" | "--all" => {
                    all = true;
                }
                "-A" | "--almost-all" => {
                    almost_all = true;
                }
                "-B" | "--ignore-backups" => {
                    ignore_backups = true;
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

        Ok(Self {
            path,
            all,
            almost_all,
            ignore_backups,
        })
    }
}

/// Sets up args from command line.
///
/// # Errors
/// * `ArgReadingError` if one or more values can't be parsed.
#[inline]
pub fn setup_args() -> Result<Arguments, ArgReadingError> {
    let all_raw_args = args().collect::<Vec<String>>();
    let raw_arg_array = all_raw_args.get(1..);
    raw_arg_array.map_or_else(
        || Arguments::new(&vec![]),
        |arg_array| Arguments::new(&arg_array.to_vec()),
    )
}
