use clap::Parser;
use serde::{Deserialize, Serialize};
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

#[derive(Parser, Debug)]
#[command(about, version)]
struct Opts {
    /// New pacakge name
    #[arg(short, long)]
    new_name: String,
    /// Path to the Cargo.toml file
    #[arg(short, long, default_value = "Cargo.toml")]
    path: String,
}

fn main() {
    // Retrieve the command line arguments using `clap`
    let opts = Opts::parse();

    let new_package_name = &opts.new_name;
    let path_to_cargo_toml = &opts.path;

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
