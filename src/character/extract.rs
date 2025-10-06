use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn extract_characters(path: &Path, language_id: &str) -> String {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    let mut lines = content.lines();
    let header_line = match lines.next() {
        Some(h) => h,
        None => return String::new(),
    };

    let headers: Vec<&str> = header_line.split(';').collect();
    let col_index = match find_column_index(&headers, language_id) {
        Some(idx) => idx,
        None => return String::new(),
    };

    extract_unique_chars(lines, col_index)
}

pub fn extract_characters_from_folder(folder_path: &Path, language_id: &str) -> String {
    let mut global_seen = HashSet::new();
    let mut result = String::new();

    let entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(_) => return String::new(),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let chars = extract_characters(&path, language_id);

            for c in chars.chars() {
                if global_seen.insert(c) {
                    result.push(c);
                }
            }
        }
    }

    result
}

fn find_column_index(headers: &[&str], language_id: &str) -> Option<usize> {
    if let Ok(idx) = language_id.parse::<usize>() {
        if idx < headers.len() {
            return Some(idx);
        }
    }

    for i in 0..headers.len() {
        if headers[i].trim() == language_id {
            return Some(i);
        }
    }

    None
}

fn extract_unique_chars<'a>(lines: impl Iterator<Item = &'a str>, col_index: usize) -> String {
    let mut seen = HashSet::new();
    let mut result = String::new();

    for line in lines {
        let cols: Vec<&str> = line.split(';').collect();
        if col_index < cols.len() {
            for c in cols[col_index].chars() {
                if seen.insert(c) {
                    result.push(c);
                }
            }
        }
    }

    result
}
