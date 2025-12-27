use regex::Regex;
use std::fs;

struct EseCategory {
    name: String,
    content: String,
    header: String,
}

/// Installs the ESE content into the LFXX.ese file.
/// Parses the remote content for categories and merges them into the local file.
pub fn install_content(
    lfxx_ese_path: &str,
    remote_ese_content: &str,
    _category_hint: &str, // Unused as we trust remote content headers
) -> Result<(), String> {
    let lfxx_bytes =
        fs::read(lfxx_ese_path).map_err(|e| format!("Failed to read LFXX.ese: {}", e))?;
    let lfxx_content = String::from_utf8_lossy(&lfxx_bytes).to_string();

    let remote_categories = parse_ese_categories(remote_ese_content)?;
    let mut current_local_content = lfxx_content;

    for remote_cat in remote_categories {
        if let Some(replaced_content) = try_replace_category(&current_local_content, &remote_cat)? {
            current_local_content = replaced_content;
        } else {
            // Append
            if !current_local_content.ends_with('\n') {
                current_local_content.push('\n');
            }
            if !current_local_content.trim_end().is_empty() {
                current_local_content.push('\n');
            }
            current_local_content.push_str(&remote_cat.header);
            current_local_content.push('\n');
            current_local_content.push_str(remote_cat.content.trim());
            current_local_content.push('\n');
        }
    }

    fs::write(lfxx_ese_path, current_local_content.as_bytes())
        .map_err(|e| format!("Failed to write LFXX.ese: {}", e))?;

    Ok(())
}

fn parse_ese_categories(content: &str) -> Result<Vec<EseCategory>, String> {
    let mut categories = Vec::new();
    let re_header =
        Regex::new(r"(?m)^;=+\s*~~\s*(.*?)\s*~~\s*=+;\s*$").map_err(|e| e.to_string())?;

    // Find all matches to get positions
    let matches: Vec<_> = re_header.find_iter(content).collect();

    if matches.is_empty() {
        return Ok(categories);
    }

    for i in 0..matches.len() {
        let m = matches[i];
        let header_str = m.as_str().to_string();

        // Extract name safely
        let name = re_header
            .captures(&header_str)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default(); // Should not happen given find match, but safe

        let start_content = m.end();
        let end_content = if i + 1 < matches.len() {
            matches[i + 1].start()
        } else {
            content.len()
        };

        if start_content > content.len() || end_content > content.len() {
            continue; // Safety check
        }

        let body = &content[start_content..end_content];

        categories.push(EseCategory {
            name,
            content: body.to_string(),
            header: header_str,
        });
    }

    Ok(categories)
}

fn try_replace_category(
    local_content: &str,
    remote_cat: &EseCategory,
) -> Result<Option<String>, String> {
    let re_header =
        Regex::new(r"(?m)^;=+\s*~~\s*(.*?)\s*~~\s*=+;\s*$").map_err(|e| e.to_string())?;
    let matches: Vec<_> = re_header.find_iter(local_content).collect();

    for i in 0..matches.len() {
        let m = matches[i];
        let header_str = m.as_str();

        let local_name_raw = re_header
            .captures(header_str)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
            .unwrap_or("");

        // Check if remote_cat.name is inside local_name_raw (handle LFFF/LFPG/...)
        // We check for EXACT match first (e.g. "LFFF/LFPG" matching "LFFF/LFPG")
        // Then we check if it is a sub-part (e.g. "LFFF" matching "LFFF/LFPG")
        let is_match = local_name_raw == remote_cat.name
            || local_name_raw
                .split('/')
                .any(|part| part.trim() == remote_cat.name);

        if is_match {
            // Found the section to replace
            let start_replace = m.start(); // Start of header
                                           // End of section is start of next header OR end of file
            let end_replace = if i + 1 < matches.len() {
                matches[i + 1].start()
            } else {
                local_content.len()
            };

            // Reconstruct string
            let mut new_content =
                String::with_capacity(local_content.len() + remote_cat.content.len());
            new_content.push_str(&local_content[..start_replace]);

            // We use the LOCAL header to preserve other categories if mixed
            new_content.push_str(header_str);
            new_content.push('\n');
            new_content.push_str(remote_cat.content.trim());
            new_content.push('\n');

            new_content.push_str(&local_content[end_replace..]);

            return Ok(Some(new_content));
        }
    }

    Ok(None)
}
