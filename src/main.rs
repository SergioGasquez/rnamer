use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
}

fn main() {
    // Get the first command-line argument as the new package name
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <new_package_name> <path_to_cargo_toml>");
        return;
    }
    let new_package_name = &args[1];
    let path_to_cargo_toml = &args[2];

    // Read the content of Cargo.toml into a String
    let mut content = String::new();
    let mut file = File::open(path_to_cargo_toml).expect("Failed to open Cargo.toml");
    file.read_to_string(&mut content)
        .expect("Failed to read Cargo.toml");

    // Parse the content into a TOML value
    let mut cargo_toml: Value = content.parse().expect("Failed to parse Cargo.toml");

    // Modify the name field with the new package name
    cargo_toml["package"]["name"] = Value::String(new_package_name.to_string());

    // Serialize the updated content back to a String
    let updated_content = toml::to_string(&cargo_toml).expect("Failed to serialize toml");

    // Write the updated content back to the file
    let mut file = File::create(path_to_cargo_toml).expect("Failed to create Cargo.toml");
    file.write_all(updated_content.as_bytes())
        .expect("Failed to write to Cargo.toml");

    println!(
        "Package name updated to '{}' in {}",
        new_package_name, path_to_cargo_toml
    );
}
