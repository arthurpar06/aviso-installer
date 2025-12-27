use std::collections::{HashMap, HashSet};
use std::fs;

pub fn install_aviso(lfxx_path: &str, aviso_path: &str) -> Result<(), String> {
    // Read both files as bytes, convert to String using from_utf8_lossy
    let lfxx_bytes = fs::read(lfxx_path).map_err(|e| format!("Failed to read LFXX.sct: {}", e))?;
    let aviso_bytes =
        fs::read(aviso_path).map_err(|e| format!("Failed to read AVISO.sct: {}", e))?;
    let lfxx_content = String::from_utf8_lossy(&lfxx_bytes);
    let aviso_content = String::from_utf8_lossy(&aviso_bytes);

    install_aviso_impl(lfxx_path, &lfxx_content, &aviso_content)
}

pub fn install_aviso_content(lfxx_path: &str, aviso_content: &str) -> Result<(), String> {
    let lfxx_bytes = fs::read(lfxx_path).map_err(|e| format!("Failed to read LFXX.sct: {}", e))?;
    let lfxx_content = String::from_utf8_lossy(&lfxx_bytes);

    install_aviso_impl(lfxx_path, &lfxx_content, aviso_content)
}

fn install_aviso_impl(
    lfxx_path: &str,
    lfxx_content: &str,
    aviso_content: &str,
) -> Result<(), String> {
    // Helper to parse groups into a HashMap
    fn parse_groups(s: &str) -> HashMap<String, Vec<String>> {
        let mut groups = HashMap::new();
        let mut current_group = String::new();
        for line in s.lines() {
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                current_group = name.to_string();
                groups.entry(current_group.clone()).or_insert_with(Vec::new);
            } else if !current_group.is_empty() {
                groups
                    .entry(current_group.clone())
                    .or_insert_with(Vec::new)
                    .push(line.to_string());
            }
        }
        groups
    }

    // Parse LFXX.sct and preserve section order
    let mut section_order: Vec<String> = Vec::new();
    let mut lfxx_groups = HashMap::new();
    let mut current_group = String::new();
    for line in lfxx_content.lines() {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            current_group = name.to_string();
            if !section_order.contains(&current_group) {
                section_order.push(current_group.clone());
            }
            lfxx_groups
                .entry(current_group.clone())
                .or_insert_with(Vec::new);
        } else if !current_group.is_empty() {
            lfxx_groups
                .entry(current_group.clone())
                .or_insert_with(Vec::new)
                .push(line.to_string());
        }
    }

    // Parse AVISO.sct
    let aviso_groups = parse_groups(&aviso_content);

    // Merge: append each aviso group to the corresponding lfxx group
    for (group, lines) in aviso_groups {
        if let Some(lfxx_lines) = lfxx_groups.get_mut(&group) {
            lfxx_lines.extend(lines);
        } else {
            // Only add new sections from aviso if they don't exist in LFXX
            lfxx_groups.insert(group.clone(), lines);
        }
    }

    // Reconstruct the file in the original LFXX section order, then add any new sections from AVISO
    let mut merged = String::new();
    let mut already_written = HashSet::new();
    for group in &section_order {
        merged.push_str(&format!("[{}]\n", group));
        if let Some(lines) = lfxx_groups.get(group) {
            for line in lines {
                merged.push_str(&line);
                merged.push('\n');
            }
        }
        already_written.insert(group.clone());
    }
    // Add any new sections from AVISO that weren't in LFXX, at the end
    for group in lfxx_groups.keys() {
        if !already_written.contains(group) {
            merged.push_str(&format!("[{}]\n", group));
            if let Some(lines) = lfxx_groups.get(group) {
                for line in lines {
                    merged.push_str(&line);
                    merged.push('\n');
                }
            }
        }
    }

    // Save back to LFXX.sct as bytes (UTF-8)
    fs::write(lfxx_path, merged.as_bytes())
        .map_err(|e| format!("Failed to write LFXX.sct: {}", e))?;
    Ok(())
}

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum InstallationStatus {
    #[serde(rename = "PARTIALLY_INSTALLED")]
    PartiallyInstalled,
    #[serde(rename = "NOT_INSTALLED")]
    NotInstalled,
}

pub fn is_aviso_installed(
    lfxx_path: &str,
    aviso_content: &str,
) -> Result<InstallationStatus, String> {
    let lfxx_content = fs::read_to_string(lfxx_path)
        .map_err(|e| format!("Failed to read LFXX file: {}", e))?;

    let aviso_geo_names = extract_geo_area_names(aviso_content);
    let aviso_region_names = extract_region_names(aviso_content);

    for name in aviso_geo_names.iter().chain(aviso_region_names.iter()) {
        if lfxx_content.contains(name) {
            return Ok(InstallationStatus::PartiallyInstalled);
        }
    }

    Ok(InstallationStatus::NotInstalled)
}

fn extract_geo_area_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    let mut in_geo = false;

    for line in content.lines() {
        let line = line.trim_end();

        if line.starts_with('[') {
            in_geo = line == "[GEO]";
            continue;
        }

        if in_geo && !line.trim().is_empty() {
            // Find start of coordinates: N or S followed by a digit
            let chars: Vec<char> = line.chars().collect();
            let mut coord_index = None;

            for i in 0..chars.len().saturating_sub(1) {
                if (chars[i] == 'N' || chars[i] == 'S') && chars[i + 1].is_ascii_digit() {
                    coord_index = Some(i);
                    break;
                }
            }

            if let Some(idx) = coord_index {
                let name = line[..idx].trim().to_string();
                if !name.is_empty() && seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }
    }

    names
}

fn extract_region_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    let mut in_regions = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            in_regions = trimmed == "[REGIONS]";
            continue;
        }

        if in_regions && trimmed.starts_with("REGIONNAME") {
            if let Some(name) = trimmed.strip_prefix("REGIONNAME") {
                let name = name.trim().to_string();
                if !name.is_empty() && seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }
    }

    names
}