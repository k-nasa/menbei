mod dialogure;
mod template;
mod utils;

use self::utils::*;
use crate::issue_link::IssueLink;
use clap::{crate_description, crate_name, crate_version, Arg, SubCommand};
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

pub fn run() -> Result<(), CliError> {
    let matches = build_app().get_matches();

    if let Some(filename) = matches.value_of("file") {
        return from_file(&filename);
    }

    match matches.subcommand() {
        ("dialogure", _) => dialogure::exec()?,
        ("template", _) => template::exec(),
        _ => build_app().print_help()?,
    }

    Ok(())
}

fn from_file(file_name: &str) -> Result<(), CliError> {
    let file_content = read_to_string(file_name)?;

    let extension = Path::new(file_name).extension();

    let issue_link: IssueLink = match extension.and_then(OsStr::to_str) {
        Some("toml") => toml::from_str(&file_content)?,
        Some("yaml") => serde_yaml::from_str(&file_content)?,
        _ => return Err(CliError::UnsupportedExtension),
    };

    issue_link
        .print_link()
        .unwrap_or_else(|e| eprintln!("{}", e));

    Ok(())
}

fn build_app() -> App {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("issue link definition file"))
        .subcommand(SubCommand::with_name("dialogure").about("Create an issue link interactively"))
    // .subcommand(
    //     SubCommand::with_name("template")
    //         .about("Output a template file for creating an issue link")
    //         .arg(
    //             Arg::with_name("name")
    //                 .help("File name to output")
    //                 .required(true),
    //         ),
    // )
}
