// use anyhow::anyhow;
// use anyhow::Result as AnyResult;
use ramhorns::Content;

pub struct CmdExit {
    pub code: exitcode::ExitCode,
    pub message: Option<String>,
}

#[derive(Content)]
pub struct Mods {
    pub name: String,
    pub suffix: String,
}

#[derive(Content)]
pub struct Schematic {
    pub name: String,
    pub mods: Vec<Mods>,
    pub services: Vec<Mods>,
    pub initial: String,
}
