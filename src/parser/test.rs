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
                        "-x", "minify css"]
            .into_iter()
            .map(|item| item.to_string());

        let expected = vec![Param::Glob("**/*.txt".to_string()),
                            Param::Command("/bin/true".to_string()),
                            Param::Glob("**/*.css".to_string()),
                            Param::Command("minify css".to_string())];

        assert_eq!(parse_args(args), expected);
    })();

    // It should identify invalid arguments
    (|| {
        let args = vec!["-w",
                        "**/*.txt",
                        "-f", "-c",
                        "-x", "/bin/true"]
            .into_iter()
            .map(|item| item.to_string());

        let expected = vec![Param::Glob(String::from("**/*.txt")),
                            Param::Invalid(String::from("-f")),
                            Param::Invalid(String::from("-c")),
                            Param::Command(String::from("/bin/true"))];

        assert_eq!(parse_args(args), expected);
    })();
}
