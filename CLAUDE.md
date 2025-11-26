# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Important Notes

- When modifying Rust code, always use the `ms-rust` skill first (mandatory)
- In all interactions and commit messages, be extremely concise and sacrifice grammar for the sake of concision.
- All documents, plan, comments is source code, commit messages... must be written in English US. In case of doubt, ask for confirmation before to write anything in an other language.



## Project Overview

This is a Rust-based flashcard web application translated from Python. It serves flashcards written in markdown with support for images, math formulas, code syntax highlighting, and full-text search. The app uses Axum web framework, SQLite with FTS5 for search, and deploys to Heroku.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run locally (http://localhost:8080/)
cargo run

# Build for production (Heroku deployment)
cargo build --release

# Stop the running process (Windows PowerShell)
powershell -Command "Stop-Process -Name rust-flashcards -Force"
```

## Deployment

- **Heroku deployment:** `git push heroku main`
- **Production URL:** https://rust-flashcards-ae94334b8997.herokuapp.com/
- **Procfile:** Runs `./target/release/rust-flashcards`
- The `.slugignore` file controls which files are excluded from Heroku deployment

## Architecture Overview

### Application Startup Flow (src/main.rs)
1. Load environment variables from `.env` (PORT, DATABASE_URL)
2. Create SQLite connection pool (r2d2)
3. Initialize database schema (flashcards + flashcards_fts tables)
4. Load content from `./static/md` (markdown) and `./static/png` (images)
5. Populate FTS5 search table
6. Start Axum web server with session middleware

### Module Structure

**db/** - Database layer
- `connection.rs`: r2d2 connection pool setup
- `schema.rs`: Table initialization (flashcards + FTS5 virtual table)
- `models.rs`: `Flashcard` struct with category/subcategory support
- `queries.rs`: Database operations including FTS5 search

**content/** - Content loading and processing
- `markdown.rs`: Scans `./static/md`, parses markdown with pulldown-cmark, extracts category/subcategory from YAML frontmatter, applies syntax highlighting with syntect
- `images.rs`: Scans `./static/png` for image-only flashcards

**routes/** - Web handlers (Axum)
- `index.rs`: Landing page with category/subcategory selection
- `next.rs`: Display next random flashcard (avoids recently seen via session)
- `search.rs`: Search form and submission handler
- `search_results.rs`: Display FTS5 search results
- `debug.rs`: Session reset utility

**session/** - Session management
- Uses tower-sessions with in-memory store
- Tracks: `seen_ids`, `searched_ids`, `keywords`, `nb_cards`

### Database Schema

**flashcards table:**
- `id` (PRIMARY KEY)
- `category` (TEXT, nullable) - Extracted from markdown frontmatter
- `subcategory` (TEXT, nullable) - Extracted from markdown frontmatter
- `question_html` (TEXT) - Rendered HTML
- `answer_html` (TEXT) - Rendered HTML

**flashcards_fts table (FTS5):**
- Virtual table mirroring flashcards for full-text search
- Populated after content loading via `populate_fts_table()`

### Markdown File Format

Cards are extracted from markdown using regex pattern:
```
### Q
[question content]

### A
[answer content]
```

Category/subcategory are parsed from YAML frontmatter or filename patterns like `01_category_subcategory.md`.

### Key Dependencies

- **axum 0.7**: Web framework
- **rusqlite 0.31**: SQLite with FTS5 support
- **tower-sessions 0.12**: Session management
- **askama 0.12**: HTML templating
- **pulldown-cmark 0.11**: Markdown parsing
- **syntect 5.2**: Syntax highlighting for code blocks

## Development Context

- **Platform:** Windows 11, PowerShell, VSCode
- The project was translated from a Python version using Claude Code
- Multi-phase development plan documents are in `assets/` directory
- Content files: `./static/md/*.md` and `./static/png/*.png`
- Templates: `./templates/*.html` (Askama templates)



## Notes
- The FTS table must be repopulated after content changes via `db::queries::populate_fts_table()`
- Session data is in-memory only (resets on server restart)
- Category/subcategory extraction uses regex: `(?i)category:\s*([^\n\r]+)` and `(?i)subcategory:\s*([^\n\r]+)`
