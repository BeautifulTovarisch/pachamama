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

    // if let Some(val) = arguments.peek() {
    //     panic!("{}", val);
    // }

    return vec![Param::Glob("test".to_string())]
}

#[cfg(test)]
mod test;
