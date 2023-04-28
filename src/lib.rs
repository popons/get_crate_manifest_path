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

/// 与えられたクレート名のマニフェストファイルへのパスを返す関数です。
///
/// `cargo metadata`コマンドを使用して、現在のプロジェクトの依存関係のメタデータを取得し、
/// 指定されたクレート名に一致するクレートのマニフェストファイル（`Cargo.toml`）へのパスを返します。
///
/// # 引数
///
/// * `target_crate_name` - 探す対象のクレート名（例: "serde"）
///
/// # 返り値
///
/// 指定されたクレートのマニフェストファイルへのパスを表す`String`。
/// クレートが見つからない場合、この関数はパニックします。
///
/// # 例
///
/// ```
/// use get_crate_manifest_path::get_crate_manifest_path;
///
/// let manifest_path = get_crate_manifest_path("serde");
///
/// println!("serde crate manifest path: {}", manifest_path);
/// ```
///
/// # 注意
///
/// この関数は、システムにcargoコマンドがインストールされていることを前提としています。
/// また、環境によってはパフォーマンスに影響する可能性があります。
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
