# CLAUDE.md

- In all interactions and commit messages, be extremely concise and sacrifice grammar for the sake of concision.
- All documents, plan, comments is source code, commit messages... must be written in English US. In case of doubt, ask for confirmation before to write anything in an other language.
- The development is done in a Windows 11 and Powershell context. Use the appropriate commands and NOT the Linux commands.



## Repository Overview

Rust web application built with Axum framework. Displays machine learning Q&A flashcards loaded from markdown files and PNG images, with full-text search capabilities (SQLite FTS5).

**Production deployment:** https://rust-flashcards-ae94334b8997.herokuapp.com/

## Rust Application

### Development Commands

```bash
# Run locally
cargo run
# Opens at http://localhost:8080

# Build release
cargo build --release

# Type checking
cargo check

# Linting
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

### Environment Setup

Create a `.env` file in project root with:
```
FLASHCARDS_SECRET_KEY=<your-secret>
PORT=8080
DATABASE_URL=./flashcards.db
RUST_LOG=info
```

The database (`flashcards.db`) auto-creates on first run by:
1. Parsing all markdown files under `./static/md/`
2. Creating SQLite tables with FTS5 virtual table for search
3. Loading PNG image paths from `./static/png/`

### Architecture

**Application Entry Point:**
- `src/main.rs`: Axum server setup with routing
- Loads content at startup (~30s for 705 cards)

**Database Design:**
```
flashcards (id, question_html, answer_html)
flashcards_fts (FTS5 virtual table for full-text search)
```

Content stored as HTML (converted from markdown during database creation).

**Content Loading:**
- Markdown files: `./static/md/**/*.md` (recursive search)
- PNG flashcards: `./static/png/**/*.png` (recursive search)
- Both support subdirectory organization

**Markdown Format:**
```markdown
Question : What is PCA?

Answer : Principal Component Analysis is...
```
- Case-sensitive keywords: `Question :` and `Answer :`
- Colon and space required after keywords
- HTML comments allowed: `<!-- ... -->`

**Image References in Markdown:**
Paths must be relative to template root:
```html
<img src="../static/md/subdirectory/assets/image.png" alt="description" width="577"/>
```

**Session Management:**
- tower-sessions with MemoryStore
- Cookie name: `flashcards_session`
- Tracks: `seen_ids`, `searched_ids`, `keywords`, `nb_cards`

**Routes:**
- `/` - Random flashcard from full set
- `/next` - Redirect to next random card
- `/search` - Search form (GET/POST keywords)
- `/search_results` - Random card from search results
- `/reset_session` - Debug route to clear session

**Session State:**
- `seen_ids`: List of card IDs already shown (prevents repeats)
- `searched_ids`: List of card IDs shown in current search
- `keywords`: Current search terms (space-separated, AND logic)
- `nb_cards`: Total card count (cached per session)

**Reset Logic:**
When `seen_ids.len() >= nb_cards`, list automatically clears.

### Project Structure

```
rust-flashcards/
├── Cargo.toml                  # Dependencies, edition 2021
├── .env                        # Environment variables
├── Procfile                    # Heroku: web: ./target/release/rust-flashcards
├── src/
│   ├── main.rs                 # Axum server, routing, startup
│   ├── config.rs               # Load env vars
│   ├── db/
│   │   ├── mod.rs              # Module exports
│   │   ├── models.rs           # Flashcard struct
│   │   ├── schema.rs           # CREATE TABLE statements
│   │   ├── connection.rs       # r2d2 pool
│   │   └── queries.rs          # DB operations (insert, get_random, search)
│   ├── content/
│   │   ├── mod.rs              # Module exports
│   │   ├── markdown.rs         # Parse markdown with pulldown-cmark + syntect
│   │   └── images.rs           # Scan PNG with walkdir
│   ├── routes/
│   │   ├── mod.rs              # Route exports
│   │   ├── index.rs            # GET / handler
│   │   ├── next.rs             # GET /next redirect
│   │   ├── search.rs           # GET/POST /search
│   │   ├── search_results.rs   # GET /search_results
│   │   └── debug.rs            # GET /reset_session
│   └── session/
│       └── mod.rs              # SessionData struct
├── templates/
│   ├── index.html              # Main flashcard template (Askama)
│   ├── search.html             # Search form
│   └── search_results.html     # Search results display
└── static/                     # Copied from py-flashcards-2/
    ├── css/default.css         # Syntax highlighting CSS
    ├── favicon.png
    ├── md/**/*.md              # Markdown flashcards
    └── png/**/*.png            # PNG flashcards
```

### Dependencies

```toml
axum = "0.7"                    # Web framework
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "trace"] }
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"
tower-sessions = "0.12"         # Session management
askama = { version = "0.12", features = ["with-axum"] }
pulldown-cmark = "0.11"         # Markdown parsing
syntect = "5.2"                 # Syntax highlighting
walkdir = "2"
regex = "1"
dotenvy = "0.15"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
rand = "0.8"
```

### Markdown Extensions

- Python `extra` → pulldown-cmark: `ENABLE_TABLES | ENABLE_STRIKETHROUGH | ENABLE_FOOTNOTES`
- Python `codehilite` → syntect: theme "InspiredGitHub"
- Python `sane_lists` → pulldown-cmark default

### Heroku Deployment

```bash
# Create app
heroku create rust-flashcards --buildpack emk/rust

# Set environment variable
heroku config:set FLASHCARDS_SECRET_KEY=$(New-Guid)

# Deploy
git push heroku main

# Open app
heroku open

# Monitor logs
heroku logs --tail
```

**Deployment Notes:**
- Build time: ~2-5 min (Rust compilation)
- Startup time: ~30s (loads 705 flashcards)
- Static files bundled: ./static/md + ./static/png
- DB rebuilt at each dyno restart (ephemeral filesystem OK)

### Known Issues

- Search engine sensitive to special characters (hyphens, parentheses won't match FTS5 query)
- No automated test suite yet (integration tests planned)

## Project Context

This repository was created for certification/educational purposes focused on machine learning flashcard study. Originally built in Python/Flask, fully rewritten in Rust/Axum for production deployment on Heroku.

**Migration completed:** All Python code removed. Rust implementation has full feature parity.
