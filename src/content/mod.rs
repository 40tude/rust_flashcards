pub mod markdown;
pub mod images;

pub use markdown::load_markdown;
pub use images::load_images;

use std::path::Path;

/// Status of a content directory.
///
/// Represents the validation state of a content directory, indicating
/// whether it can be used for loading flashcard content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentDirStatus {
    /// Directory exists, is readable, and is a valid directory.
    Valid,
    /// Path does not exist.
    Missing,
    /// Path exists but is not a directory (e.g., a file).
    NotADirectory,
    /// Path exists and is a directory but cannot be read (permission denied).
    PermissionDenied,
}

/// Validates a content directory for flashcard loading.
///
/// Checks if the directory exists, is actually a directory (not a file),
/// and has read permissions. Returns the validation status.
///
/// # Examples
///
/// ```rust,ignore
/// let status = validate_content_directory("./static/md");
/// match status {
///     ContentDirStatus::Valid => println!("Directory is ready"),
///     ContentDirStatus::Missing => println!("Directory doesn't exist"),
///     _ => println!("Directory has issues"),
/// }
/// ```
pub fn validate_content_directory(path: &str) -> ContentDirStatus {
    let dir_path = Path::new(path);

    // Check if path exists
    if !dir_path.exists() {
        return ContentDirStatus::Missing;
    }

    // Check if it's actually a directory
    if !dir_path.is_dir() {
        return ContentDirStatus::NotADirectory;
    }

    // Try to read the directory to check permissions
    match std::fs::read_dir(dir_path) {
        Ok(_) => ContentDirStatus::Valid,
        Err(_) => ContentDirStatus::PermissionDenied,
    }
}

// Rust guideline compliant 2025-01-27