# Multi-Deck Implementation Plan - Rust Flashcards

## Executive Summary

Implementation of multi-deck support using **Option 2 directory structure** (decks under `static/` directory with shared CSS/JS/favicon). Plan divided into 5 phases to minimize risk, enable testing after each phase, and maintain backward compatibility.

**Target Architecture:**
```
static/
    css/          # Shared across all decks
    js/           # Shared across all decks
    favicon.png   # Shared across all decks
    deck/         # Default deck
        img/
        md/
    rust/         # Example deck
        img/
        md/
    python/       # Example deck
        md/
```

---

## Phase 1: Configuration Layer - Add Deck Path Resolution

**Duration:** 2-3 hours
**Goal:** Add deck path resolution logic to Config without changing directory structure or CLI args. Test with default deck using new config structure.

### Success Criteria
- ✅ Application starts and loads content from existing `./static/md` and `./static/img`
- ✅ Database created at `./flashcards.db` (unchanged)
- ✅ All existing functionality works unchanged

### Files to Modify

#### 1. `src/config.rs` (lines 4-20)

**Add new fields to Config struct:**
```rust
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub deck_id: String,              // NEW: filesystem directory name
    pub deck_display_name: String,    // NEW: HTML display name
    pub md_path: String,              // NEW: computed markdown path
    pub img_path: String,             // NEW: computed images path
}
```

**Update `from_env()` method:**
```rust
pub fn from_env() -> anyhow::Result<Self> {
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let deck_id = env::var("DECK_ID")
        .unwrap_or_else(|_| "deck".to_string());

    let deck_display_name = env::var("DECK_DISPLAY_NAME")
        .or_else(|_| env::var("DECK_NAME"))  // Backward compat
        .unwrap_or_else(|_| "Data Science Flashcards".to_string());

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "./flashcards.db".to_string());

    // Compute content paths
    let md_path = format!("./static/{}/md", deck_id);
    let img_path = format!("./static/{}/img", deck_id);

    Ok(Config {
        port,
        database_url,
        deck_id,
        deck_display_name,
        md_path,
        img_path,
    })
}
```

#### 2. `src/main.rs` (lines 63-64, 92, 99, 73-86)

**Update content directory validation (lines 63-64):**
```rust
// Before
let md_status = content::validate_content_directory("./static/md");
let img_status = content::validate_content_directory("./static/img");

// After
let md_status = content::validate_content_directory(&config.md_path);
let img_status = content::validate_content_directory(&config.img_path);
```

**Update content loading calls (lines 92, 99):**
```rust
// Line 92: Before
content::load_markdown(&pool, "./static/md")?;

// Line 92: After
content::load_markdown(&pool, &config.md_path)?;

// Line 99: Before
content::load_images(&pool, "./static/img")?;

// Line 99: After
content::load_images(&pool, &config.img_path)?;
```

**Update error messages (lines 73-74, 76-77, 84-85):**
```rust
// Before
tracing::error!("Markdown directory ./static/md does not exist or is not accessible");

// After
tracing::error!("Markdown directory {} does not exist or is not accessible", config.md_path);

// Apply same pattern to all error messages referencing static directories
```

#### 3. `src/routes/landing.rs` (line 126)

**Update template data:**
```rust
// Before
deck_name: state.config.deck_name.clone(),

// After
deck_name: state.config.deck_display_name.clone(),
```

#### 4. `.env` (line 14)

**Add comment about new variables:**
```env
# Current (keep for backward compatibility)
DECK_NAME=Data Science Flashcards

# New variables will be added in Phase 2
# DECK_ID=deck
# DECK_DISPLAY_NAME=Data Science Flashcards
```

### Testing Phase 1

**Test with temporary symlinks:**
```powershell
# Create test structure
New-Item -ItemType Directory -Force -Path "static\deck"
New-Item -ItemType SymbolicLink -Path "static\deck\md" -Target "..\md"
New-Item -ItemType SymbolicLink -Path "static\deck\img" -Target "..\img"

# Build and run
cargo build
cargo run

# Test in browser
# - Navigate to http://localhost:8080/
# - Test filters
# - Test practice mode
# - Verify images display

# Cleanup
Remove-Item -Recurse -Force "static\deck"
```

### Rollback Strategy Phase 1
```powershell
# Revert code changes
git checkout HEAD -- src/config.rs src/main.rs src/routes/landing.rs .env
```

**Dependencies:** None (foundation phase)

---

## Phase 2: Directory Reorganization - Move to Option 2 Structure

**Duration:** 4-6 hours
**Goal:** Physically reorganize directories to Option 2 structure. Update image path generation. Test default deck with new structure.

### Success Criteria
- ✅ Content loads from `./static/deck/md/` and `./static/deck/img/`
- ✅ Database renamed to `./deck.db`
- ✅ Images display correctly with new paths
- ✅ All markdown image references work

### Pre-Phase Manual Steps (CRITICAL)

**1. Backup current state:**
```powershell
# Backup directories
Copy-Item -Recurse "static" "static_backup"

# Backup database
Copy-Item "flashcards.db" "flashcards_backup.db"
```

**2. Create new directory structure:**
```powershell
# Create deck subdirectory
New-Item -ItemType Directory -Force -Path "static\deck"

# Move content directories
Move-Item "static\md" "static\deck\md"
Move-Item "static\img" "static\deck\img"
```

