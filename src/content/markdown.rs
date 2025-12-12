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
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
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
    let content = fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))?;

    // Compile all regex patterns once (performance optimization)
    // Strip HTML comments (with DOTALL for multiline comments)
    let comment_regex = Regex::new(r"(?s)<!--.*?-->").unwrap();

    // Split by "Question :" at line start (case-insensitive, multiline mode)
    // (?m) enables multiline mode where ^ matches line start
    // (?i) enables case-insensitive matching
    // ^\s* allows optional leading whitespace before Question
    let question_regex = Regex::new(r"(?mi)^\s*Question\s+:").unwrap();

    // Find where Answer starts (case-insensitive, multiline mode, line-anchored)
    let answer_regex = Regex::new(r"(?mi)^\s*Answer\s+:").unwrap();

    // Regex to extract CATEGORY - SUBCATEGORY - QUESTION
    // Use lookahead to match " - " (space-dash-space) to allow hyphens in category names
    // Note: No leading ":" since question_regex.split() removes "Question :" entirely
    let category_regex = Regex::new(r"^\s*(.+?)\s-\s(.+?)\s-\s(.+)").unwrap();

    let cleaned = comment_regex.replace_all(&content, "");

    // Split by "Question :" keyword to get individual Q&A blocks
    let parts: Vec<&str> = question_regex.split(&cleaned).collect();

    let mut count = 0;

    for part in parts.iter().skip(1) {
        // Each part should start with " : " followed by question text,
        // then contain "Answer  :" (with variable spaces) followed by answer text

        // Find where Answer starts
        if let Some(answer_match) = answer_regex.find(part) {
            let question_part = part[..answer_match.start()].trim();
            let answer_md = part[answer_match.end()..].trim();

            // Extract category, subcategory, and question
            let (category, subcategory, question_md) = if let Some(caps) = category_regex.captures(question_part) {
                (
                    Some(caps.get(1).unwrap().as_str().trim().to_string()),
                    Some(caps.get(2).unwrap().as_str().trim().to_string()),
                    caps.get(3).unwrap().as_str().trim(),
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
            queries::insert_flashcard(pool, category.as_deref(), subcategory.as_deref(), &q_html, &a_html)?;
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
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
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
    let syntax = ss.find_syntax_by_token(lang).unwrap_or_else(|| ss.find_syntax_plain_text());

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;
    use tempfile::TempDir;

    /// Creates temporary directory with markdown files for testing.
    fn setup_test_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    /// Creates in-memory database for markdown loading tests.
    fn setup_test_db() -> DbPool {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();

        let conn = pool.get().unwrap();
        conn.execute(
            "CREATE TABLE flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT,
                subcategory TEXT,
                question_html TEXT NOT NULL,
                answer_html TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE VIRTUAL TABLE flashcards_fts USING fts5(
                id UNINDEXED,
                category,
                subcategory,
                question_html,
                answer_html
            )",
            [],
        )
        .unwrap();

        pool
    }

    // ========== Tests for markdown_to_html ==========

    #[test]
    fn test_markdown_to_html_basic_formatting() {
        let markdown = "**bold** *italic* ~~strikethrough~~";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<del>strikethrough</del>"));
    }

    #[test]
    fn test_markdown_to_html_headers() {
        let markdown = "### Header 3\n\n#### Header 4";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<h3>Header 3</h3>"));
        assert!(html.contains("<h4>Header 4</h4>"));
    }

    #[test]
    fn test_markdown_to_html_code_inline() {
        let markdown = "Use `println!()` macro";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<code>println!()</code>"));
    }

    #[test]
    fn test_markdown_to_html_code_block_with_syntax_highlighting() {
        let markdown = r#"```rust
fn main() {
    println!("Hello");
}
```"#;
        let html = markdown_to_html(markdown).unwrap();

        // Should contain pre/code tags from syntax highlighting
        assert!(html.contains("<pre><code>"));
        assert!(html.contains("</code></pre>"));
        // Should contain the code content (may be HTML-escaped or styled)
        assert!(html.contains("main") || html.contains("&quot;Hello&quot;"));
    }

    #[test]
    fn test_markdown_to_html_table_support() {
        let markdown = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |"#;
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header 1</th>"));
        assert!(html.contains("<td>Cell 1</td>"));
    }

    #[test]
    fn test_markdown_to_html_links() {
        let markdown = "[Rust](https://rust-lang.org)";
        let html = markdown_to_html(markdown).unwrap();

        assert!(html.contains(r#"<a href="https://rust-lang.org">Rust</a>"#));
    }

    #[test]
    fn test_markdown_to_html_empty_input() {
        let html = markdown_to_html("").unwrap();
        // Empty markdown should produce minimal HTML
        assert!(html.is_empty() || html == "\n");
    }

    // ========== Parametrized Tests for process_markdown_file ==========

    #[rstest]
    #[case(
        "Question : Math - Algebra - What is 2+2?\nAnswer : 4",
        1,
        Some("Math"),
        Some("Algebra")
    )]
    #[case(
        "Question : Science - Physics - What is gravity?\nAnswer  : Force",
        1,
        Some("Science"),
        Some("Physics")
    )]
    #[case(
        "Question : What is the capital?\nAnswer : Paris",
        1,
        None,
        None
    )]
    #[case(
        "Question : Machine-Learning - Supervised - What is a neural network?\nAnswer : Computational model",
        1,
        Some("Machine-Learning"),
        Some("Supervised")
    )]
    #[case(
        "<!-- Comment -->\nQuestion : Math - Algebra - Q?\nAnswer : A",
        1,
        Some("Math"),
        Some("Algebra")
    )]
    #[case(
        "Question : Cat1 - Sub1 - Q1\nAnswer : A1\n\nQuestion : Cat2 - Sub2 - Q2\nAnswer : A2",
        2,
        Some("Cat1"),
        Some("Sub1")
    )]
    fn test_process_markdown_file(
        #[case] content: &str,
        #[case] expected_count: usize,
        #[case] expected_first_category: Option<&str>,
        #[case] expected_first_subcategory: Option<&str>,
    ) {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("test.md");

        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, expected_count);

        if expected_count > 0 {
            // Verify first card's category/subcategory
            let conn = pool.get().unwrap();
            let (cat, subcat): (Option<String>, Option<String>) = conn
                .query_row(
                    "SELECT category, subcategory FROM flashcards WHERE id = 1",
                    [],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap();

            assert_eq!(cat.as_deref(), expected_first_category);
            assert_eq!(subcat.as_deref(), expected_first_subcategory);
        }
    }

    #[test]
    fn test_process_markdown_file_skips_empty_qa() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("empty.md");

        // Empty Q&A followed by valid Q&A
        let content = "Question : \nAnswer : \n\nQuestion : Valid - Question - Q\nAnswer : A";
        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        // First Q&A has both question and answer empty (after trim), so it's skipped
        // Second Q&A is valid, so only 1 card is created
        assert_eq!(count, 1);
    }

    #[test]
    fn test_process_markdown_file_strips_html_comments() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("comments.md");

        let content = r#"<!-- This is a comment
spanning multiple lines -->
Question : Cat - Sub - Q
Answer : A"#;
        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, 1);

        // Verify HTML doesn't contain comment
        let conn = pool.get().unwrap();
        let html: String = conn
            .query_row(
                "SELECT question_html FROM flashcards WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(!html.contains("<!--"));
        assert!(!html.contains("comment"));
    }

    #[test]
    fn test_process_markdown_file_handles_multiple_spaces_in_answer() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("spaces.md");

        // "Answer  :" with multiple spaces
        let content = "Question : Cat - Sub - Q\nAnswer  : A";
        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_process_markdown_file_handles_hyphens_in_category_name() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("hyphen.md");

        let content = "Question : Machine-Learning - Deep-Learning - Q?\nAnswer : A";
        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, 1);

        let conn = pool.get().unwrap();
        let (cat, subcat): (Option<String>, Option<String>) = conn
            .query_row(
                "SELECT category, subcategory FROM flashcards WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        assert_eq!(cat.as_deref(), Some("Machine-Learning"));
        assert_eq!(subcat.as_deref(), Some("Deep-Learning"));
    }

    // ========== Tests for Keywords in Text (Bug Fix) ==========

    #[rstest]
    #[case(
        "Question : Culture - Hitchhiker - The Answer to Life is?\nAnswer : 42",
        1,
        Some("Culture"),
        "Answer word in question"
    )]
    #[case(
        "Question : Meta - Testing - This Question contains Question\nAnswer : Parsed",
        1,
        Some("Meta"),
        "Question word in question"
    )]
    #[case(
        "Question : Grammar - Usage - Define\nAnswer : An answer is a response to a question",
        1,
        Some("Grammar"),
        "answer/question words in answer text"
    )]
    #[case(
        "question : Test - Case - Lowercase\nanswer : Works",
        1,
        Some("Test"),
        "Lowercase keywords"
    )]
    #[case(
        "QUESTION : Test - Case - Uppercase\nANSWER : Works",
        1,
        Some("Test"),
        "Uppercase keywords"
    )]
    #[case(
        "Question : Test - Multi - Line\nAnswer this\nAnswer : Correct",
        1,
        Some("Test"),
        "Answer at line start in question"
    )]
    #[case(
        "   Question : Test - Space - Indented\n   Answer : Works",
        1,
        Some("Test"),
        "Leading whitespace before keywords"
    )]
    #[case(
        "Not a Question : fake\nQuestion : Real - Start - Q\nAnswer : A",
        1,
        Some("Real"),
        "Keywords mid-line ignored"
    )]
    fn test_process_markdown_file_keywords_in_text(
        #[case] content: &str,
        #[case] expected_count: usize,
        #[case] expected_category: Option<&str>,
        #[case] description: &str,
    ) {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("test.md");

        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, expected_count, "Failed for case: {}", description);

        if expected_count > 0 {
            let conn = pool.get().unwrap();
            let cat: Option<String> = conn
                .query_row(
                    "SELECT category FROM flashcards WHERE id = 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap();

            assert_eq!(
                cat.as_deref(),
                expected_category,
                "Category mismatch for: {}",
                description
            );
        }
    }

    #[test]
    fn test_exact_hitchhiker_case() {
        // Test the EXACT failing case from user's markdown
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("hitchhiker.md");

        let content = r#"Question : Culture - The Hitchhiker's Guide to the Galaxy - The Answer to the Ultimate Question of Life, the Universe, and Everything is?
Answer  :

42"#;

        fs::write(&file_path, content).unwrap();

        let count = process_markdown_file(&pool, &file_path).unwrap();
        assert_eq!(count, 1, "Should parse Hitchhiker's Guide question");

        let conn = pool.get().unwrap();
        let (cat, subcat, q_html): (Option<String>, Option<String>, String) = conn
            .query_row(
                "SELECT category, subcategory, question_html FROM flashcards WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(cat, Some("Culture".to_string()));
        assert_eq!(
            subcat,
            Some("The Hitchhiker's Guide to the Galaxy".to_string())
        );
        // Question should contain "The Answer to" phrase
        assert!(
            q_html.contains("The Answer to"),
            "Question text should be preserved"
        );
    }

    #[test]
    fn test_question_regex_pattern() {
        let question_regex = Regex::new(r"(?mi)^\s*Question\s+:").unwrap();

        // Should match
        assert!(question_regex.is_match("Question : text"));
        assert!(question_regex.is_match("question : text"));
        assert!(question_regex.is_match("QUESTION : text"));
        assert!(question_regex.is_match("  Question : text"));
        assert!(question_regex.is_match("\nQuestion : text"));

        // Should NOT match
        assert!(!question_regex.is_match("Not Question : text"));
        assert!(!question_regex.is_match("The Question : text"));
    }

    #[test]
    fn test_answer_regex_pattern() {
        let answer_regex = Regex::new(r"(?mi)^\s*Answer\s+:").unwrap();

        // Should match
        assert!(answer_regex.is_match("Answer : text"));
        assert!(answer_regex.is_match("answer : text"));
        assert!(answer_regex.is_match("ANSWER : text"));
        assert!(answer_regex.is_match("  Answer : text"));
        assert!(answer_regex.is_match("\nAnswer : text"));

        // Should NOT match
        assert!(!answer_regex.is_match("The Answer : text"));
        assert!(!answer_regex.is_match("Not Answer : text"));
    }

    #[test]
    fn test_process_markdown_file_adds_headers_to_html() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("headers.md");

        let content = "Question : Cat - Sub - What is 2+2?\nAnswer : 4";
        fs::write(&file_path, content).unwrap();

        process_markdown_file(&pool, &file_path).unwrap();

        let conn = pool.get().unwrap();
        let (q_html, a_html): (String, String) = conn
            .query_row(
                "SELECT question_html, answer_html FROM flashcards WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        // Should contain "Question :" and "Answer :" headers
        assert!(q_html.contains("Question :"));
        assert!(a_html.contains("Answer :"));
    }

    // ========== Tests for load_markdown ==========

    #[test]
    fn test_load_markdown_single_file() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        let content = "Question : Math - Algebra - Q?\nAnswer : A";
        fs::write(temp_dir.path().join("test.md"), content).unwrap();

        load_markdown(&pool, temp_dir.path().to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_load_markdown_multiple_files() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        fs::write(
            temp_dir.path().join("math.md"),
            "Question : Math - Algebra - Q1\nAnswer : A1",
        )
        .unwrap();

        fs::write(
            temp_dir.path().join("science.md"),
            "Question : Science - Physics - Q2\nAnswer : A2",
        )
        .unwrap();

        load_markdown(&pool, temp_dir.path().to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_load_markdown_recursive_directories() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        // Create nested directory structure
        let nested_dir = temp_dir.path().join("subdir");
        fs::create_dir(&nested_dir).unwrap();

        fs::write(
            temp_dir.path().join("root.md"),
            "Question : Cat1 - Sub1 - Q1\nAnswer : A1",
        )
        .unwrap();

        fs::write(
            nested_dir.join("nested.md"),
            "Question : Cat2 - Sub2 - Q2\nAnswer : A2",
        )
        .unwrap();

        load_markdown(&pool, temp_dir.path().to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should find both files (root and nested)
        assert_eq!(count, 2);
    }

    #[test]
    fn test_load_markdown_skips_non_md_files() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        fs::write(
            temp_dir.path().join("valid.md"),
            "Question : Cat - Sub - Q\nAnswer : A",
        )
        .unwrap();

        fs::write(temp_dir.path().join("readme.txt"), "This is a text file")
            .unwrap();

        load_markdown(&pool, temp_dir.path().to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should only load .md file, skip .txt
        assert_eq!(count, 1);
    }

    #[test]
    fn test_load_markdown_clears_existing_flashcards() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        // Insert initial flashcard
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO flashcards (category, subcategory, question_html, answer_html)
             VALUES ('Old', 'Old', '<p>Old Q</p>', '<p>Old A</p>')",
            [],
        )
        .unwrap();
        drop(conn);

        // Load new flashcards
        fs::write(
            temp_dir.path().join("new.md"),
            "Question : New - New - Q\nAnswer : A",
        )
        .unwrap();

        load_markdown(&pool, temp_dir.path().to_str().unwrap()).unwrap();

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should only have new flashcard (old one cleared)
        assert_eq!(count, 1);

        let category: Option<String> = conn
            .query_row(
                "SELECT category FROM flashcards ORDER BY id LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(category, Some("New".to_string()));
    }

    #[test]
    fn test_load_markdown_continues_on_file_error() {
        let pool = setup_test_db();
        let temp_dir = setup_test_dir();

        // Valid file
        fs::write(
            temp_dir.path().join("valid.md"),
            "Question : Cat - Sub - Q\nAnswer : A",
        )
        .unwrap();

        // Invalid file (missing Answer section)
        fs::write(
            temp_dir.path().join("invalid.md"),
            "Question : Cat - Sub - Q without answer",
        )
        .unwrap();

        // Should not panic, should continue processing
        let result = load_markdown(&pool, temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());

        let conn = pool.get().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM flashcards", [], |row| row.get(0))
            .unwrap();

        // Should have loaded at least the valid file
        assert!(count >= 1);
    }
}

