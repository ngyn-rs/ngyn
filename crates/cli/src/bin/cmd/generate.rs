use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use tracing::info;

/// `ngyn generate` command
///
/// Allows the generation of schematics and other artifacts such as controllers, services, gates, middlewares, and modules.
pub fn command() -> Command {
    Command::new("generate")
        .alias("g")
        .about("Generate schematics and other artifacts")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the artifact to generate")
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
            Arg::new("controller")
                .short('c')
                .long("controller")
                .value_name("CONTROLLER")
                .help("Generate a controller to handle requests (default)"),
        )
        .arg(
            Arg::new("service")
                .short('s')
                .long("service")
                .value_name("SERVICE")
                .help("Generate a service"),
        )
        .arg(
            Arg::new("module")
                .short('m')
                .long("module")
                .value_name("MODULE")
                .help("Generate a module"),
        )
        .arg(
            Arg::new("middleware")
                .short('w')
                .long("middleware")
                .value_name("MIDDLEWARE")
                .help("Generate a middleware"),
        )
        .arg(
            Arg::new("gate")
                .short('g')
                .long("gate")
                .value_name("GATE")
                .help("Generate a gate"),
        )
        .arg(
            Arg::new("route")
                .short('r')
                .long("route")
                .value_name("ROUTE")
                .help("Generate a route"),
        )
}

pub fn run(_matches: &ArgMatches, subcommand_matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    info!(
        "generate cmd {:?}",
        subcommand_matches.get_one::<String>("name")
    );

    let schematic_name = subcommand_matches.get_one::<String>("name").unwrap();

    if let Some(controller) = subcommand_matches.get_one::<String>("controller") {
        info!("generate controller {:?}", controller);
        let file_name = generate_file_name(controller, "controller");
        let template = read_template_file("controller")?;
        let tpl = ramhorns::Template::new(template).unwrap();
    }

    if let Some(service) = subcommand_matches.get_one::<String>("service") {
        info!("generate service {:?}", service);
        let file_name = generate_file_name(service, "service");
    }

    if let Some(module) = subcommand_matches.get_one::<String>("module") {
        info!("generate module {:?}", module);
        let file_name = generate_file_name(module, "module");
    }

    if let Some(middleware) = subcommand_matches.get_one::<String>("middleware") {
        info!("generate middleware {:?}", middleware);
        let file_name = generate_file_name(middleware, "middleware");
    }

    if let Some(gate) = subcommand_matches.get_one::<String>("gate") {
        info!("generate gate {:?}", gate);
        let file_name = generate_file_name(gate, "gate");
    }

    if let Some(route) = subcommand_matches.get_one::<String>("route") {
        info!("generate route {:?}", route);
        let file_name = generate_file_name(route, "route");
    }

    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

fn generate_file_name(name: &str, suffix: &str) -> String {
    format!("{}_{}", name, suffix)
}

fn read_template_file(file_name: &str) -> Result<String> {
    let template = std::fs::read_to_string(format!("../template/{}.hbs", file_name))?;
    Ok(template)
}