**3. Verify structure:**
```powershell
Get-ChildItem -Recurse "static" -Directory | Select-Object FullName
# Should show: static/, static/css/, static/js/, static/deck/, static/deck/md/, static/deck/img/
```

### Files to Modify

#### 1. `src/content/images.rs` (line 50)

**Update image URL generation:**
```rust
// Before (line 50)
format!("<h3>Answer :</h3>\n<p align=\"center\"><img src='/static/img/{}' class='img-fluid'></p>", relative_path)

// After - extract deck_id from base_dir path
// Add at top of function:
let deck_id = Path::new(base_dir)
    .parent()
    .and_then(|p| p.file_name())
    .and_then(|n| n.to_str())
    .unwrap_or("deck");

// Update format string:
format!("<h3>Answer :</h3>\n<p align=\"center\"><img src='/static/{}/img/{}' class='img-fluid'></p>",
        deck_id, relative_path)
```

#### 2. `src/config.rs` (line 15)

**Change default database path:**
```rust
// Before
let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "./flashcards.db".to_string());

// After
let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| format!("./{}.db", deck_id));
```

#### 3. `.env` (line 8)

**Update database URL:**
```env
# Before
DATABASE_URL=./flashcards.db

# After
DATABASE_URL=./deck.db
```

#### 4. Update markdown files (15 files)

**Script to update image references:**
```powershell
# Find all markdown files and update image paths
Get-ChildItem -Recurse -Path "static\deck\md" -Filter "*.md" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $content = $content -replace '\.\./static/md/', '/static/deck/md/'
    Set-Content -Path $_.FullName -Value $content
}
```

**Manual verification needed for:**
- `static/deck/md/00_no_category.md`
- `static/deck/md/95_non_technical.md`
- Any files with image references in `assets/` or `book_covers/` subdirectories

#### 5. `.slugignore` (line 5)

**Add backup exclusions:**
```
/flashcards_staging
/static_backup
/flashcards_backup.db
/deck_backup.db
```

### Testing Phase 2

```powershell
# 1. Delete old database
Remove-Item -Force "deck.db" -ErrorAction SilentlyContinue

# 2. Build
cargo build

# 3. Run with rebuild (using old flag temporarily)
cargo run -- --rebuild-db

# 4. Verify database created
Get-Item "deck.db"

# 5. Start application
cargo run

# 6. Test in browser (http://localhost:8080/)
# - Check all pages load
# - Verify images display (especially from md/assets/ and md/book_covers/)
# - Test category/subcategory filters
# - Test keyword search
# - Test practice mode
# - Open browser DevTools Console - check for 404 errors on images

# 7. Test specific image types
# - Markdown-embedded images (in question/answer text)
# - Image-only flashcards (from static/deck/img/)
# - Images in subdirectories (assets/, book_covers/)
```

### Rollback Strategy Phase 2

```powershell
# Restore directories
Remove-Item -Recurse -Force "static\deck"
Move-Item "static_backup\md" "static\md"
Move-Item "static_backup\img" "static\img"

# Restore database
Remove-Item -Force "deck.db"
Move-Item "flashcards_backup.db" "flashcards.db"

# Restore code
git checkout HEAD -- src/content/images.rs src/config.rs .env
Get-ChildItem -Recurse "static\deck\md\*.md" | ForEach-Object {
    git checkout HEAD -- $_.FullName
}
```

**Dependencies:** Phase 1 must be complete and tested

---

## Phase 3: CLI Arguments - Add Multi-Deck Selection

**Duration:** 3-4 hours
**Goal:** Add CLI arguments for deck selection and rebuild. Test switching between default deck and test deck.

### Success Criteria
- ✅ Can specify deck via CLI: `cargo run -- --deck rust`
- ✅ Can rebuild specific deck: `cargo run -- --rebuild-deck rust`
- ✅ CLI args override env vars
- ✅ Display name can be specified: `cargo run -- --deck rust --deck-name "Rust Flashcards"`

### Files to Modify

#### 1. `src/cli.rs` (lines 10-23)

**Replace old `rebuild_db` field with new deck-specific args:**
```rust
#[derive(Parser, Debug)]
#[command(name = "rust-flashcards")]
#[command(version, about = "Flashcard web application", long_about = None)]
pub struct Cli {
    /// Rebuild deck by deleting existing DB file before startup
    #[arg(short = 'r', long = "rebuild-deck", value_name = "DECK")]
    pub rebuild_deck: Option<String>,

    /// Deck to load (directory name under ./static/)
    #[arg(short = 'd', long = "deck", value_name = "DECK")]
    pub deck: Option<String>,

    /// Display name for deck in HTML (overrides deck directory name)
    #[arg(short = 'n', long = "deck-name", value_name = "NAME")]
    pub deck_name: Option<String>,
}
```

#### 2. `src/config.rs` (lines 12-20)

**Update `from_env()` signature to accept CLI args:**
```rust
pub fn from_env(cli_deck: Option<String>, cli_deck_name: Option<String>) -> anyhow::Result<Self> {
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    // Priority: CLI > Env Var > Default
    let deck_id = cli_deck
        .or_else(|| env::var("DECK_ID").ok())
        .unwrap_or_else(|| "deck".to_string());

    let deck_display_name = cli_deck_name
        .or_else(|| env::var("DECK_DISPLAY_NAME").ok())
        .or_else(|| env::var("DECK_NAME").ok())  // Backward compat
        .unwrap_or_else(|| deck_id.clone());     // Fallback to deck_id

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("./{}.db", deck_id));

    let md_path = format!("./static/{}/md", deck_id);
    let img_path = format!("./static/{}/img", deck_id);

    Ok(Config {
        port,
        database_url,
        deck_id,
        deck_display_name,
        md_path,
        img_path,
    })
}
```

