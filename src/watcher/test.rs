use super::{
    Param,
    parse_args
};

#[test]
fn test_parse_args() {
    // It should correctly tokenize the arguments
    (|| {
        let args = vec!["-w", "**/*.txt",
                        "-x", "/bin/true",
                        "-w", "**/*.css",
                        "-x", "minify css",
                        "-h"]
            .into_iter()
            .map(|item| item.to_string());

        let expected = vec![Param::Glob("**/*.txt".to_string()),
                            Param::Command("/bin/true".to_string()),
                            Param::Glob("**/*.css".to_string()),
                            Param::Command("minify css".to_string()),
                            Param::Help];

        assert_eq!(parse_args(args), expected);
    })();

    // Either -h or --help should resolve to Help
    (|| {
        let args = vec![String::from("-h")].into_iter();
        let expected = vec![Param::Help];

        assert_eq!(parse_args(args), expected);

        let args = vec![String::from("--help")].into_iter();

        assert_eq!(parse_args(args), expected);
    })();

    // It should ignore other arguments after reading Help
    (||{
        let args = vec!["-w", "**/*.txt",
                        "--help",
                        "-x", "/bin/true",
                        "-w", "**/*.css",
                        "-x", "minify css"]
            .into_iter()
            .map(|item| item.to_string());

        let expected = vec![Param::Glob("**/*.txt".to_string()),
                            Param::Help];

        assert_eq!(parse_args(args), expected);
    })();
}
