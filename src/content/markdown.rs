use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use walkdir::WalkDir;

use crate::db::connection::DbPool;
use crate::db::queries;

pub fn load_markdown(pool: &DbPool, md_dir: &str) -> Result<()> {
    tracing::info!("Loading markdown files from {}", md_dir);

    // Clear existing flashcards
    queries::clear_flashcards(pool)?;

    let mut count = 0;

    // Walk through all .md files recursively
    for entry in WalkDir::new(md_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
    {
        let path = entry.path();
        tracing::debug!("Processing markdown file: {:?}", path);

        match process_markdown_file(pool, path) {
            Ok(n) => {
                count += n;
                tracing::debug!("Loaded {} flashcards from {:?}", n, path);
            }
            Err(e) => {
                tracing::warn!("Failed to process {:?}: {:?}", path, e);
            }
        }
    }

    tracing::info!("Loaded {} flashcards from markdown files", count);
    Ok(())
}

fn process_markdown_file(pool: &DbPool, path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    // Strip HTML comments (with DOTALL for multiline comments)
    let comment_regex = Regex::new(r"(?s)<!--.*?-->").unwrap();
    let cleaned = comment_regex.replace_all(&content, "");

    // Split by "Question" keyword to get individual Q&A blocks
    let parts: Vec<&str> = cleaned.split("Question").collect();

    let mut count = 0;

    // Regex to extract CATEGORY - SUBCATEGORY - QUESTION
    let category_regex = Regex::new(r"^\s*:\s*([^-]+?)\s-\s([^-]+?)\s-\s(.+)").unwrap();

    for part in parts.iter().skip(1) {
        // Each part should start with " : " followed by question text,
        // then contain "Answer  :" (with variable spaces) followed by answer text

        // Find where Answer starts
        let answer_re = Regex::new(r"\nAnswer\s+:").unwrap();
        if let Some(answer_match) = answer_re.find(part) {
            let question_part = part[..answer_match.start()].trim();
            let answer_md = part[answer_match.end()..].trim();

            // Extract category, subcategory, and question
            let (category, subcategory, question_md) = if let Some(caps) = category_regex.captures(question_part) {
                (
                    Some(caps.get(1).unwrap().as_str().trim().to_string()),
                    Some(caps.get(2).unwrap().as_str().trim().to_string()),
                    caps.get(3).unwrap().as_str().trim()
                )
            } else {
                // Question non-conforme: catégorie = None
                tracing::warn!("Non-compliant question format in {:?}: {}", path, question_part);
                (None, None, question_part)
            };

            // Skip empty Q&A pairs
            if question_md.is_empty() && answer_md.is_empty() {
                continue;
            }

            // Prepend headers to markdown BEFORE conversion
            let question_with_header = format!("### Question :\n{}", question_md);
            let answer_with_header = format!("### Answer :\n{}", answer_md);

            // Convert markdown to HTML with syntax highlighting
            let q_html = markdown_to_html(&question_with_header)?;
            let a_html = markdown_to_html(&answer_with_header)?;

            // Insert into database with category and subcategory
            queries::insert_flashcard(
                pool,
                category.as_deref(),
                subcategory.as_deref(),
                &q_html,
                &a_html
            )?;
            count += 1;
        }
    }

    Ok(count)
}

fn markdown_to_html(markdown: &str) -> Result<String> {
    // Enable markdown extensions to match Python's "extra" extension
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);

    // Load syntax highlighting
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];

    // Collect events and apply syntax highlighting to code blocks
    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    for event in parser {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(
                pulldown_cmark::CodeBlockKind::Fenced(lang),
            )) => {
                in_code_block = true;
                code_block_lang = lang.to_string();
                code_block_content.clear();
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                if in_code_block {
                    // Apply syntax highlighting
                    let highlighted = highlight_code(&code_block_content, &code_block_lang, &ss, theme);
                    events.push(pulldown_cmark::Event::Html(highlighted.into()));
                    in_code_block = false;
                }
            }
            pulldown_cmark::Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    events.push(pulldown_cmark::Event::Text(text));
                }
            }
            _ => {
                if !in_code_block {
                    events.push(event);
                }
            }
        }
    }

    // Convert to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    Ok(html_output)
}

fn highlight_code(code: &str, lang: &str, ss: &SyntaxSet, theme: &syntect::highlighting::Theme) -> String {
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut html = String::from("<pre><code>");

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter.highlight_line(line, ss).unwrap_or_default();
        let escaped = syntect::html::styled_line_to_highlighted_html(&ranges, syntect::html::IncludeBackground::No).unwrap_or_default();
        html.push_str(&escaped);
    }

    html.push_str("</code></pre>");
    html
}