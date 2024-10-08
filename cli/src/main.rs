use clap::Parser;
use heck::ToSnakeCase;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, write_keypair_file, Keypair};
use solana_sdk::signer::Signer;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use toml::Value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "Kaptn CLI",
    version = VERSION,
    about = "CLI for Kaptn Framework"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        long,
        value_name = "NETWORK_URL",
        help = "Network address of your RPC provider",
        global = true
    )]
    rpc: Option<String>,

    #[clap(
        global = true,
        short = 'C',
        long = "config",
        id = "PATH",
        help = "Filepath to config file."
    )]
    config_file: Option<String>,

    #[arg(
        long,
        value_name = "KEYPAIR_FILEPATH",
        help = "Filepath to signer keypair.",
        global = true
    )]
    keypair: Option<String>,

    #[arg(
        long,
        value_name = "FEE_PAYER",
        help = "Filepath or URL to a keypair to pay transaction fee [default:client keypair]",
        global = true
    )]
    fee_payer: Option<String>,
}

#[derive(Parser)]
enum Commands {
    #[command(about = "Create a new Kaptn project")]
    New(NewArgs),
    #[command(about = "Create the extra account metas")]
    CreateExtraMetas(CreateExtraMetasArgs),
    #[command(about = "Update the extra account metas")]
    UpdateExtraMetas(UpdateExtraMetasArgs),
    #[command(about = "Deploy the program")]
    Deploy(DeployArgs),
    #[command(about = "Build the program")]
    Build(BuildArgs),
}

#[derive(Parser)]
struct NewArgs {
    #[arg(help = "The path where to create the project")]
    path: String,
    #[arg(
        long,
        short,
        help = "The name of the new project (defaults to directory name)"
    )]
    name: Option<String>,
}

#[derive(Parser)]
struct CreateExtraMetasArgs {}

#[derive(Parser)]
struct UpdateExtraMetasArgs {}

#[derive(Parser)]
struct DeployArgs {
    #[arg(
        help_heading = "Flags",
        long,
        value_name = "USE_QUIC",
        help = "Use QUIC when sending transactions."
    )]
    use_quic: bool,

    #[arg(
        help_heading = "Flags",
        long,
        value_name = "USE_RPC",
        help = "Send write transactions to the configured RPC instead of validator TPUs."
    )]
    use_rpc: bool,

    #[arg(
        help_heading = "Flags",
        long = "final",
        value_name = "FINAL",
        help = "The program will not be upgradeable."
    )]
    is_final: bool,

    #[arg(
        long,
        value_name = "COMPUTE_UNIT_PRICE",
        help = "Set compute unit price for transaction, in increments of 0.000001 lamports per compute unit."
    )]
    with_compute_unit_price: Option<String>,

    #[arg(
        long,
        value_name = "MAX_SIGN_ATTEMPTS",
        help = "Maximum number of attempts to sign or resign transactions after blockhash expiration. If any transactions
            sent during the program deploy are still unconfirmed after the initially chosen recent blockhash expires,
            those transactions will be resigned with a new recent blockhash and resent. Use this setting to adjust the
            maximum number of transaction signing iterations. Each blockhash is valid for about 60 seconds, which means
            using the default value of 5 will lead to sending transactions for at least 5 minutes or until all
            transactions are confirmed,whichever comes first. [default: 5]"
    )]
    max_sign_attempts: Option<String>,
}

#[derive(Parser)]
struct BuildArgs {}

struct Config {
    rpc: String,
    keypair: String,
    fee_payer: Option<String>,
    name: String,
    program_keypair: String,
    mint_keypair: String,
}

