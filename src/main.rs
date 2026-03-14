use clap::{Parser, ValueEnum};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Bootstrap a new Minecraft mod project with Fabric, NeoForge or Forge (wip)"
)]
struct Args {
    #[arg(value_enum, short, long, default_value_t = ModLoader::Fabric)]
    modloader: ModLoader,

    #[arg(index = 1)]
    projectname: String,

    #[arg(short, long, default_value = "1.21.1")]
    mcversion: String,
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
enum ModLoader {
    Fabric,
    NeoForge,
    Forge,
}

fn run_git_clone(url: &str, branch: Option<&str>, dest: &str) -> io::Result<ExitStatus> {
    let mut cmd = Command::new("git");
    cmd.arg("clone");

    if let Some(b) = branch {
        cmd.arg("--branch").arg(b);
    }

    cmd.arg(url).arg(dest);

    let output = cmd.output()?;

    if !output.status.success() {
        io::stderr().write_all(&output.stderr)?;
    }

    Ok(output.status)
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

fn setup_fabric(project_name: &str, mcversion: &str) -> io::Result<()> {
    println!(
        "Cloning Fabric example mod (branch {})...",
        fabric_branch(mcversion)
    );

    let status = run_git_clone(
        "https://github.com/FabricMC/fabric-example-mod.git",
        Some(fabric_branch(mcversion)),
        project_name,
    )?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "git clone failed for Fabric",
        ));
    }

    println!("\nFabric project cloned successfully!");
    println!("Next steps:");
    println!("  cd {}", project_name);

    Ok(())
}

fn neoforge_repo_name(mcversion: &str) -> String {
    if mcversion.starts_with("1.21") {
        format!("MDK-1.21-NeoGradle")
    } else if mcversion.starts_with("1.20") {
        "MDK-1.20.6-NeoGradle".to_string()
    } else {
        eprintln!(
            "Warning: no specific NeoForge MDK known for {}, trying 1.21",
            mcversion
        );
        "MDK-1.21-NeoGradle".to_string()
    }
}

fn setup_neoforge(project_name: &str, mcversion: &str) -> io::Result<()> {
    let repo_name = neoforge_repo_name(mcversion);
    let url = format!("https://github.com/NeoForgeMDKs/{}.git", repo_name);

    let status = run_git_clone(&url, None, project_name)?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("couldnt create project for NeoForge ({})", url),
        ));
    }

    println!("\nNeoForge project cloned successfully!");
    println!("Next steps:");
    println!("  cd {}", project_name);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if Path::new(&args.projectname).exists() {
        return Err(format!("Directory '{}' already exists!", args.projectname).into());
    }

    match args.modloader {
        ModLoader::Fabric => setup_fabric(&args.projectname, &args.mcversion)?,

        ModLoader::NeoForge => setup_neoforge(&args.projectname, &args.mcversion)?,
        ModLoader::Forge => todo!(),
    }

    println!("\nDone! Happy modding! ✨");
    Ok(())
}
