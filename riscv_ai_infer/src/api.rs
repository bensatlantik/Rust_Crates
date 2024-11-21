use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct CrateStats {
    pub downloads: u64,
    pub recent_downloads: Option<u64>,
}

pub fn fetch_crate_stats(crate_name: &str) -> Result<CrateStats, Box<dyn Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let response = get(&url)?;
    let crate_info: serde_json::Value = response.json()?;
    
    let downloads = crate_info["crate"]["downloads"].as_u64().unwrap_or(0);
    let recent_downloads = crate_info["crate"]["recent_downloads"].as_u64();

    Ok(CrateStats {
        downloads,
        recent_downloads,
    })
}
