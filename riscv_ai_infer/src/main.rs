mod api;
mod ui;
mod config;
mod cache;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration
    config::load_config()?;

    // Initialize cache
    cache::initialize_cache()?;

    // Test saving and loading cache
    cache::save_to_cache("example_key", "example_value")?;
    let cache_data = cache::load_cache()?;
    println!("Cache data: {:?}", cache_data);

    // Fetch data using API
    let crate_name = "serde";
    let stats = api::fetch_crate_stats(&crate_name)?;

    // Display data using UI
    ui::display_crate_stats(&crate_name, &stats);
    
    Ok(())
}
