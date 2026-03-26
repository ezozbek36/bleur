use crate::{
    execute::task::{Task, ToTask},
    Error, Result,
};
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
}

impl Variable {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        inquire::Text::new(&self.message)
            .with_default(&self.default)
            .with_placeholder(&self.default)
            .prompt()
            .map_err(Error::CantParseUserPrompt)
            .and_then(|s| {
                global
                    .insert(self.variable.clone(), s)
                    .map(|_| ())
                    .ok_or(Error::CouldInsertToMap)
            })
    }
}

impl ToTask for Variable {
    fn to_task(self, _: &Path) -> Task {
        Task::Variable(Variable {
            message: self.message,
            default: self.default,
            variable: self.variable,
        })
    }
}
