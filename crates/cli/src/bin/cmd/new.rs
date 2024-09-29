use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use convert_case::Casing;
use include_dir::Dir;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEMPLATES: Vec<&'static str> = vec!["basic", "ramhorns", "vercel"];
    static ref TEMPLATES_DIR: Vec<Dir<'static>> = vec![
        include_dir::include_dir!("./starter-templates/with-basic"),
        include_dir::include_dir!("./starter-templates/with-ramhorns"),
        include_dir::include_dir!("./starter-templates/with-vercel")
    ];
}

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

pub fn run(_matches: &ArgMatches, subcommand_matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    if let Some(name) = subcommand_matches.get_one::<String>("name") {
        println!("Creating new project: {}", name);
    } else {
        let name = dialoguer::Input::<String>::new()
            .with_prompt("Name of the project to create")
            .interact()?;
        let name = name.to_case(convert_case::Case::Snake);

        let template = dialoguer::Select::new()
            .with_prompt("Use a template to create the project")
            .items(&TEMPLATES)
            .default(0)
            .interact()?;

        let cwd = std::env::current_dir()?;
        let project_dir = cwd.join(name);

        let force = if project_dir.exists() {
            dialoguer::Confirm::new()
                .with_prompt("Project directory already exists. Overwrite?")
                .interact()?
        } else {
            true
        };

        if !force {
            return Ok(cargo_ngyn::CmdExit {
                code: exitcode::OK,
                message: Some("Project directory already exists. Exiting...".to_string()),
            });
        }

        // safely remove current project directory and create a new one
        if project_dir.exists() {
            std::fs::remove_dir_all(&project_dir)?;
        } else {
            std::fs::create_dir_all(&project_dir)?;
        }

        let template_dir = &TEMPLATES_DIR[template];
        let template_files = template_dir.files();

        for file in template_files {
            let file_path = project_dir.join(file.path());
            let file_content = file.contents_utf8().unwrap();

            std::fs::create_dir_all(file_path.parent().unwrap())?;
            std::fs::write(file_path, file_content)?;
        }
    }

    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
