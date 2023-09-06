use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn command() -> Command<'static> {
    Command::new("validate")
        .about("Validate keys")
        .arg(
            Arg::new("list")
                .long("list")
                .help("Show provider list")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("csv_in")
                .long("--csv-in")
                .value_name("FILE")
                .help("Read providers and params via CSV"),
        )
}

pub fn run(_matches: &ArgMatches, _subcommand_matches: &ArgMatches) -> Result<ngyn_cli::CmdExit> {
    Ok(ngyn_cli::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
