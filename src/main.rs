mod parser;

use std::fs;
use parser::parse_nix;

fn main() {
    let nix_file = "workflow.nix";

    // Read workflow.nix
    let nix_code = fs::read_to_string(nix_file)
        .expect("Failed to read Nix workflow file");

    // Parse and extract tasks
    let tasks = parse_nix(&nix_code);
    
    println!("Extracted Tasks:");
    for task in tasks {
        println!("- {}", task);
    }
}

