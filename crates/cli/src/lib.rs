#![allow(clippy::missing_const_for_fn)]

mod data;
use anyhow::{anyhow, Result};
use convert_case::Casing;
pub use data::{CmdExit, CMD};
use data::{Mods, Schematic};
use tracing::info;

pub fn generate_file_name(name: &str, suffix: &str) -> String {
    format!("{}_{}", name, suffix).to_case(convert_case::Case::Snake)
}

fn read_template_file(file_name: &str) -> Result<String> {
    match file_name {
        "service" => Ok(include_str!("templates/service.hbs").to_string()),
        "gate" => Ok(include_str!("templates/gate.hbs").to_string()),
        // "middleware" => Ok(include_str!("templates/middleware.hbs").to_string()),
        "controller" => Ok(include_str!("templates/controller.hbs").to_string()),
        "module" => Ok(include_str!("templates/module.hbs").to_string()),
        "route" => Ok(include_str!("templates/route.hbs").to_string()),
        "mod" => Ok(include_str!("templates/mod.hbs").to_string()),
        _ => Err(anyhow!("Template not found")),
    }
}

fn render_mods(mod_path: &str, schematic: &Schematic) -> Result<()> {
    let mod_template = read_template_file("mod")?;
    let mod_tpl = ramhorns::Template::new(mod_template)?;

    mod_tpl.render_to_file(&mod_path, &schematic)?;
    Ok(())
}

fn render_templates(
    schematic_path: &str,
    mod_path: &str,
    suffix: &str,
    schematic: Schematic,
) -> Result<bool> {
    // read templates to render
    let template = read_template_file(suffix)?;
    let mod_template = read_template_file("mod")?;

    // parse templates
    let tpl = ramhorns::Template::new(template)?;
    let mod_tpl = ramhorns::Template::new(mod_template)?;

    mod_tpl.render_to_file(&mod_path, &schematic)?;

    tpl.render_to_file(
        schematic_path,
        &Schematic {
            initial: String::default(),
            mods: Vec::new(),
            ..schematic
        },
    )
    .unwrap();

    Ok(true)
}

pub fn generate_generic(name: &str, suffix: &str) -> Result<bool> {
    let cwd = std::env::current_dir()?.to_str().unwrap().to_string();
    let file_name = generate_file_name(name, suffix);
    let schematic_root = format!("{}/src/{}", cwd, suffix);

    // ensure the module directory exists
    // if it doesn't, create it
    if !std::path::Path::new(&schematic_root).exists() {
        std::fs::create_dir_all(&schematic_root)?;
        let mod_path = format!("{}/mod.rs", schematic_root);
        let mods = vec![Mods {
            name: name.to_string(),
            suffix: suffix.to_string(),
        }];
        let schmatic = Schematic {
            name: name.to_string(),
            mods,
            services: Vec::new(),
            initial: std::fs::read_to_string(&mod_path)?,
        };
        render_mods(&mod_path, &schmatic)?;
    }

    // path to the module
    let schematic_path = format!("{}/{}.rs", schematic_root, file_name);
    let mod_path = format!("{}/mod.rs", schematic_root);

    let schematic = Schematic {
        name: name.to_string(),
        mods: Vec::new(),
        services: Vec::new(),
        initial: std::fs::read_to_string(&mod_path)?,
    };

    render_templates(&schematic_path, &mod_path, suffix, schematic)
}

pub fn generate_schematic(name: &str, suffix: &str, services: Vec<Mods>) -> Result<bool> {
    let cwd = std::env::current_dir()?.to_str().unwrap().to_string();
    let file_name = generate_file_name(name, suffix);
    let schematic_root = format!("{}/src/modules/{}", cwd, name);

    // ensure the module directory exists
    // if it doesn't, create it
    if !std::path::Path::new(&schematic_root).exists() {
        std::fs::create_dir_all(&schematic_root)?;
        let mod_path = format!("{}/../mod.rs", schematic_root);
        let mods = vec![Mods {
            name: name.to_string(),
            suffix: suffix.to_string(),
        }];
        let schmatic = Schematic {
            name: name.to_string(),
            mods,
            services: Vec::new(),
            initial: std::fs::read_to_string(&mod_path)?,
        };
        render_mods(&mod_path, &schmatic)?;
    }

    // path to the module
    let schematic_path = format!("{}/{}.rs", schematic_root, file_name);
    let mod_path = format!("{}/mod.rs", schematic_root);

    // ensure the mod file exists
    // if it doesn't, create it
    if !std::path::Path::new(&mod_path).exists() {
        std::fs::write(&mod_path, "")?;
    }
    let mods = vec![Mods {
        name: file_name.to_string(),
        suffix: file_name.to_case(convert_case::Case::Pascal),
    }];

    info!("Writing out templates {}", mod_path);

    let schematic = Schematic {
        name: file_name.to_case(convert_case::Case::Pascal),
        mods,
        services,
        initial: std::fs::read_to_string(&mod_path)?,
    };

    render_templates(&schematic_path, &mod_path, suffix, schematic)
}
