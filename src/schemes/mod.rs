pub mod collections;
pub mod template;

use crate::schemes::{collections::Collections, template::Template};
use crate::{Error, Result};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;

static MAX_COLLECTIONS_DEPTH: u8 = 5;

#[derive(Debug, Default, Clone)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collections(Collections),

    // If repo doesn't have any configuration
    #[default]
    Empty,
}

impl Configuration {
    pub fn parse(path: PathBuf) -> Self {
        let config = Some(path.clone())
            .filter(|p| p.exists())
            .map(|p| p.join("bleur.toml"))
            .filter(|p| p.exists())
            .and_then(|p| fs::read_to_string(p).ok());

        // If there's string inside config file
        if let Some(text) = config {
            // And if it's parsible to Template type
            if let Ok(t) = toml::from_str::<Template>(&text) {
                return Configuration::Template(t.with_path(path));
            }

            // And if it's parsible to Collection type
            if let Ok(c) = toml::from_str::<Collections>(&text) {
                return Configuration::Collections(c);
            }
        };

        // Nothing's there + invalid config file
        Self::Empty
    }

    pub fn surely_template(path: PathBuf, depth: u8) -> Result<Self> {
        use Configuration::*;
        depth
            .gt(&MAX_COLLECTIONS_DEPTH)
            .not()
            .then_some(())
            .ok_or(Error::AintNoWayThisDeepCollection(MAX_COLLECTIONS_DEPTH))
            .map(|_| path.clone())
            .map(Self::parse)
            .and_then(|c| match c {
                Template(t) => Ok(Self::Template(t)),
                Empty => Err(Error::NoTemplateConfiguration),
                Collections(c) => inquire::Select::new(
                    "Choose the template you would like to bootstrap:",
                    c.keys(),
                )
                .prompt()
                .map_err(Error::CantParseUserPrompt)
                .and_then(|s| c.select(s).ok_or(Error::NoSuchTemplateInCollection))
                .and_then(|c| Self::surely_template(c.path(path), depth + 1)),
            })
    }

    pub fn template(self) -> Result<Template> {
        match self {
            Configuration::Template(template) => Ok(template),
            Configuration::Empty => Err(Error::TemplateIsInvalid),
            Configuration::Collections(_) => Err(Error::TemplateIsInvalid),
        }
    }
}
