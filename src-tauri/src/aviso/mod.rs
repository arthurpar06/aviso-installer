use serde::Serialize;
use std::fs;

mod ese;
mod sct;

#[derive(Serialize, Debug, PartialEq)]
pub enum InstallationStatus {
    #[serde(rename = "PARTIALLY_INSTALLED")]
    PartiallyInstalled,
    #[serde(rename = "NOT_INSTALLED")]
    NotInstalled,
}

pub fn install_aviso(lfxx_path: &str, aviso_path: &str) -> Result<(), String> {
    let aviso_bytes =
        fs::read(aviso_path).map_err(|e| format!("Failed to read AVISO.sct: {}", e))?;
    let aviso_content = String::from_utf8_lossy(&aviso_bytes);
    install_aviso_content(lfxx_path, &aviso_content)
}

pub fn install_aviso_content(lfxx_path: &str, aviso_content: &str) -> Result<(), String> {
    sct::install_content(lfxx_path, aviso_content)
}

pub fn is_aviso_installed(
    lfxx_path: &str,
    aviso_content: &str,
) -> Result<InstallationStatus, String> {
    sct::is_installed(lfxx_path, aviso_content)
}

pub fn install_aviso_package(
    lfxx_path: &str,
    lfxx_ese_path: Option<&str>,
    aviso_content: &str,
    ese_content: Option<&str>,
    filename: &str,
) -> Result<(), String> {
    // 1. Install SCT content
    sct::install_content(lfxx_path, aviso_content)?;

    // 2. Install ESE content if provided
    if let (Some(ese_path), Some(ese_remote)) = (lfxx_ese_path, ese_content) {
        // Extract category from filename (e.g. LFFF_AVISO.sct -> LFFF)
        // Assuming filename follows pattern: ICAO_...
        let category = filename.split('_').next().unwrap_or(filename);
        ese::install_content(ese_path, ese_remote, category)?;
    }

    Ok(())
}
