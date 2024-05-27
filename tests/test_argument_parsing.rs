#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test_argument_parsing {
    use rust_ls::argument_parsing::{
        ArgReadingError, ArgumentNotFoundError, Arguments,
    };

    #[allow(clippy::needless_update)]
    #[test]
    fn test_new() {
        let no_args: Vec<String> = vec![];
        let no_arg_parser = Arguments::new(&no_args).unwrap_or_default();
        assert_eq!(no_arg_parser, Arguments::default());

        let all_only_arg: Vec<String> = vec!["-a".to_owned()];
        let long_listing_only_arg_parser =
            Arguments::new(&all_only_arg).unwrap();
        assert_eq!(
            long_listing_only_arg_parser,
            Arguments {
                all: true,
                ..Default::default()
            }
        );

        let has_invalid_args: Vec<String> =
            vec!["--unused".to_owned(), "-l".to_owned()];
        let has_invalid_args_parser = Arguments::new(&has_invalid_args);
        assert_eq!(
            has_invalid_args_parser,
            Err(ArgReadingError::ArgumentNotFoundError(
                ArgumentNotFoundError {
                    argument: "--unused".to_owned()
                }
            ))
        );
    }
}