#### 3. `src/main.rs` (lines 33-34, 37-50)

**Update Config initialization (line 33):**
```rust
// Before
let config = config::Config::from_env()?;

// After
let config = config::Config::from_env(cli_args.deck.clone(), cli_args.deck_name.clone())?;
```

**Update rebuild logic (lines 37-50):**
```rust
// Handle database rebuild if requested
if let Some(deck_id) = cli_args.rebuild_deck.as_ref() {
    // Rebuild specified deck (may differ from loaded deck)
    let db_path_to_rebuild = format!("./{}.db", deck_id);
    let path = std::path::Path::new(&db_path_to_rebuild);

    if path.exists() {
        tracing::info!("Deleting existing database: {}", db_path_to_rebuild);
        std::fs::remove_file(path).with_context(|| {
            format!("Failed to delete database: {}", db_path_to_rebuild)
        })?;
        tracing::info!("Database deleted, will rebuild from content");
    } else {
        tracing::warn!("Database file not found: {}, nothing to delete", db_path_to_rebuild);
    }

    // Warn if rebuilding different deck than loading
    if deck_id != &config.deck_id {
        tracing::warn!(
            "Rebuilding deck '{}' but loading deck '{}'. Consider using --deck {} as well.",
            deck_id, config.deck_id, deck_id
        );
    }
}
```

#### 4. `.env` (add after line 14)

**Add new environment variables:**
```env
# Deck configuration (optional, CLI args take priority)
# DECK_ID=deck
# DECK_DISPLAY_NAME=Data Science Flashcards

# Legacy variable (still supported for backward compatibility)
DECK_NAME=Data Science Flashcards
```

### Testing Phase 3

```powershell
# Test 1: Default deck (no args)
cargo run
# Expected: Load from ./static/deck/, use ./deck.db

# Test 2: Create test deck
New-Item -ItemType Directory -Force -Path "static\rust\md", "static\rust\img"
Copy-Item "static\deck\md\00_no_category.md" "static\rust\md\"

# Test 3: CLI deck selection
cargo run -- --deck rust --deck-name "Rust Flashcards"
# Expected: Create ./rust.db, show "Rust Flashcards" in browser

# Test 4: Rebuild with deck name
cargo run -- --rebuild-deck rust --deck rust
# Expected: Delete rust.db, rebuild from ./static/rust/

# Test 5: Env var override
$env:DECK_ID="rust"; cargo run
# Expected: Load rust deck via env var

# Test 6: Priority test (CLI > env)
$env:DECK_ID="rust"; cargo run -- --deck deck
# Expected: Load "deck" (CLI wins)

# Test 7: Help text
cargo run -- --help
# Verify new args documented:
# -r, --rebuild-deck <DECK>
# -d, --deck <DECK>
# -n, --deck-name <NAME>

# Test 8: Version
cargo run -- --version

# Test 9: Rebuild different deck than loading
cargo run -- --rebuild-deck deck --deck rust
# Expected: Warning about rebuilding different deck
```

### Rollback Strategy Phase 3

```powershell
# Remove test deck
Remove-Item -Recurse -Force "static\rust"
Remove-Item -Force "rust.db"

# Revert code changes
git checkout HEAD -- src/cli.rs src/config.rs src/main.rs .env
```

**Dependencies:** Phase 2 must be complete and tested

---

## Phase 4: Heroku Multi-App Deployment Support

**Duration:** 2-3 hours
**Goal:** Verify Heroku can deploy multiple apps from same repo with different env vars. Update deployment documentation.

### Success Criteria
- ✅ Can deploy "default" deck app with `DECK_ID=deck`
- ✅ Can deploy "rust" deck app with `DECK_ID=rust`
- ✅ Both apps run independently on Heroku
- ✅ `.slugignore` properly excludes unnecessary files

### Files to Modify

#### 1. `.slugignore`

**Verify excludes development artifacts:**
```
# Development and staging
/flashcards_staging
/static_backup
/flashcards_backup.db
/deck_backup.db
*.db

# Assets (optional - exclude test decks from production)
# Uncomment to reduce slug size:
# /static/test_deck
```

#### 2. Create `docs/heroku_multi_deck_deployment.md`

```markdown
# Heroku Multi-Deck Deployment Guide

## Overview

Deploy multiple flashcard decks as separate Heroku apps from a single Git repository. Each app loads a different deck via environment variables.

## Prerequisites

- Heroku CLI installed: https://devcenter.heroku.com/articles/heroku-cli
- Git repository with multiple decks in `static/` directory
- Heroku account

## Setup Multiple Apps

### Create Apps

```bash
# Create app for data science deck (default)
heroku create rust-flashcards-data-science

# Create app for rust deck
heroku create rust-flashcards-rust

