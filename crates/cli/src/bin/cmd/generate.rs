use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};
use convert_case::Casing;
use ramhorns::Content;
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
                .help("Name of the artifact to generate"), // .required(true),
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
                .action(ArgAction::SetTrue)
                .help("Generate a controller to handle requests"),
        )
        .arg(
            Arg::new("service")
                .short('s')
                .long("service")
                .value_name("SERVICE")
                .action(ArgAction::SetTrue)
                .help("Generate a service"),
        )
        .arg(
            Arg::new("module")
                .short('m')
                .long("module")
                .value_name("MODULE")
                .action(ArgAction::SetTrue)
                .default_value("true")
                .help("Generate a module  (default)"),
        )
        .arg(
            Arg::new("middleware")
                .short('w')
                .long("middleware")
                .value_name("MIDDLEWARE")
                .action(ArgAction::SetTrue)
                .help("Generate a middleware"),
        )
        .arg(
            Arg::new("gate")
                .short('g')
                .long("gate")
                .value_name("GATE")
                .action(ArgAction::SetTrue)
                .help("Generate a gate"),
        )
        .arg(
            Arg::new("route")
                .short('r')
                .long("route")
                .action(ArgAction::SetTrue)
                .value_name("ROUTE")
                .help("Generate a route"),
        )
}

pub fn run(_matches: &ArgMatches, subcommand_matches: &ArgMatches) -> Result<cargo_ngyn::CmdExit> {
    info!(
        "generate cmd {:?}",
        subcommand_matches.get_one::<String>("name")
    );

    let schematic_name = if let Some(cmd) = subcommand_matches.get_one::<String>("name") {
        cmd
    } else {
        "kluhj"
    };
    let mut services = Vec::new();

    if let Some(service) = subcommand_matches.get_one::<bool>("service") {
        info!("generate service {:?}", service);
        services.push(generate_file_name(schematic_name, "service"));
        generate_schematic(schematic_name, "service", Vec::new(), Vec::new())?;
    }

    if let Some(gate) = subcommand_matches.get_one::<bool>("gate") {
        info!("generate gate {:?}", gate);
        services.push(generate_file_name(schematic_name, "gate"));
        generate_schematic(schematic_name, "gate", Vec::new(), Vec::new())?;
    }

    if let Some(middleware) = subcommand_matches.get_one::<bool>("middleware") {
        info!("generate middleware {:?}", middleware);
        services.push(generate_file_name(schematic_name, "middleware"));
        generate_generic(schematic_name, "middleware")?;
    }

    if let Some(controller) = subcommand_matches.get_one::<bool>("controller") {
        info!("generate controller {:?}", controller);
        generate_schematic(schematic_name, "controller", Vec::new(), Vec::new())?;
    }

    if let Some(route) = subcommand_matches.get_one::<String>("route") {
        info!("generate route {:?}", route);
        generate_generic(schematic_name, "route")?;
    }

    if let Some(module) = subcommand_matches.get_one::<bool>("module") {
        info!("generate module {:?}", module);
        generate_schematic(schematic_name, "module", Vec::new(), Vec::new())?;
    }

    Ok(cargo_ngyn::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

fn generate_file_name(name: &str, suffix: &str) -> String {
    format!("{}_{}", name, suffix).to_case(convert_case::Case::Snake)
}

fn read_template_file(file_name: &str) -> Result<String> {
    let template = std::fs::read_to_string(format!("../template/{}.hbs", file_name)).unwrap();
    Ok(template)
}

#[derive(Content)]
struct Mods {
    name: String,
    suffix: String,
}

#[derive(Content)]
struct Schematic {
    module_name: String,
    mods: Vec<Mods>,
    services: Vec<Mods>,
    initial: String,
}

fn generate_generic(name: &str, suffix: &str) -> Result<bool> {
    let file_name = generate_file_name(name, suffix);
    let path = format!("src/{}", suffix);
    let file_path = format!("{}/{}.rs", path, file_name);
    let mod_path = format!("{}/mod.rs", path);
    let template = read_template_file(suffix)?;
    let mod_template = read_template_file("mod")?;
    let tpl = ramhorns::Template::new(template).unwrap();
    let mod_tpl = ramhorns::Template::new(mod_template).unwrap();

    let cwd = std::env::current_dir().unwrap();
    let mod_content = std::fs::read_to_string(&mod_path)?;
    let schematic = Schematic {
        module_name: name.to_string(),
        mods: Vec::new(),
        services: Vec::new(),
        initial: mod_content,
    };
    mod_tpl.render_to_file(&mod_path, &schematic)?;

    tpl.render_to_file(
        file_path,
        &Schematic {
            initial: String::default(),
            mods: Vec::new(),
            ..schematic
        },
    )?;

    Ok(true)
}

fn generate_schematic(
    name: &str,
    suffix: &str,
    services: Vec<Mods>,
    mods: Vec<Mods>,
) -> Result<bool> {
    let file_name = generate_file_name(name, suffix);
    let path = format!("src/modules/{}/{}", name, file_name);
    let file_path = format!("{}/{}.rs", path, file_name);
    let mod_path = format!("{}/mod.rs", path);
    println!("Mod:{:?}", mod_path);
    let template = read_template_file(suffix)?;
    println!("Template:{:?}", template);
    let mod_template = read_template_file("mod")?;
    let tpl = ramhorns::Template::new(template).unwrap();
    let mod_tpl = ramhorns::Template::new(mod_template).unwrap();

    let mod_content = std::fs::read_to_string(&mod_path)?;
    let schematic = Schematic {
        module_name: name.to_string(),
        mods,
        services,
        initial: mod_content,
    };
    mod_tpl.render_to_file(&mod_path, &schematic)?;

    tpl.render_to_file(
        file_path,
        &Schematic {
            initial: String::default(),
            mods: Vec::new(),
            ..schematic
        },
    )?;

    Ok(true)
}
