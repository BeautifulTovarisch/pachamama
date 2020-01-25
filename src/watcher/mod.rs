#[derive(Debug)]
pub enum Param {
    Help,
    Glob(String),
    Command(String)
}

impl PartialEq for Param {
    fn eq(&self, other: &Param) -> bool {
        match (&self, other) {
            (Param::Help, Param::Help) => true,
            (Param::Glob(ref str1), Param::Glob(ref str2)) => *str1 == *str2,
            (Param::Command(ref str1), Param::Command(ref str2)) => *str1 == *str2,
            (_, _) => false
        }
    }
}

// Allow any string iterable types. Generic for testing purposes
// std::env::Args satifies this type
pub fn parse_args<I>(args: I) -> Vec<Param>
    where I: Iterator<Item = String>
{
    let mut arguments = args.peekable();

    let mut parsed = Vec::new();

    // While the iterator has elements remaining
    while let Some(arg) = arguments.next() {

        // As soon as we see the user ask for help we leave
        if arg.as_str() == "-h" || arg.as_str() == "--help" {
            parsed.push(Param::Help);
            return parsed;
        }

        // If the next element exists
        if let Some(val) = arguments.next() {
            match arg.as_str() {
                "-w" => parsed.push(Param::Glob(val)),
                "-x" => parsed.push(Param::Command(val)),
                _ => ()
            }
        }
    }
    parsed
}

#[cfg(test)]
mod test;