# Create app for python deck
heroku create rust-flashcards-python
```

### Configure Git Remotes

```bash
# Add remotes for each app
git remote add heroku-data-science https://git.heroku.com/rust-flashcards-data-science.git
git remote add heroku-rust https://git.heroku.com/rust-flashcards-rust.git
git remote add heroku-python https://git.heroku.com/rust-flashcards-python.git
```

### Configure Environment Variables

```bash
# Data science deck
heroku config:set DECK_ID=deck DECK_DISPLAY_NAME="Data Science Flashcards" -a rust-flashcards-data-science

# Rust deck
heroku config:set DECK_ID=rust DECK_DISPLAY_NAME="Rust Flashcards" -a rust-flashcards-rust

# Python deck
heroku config:set DECK_ID=python DECK_DISPLAY_NAME="Python Flashcards" -a rust-flashcards-python
```

### Verify Configuration

```bash
# Check env vars for each app
heroku config -a rust-flashcards-data-science
heroku config -a rust-flashcards-rust
heroku config -a rust-flashcards-python
```

## Deployment

### Deploy to All Apps

```bash
# Deploy to data science app
git push heroku-data-science main

# Deploy to rust app
git push heroku-rust main

# Deploy to python app
git push heroku-python main
```

### Deploy to Specific App

```bash
# Deploy only rust app
git push heroku-rust main
```

## Verification

### Check Logs

```bash
# View logs for rust app
heroku logs --tail -a rust-flashcards-rust
```

### Open App

```bash
# Open rust app in browser
heroku open -a rust-flashcards-rust
```

### Verify Deck Loaded

1. Open app URL in browser
2. Check page title shows correct deck name
3. Test filtering and practice mode
4. Verify images display correctly

## Updating Decks

### Add New Content

```bash
# 1. Add content locally
# - Add markdown files to static/rust/md/
# - Add images to static/rust/img/

# 2. Commit changes
git add static/rust/
git commit -m "add: new rust flashcards"

# 3. Deploy to rust app (database will rebuild automatically on first run)
git push heroku-rust main
```

### Switch Deck for Existing App

```bash
# Change rust app to use python deck
heroku config:set DECK_ID=python DECK_DISPLAY_NAME="Python Flashcards" -a rust-flashcards-rust

# Restart app to apply changes
heroku restart -a rust-flashcards-rust
```

## Cost Optimization

- **Hobby dynos:** Free (with limitations)
- **Eco dynos:** $5/month per dyno (recommended for production)
- **Database:** Not persistent (rebuilt from content files on each deploy)

## Troubleshooting

### Deck Not Found Error

```bash
# Check DECK_ID matches directory name
heroku config:get DECK_ID -a rust-flashcards-rust
# Should match: static/rust/

# List available decks in repo
ls static/
```

### Images Not Displaying

- Verify images in `static/{DECK_ID}/img/`
- Check markdown image references use `/static/{DECK_ID}/md/assets/`
- Check browser console for 404 errors

### Database Empty After Deploy

- Database is built from content files on startup
- Check logs: `heroku logs --tail -a rust-flashcards-rust`
- Verify content directories exist: `static/{DECK_ID}/md/` and `static/{DECK_ID}/img/`
```

### Testing Phase 4

#### Local Testing with Heroku Simulation

```powershell
# Test 1: Simulate data science deck Heroku env
$env:DECK_ID="deck"; $env:DECK_DISPLAY_NAME="Data Science Flashcards"; $env:PORT="8080"
cargo run

# Test 2: Simulate rust deck Heroku env
$env:DECK_ID="rust"; $env:DECK_DISPLAY_NAME="Rust Flashcards"; $env:PORT="8080"
cargo run

# Test 3: Production build
cargo build --release
.\target\release\rust-flashcards.exe
```

#### Heroku Staging Deployment (if available)

```bash
# Deploy to staging
git push heroku-staging main

# Verify at staging URL
heroku open -a rust-flashcards-staging

# Check logs
heroku logs --tail -a rust-flashcards-staging
```

#### Production Deployment Test

```bash
# Rebuild default deck database locally
cargo run -- --rebuild-deck deck

# Commit any pending changes
git add .
git commit -m "update: multi-deck support ready for production"

# Deploy to production
git push heroku main

# Verify at production URL
# https://rust-flashcards-ae94334b8997.herokuapp.com/
```

### Rollback Strategy Phase 4

- Documentation only (no code changes)
- If Heroku deployment fails, previous release still deployed
- Revert to previous Heroku release:
  ```bash
  heroku releases -a rust-flashcards-data-science
  heroku rollback v42 -a rust-flashcards-data-science
  ```

**Dependencies:** Phase 3 must be complete and tested

---

## Phase 5: Documentation and Comprehensive Testing

**Duration:** 3-4 hours
**Goal:** Update all documentation. Create comprehensive testing checklist. Document migration path for existing users.

### Success Criteria
- ✅ README.md updated with multi-deck usage
- ✅ CLAUDE.md updated with new architecture
- ✅ Migration guide created
- ✅ All comprehensive tests pass

### Files to Modify

#### 1. `README.md`

**Update "Build and Run Commands" section:**
```markdown
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

# Rebuild specific deck
cargo run -- --rebuild-deck rust --deck rust --deck-name "Rust Flashcards"

# Show help
cargo run -- --help

# Show version
cargo run -- --version

# Build for production (Heroku deployment)
cargo build --release
```

## Multi-Deck Support

### Directory Structure

