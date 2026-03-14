use clap::{Parser, ValueEnum};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, short, long)]
    modloader: ModLoader,
    #[arg(index = 1)]
    projectname: String,
    #[arg(short, long)]
    mcversion: String,
}

#[derive(Clone, Debug, ValueEnum)]
enum ModLoader {
    Fabric,
    Forge,
    NeoForge,
}

fn fabric_branch(mcversion: &str) -> &'static str {
    if mcversion.starts_with("1.21") {
        "1.21"
    } else if mcversion.starts_with("1.20") {
        "1.20"
    } else if mcversion.starts_with("1.19") {
        "1.19"
    } else if mcversion.starts_with("1.18") {
        "1.18"
    } else if mcversion.starts_with("1.17") {
        "1.17"
    } else {
        "1.21"
    }
}
fn download_fabric(project_name: &str, mcversion: &str) -> std::io::Result<()> {
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/FabricMC/fabric-example-mod")
        .arg("--branch")
        .arg(fabric_branch(mcversion))
        .arg(project_name)
        .output()?;
    Ok(())
}

fn clone_neoforge(project_name: &str, mcversion: &str) -> std::io::Result<()> {
    Command::new("git")
        .arg("clone")
        .arg(format!(
            "https://github.com/NeoForgeMDKs/MDK-{mcversion}-ModDevGradle"
        ))
        .arg(project_name)
        .output()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.modloader {
        ModLoader::NeoForge => clone_neoforge(&args.projectname, &args.mcversion)?,
        ModLoader::Fabric => download_fabric(&args.projectname, &args.mcversion)?,
        _ => todo!("Not yet implemented"),
    }
    Ok(())
}
