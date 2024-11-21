use std::fs;
use std::error::Error;
use serde_json::Value;

const CACHE_FILE: &str = "cache.json";

/// Initialize an empty cache if it doesn't exist
pub fn initialize_cache() -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(CACHE_FILE).exists() {
        fs::write(CACHE_FILE, "{}")?; // Create an empty cache file
    }
    Ok(())
}

/// Save a key-value pair to the cache
pub fn save_to_cache(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mut cache: Value = load_cache()?;
    cache[key] = Value::String(value.to_string());
    fs::write(CACHE_FILE, serde_json::to_string(&cache)?)?;
    Ok(())
}

/// Load the cache data from the file
pub fn load_cache() -> Result<Value, Box<dyn Error>> {
    let cache_data = fs::read_to_string(CACHE_FILE)?;
    let cache: Value = serde_json::from_str(&cache_data)?;
    Ok(cache)
}