```
static/
    css/              # Shared stylesheets
    js/               # Shared JavaScript
    favicon.png       # Shared favicon
    deck/             # Default deck
        md/           # Markdown flashcards
        img/          # Image flashcards
    rust/             # Example: Rust deck
        md/
        img/
    python/           # Example: Python deck
        md/
```

### Creating a New Deck

1. Create directory structure:
   ```bash
   mkdir -p static/mydeck/md static/mydeck/img
   ```

2. Add markdown files to `static/mydeck/md/`
3. Add image files to `static/mydeck/img/` (optional)

4. Build deck database:
   ```bash
   cargo run -- --rebuild-deck mydeck --deck mydeck --deck-name "My Deck"
   ```

5. Load deck:
   ```bash
   cargo run -- --deck mydeck --deck-name "My Deck"
   ```

### Configuration Priority

Deck configuration follows this priority order:

1. **CLI Arguments** (highest priority)
   - `--deck <name>`
   - `--deck-name <display_name>`
   - `--rebuild-deck <name>`

2. **Environment Variables**
   - `DECK_ID=<name>`
   - `DECK_DISPLAY_NAME=<display_name>`

3. **Default Values** (fallback)
   - Deck: `deck`
   - Display name: same as deck ID

### Environment Variables

```bash
# Deck selection
DECK_ID=rust
DECK_DISPLAY_NAME="Rust Flashcards"

# Server configuration
PORT=8080
DATABASE_URL=./rust.db  # Auto-generated from DECK_ID if not specified
```
```

**Add "Heroku Multi-App Deployment" section:**
```markdown
## Heroku Multi-App Deployment

You can deploy multiple decks as separate Heroku apps from the same repository.

See [docs/heroku_multi_deck_deployment.md](docs/heroku_multi_deck_deployment.md) for detailed instructions.

**Quick Setup:**
```bash
# Create apps
heroku create rust-flashcards-rust
heroku create rust-flashcards-python

# Configure decks
heroku config:set DECK_ID=rust DECK_DISPLAY_NAME="Rust Flashcards" -a rust-flashcards-rust
heroku config:set DECK_ID=python DECK_DISPLAY_NAME="Python Flashcards" -a rust-flashcards-python

# Deploy
git push heroku-rust main
git push heroku-python main
```
```

#### 2. `CLAUDE.md`

**Update "Build and Run Commands" section:**
```markdown
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
cargo run -- --rebuild-deck python --deck python --deck-name "Python Flashcards"

# Show help
cargo run -- --help

# Show version
cargo run -- --version

# Build for production (Heroku deployment)
cargo build --release
```
```

**Update "Architecture Overview" section - add after "Application Startup Flow":**
```markdown
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
```

**Update "Application Startup Flow":**
```markdown
### Application Startup Flow (src/main.rs)
1. Parse CLI arguments (--rebuild-deck, --deck, --deck-name, --help, --version)
2. Load environment variables from `.env` (PORT, DECK_ID, DECK_DISPLAY_NAME, DATABASE_URL)
3. Resolve deck configuration (CLI > env > defaults)
4. Handle database rebuild if `--rebuild-deck` flag present
   - Delete database file: `./{deck_name}.db`
   - Warn if rebuilding different deck than loading
5. Create SQLite connection pool (r2d2)
6. Initialize database schema (flashcards + flashcards_fts tables)
7. Check if database empty - only load content if needed (fast startup optimization)
   - Validate `./static/{deck_id}/md` and `./static/{deck_id}/img` directories
   - Load content from validated directories
   - Populate FTS5 search table
8. Start Axum web server with session middleware
```

#### 3. Create `MIGRATION_GUIDE.md`

```markdown
# Migration Guide: Single Deck to Multi-Deck Architecture

## Overview

This guide helps you migrate from the old single-deck architecture (with `./flashcards.db` and `./static/md/`) to the new multi-deck architecture.

## Quick Migration (Recommended)

### Step 1: Backup Current Installation

```powershell
# Backup database
Copy-Item "flashcards.db" "flashcards_backup.db"

# Backup static directory
Copy-Item -Recurse "static" "static_backup"
```

### Step 2: Reorganize Directories

```powershell
# Create default deck directory
New-Item -ItemType Directory -Force -Path "static\deck"

# Move content directories
Move-Item "static\md" "static\deck\md"
Move-Item "static\img" "static\deck\img"
```

### Step 3: Update Image References in Markdown

```powershell
# Update image paths in all markdown files
Get-ChildItem -Recurse -Path "static\deck\md" -Filter "*.md" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $content = $content -replace '\.\./static/md/', '/static/deck/md/'
    Set-Content -Path $_.FullName -Value $content
}
```

### Step 4: Update Configuration

**Option A: Rename database (recommended)**
```powershell
Remove-Item "flashcards.db"  # Will be rebuilt
```

**Option B: Keep old database name**
Update `.env`:
```env
DATABASE_URL=./flashcards.db
DECK_ID=deck
DECK_DISPLAY_NAME=Data Science Flashcards
```

### Step 5: Rebuild Database

```powershell
cargo run -- --rebuild-deck deck
```

### Step 6: Test Application

```powershell
cargo run

# Test in browser (http://localhost:8080/)
# - Verify all pages load
# - Check images display correctly
# - Test filters and search
# - Test practice mode
```

## Creating Additional Decks

### From Scratch

```powershell
# 1. Create directory structure
New-Item -ItemType Directory -Force -Path "static\mydeck\md", "static\mydeck\img"

