use std::process::{ exit };

/// Enumeration of potential parameters accepted from the command line.
#[derive(Debug)]
pub enum Param {
    Glob(String),
    Invalid(String),
    Command(String)
}

impl PartialEq for Param {
    fn eq(&self, other: &Param) -> bool {
        match (&self, other) {
            (Param::Glob(ref str1), Param::Glob(ref str2)) => *str1 == *str2,
            (Param::Invalid(ref str1), Param::Invalid(ref str2)) => *str1 == *str2,
            (Param::Command(ref str1), Param::Command(ref str2)) => *str1 == *str2,
            (_, _) => false
        }
    }
}

/// Returns a formatted usage string for
/// when -h, --help, or invalid options are found
fn help() -> &'static str {
"Watch This Run That (wtrt)
Watches globs for changes, runs supplied command(s).

USAGE:
    wtrt [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Display this help message.
    -w <watch>       File glob to watch for changes.

OPTIONS:
    -x <execute>     Command to execute on file change. May supply more than one command per glob.

EXAMPLES:
    Watch text files and output a simple message on changes:
        wtrt -w '**/*.txt' -x 'echo hello!'

    Watch project directory and run tests:
        wtrt -w src/ -x 'npm test'

    Watch multiple globs, run multiple commands per glob:
        wtrt -w **/*.go
             -x 'go test project/...' -x 'go run project'
             -w **/*.py -x 'python -m unittest'"
}

/// Evaluates strings representing command line arguments.
/// Function accepts an Iterator of type String in order to mimic
/// [std::env::Args](https://doc.rust-lang.org/std/env/struct.Args.html#impl-Iterator)
/// Returns a vector of Param enums.
pub fn parse_args<I>(args: I) -> Vec<Param>
    where I: Iterator<Item = String>
{
    let mut arguments = args.peekable();

    let mut parsed = Vec::new();

    while let Some(arg) = arguments.next() {

        // As soon as we see the user ask for help it's time to leave
        if arg.as_str() == "-h" || arg.as_str() == "--help" {
            println!("{}", help());
            exit(0)
        }

        // Peek ahead required in order to encapsulate the value of the cli arg.
        // Advances the iterator when value is captured.
        if let Some(val) = arguments.peek() {
            match arg.as_str() {
                "-w" => {
                    parsed.push(Param::Glob(String::from(val)));
                    arguments.next();
                },
                "-x" => {
                    parsed.push(Param::Command(String::from(val)));
                    arguments.next();
                },
                _ => parsed.push(Param::Invalid(arg))
            }
        }
    }
    parsed
}

#[cfg(test)]
mod test;
