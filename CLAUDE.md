# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Important Notes

- When modifying Rust code, always use the `ms-rust` skill first (mandatory)



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
4. Check if database empty - only load content if needed (fast startup optimization)
   - Load content from `./static/md` (markdown) and `./static/png` (images)
   - Populate FTS5 search table
5. Start Axum web server with session middleware

### Module Structure

**db/** - Database layer
- `connection.rs`: r2d2 connection pool setup
- `schema.rs`: Table initialization (flashcards + FTS5 virtual table)
- `models.rs`: `Flashcard` struct with category/subcategory support, `FilterCriteria` struct for filtering
- `queries.rs`: Database operations including FTS5 search, filtered queries (by category/subcategory/keywords/images)

**content/** - Content loading and processing
- `markdown.rs`: Scans `./static/md`, parses markdown with pulldown-cmark, extracts category/subcategory from YAML frontmatter, applies syntax highlighting with syntect
- `images.rs`: Scans `./static/png` for image-only flashcards

**routes/** - Web handlers (Axum)
- `landing.rs`: Filter form at `/` (keywords, categories, subcategories, images) + POST `/apply_filters`
- `practice.rs`: Display filtered flashcard at `/practice` (avoids recently seen via session)
- `debug.rs`: Session reset utility at `/reset_session`

**session/** - Session management
- Uses tower-sessions with in-memory store
- Tracks: `seen_ids`, filter state (`filter_keywords`, `filter_categories`, `filter_subcategories`, `filter_include_images`), cached counts, error messages

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



## User Flow

1. User lands on filter form at `/` (landing page)
2. User selects filters: keywords, categories, subcategories, images
3. User clicks "Practice" or presses ENTER → POST to `/apply_filters` → redirects to `/practice`
4. `/practice` displays random flashcard matching filters (avoids recently seen)
5. User clicks "Next card" → loads another card from `/practice`
6. User clicks "Back to Filters" → returns to `/` with filters preserved in session
7. If no cards match filters → redirects to `/` with error message displayed

## Notes
- Content loaded only on first startup (database empty check for fast subsequent starts)
- FTS table must be repopulated after content changes via `db::queries::populate_fts_table()`
- Session data is in-memory only (resets on server restart)
- Filter state persists in session across page visits
- Category/subcategory extraction uses regex: `(?i)category:\s*([^\n\r]+)` and `(?i)subcategory:\s*([^\n\r]+)`
- Keyword search scoped to selected categories/subcategories (FTS5 with category/subcategory filtering)
