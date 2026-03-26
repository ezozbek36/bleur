#![allow(unused_variables)]

use bleur::*;
use clap::Parser;
use std::{env::current_dir, fs::File, io::Write};

fn main() -> Result<()> {
    run().or_else(|e| beautiful_exit(e.to_string()))
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Commands::New {
            template,
            path,
            method,
        } => path
            .map_or_else(|| current_dir().map_err(Error::IOError), Ok)
            .and_then(|p| {
                manager::ManageBuilder::new()
                    .source(template)
                    .and_then(|b| b.tempdir())
                    .and_then(|b| b.fetch_method(method))
                    .and_then(|b| b.build())
                    .and_then(|m| m.instantiate())
                    .and_then(|m| m.parse())
                    .and_then(|m| m.evaluate())
                    .and_then(|m| m.recursively_copy(p))
            })
            .map(|_| ()),
        Commands::Init => current_dir()
            .map_err(Error::IOError)
            .and_then(|directory| {
                File::create(directory.join("bleur.toml")).map_err(Error::IOError)
            })
            .and_then(|mut file| {
                match inquire::Select::new(
                    "Are you creating a single project template or a collection?",
                    vec!["template", "collection"],
                )
                .prompt()
                .map_err(Error::CantParseUserPrompt)
                .map(|choice| match choice {
                    "collection" => COLLECTION,
                    _ => TEMPLATE,
                }) {
                    Ok(content) => file.write_all(content.as_bytes()).map_err(Error::IOError),
                    Err(e) => Err(e),
                }
            }),
    }
}