# 2. Add content files
# - Add .md files to static\mydeck\md\
# - Add .png/.webp files to static\mydeck\img\

# 3. Build deck database
cargo run -- --rebuild-deck mydeck --deck mydeck --deck-name "My Deck"

# 4. Run application
cargo run -- --deck mydeck --deck-name "My Deck"
```

### By Copying Existing Deck

```powershell
# Copy default deck to new deck
Copy-Item -Recurse "static\deck" "static\rust"

# Modify content in static\rust\md\ and static\rust\img\

# Build new deck
cargo run -- --rebuild-deck rust --deck rust --deck-name "Rust Flashcards"
```

## Configuration Examples

### Via CLI Arguments

```powershell
# Load default deck
cargo run

# Load specific deck
cargo run -- --deck rust --deck-name "Rust Flashcards"

# Rebuild and load
cargo run -- --rebuild-deck python --deck python --deck-name "Python Flashcards"
```

### Via Environment Variables

Update `.env`:
```env
DECK_ID=rust
DECK_DISPLAY_NAME=Rust Flashcards
DATABASE_URL=./rust.db
```

Then:
```powershell
cargo run
```

### Via PowerShell Session Variables

```powershell
$env:DECK_ID="rust"
$env:DECK_DISPLAY_NAME="Rust Flashcards"
cargo run
```

## Heroku Migration

### Update Environment Variables

```bash
# Set deck configuration
heroku config:set DECK_ID=deck DECK_DISPLAY_NAME="Data Science Flashcards"

# Restart app
heroku restart
```

### Deploy Multiple Decks

See [docs/heroku_multi_deck_deployment.md](docs/heroku_multi_deck_deployment.md) for detailed multi-app setup.

## Troubleshooting

### Issue: "Content directory not found"

**Cause:** Deck directory doesn't exist or is misnamed.

**Solution:**
```powershell
# Check deck directories
Get-ChildItem "static" -Directory

# Verify deck_id matches directory name
# If using DECK_ID=rust, directory should be: static\rust\
```

### Issue: Images not displaying

**Cause:** Image references in markdown still use old paths.

**Solution:**
```powershell
# Re-run update script
Get-ChildItem -Recurse -Path "static\{deck_name}\md" -Filter "*.md" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $content = $content -replace '\.\./static/md/', "/static/{deck_name}/md/"
    Set-Content -Path $_.FullName -Value $content
}
```

### Issue: "Database file not found"

**Cause:** Database name doesn't match expected pattern.

**Solution:**
```powershell
# Check database file
Get-ChildItem "*.db"

# Should match: {deck_id}.db (e.g., deck.db, rust.db)

# Rebuild if needed
cargo run -- --rebuild-deck {deck_id}
```

## Rollback to Old Architecture

If you need to revert:

```powershell
# Restore directories
Remove-Item -Recurse -Force "static\deck"
Move-Item "static_backup\md" "static\md"
Move-Item "static_backup\img" "static\img"

# Restore database
Copy-Item "flashcards_backup.db" "flashcards.db"

# Checkout previous git commit
git checkout <commit_before_migration>
```
```

#### 4. Update `assets/16_multi_deck_cdc.md`

**Add implementation status at top:**
```markdown
# Multi-Deck Support - Cahier des Charges

**STATUS: IMPLEMENTED ✅**
- Implementation date: [DATE]
- Architecture chosen: **Option 2** (Shared static assets)
- See [MIGRATION_GUIDE.md](../MIGRATION_GUIDE.md) for migration instructions
- See [docs/heroku_multi_deck_deployment.md](../docs/heroku_multi_deck_deployment.md) for deployment

---

[Original CDC content follows...]
```

#### 5. Update `src/routes/practice.rs`

**Add deck_name to PracticeTemplate:**
```rust
// Find PracticeTemplate struct definition
#[derive(Template)]
#[template(path = "practice.html")]
struct PracticeTemplate {
    deck_name: String,  // Add if not present
    question: String,
    answer: String,
    card_id: i64,
    filter_count: usize,
    total_count: usize,
}

// In practice handler, populate deck_name:
let template = PracticeTemplate {
    deck_name: state.config.deck_display_name.clone(),
    question: card.question_html,
    answer: card.answer_html,
    card_id: card.id,
    filter_count,
    total_count,
};
```

### Testing Phase 5 - Comprehensive Test Suite

#### A. Single Deck Tests

```powershell
# Test 1: Fresh install with default deck
Remove-Item "*.db"
cargo run -- --rebuild-deck deck

# Test 2: Load markdown-only flashcards
# - Filter by "Data Processing"
# - Verify markdown rendering

# Test 3: Load image-only flashcards
# - Filter by "Include images only"
# - Verify images display

# Test 4: Mixed markdown + image flashcards
# - Filter by specific subcategory
# - Verify both types work

# Test 5: Keyword search
# - Search "python"
# - Verify FTS5 results

# Test 6: Session persistence
# - Practice 5 cards
# - Refresh page
# - Verify seen_ids persist
# - Verify filters persist

# Test 7: Database rebuild
cargo run -- --rebuild-deck deck
# Verify database recreated
```

#### B. Multi-Deck Tests

