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

# Rebuild database from content files (default deck)
cargo run -- --rebuild-deck deck
cargo run -- -r deck

# Load specific deck
cargo run -- --deck rust --deck-name "Rust Flashcards"

# Rebuild and load specific deck
cargo run -- --rebuild-deck test --deck test --deck-name "Test Deck"
cargo run -- -r test -d test -n "Test Deck"

# Show help
cargo run -- --help

# Show version
cargo run -- --version

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

### Multi-Deck Heroku Deployment

To deploy multiple decks as separate apps:
```bash
# Create apps
heroku create rust-flashcards-deck1
heroku create rust-flashcards-deck2

# Configure deck for each app
heroku config:set DECK_ID=deck DECK_DISPLAY_NAME="Default Deck" -a rust-flashcards-deck1
heroku config:set DECK_ID=rust DECK_DISPLAY_NAME="Rust Flashcards" -a rust-flashcards-deck2

# Deploy same codebase to both
git remote add heroku-deck1 https://git.heroku.com/rust-flashcards-deck1.git
git remote add heroku-deck2 https://git.heroku.com/rust-flashcards-deck2.git
git push heroku-deck1 main
git push heroku-deck2 main
```

## Architecture Overview

### Application Startup Flow (src/main.rs)
1. Parse CLI arguments (--rebuild-deck, --deck, --deck-name, --help, --version)
2. Load environment variables from `.env` (PORT, DECK_ID, DECK_DISPLAY_NAME, DATABASE_URL)
3. Resolve deck configuration (CLI args > Env vars > Defaults)
4. Handle database rebuild if `--rebuild-deck` flag present (delete deck-specific DB file)
5. Create SQLite connection pool (r2d2)
6. Initialize database schema (flashcards + flashcards_fts tables)
7. Check if database empty - only load content if needed (fast startup optimization)
   - Load content from `./static/{deck_id}/md` and `./static/{deck_id}/img`
   - Populate FTS5 search table
8. Start Axum web server with session middleware

### Multi-Deck Support

**Directory Structure (Option 2 - Shared Static Assets):**
```
static/
    css/              # Shared across all decks
    js/               # Shared across all decks
    favicon.png       # Shared across all decks
    deck/             # Default deck
        md/           # Markdown flashcards
        img/          # Image flashcards
    rust/             # Example deck
        md/
        img/
```

**Deck Configuration Priority:**
1. CLI Arguments (highest): `--deck <name>`, `--deck-name <display>`
2. Environment Variables: `DECK_ID`, `DECK_DISPLAY_NAME`
3. Default Values: `deck` (directory), deck ID (display name)

**CLI Arguments:**
- `--rebuild-deck <name>` / `-r <name>`: Delete and rebuild deck database
- `--deck <name>` / `-d <name>`: Load specific deck
- `--deck-name <display>` / `-n <display>`: Set HTML display name

**Database Naming:**
- Default deck: `./deck.db`
- Named decks: `./{name}.db` (e.g., `./rust.db`)

**Image Path Resolution:**
- Markdown-embedded images: `/static/{deck_id}/md/assets/`
- Image flashcards: `/static/{deck_id}/img/`
- Generated dynamically based on loaded deck

### Module Structure

**db/** - Database layer
- `connection.rs`: r2d2 connection pool setup
- `schema.rs`: Table initialization (flashcards + FTS5 virtual table)
- `models.rs`: `Flashcard` struct with category/subcategory support, `FilterCriteria` struct for filtering
- `queries.rs`: Database operations including FTS5 search, filtered queries (by category/subcategory/keywords/images)

**content/** - Content loading and processing
- `markdown.rs`: Scans `./static/{deck_id}/md`, parses markdown with pulldown-cmark, extracts category/subcategory from YAML frontmatter, applies syntax highlighting with syntect
- `images.rs`: Scans `./static/{deck_id}/img` for image-only flashcards, generates deck-aware image URLs

**cli.rs** - Command-line interface
- Parses CLI arguments using clap with derive API
- Supports: `--rebuild-deck` / `-r`, `--deck` / `-d`, `--deck-name` / `-n`, `--help`, `--version`

**routes/** - Web handlers (Axum)
- `landing.rs`: Filter form at `/` (keywords, categories, subcategories, images) + POST `/apply_filters`
- `practice.rs`: Display filtered flashcard at `/practice` (avoids recently seen via session)
- `debug.rs`: Session reset utility at `/reset_session`

**session/** - Session management
- Uses tower-sessions with in-memory store
- Tracks: `seen_ids`, filter state (`filter_keywords`, `filter_categories`, `filter_subcategories`, `filter_include_images`), cached counts, error messages

**cli.rs** - Command-line interface
- Parses CLI arguments using clap with derive API
- Currently supports: `--rebuild-db` / `-r`, `--help`, `--version`
- Extensible structure ready for future args (--database, --port)

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
- **clap 4.5**: CLI argument parsing with derive API

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
