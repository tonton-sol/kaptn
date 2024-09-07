use clap::{Arg, Command};
use heck::ToSnakeCase;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, write_keypair_file, Keypair};
use solana_sdk::signer::Signer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};
use toml::Value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new("Kaptn CLI")
        .version("0.1.0")
        .about("CLI for Kaptn projects")
        .subcommand(
            Command::new("new")
                .about("Create a new Kaptn project")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("The path where to create the project"),
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .help("The name of the new project (defaults to directory name)")
                        .required(false),
                ),
        )
        .subcommand(Command::new("create-extra-metas").about("Create the extra account metas"))
        .subcommand(Command::new("update-extra-metas").about("Update the extra account metas"))
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").unwrap();
            let name = sub_matches
                .get_one::<String>("name")
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    PathBuf::from(path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                });

            if let Err(err) = create_new_project(path, &name) {
                eprintln!("Error creating project: {}", err);
                process::exit(1);
            }
        }
        Some(("create-extra-metas", _)) => {
            println!("Creating extra account metas...");
            let name = get_project_name().unwrap();

            let program_keypair_filepath =
                Path::new("target/deploy").join(format!("{}-keypair.json", name.to_snake_case()));
            let mint_keypair_filepath = Path::new("target/deploy")
                .join(format!("{}-mint-keypair.json", name.to_snake_case()));

            let program_keypair = read_keypair_file(&program_keypair_filepath).unwrap();
            let mint_keypair = read_keypair_file(&mint_keypair_filepath).unwrap();

            let program_pubkey = program_keypair.pubkey();
            let mint_pubkey = mint_keypair.pubkey();

            let exit = std::process::Command::new("spl-transfer-hook")
                .arg("create-extra-metas")
                .arg(program_pubkey.to_string())
                .arg(mint_pubkey.to_string())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("Must create extra metas");

            // Check if deployment was successful
            if !exit.status.success() {
                println!("There was a problem creating extra metas: {exit:?}.");
                std::process::exit(exit.status.code().unwrap_or(1));
            }
        }
        Some(("update-extra-metas", _)) => {
            println!("Updating extra account metas...");
            let name = get_project_name().unwrap();

            let program_keypair_filepath =
                Path::new("target/deploy").join(format!("{}-keypair.json", name.to_snake_case()));
            let mint_keypair_filepath = Path::new("target/deploy")
                .join(format!("{}-mint-keypair.json", name.to_snake_case()));

            let program_keypair = read_keypair_file(&program_keypair_filepath).unwrap();
            let mint_keypair = read_keypair_file(&mint_keypair_filepath).unwrap();

            let program_pubkey = program_keypair.pubkey();
            let mint_pubkey = mint_keypair.pubkey();

            let exit = std::process::Command::new("spl-transfer-hook")
                .arg("update-extra-metas")
                .arg(program_pubkey.to_string())
                .arg(mint_pubkey.to_string())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("Must update extra metas");

            // Check if deployment was successful
            if !exit.status.success() {
                println!("There was a problem creating extra metas: {exit:?}.");
                std::process::exit(exit.status.code().unwrap_or(1));
            }
        }
        _ => {
            eprintln!("Please specify a subcommand. Run with --help for more information.");
            process::exit(1);
        }
    }
}

fn create_new_project(path: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating new Kaptn project '{}' at: {}", name, path);

    // Create the project directory
    fs::create_dir_all(path)?;

    // Change to the project directory
    std::env::set_current_dir(path)?;

    // Create Cargo.toml
    let cargo_toml_content = create_cargo_toml_content(name);

    fs::write("Cargo.toml", cargo_toml_content)?;

    // Create src directory
    fs::create_dir_all("src")?;

    // Create the program file
    let lib_rs_content = create_program_content(name);

    fs::write("src/lib.rs", lib_rs_content)?;

    println!("Kaptn project '{}' created successfully at: {}", name, path);
    Ok(())
}

pub fn get_or_create_program_id(name: &str) -> Pubkey {
    let keypair_path = Path::new("target")
        .join("deploy")
        .join(format!("{}-keypair.json", name.to_snake_case()));

    read_keypair_file(&keypair_path)
        .unwrap_or_else(|_| {
            let keypair = Keypair::new();
            write_keypair_file(&keypair, keypair_path).expect("Unable to create program keypair");
            keypair
        })
        .pubkey()
}

pub fn get_or_create_mint_id(name: &str) -> Pubkey {
    let keypair_path = Path::new("target")
        .join("deploy")
        .join(format!("{}-mint-keypair.json", name.to_snake_case()));

    read_keypair_file(&keypair_path)
        .unwrap_or_else(|_| {
            let keypair = Keypair::new();
            write_keypair_file(&keypair, keypair_path).expect("Unable to create mint keypair");
            keypair
        })
        .pubkey()
}

pub fn create_cargo_toml_content(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.2"
edition = "2021"

[dependencies]
solana-program = "2.0.8"
kaptn-lang = "{}" 

[lib]
crate-type = ["cdylib", "lib"]
name = "{}"
"#,
        name,
        VERSION,
        name.to_snake_case()
    )
}

pub fn create_program_content(name: &str) -> String {
    format!(
        r#"use kaptn_lang::prelude::*;

declare_id!("{}");
declare_mint!("{}");

#[transfer_hook]
pub fn {}(ctx: TransferContext<MyExtraMetas>) -> ProgramResult {{
    msg!("Transfer hook called!");
    Ok(())
}}

#[derive(ExtraMetas)]
pub struct MyExtraMetas {{}}
"#,
        get_or_create_program_id(name),
        get_or_create_mint_id(name),
        name.to_snake_case()
    )
}

fn get_project_name() -> Option<String> {
    // Determine the path to Cargo.toml
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let cargo_toml_path = current_dir.join("Cargo.toml");

    // Read the Cargo.toml file
    let cargo_toml_content =
        fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");

    // Parse the Cargo.toml content
    let cargo_toml: Value =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    // Extract the project name from [package] section
    cargo_toml
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .map(|name| name.to_string())
}