```powershell
# Setup: Create 3 test decks
Copy-Item -Recurse "static\deck" "static\test1"
Copy-Item -Recurse "static\deck" "static\test2"

# Test 1: Switch between decks
cargo run -- --deck deck --deck-name "Default"
# Stop (Ctrl+C), then:
cargo run -- --deck test1 --deck-name "Test 1"
# Stop, then:
cargo run -- --deck test2 --deck-name "Test 2"

# Test 2: Rebuild each deck independently
cargo run -- --rebuild-deck deck
cargo run -- --rebuild-deck test1
cargo run -- --rebuild-deck test2

# Test 3: Verify separate DB files
Get-ChildItem "*.db"
# Should show: deck.db, test1.db, test2.db

# Test 4: Verify deck display names
cargo run -- --deck test1 --deck-name "Custom Name"
# Check browser shows "Custom Name"

# Test 5: CLI priority over env vars
$env:DECK_ID="test1"
cargo run -- --deck deck
# Should load "deck", not "test1"
```

#### C. Image Path Tests

```powershell
# Test 1: Images in deck/img/ display
cargo run -- --deck deck
# Navigate to image flashcard
# Verify URL: /static/deck/img/{filename}

# Test 2: Markdown-embedded images display
# Navigate to card with embedded image
# Verify URL: /static/deck/md/assets/{filename}

# Test 3: Image-only flashcards work
# Filter: "Include images only"
# Verify all images display

# Test 4: No 404 errors
# Open browser DevTools Console
# Navigate through multiple cards
# Verify no 404 errors for images or assets
```

#### D. Deployment Tests

```powershell
# Test 1: Local production build
cargo build --release

# Test 2: Run production binary
.\target\release\rust-flashcards.exe

# Test 3: Simulated Heroku env
$env:DECK_ID="deck"
$env:DECK_DISPLAY_NAME="Production Deck"
$env:PORT="8080"
.\target\release\rust-flashcards.exe

# Test 4: Deploy to Heroku (if applicable)
git push heroku main
heroku open
# Verify production deployment works
```

#### E. Regression Tests

```powershell
# Test all existing functionality still works

# 1. Landing page filters
# - Keyword filter
# - Category dropdown
# - Subcategory checkboxes
# - "Include images" checkbox

# 2. Practice mode
# - Random card selection
# - "Next card" button
# - Avoid recently seen cards
# - "Back to Filters" button

# 3. Session management
# - Filters persist across page loads
# - Seen cards list persists
# - Session reset works (/reset_session)

# 4. Error handling
# - Missing content directories
# - Missing database
# - Empty database

# 5. Content loading
# - Markdown parsing
# - Syntax highlighting (code blocks)
# - Math formulas (MathJax)
# - Category/subcategory extraction from YAML frontmatter
```

### Testing Script

Create `tests/test_multi_deck.ps1`:
```powershell
#!/usr/bin/env pwsh
# Multi-deck comprehensive test script

Write-Host "=== Multi-Deck Test Suite ===" -ForegroundColor Cyan

# Test 1: Default deck loads
Write-Host "`nTest 1: Default deck loads" -ForegroundColor Yellow
cargo run -- --deck deck --deck-name "Default" | Select-String "deck_id=deck"

# Test 2: Rebuild deck
Write-Host "`nTest 2: Rebuild deck" -ForegroundColor Yellow
cargo run -- --rebuild-deck deck 2>&1 | Select-String "Database deleted"

# Test 3: Create and load test deck
Write-Host "`nTest 3: Create and load test deck" -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path "static\test\md" | Out-Null
@"
---
category: Test
subcategory: Unit Test
---

### Q
Test question

### A
Test answer
"@ | Set-Content "static\test\md\test.md"

cargo run -- --rebuild-deck test --deck test --deck-name "Test Deck"

# Test 4: Priority check (CLI > env)
Write-Host "`nTest 4: CLI priority over env vars" -ForegroundColor Yellow
$env:DECK_ID="wrong"
cargo run -- --deck deck 2>&1 | Select-String "deck_id=deck"
Remove-Item Env:\DECK_ID

# Test 5: Database files exist
Write-Host "`nTest 5: Database files created" -ForegroundColor Yellow
if (Test-Path "deck.db") {
    Write-Host "✓ deck.db exists" -ForegroundColor Green
} else {
    Write-Host "✗ deck.db missing" -ForegroundColor Red
}

if (Test-Path "test.db") {
    Write-Host "✓ test.db exists" -ForegroundColor Green
} else {
    Write-Host "✗ test.db missing" -ForegroundColor Red
}

# Cleanup
Write-Host "`nCleaning up test artifacts..." -ForegroundColor Yellow
Remove-Item -Recurse -Force "static\test" -ErrorAction SilentlyContinue
Remove-Item "test.db" -ErrorAction SilentlyContinue

