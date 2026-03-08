use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;

use crate::app::FsEntry;

pub fn read_dir(path: &Path, show_hidden: bool) -> Result<Vec<FsEntry>, String> {
    let read = fs::read_dir(path).map_err(|e| format!("CANNOT READ DIRECTORY: {}", e))?;
    let mut entries: Vec<FsEntry> = Vec::new();

    for entry in read {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let (is_dir, size, modified, permissions) = match &metadata {
            Ok(m) => {
                let is_dir = m.is_dir();
                let size = if is_dir { None } else { Some(m.len()) };
                let modified = m.modified().ok();
                let perms = format_permissions(m.permissions().mode(), is_dir);
                (is_dir, size, modified, Some(perms))
            }
            Err(_) => (false, None, None, None),
        };

        entries.push(FsEntry {
            name,
            path: entry.path(),
            is_dir,
            size,
            modified,
            permissions,
            depth: 0,
        });
    }

    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}

/// Collect entries from subdirectories up to `max_depth` levels deep.
/// Returns entries with depth set relative to the starting directory.
pub fn read_dir_recursive(
    path: &Path,
    show_hidden: bool,
    max_depth: u32,
    base_path: &Path,
) -> Vec<FsEntry> {
    let mut result = Vec::new();
    collect_recursive(path, show_hidden, max_depth, 0, base_path, &mut result);
    result
}

fn collect_recursive(
    path: &Path,
    show_hidden: bool,
    max_depth: u32,
    current_depth: u32,
    base_path: &Path,
    result: &mut Vec<FsEntry>,
) {
    if current_depth > max_depth {
        return;
    }
    let read = match fs::read_dir(path) {
        Ok(r) => r,
        Err(_) => return,
    };

    for entry in read {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let (is_dir, size, modified, permissions) = match &metadata {
            Ok(m) => {
                let is_dir = m.is_dir();
                let size = if is_dir { None } else { Some(m.len()) };
                let modified = m.modified().ok();
                let perms = format_permissions(m.permissions().mode(), is_dir);
                (is_dir, size, modified, Some(perms))
            }
            Err(_) => (false, None, None, None),
        };

        let entry_path = entry.path();
        // Build display name relative to base
        let relative = entry_path
            .strip_prefix(base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| name.clone());

        let depth = current_depth as i32;

        result.push(FsEntry {
            name: relative,
            path: entry_path.clone(),
            is_dir,
            size,
            modified,
            permissions,
            depth,
        });

        if is_dir && current_depth < max_depth {
            collect_recursive(
                &entry_path,
                show_hidden,
                max_depth,
                current_depth + 1,
                base_path,
                result,
            );
        }
    }
}

/// Collect entries from the parent directory (depth -1).
pub fn read_parent_entries(path: &Path, show_hidden: bool) -> Vec<FsEntry> {
    let parent = match path.parent() {
        Some(p) => p,
        None => return Vec::new(),
    };
    let read = match fs::read_dir(parent) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    let mut entries = Vec::new();
    for entry in read {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_hidden && name.starts_with('.') {
            continue;
        }
        // Skip the current directory itself
        let entry_path = entry.path();
        if entry_path.canonicalize().ok() == path.canonicalize().ok() {
            continue;
        }

        let metadata = entry.metadata();
        let (is_dir, size, modified, permissions) = match &metadata {
            Ok(m) => {
                let is_dir = m.is_dir();
                let size = if is_dir { None } else { Some(m.len()) };
                let modified = m.modified().ok();
                let perms = format_permissions(m.permissions().mode(), is_dir);
                (is_dir, size, modified, Some(perms))
            }
            Err(_) => (false, None, None, None),
        };

        let display = format!("../{}", name);
        entries.push(FsEntry {
            name: display,
            path: entry_path,
            is_dir,
            size,
            modified,
            permissions,
            depth: -1,
        });
    }
    entries
}

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{} KB", bytes / 1024)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn format_permissions(mode: u32, is_dir: bool) -> String {
    let mut s = String::with_capacity(10);
    s.push(if is_dir { 'd' } else { '-' });
    for shift in [6, 3, 0] {
        let bits = (mode >> shift) & 0o7;
        s.push(if bits & 4 != 0 { 'r' } else { '-' });
        s.push(if bits & 2 != 0 { 'w' } else { '-' });
        s.push(if bits & 1 != 0 { 'x' } else { '-' });
    }
    s
}

pub fn format_modified(time: SystemTime) -> String {
    let duration = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => return "--.--.".to_string(),
    };
    let total_secs = duration.as_secs();
    // Convert unix timestamp to month.day using a simple approach
    let total_days = (total_secs / 86400) as i64;

    // Days from epoch (1970-01-01) to approximate date
    let mut year: i64 = 1970;
    let mut remaining = total_days;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let leap = is_leap(year);
    let month_days: [i64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut month: i64 = 1;
    for &md in &month_days {
        if remaining < md {
            break;
        }
        remaining -= md;
        month += 1;
    }
    let day = remaining + 1;
    format!("{:02}.{:02}", month, day)
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn type_badge_str(entry: &FsEntry) -> String {
    if entry.is_dir {
        "DIR".to_string()
    } else {
        // Use the basename for extension detection
        let basename = entry
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| entry.name.clone());
        basename
            .rsplit('.')
            .next()
            .filter(|ext| ext.len() <= 4 && *ext != basename)
            .map(|ext| ext.to_uppercase())
            .unwrap_or_else(|| "FILE".to_string())
    }
}
