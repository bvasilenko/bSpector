use crate::{BspectorError, VulnerabilityPattern, scan};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(name = "bspector")]
#[command(
    about = "Prompt lookup tool. Scans a skill manifest against a closed vulnerability-pattern taxonomy; emits a directive on stdout; exits with a discriminating code."
)]
pub struct BspectorCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Scan(ScanArgs),
    VulnPatterns,
    Update,
}

#[derive(Debug, Clone, Eq, PartialEq, clap::Args)]
pub struct ScanArgs {
    pub artefact: String,
    #[arg(long, value_name = "shape")]
    pub artefact_shape: Option<String>,
    #[arg(long, value_name = "level")]
    pub strictness: Option<String>,
    #[arg(long)]
    pub llm_stage: bool,
    #[arg(long)]
    pub osv_online: bool,
    #[arg(long, value_name = "path")]
    pub manifest: Option<PathBuf>,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long, value_name = "text")]
    pub reason: Option<String>,
}

impl BspectorCli {
    pub fn run(self) -> Result<ExitCode, BspectorError> {
        match self.command {
            Command::Scan(args) => scan::run(args),
            Command::VulnPatterns => {
                for pattern in VulnerabilityPattern::ALL {
                    println!("{pattern}");
                }
                Ok(ExitCode::SUCCESS)
            }
            Command::Update => deferred_command("update"),
        }
    }
}

fn deferred_command(name: &str) -> Result<ExitCode, BspectorError> {
    println!("bspector {name} placeholder: behavior is deferred.");
    Ok(ExitCode::SUCCESS)
}