Write-Host "`n=== All Tests Completed ===" -ForegroundColor Cyan
```

### Rollback Strategy Phase 5

- Documentation changes are non-breaking
- Revert individual files via git:
  ```powershell
  git checkout HEAD -- README.md CLAUDE.md assets/16_multi_deck_cdc.md
  Remove-Item MIGRATION_GUIDE.md, docs/heroku_multi_deck_deployment.md
  ```

**Dependencies:** Phases 1-4 must be complete and tested

---

## Summary Tables

### Critical Files by Phase

| Phase | Files Modified | Purpose |
|-------|----------------|---------|
| 1 | config.rs, main.rs, routes/landing.rs, .env | Add deck path resolution |
| 2 | content/images.rs, config.rs, .env, .slugignore, markdown files | Directory reorganization |
| 3 | cli.rs, config.rs, main.rs, .env | CLI arguments for deck selection |
| 4 | .slugignore, docs/heroku_*.md | Heroku multi-app deployment |
| 5 | README.md, CLAUDE.md, MIGRATION_GUIDE.md, routes/practice.rs | Documentation and testing |

### Risk Mitigation Matrix

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Directory reorg breaks app | Medium | High | Backup before Phase 2, test with symlinks in Phase 1 |
| Image paths broken | High | Medium | Automated sed script, comprehensive image tests |
| Database naming conflicts | Low | Medium | Clear error messages, warn on deck mismatch |
| Heroku deployment fails | Low | High | Local production build test, staging deployment |
| Lost backward compatibility | Medium | Low | Keep DECK_NAME env var, provide migration guide |

### Testing Checklist

- [ ] Phase 1: Config layer works with symlinks
- [ ] Phase 2: Directory reorg complete, images display
- [ ] Phase 3: CLI args work, priority correct
- [ ] Phase 4: Heroku simulation successful
- [ ] Phase 5: All comprehensive tests pass
- [ ] Regression: All existing features work
- [ ] Documentation: Complete and accurate
- [ ] Production build: Succeeds
- [ ] Heroku deploy: Succeeds (if applicable)

---

## Post-Implementation Actions

### 1. Git Commit Strategy

```powershell
# Phase 1
git add src/config.rs src/main.rs src/routes/landing.rs .env
git commit -m "add: deck path resolution config layer"

# Phase 2
git add static/ src/content/images.rs src/config.rs .env .slugignore
git commit -m "refactor: reorganize to Option 2 directory structure"

# Phase 3
git add src/cli.rs src/config.rs src/main.rs .env
git commit -m "add: CLI args for multi-deck selection"

# Phase 4
git add .slugignore docs/
git commit -m "add: Heroku multi-app deployment docs"

# Phase 5
git add README.md CLAUDE.md MIGRATION_GUIDE.md assets/ src/routes/practice.rs tests/
git commit -m "docs: comprehensive multi-deck documentation"

# Tag release
git tag -a v2.0.0 -m "Multi-deck support"
git push origin main --tags
```

### 2. Heroku Deployment

```bash
# Deploy to production
git push heroku main

# Verify deployment
heroku logs --tail

# Open in browser
heroku open
```

### 3. Update Project Documentation

- [ ] Update README.md with migration notice
- [ ] Update CHANGELOG.md (if exists)
- [ ] Update version in Cargo.toml
- [ ] Add release notes on GitHub (if using)

### 4. User Communication

- [ ] Notify users of new multi-deck feature
- [ ] Provide link to MIGRATION_GUIDE.md
- [ ] Update any external documentation or wikis

---

## Estimated Timeline

| Phase | Duration | Cumulative |
|-------|----------|-----------|
| Phase 1: Config Layer | 2-3 hours | 2-3 hours |
| Phase 2: Directory Reorg | 4-6 hours | 6-9 hours |
| Phase 3: CLI Arguments | 3-4 hours | 9-13 hours |
| Phase 4: Heroku Deployment | 2-3 hours | 11-16 hours |
| Phase 5: Documentation | 3-4 hours | 14-20 hours |

**Total: 14-20 hours**

Factors affecting timeline:
- Number of markdown files needing image path updates (currently 15)
- Heroku deployment complexity
- Testing thoroughness
- Documentation detail level

---

## Open Questions for Final Confirmation

Before starting implementation:

1. **Directory Structure:** Confirmed Option 2 (shared static assets)? ✅
2. **Database Naming:** Acceptable to rename `flashcards.db` → `deck.db`?
3. **Default Deck ID:** Use "deck" as default deck ID?
4. **Markdown Updates:** Okay to bulk-update image refs with PowerShell script?
5. **Backward Compatibility:** Keep `DECK_NAME` env var temporarily?
6. **Heroku:** Will you deploy multiple Heroku apps from same repo?
7. **Git Strategy:** Stay on `main` branch (no feature branch)?
8. **Testing:** Need automated integration tests or manual testing sufficient?

---

## Appendix: Architecture Decisions

### Why Option 2?

**Pros:**
- No duplication of CSS/JS/favicon
- Simpler maintenance of shared assets
- Compatible with existing Axum ServeDir setup
- Cleaner separation: content vs. assets

**Cons:**
- Cannot customize CSS/JS per deck
- Slightly more complex image path resolution

**Decision:** Option 2 chosen because CSS/JS customization per deck not required, and avoiding duplication is more maintainable.

### Why CLI + Env Vars?

**Pros:**
- Flexible: CLI for dev, env vars for production
- Standard pattern (matches PORT, DATABASE_URL)
- Clear priority: CLI > env > defaults
- No config file parsing needed

**Cons:**
- More parameters to manage
- Must document priority clearly

**Decision:** CLI + env vars chosen for maximum flexibility and alignment with existing config patterns.

### Why Separate Databases per Deck?

**Pros:**
- Isolation: Each deck has own FTS5 index
- Simple backup/restore per deck
- No cross-deck data contamination
- Matches deck-as-unit-of-deployment model

**Cons:**
- Multiple .db files in repo root
- Cannot query across decks

**Decision:** Separate databases chosen for simplicity and isolation. Cross-deck queries not required.