use anyhow::Result;
use clap::{crate_version, Arg, ArgAction};
use clap::{ArgMatches, Command};
use tracing::info;

pub fn command() -> Command {
    Command::new("ngyn ")
        .version(crate_version!())
        .about("A powerful and flexible web application framework for Rust.")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Show details about interactions")
                .action(ArgAction::SetTrue),
        )
        .after_help("For more information on a specific command, run 'ngyn help [SUBCOMMAND]' or 'ngyn [SUBCOMMAND] --help'.\n\nDocumentation: https://ngyn.rs/docs\nReport bugs: https://github.com/ngyn-rs/ngyn/issues")
}

pub fn run(cmd: &mut Command, matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    info!("default cmd {:?}", matches.get_one::<String>("reporter"));
    let version = crate_version!();

    println!("ngyn {}", version);

    cmd.print_long_help().unwrap();

    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
