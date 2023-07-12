use std::collections::HashMap;
use anyhow::Result;
use clap::{builder::ValueParser, Parser, Subcommand};

/// A struct representing the CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Subcommand of the CLI
    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the HTTP server
    Server,
    /// Start the console interface 
    Console {
        /// Command to execute
        console_command: String,
        /// Arguments of the command
        #[arg(num_args(0..), value_parser = ValueParser::new(parse_subcommand_args))]
        args: HashMap<String, Option<String>>
    },
}

/// This function will parse the arg string into a map formatted as KEY => Option(VALUE).
///
/// The format of the args must be :
/// - `key=val` for key-value pairs
/// - `flag` for flags only
///
/// All separated by `;`
fn parse_subcommand_args(arg_str: &str) -> Result<HashMap<String, Option<String>>> {
    let mut args = HashMap::<String, Option<String>>::new();

    let arg_packs = arg_str.split(';').collect::<Vec<&str>>();

    for arg_pack in arg_packs.iter() {
        if arg_pack.contains('=') {
            let arg_body = arg_pack.split('=').collect::<Vec<&str>>();

            let arg_name = arg_body.first().unwrap().to_string();
            let arg_value = arg_body.get(1).unwrap().to_string();

            args.insert(arg_name, Some(arg_value));
        } else {
            args.insert(arg_pack.to_string(), None);
        }
    }

    Ok(args)
}