use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Ecosystem {
    /// Rust (crates.io)
    CratesIo,
    /// Python (PyPI)
    Pypi,
    /// JavaScript (npm)
    Npm,
    /// Ruby (Rubygems)
    Rubygems,
    /// Java (Maven)
    Maven,
    /// PHP (Packagist/Composer)
    Packagist,
    /// Go
    Go,
    /// .NET (NuGet)
    Nuget,
    /// Dart (pub.dev)
    Pub,
}

impl Ecosystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            Ecosystem::CratesIo => "crates.io",
            Ecosystem::Pypi => "PyPI",
            Ecosystem::Npm => "npm",
            Ecosystem::Rubygems => "RubyGems",
            Ecosystem::Maven => "Maven",
            Ecosystem::Packagist => "Packagist",
            Ecosystem::Go => "Go",
            Ecosystem::Nuget => "NuGet",
            Ecosystem::Pub => "Pub",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OsvSeverity {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub score: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OsvAdvisory {
    pub id: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub severity: Vec<OsvSeverity>,
}

impl OsvAdvisory {
    pub fn best_severity(&self) -> Option<&str> {
        self.severity
            .first()
            .and_then(|s| s.kind.as_deref())
            .or_else(|| self.severity.first().and_then(|s| s.score.as_deref()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OsvResponse {
    #[serde(default)]
    pub vulns: Vec<OsvAdvisory>,
}

#[derive(Serialize)]
struct OsvQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
    package: OsvPackage<'a>,
}

#[derive(Serialize)]
struct OsvPackage<'a> {
    name: &'a str,
    ecosystem: &'a str,
}

/// A compact view used for table/JSON output
#[derive(Debug, Serialize, Clone)]
pub struct VulnLite {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl VulnLite {
    pub fn from_advisory(v: &OsvAdvisory) -> Self {
        Self {
            id: v.id.clone(),
            summary: v.summary.clone(),
            severity: v
                .best_severity()
                .map(|s| s.to_string())
                .or_else(|| v.severity.first().and_then(|x| x.score.clone())),
        }
    }
}

pub async fn query_osv(
    http: &Client,
    name: &str,
    eco: Ecosystem,
    version: Option<&str>,
) -> Result<OsvResponse> {
    let body = OsvQuery {
        version,
        package: OsvPackage {
            name,
            ecosystem: eco.as_str(),
        },
    };

    let resp = http
        .post("https://api.osv.dev/v1/query")
        .json(&body)
        .send()
        .await
        .context("request to OSV failed")?;

    let status = resp.status();
    let text = resp.text().await.context("reading OSV body failed")?;
    if !status.is_success() {
        anyhow::bail!("OSV error {status}: {text}");
    }

    let parsed: OsvResponse = serde_json::from_str(&text).context("parsing OSV JSON failed")?;
    Ok(parsed)
}
