use super::{
    Param,
    parse_args
};

#[test]
fn test_parse_args() {
    let args = vec!["-w",
                    "**/*.txt",
                    "-x /bin/true",
                    "-w",
                    "**/*.css",
                    "-x", "minify css",
                    "-h",
                    "--help"]
        .into_iter()
        .map(|item| item.to_string());

    let expected = vec![Param::Glob("**/*.txt".to_string()),
                        Param::Command("/bin/true".to_string()),
                        Param::Glob("**/*.css".to_string()),
                        Param::Command("minify css".to_string()),
                        Param::Help,
                        Param::Help];

    assert_eq!(parse_args(args), expected);
}
