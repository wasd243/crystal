use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use std::collections::HashMap;

mod impls;

/// The Crystal manifest is the root of the TOML file
#[derive(Debug, Serialize, Deserialize)]
pub struct CrystalManifest {
    pub package: CrystalPackage,
    #[serde(flatten)]
    pub toolchains: HashMap<String, CrystalToolchain>,
}

/// The Crystal package is the name of the package and the platform and CPU it supports
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrystalPackage {
    pub name: String,
    pub platform: Platform,
    pub cpu: Cpu,
}

/// A Crystal toolchain can be either a built-in version or a custom one
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum CrystalToolchain {
    BuiltIn {
        version: String,
    },
    Custom {
        url: Url,
        method: CrystalToolchainMethod,
    },
}

/// When downloading a Crystal toolchain, the method used is determined by this enum
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CrystalToolchainMethod {
    Installer,
    Zip,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Macos,
    Linux,
}

/// CPU structure supported by Crystal
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Cpu {
    X86,
    X86_64,
    Arm,
    Arm64,
}

#[derive(Debug, Error)]
pub enum CrystalParseErr {
    #[error("[ParseErr01] `\\` and `/` are not valid for package name.")]
    InvalidPackageName,
    #[error("[TomlParseErr] Failed to parse Toml: {0}")]
    TomlSyntaxError(#[from] toml::de::Error),
}

/// Parse a TOML string into a CrystalManifest
pub fn parse_toml(toml: &str) -> Result<CrystalManifest, CrystalParseErr> {
    let manifest: CrystalManifest = toml::from_str(toml)?;
    manifest.package.check()?;
    Ok(manifest)
}
