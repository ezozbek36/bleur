use crate::{
    execute::task::{Task, ToTask},
    Error, Result,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Variable {
    /// Variable to make use of in names of files while moving
    variable: String,

    /// Default value to be picked up
    default: String,

    /// Question to ask from user to get value
    message: String,

    /// Regex for validating user input
    pattern: Option<String>,

    /// Input validation feedback for user
    pattern_error: Option<String>,
}

impl Variable {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        let mut inquire_prompt = inquire::Text::new(&self.message)
            .with_default(&self.default)
            .with_placeholder(&self.default);

        if let Some(pattern) = self.pattern.as_ref() {
            let pattern = Regex::new(pattern).expect("Failed to parse pattern");
            let error_message = self.pattern_error.as_ref();

            inquire_prompt = inquire_prompt.with_validator(move |input: &str| {
                if pattern.is_match(input) {
                    Ok(inquire::validator::Validation::Valid)
                } else {
                    Ok(inquire::validator::Validation::Invalid(
                        error_message
                            .map(Into::into)
                            .unwrap_or(inquire::validator::ErrorMessage::Default),
                    ))
                }
            });
        }

        inquire_prompt
            .prompt()
            .map_err(Error::CantParseUserPrompt)
            .and_then(|s| {
                global.insert(self.variable.clone(), s);
                Ok(())
            })
    }
}

impl ToTask for Variable {
    fn to_task(self, _: &Path) -> Task {
        Task::Variable(Variable {
            message: self.message,
            default: self.default,
            variable: self.variable,
            pattern: self.pattern,
            pattern_error: self.pattern_error,
        })
    }
}
