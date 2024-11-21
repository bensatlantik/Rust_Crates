use crate::api::CrateStats;

pub fn display_crate_stats(crate_name: &str, stats: &CrateStats) {
    println!("Crate: {}", crate_name);
    println!("Total Downloads: {}", stats.downloads);
    
    if let Some(recent) = stats.recent_downloads {
        println!("Recent Downloads: {}", recent);
    } else {
        println!("No recent downloads data available.");
    }
}
