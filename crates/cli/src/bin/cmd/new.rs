use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

/// `ngyn new` command
///
/// Used to create a new project.
pub fn command() -> Command {
    Command::new("new")
        .alias("n")
        .about("Create a new project")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the project to create")
                .required(true),
        )
        .arg(
            Arg::new("dry_run")
                .short('d')
                .long("dry-run")
                .value_name("DRY_RUN")
                .help("Print out the generated files without writing them to disk"),
        )
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE")
                .help("Use a template to create the project"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .value_name("FORCE")
                .help("Force the creation of the project"),
        )
}

pub fn run(_matches: &ArgMatches, _subcommand_matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
