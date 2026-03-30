enum Instructions {
    /// Make whole value uppercase
    Uppercase,

    /// Make whole value lowercase
    Lowercase,

    /// Replace chars in string
    ReplaceAll(String, String),

    /// If the function name is not recognized
    Unknown,
}

impl<T: AsRef<str>> From<T> for Instructions {
    fn from(value: T) -> Self {
        match value.as_ref() {
            s if s.eq_ignore_ascii_case("uppercase") => Self::Uppercase,
            s if s.eq_ignore_ascii_case("lowercase") => Self::Lowercase,
            value => {
                if let Some((name, args)) = value.split_once(':') {
                    if name.eq_ignore_ascii_case("replaceAll") {
                        if let Some((from, to)) = args.split_once("->") {
                            return Self::ReplaceAll(from.into(), to.into());
                        }
                    }
                }

                Self::Unknown
            }
        }
    }
}

pub struct Apply(Vec<Instructions>);

impl Apply {
    pub fn parse<T: ToString>(input: T) -> Apply {
        Apply(
            input
                .to_string()
                .split(",")
                .map(Instructions::from)
                .collect::<Vec<Instructions>>(),
        )
    }

    pub fn execute<T: ToString>(&self, input: T) -> String {
        self.0.iter().fold(
            input.to_string(),
            |current, instruction| match instruction {
                Instructions::Uppercase => current.to_uppercase(),
                Instructions::Lowercase => current.to_lowercase(),
                Instructions::ReplaceAll(from, to) => current.replace(from, to),
                Instructions::Unknown => current,
            },
        )
    }
}
