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
                .help("Name of the project to create"),
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

pub fn run(matches: &ArgMatches, subcommand_matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    if let Some(name) = subcommand_matches.get_one::<String>("name") {
        println!("Creating new project: {}", name);
    } else {
        let name = dialoguer::Input::<String>::new()
            .with_prompt("Name of the project to create")
            .interact()?;
        let force = dialoguer::Confirm::new()
            .with_prompt("Force the creation of the project")
            .default(false)
            .interact()?;
        let template = dialoguer::Select::new()
        .with_prompt("Use a template to create the project")
        .default(0)
        .interact()?;
    }

    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