fn main() {
    let cli = Cli::parse();

    // Load the config file from custom path, the default path, or use default config values
    let cli_config = if let Some(config_file) = &cli.config_file {
        solana_cli_config::Config::load(config_file).unwrap_or_else(|_| {
            eprintln!("error: Could not find config file `{}`", config_file);
            std::process::exit(1);
        })
    } else if let Some(config_file) = &*solana_cli_config::CONFIG_FILE {
        solana_cli_config::Config::load(config_file).unwrap_or_default()
    } else {
        solana_cli_config::Config::default()
    };

    let config = Config {
        rpc: cli.rpc.unwrap_or(cli_config.json_rpc_url),
        keypair: cli.keypair.unwrap_or(cli_config.keypair_path),
        fee_payer: cli.fee_payer,
        name: get_project_name().unwrap(),
        program_keypair: format!(
            "target/deploy/{}-keypair.json",
            get_project_name().unwrap().to_snake_case()
        ),
        mint_keypair: format!(
            "target/deploy/{}-mint-keypair.json",
            get_project_name().unwrap().to_snake_case()
        ),
    };

    match cli.command {
        Commands::New(args) => new(args, config).unwrap(),
        Commands::CreateExtraMetas(args) => create_extra_metas(args, config).unwrap(),
        Commands::UpdateExtraMetas(args) => update_extra_metas(args, config).unwrap(),
        Commands::Deploy(args) => deploy(args, config).unwrap(),
        Commands::Build(args) => build(args, config).unwrap(),
    }
}

fn new(args: NewArgs, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let name = config.name;

    create_new_project(&args.path, &name)
}

fn build(_args: BuildArgs, _config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building the program...");

    let mut command = std::process::Command::new("cargo");
    command
        .arg("build-bpf")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    Ok(())
}

fn deploy(args: DeployArgs, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating extra account metas...");

    let mut command = std::process::Command::new("solana");
    command
        .arg("program")
        .arg("deploy")
        .arg(config.program_keypair)
        .arg("--url")
        .arg(config.rpc)
        .arg("--upgrade-authority")
        .arg(config.keypair)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if let Some(fee_payer) = config.fee_payer {
        command.arg("--fee-payer").arg(fee_payer);
    }

    if let Some(max_sign_attempts) = args.max_sign_attempts {
        command.arg("--max-sign-attempts").arg(max_sign_attempts);
    }

    if let Some(compute_unit_price) = args.with_compute_unit_price {
        command.arg("--compute-unit-price").arg(compute_unit_price);
    }

    if args.is_final {
        command.arg("--final");
    }
    if args.use_quic {
        command.arg("--use-quic");
    }
    if args.use_rpc {
        command.arg("--use-rpc");
    }

    let exit = command.output()?;

    if !exit.status.success() {
        return Err(format!("There was a problem deploying the program: {:?}", exit).into());
    }

    Ok(())
}

fn create_extra_metas(
    _args: CreateExtraMetasArgs,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating extra account metas...");

    let mut command = std::process::Command::new("spl-transfer-hook");
    command
        .arg("create-extra-metas")
        .arg(config.program_keypair)
        .arg(config.mint_keypair)
        .arg("--url")
        .arg(config.rpc)
        .arg("--mint-authority")
        .arg(config.keypair)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if let Some(fee_payer) = config.fee_payer {
        command.arg("--fee-payer").arg(fee_payer);
    }

    let exit = command.output()?;

    if !exit.status.success() {
        return Err(format!("There was a problem creating extra metas: {:?}", exit).into());
    }
    Ok(())
}

fn update_extra_metas(
    _args: UpdateExtraMetasArgs,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Updating extra account metas...");

    let mut command = std::process::Command::new("spl-transfer-hook");
    command
        .arg("update-extra-metas")
        .arg(config.program_keypair)
        .arg(config.mint_keypair)
        .arg("--url")
        .arg(config.rpc)
        .arg("--mint-authority")
        .arg(config.keypair)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if let Some(fee_payer) = config.fee_payer {
        command.arg("--fee-payer").arg(fee_payer);
    }

    let exit = command.output()?;

    if !exit.status.success() {
        return Err(format!("There was a problem updating extra metas: {:?}", exit).into());
    }
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
version = "0.1.0"
description = "Created with Kaptn"
edition = "2021"

[dependencies]
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
    msg!("Ahoy from transfer-hook program: {{:?}}", ctx.program_id);
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

fn get_project_name() -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let cargo_toml_path = current_dir.join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;

    let cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

    cargo_toml
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .map(|name| name.to_string())
        .ok_or_else(|| "Failed to get project name from Cargo.toml".into())
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
