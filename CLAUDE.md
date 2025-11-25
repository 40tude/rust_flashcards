# CLAUDE.md

- In all interaction and commit messages, be extremely concise and sacrifice grammar for the sake of concision.

## Repository Overview

This is a **dual-language repository in transition** from Python to Rust. Both implementations coexist during the migration phase:

- **`py-flashcards-2/`**: Fully functional Python Flask web application deployed to Heroku
- **Root level Rust project**: Early-stage reimplementation (currently Hello World)

The flashcard application displays machine learning Q&A content loaded from markdown files and PNG images, with full-text search capabilities.

## Python Flask Application

### Development Commands

```bash
# Run locally (from py-flashcards-2/ directory)
uv run py-flashcards.py
# Opens at http://127.0.0.1:5000

# Type checking
mypy py-flashcards.py

# Deployment preparation
uv pip compile pyproject.toml -o requirements.txt
# Then manually add: gunicorn==23.0.0 (Windows can't install it locally)
git push heroku main

# Heroku configuration
heroku config:set FLASK_ENV=production
heroku config:set FLASHCARDS_SECRET_KEY=<your-secret-key>
heroku open
```

### Environment Setup

Create a `.env` file in `py-flashcards-2/` with:
```
FLASHCARDS_SECRET_KEY=<your-secret>
```

The database (`flashcards.db`) auto-creates on first run by:
1. Parsing all markdown files under `./static/md/`
2. Creating SQLite tables with FTS5 virtual table for search
3. Loading PNG image paths from `./static/png/`

### Architecture

**Application Entry Point:**
- Factory pattern: `create_app()` function in `py-flashcards.py`
- Referenced by Procfile: `web: gunicorn --workers=3 'py-flashcards:create_app()'`

**Database Design:**
```
flashcards (id, question_html, answer_html)
flashcards_fts (FTS5 virtual table for full-text search)
```

Content is stored as HTML (converted from markdown during database creation).

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
Paths must be relative to `templates/index.html`:
```html
<img src="../static/md/subdirectory/assets/image.png" alt="description" width="577"/>
```

**Critical Flask Session Behavior:**
Flask requires reassignment to trigger session serialization:
```python
# WRONG - won't persist
session["seen_ids"].append(card_id)

# CORRECT - triggers serialization
seen_list = session["seen_ids"]
seen_list.append(card_id)
session["seen_ids"] = seen_list  # Must reassign
```
See `py-flashcards.py:378-383` for implementation.

**Routes:**
- `/` - Random flashcard from full set
- `/search` - Search form (POST keywords)
- `/search_results` - Random card from search results
- `/next` - Redirect to next random card
- `/reset_session` - Debug route to clear session

**Session State:**
- `seen_ids`: List of card IDs already shown (prevents repeats)
- `searched_ids`: List of card IDs shown in current search
- `keywords`: Current search terms (space-separated)
- `nb_cards`: Total card count (cached per session)

### Known Issues

- Search engine sensitive to special characters (hyphens, parentheses won't match)
- No automated test suite yet (pytest + Flask testing planned)
- Werkzeug reloader causes double startup in debug mode (see comments in `__main__`)

## Rust Project

### Commands

```bash
cargo build         # Compile the project
cargo run           # Run (currently prints "Hello, world!")
cargo test          # Run test suite (none implemented yet)
cargo check         # Fast syntax validation
cargo clippy        # Linting (if available)
cargo fmt           # Format code
```

### Status

Early development stage. The Rust implementation will replicate the Python flashcard application's functionality.

**Note:** `Cargo.toml` specifies `edition = "2024"` which may need verification (typical editions are 2015, 2018, 2021).

## Project Context

This repository was created for certification/educational purposes focused on machine learning flashcard study. The Python version is production-ready and deployed. The Rust version represents an ongoing learning exercise in systems programming and web frameworks.
