use anyhow::Result;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::RouteInfo;

pub fn generate(routes: &[RouteInfo], output_path: &Path) -> Result<()> {
    let json = to_string_pretty(routes)?;

    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
