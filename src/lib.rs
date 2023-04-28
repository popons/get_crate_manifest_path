use serde::Deserialize;
use serde_json::from_str;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    id: String,
    manifest_path: String,
}

/// Returns the manifest file path of the specified crate.
///
/// Uses the `cargo metadata` command to obtain metadata about the current project's dependencies,
/// and returns the path to the manifest file (`Cargo.toml`) of the crate that matches the specified crate name.
///
/// # Arguments
///
/// * `target_crate_name` - The crate name to search for (e.g., "serde")
///
/// # Returns
///
/// A `String` representing the path to the specified crate's manifest file.
/// This function will panic if the crate cannot be found.
///
/// # Example
///
/// ```
/// use get_crate_manifest_path::get_crate_manifest_path;
///
/// let manifest_path = get_crate_manifest_path("serde");
///
/// println!("serde crate manifest path: {}", manifest_path);
/// ```
///
/// # Note
///
/// This function assumes that the cargo command is installed on the system.
/// Also, depending on the environment, it may affect performance.
pub fn get_crate_manifest_path(target_crate_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version=1"])
        .output()
        .expect("Failed to execute `cargo metadata`");

    let metadata: Metadata = from_str(
        &String::from_utf8(output.stdout).expect("Failed to parse `cargo metadata` output"),
    )
    .expect("Failed to deserialize metadata");
    let crate_a_manifest_path = metadata
        .packages
        .into_iter()
        .find(|package| package.name == target_crate_name)
        .map(|package| package.manifest_path)
        .expect("Could not find the specified crate");
    crate_a_manifest_path
}
