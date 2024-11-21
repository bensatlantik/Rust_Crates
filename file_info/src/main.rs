use std::fs;
use std::os::unix::fs::MetadataExt;

fn main() {
    let filename = "example.txt"; // File to inspect

    match fs::metadata(filename) {
        Ok(metadata) => {
            println!("File Size: {} bytes", metadata.size());
            println!("File Type: {}", if metadata.is_dir() {
                "Directory"
            } else if metadata.is_file() {
                "Regular File"
            } else {
                "Other"
            });
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
