use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

// Define the RouteInfo struct first
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub route: String,
    pub path: String,
    pub label: String,
    pub description: String,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

mod scanner;
mod sitemap_json;
mod sitemap_xml;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to Next.js project
    #[arg(short, long, default_value = ".")]
    project: PathBuf,

    /// Output path for sitemap.xml
    #[arg(long, default_value = "sitemap.xml")]
    xml_output: PathBuf,

    /// Output path for sitemap.json
    #[arg(long, default_value = "sitemap.json")]
    json_output: PathBuf,

    /// Base URL for sitemap
    #[arg(short, long, default_value = "https://example.com")]
    base_url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Scan project for routes
    let mut routes = scanner::scan_project(&args.project).context("Failed to scan project")?;

    // Sort routes for consistent output
    routes.sort_by(|a, b| a.route.cmp(&b.route));

    // Generate sitemap.xml
    sitemap_xml::generate(&routes, &args.xml_output, &args.base_url)
        .context("Failed to generate sitemap.xml")?;

    // Generate sitemap.json
    sitemap_json::generate(&routes, &args.json_output)
        .context("Failed to generate sitemap.json")?;

    println!("Generated sitemap.xml at {}", args.xml_output.display());
    println!("Generated sitemap.json at {}", args.json_output.display());

    Ok(())
}
