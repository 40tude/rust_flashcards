use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

use crate::db::connection::DbPool;
use crate::db::queries;

pub fn load_images(pool: &DbPool, png_dir: &str) -> Result<()> {
    tracing::info!("Loading image flashcards from {}", png_dir);

    let mut count = 0;

    // Walk through all .png and .webp files recursively
    for entry in WalkDir::new(png_dir).follow_links(true).into_iter().filter_map(|e| e.ok()).filter(|e| {
        e.path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                let ext_lower = ext.to_ascii_lowercase();
                ext_lower == "png" || ext_lower == "webp"
            })
            .unwrap_or(false)
    }) {
        let path = entry.path();
        tracing::debug!("Processing image file: {:?}", path);

        match process_image_file(pool, path, png_dir) {
            Ok(()) => {
                count += 1;
                tracing::debug!("Loaded image flashcard from {:?}", path);
            }
            Err(e) => {
                tracing::warn!("Failed to process {:?}: {}", path, e);
            }
        }
    }

    tracing::info!("Loaded {} image flashcards", count);
    Ok(())
}

fn process_image_file(pool: &DbPool, path: &Path, base_dir: &str) -> Result<()> {
    // Convert absolute path to relative path from static/img/
    let relative_path = path.strip_prefix(base_dir).unwrap_or(path).to_string_lossy().replace('\\', "/");

    // Question is empty (just the header as HTML)
    let question_html = "<h3>Question :</h3>\n".to_string();

    // Answer contains the image with Bootstrap class, centered
    let answer_html = format!("<h3>Answer :</h3>\n<p align=\"center\"><img src='/static/img/{}' class='img-fluid'></p>", relative_path);

    // Insert into database - Images: category and subcategory = None
    queries::insert_flashcard(pool, None, None, &question_html, &answer_html)?;

    Ok(())
}
