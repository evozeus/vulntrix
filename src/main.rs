// src/main.rs
#![deny(warnings)]

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use serde::Serialize;
use std::time::Duration;

mod osv;
use crate::osv::{query_osv, Ecosystem, OsvResponse, VulnLite};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Table,
    Json,
    Ndjson,
}

#[derive(Parser, Debug)]
#[command(
    name = "vulntrix",
    about = "Fast OSV vulnerability lookups across ecosystems"
)]
struct Cli {
    /// Output format
    // Option A — string default (works because `Format` derives ValueEnum)
    #[arg(short = 'f', long = "format", value_enum, default_value = "table")]
    format: Format,

    /// Verbose (can be repeated)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Scan a single package
    Scan {
        /// Package name (e.g. openssl, requests, lodash)
        package: String,
        /// Ecosystem (e.g. crates-io, pypi, npm …)
        #[arg(long = "ecosystem", value_enum)]
        ecosystem: Ecosystem,
        /// Optional: version string to filter advisories that affect this version
        #[arg(long = "version")]
        version: Option<String>,
        /// Per-request timeout (ms)
        #[arg(long = "timeout-ms", default_value_t = 5000u64)]
        timeout_ms: u64,
    },
    /// (Prototype placeholder) Scan many from a file (one per line: <ecosystem> <package> [version])
    Bulk {
        /// Input file path
        file: String,
        /// Per-request timeout (ms)
        #[arg(long = "timeout-ms", default_value_t = 5000u64)]
        timeout_ms: u64,
        /// Concurrency (not yet wired)
        #[arg(long = "concurrency", default_value_t = 8usize)]
        _concurrency: usize,
    },
}

#[derive(Serialize)]
struct JsonOut<'a> {
    package: &'a str,
    ecosystem: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
    vulns: Vec<VulnLite>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Scan {
            package,
            ecosystem,
            version,
            timeout_ms,
        } => {
            let dur = Duration::from_millis(timeout_ms);
            let http = reqwest::Client::builder().timeout(dur).build()?;

            let resp: OsvResponse =
                query_osv(&http, &package, ecosystem, version.as_deref()).await?;

            match cli.format {
                Format::Table => print_table(&package, ecosystem, version.as_deref(), &resp),
                Format::Json => {
                    let out = JsonOut {
                        package: &package,
                        ecosystem: ecosystem.as_str(),
                        version: version.as_deref(),
                        vulns: resp
                            .vulns
                            .iter()
                            .map(VulnLite::from_advisory)
                            .collect::<Vec<_>>(),
                    };
                    println!("{}", serde_json::to_string_pretty(&out)?);
                }
                Format::Ndjson => {
                    let out = JsonOut {
                        package: &package,
                        ecosystem: ecosystem.as_str(),
                        version: version.as_deref(),
                        vulns: resp
                            .vulns
                            .iter()
                            .map(VulnLite::from_advisory)
                            .collect::<Vec<_>>(),
                    };
                    println!("{}", serde_json::to_string(&out)?);
                }
            }
        }

        Command::Bulk { file, .. } => {
            // Keeping it explicit that Bulk is a placeholder so clippy stays happy and users aren’t confused.
            println!(
               "{} prototype placeholder — expects lines like: \"<ecosystem> <package> [version]\"",
                "Bulk mode:".bold()
            );
            println!("Input file: {file}");
        }
    }

    Ok(())
}

fn print_table(pkg: &str, eco: Ecosystem, ver: Option<&str>, resp: &OsvResponse) {
    println!("{} ", "vulntrix — OSV results".bold());
    if let Some(v) = ver {
        println!(
            "Package: {}  Ecosystem: {}  Version: {}",
            pkg,
            eco.as_str(),
            v
        );
    } else {
        println!("Package: {}  Ecosystem: {}", pkg, eco.as_str());
    }

    if resp.vulns.is_empty() {
        println!("{}", "No vulnerabilities found.".green());
        return;
    }

    println!("{}", "Vulnerabilities:".yellow().bold());
    for v in &resp.vulns {
        let sev = v.best_severity().unwrap_or("-");
        println!(
            "• {} [{}] {}",
            v.id.bold(),
            sev,
            v.summary.as_deref().unwrap_or("-")
        );
    }
}
