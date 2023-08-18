use clap::{crate_authors, crate_description, crate_name, crate_version, value_parser, Subcommand};
use clap::{Arg, Command};

fn command() -> clap::Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("model")
                .help("path to zip input file")
                .long("model")
                .short('m')
                .num_args(1)
                .required(false),
        )
}

pub struct Arguments {
    pub input_file: String,
    pub workers: Option<usize>,
    pub charsets: Vec<char>,
    pub min_password_len: usize,
    pub max_password_len: usize,
    pub password_dictionary: Option<String>,
    pub custom_chars: Vec<char>,
}
