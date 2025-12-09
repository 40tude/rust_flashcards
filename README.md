# Rust Flashcards

A web-based flashcard application with full-text search and multi-deck support, built with Rust.

## Features

- **Markdown-based flashcards** with support for images, math formulas, and code syntax highlighting
- **Full-text search** using SQLite FTS5
- **Multi-deck support** with CLI arguments and environment variables
- **Category and subcategory filtering**
- **Image-only flashcards** support
- **Session-based practice** with spaced repetition (avoids recently seen cards)
- **Responsive design** with Bootstrap
- **Heroku deployment ready**



## Quick Start

```powershell
# Clone and run
git clone https://github.com/40tude/rust_flashcards
cd rust_flashcards
```
### Create `.env` file

```
# Server configuration
PORT=8080
# RUST_LOG=info

# Default Display Configuration
DECK_DISPLAY_NAME="My Flashcards"

# Secret key for session management (change this, you can use `New-Guid` in PowerShell)
FLASHCARDS_SECRET_KEY=change-this-to-a-secure-random-key
```

```powershell
cargo run
# Access at http://localhost:8080
# Press ENTER
```







## CLI Arguments

```bash
# Run with default deck
cargo run

# Help
cargo run -- --help

# Rebuild and load specific deck
cargo run -- --rebuild-deck-id test --deck-id test

# Load deck with custom display name
cargo run -- --deck-id rust --deck-display-name "Rust Programming"

# Short form
cargo run -- -r deck -d deck -n "My Deck"
```


### Priority Rules

Configuration priority (highest to lowest):
1. CLI arguments (`--deck-id`, `--deck-display-name`)
2. Environment variables (`DECK_DISPLAY_NAME`)
3. Default values (`deck`, uses deck_id as display name)

**Note:** When `--deck-id` is provided without `--deck-display-name`, the display name defaults to the deck ID, **not** the environment variable but the value of `--deck-id`



## Directory Structure
* Decks are located under static/

```
.
├── static/
│   ├── deck/          # Default deck
│   │   ├── md/        # Markdown flashcards
│   │   └── img/       # Images (PNG/WebP)
│   ├── rust/          # Example: Rust deck
│   │   ├── md/
│   │   └── img/
│   ├── css/           # Shared CSS
│   ├── js/            # Shared JavaScript
│   └── favicon.png    # Shared favicon
├── templates/         # HTML templates (Askama)
├── src/              # Rust source code
└── .env              # Environment configuration
```

## Cards Format
* Decks are made of cards
* Cards can be either markdown files or images

### Images: my_deck/img/
* The `img/` directory is optional
* If `img/` exists, it and its subdirectories are scanned to search for images.
* Images are in `.png` or `.webp` format
* We recommend `.webp` and width=600px

### Markdown: my_deck/md
* The `md/` directory is optional
* If `md/` exists, it and its subdirectories are scanned to search for markdown files.
* Files use markdown format so they can include images, math formulas...

```markdown
<!--
############################################################
##
############################################################
-->
Question : Category - Subcategory - Do you believe in life after love?

Answer  :

## May be
Blablabla...

## Insert maths
<!-- ## Mathjax is supported -->
$V = \frac{d}{t} = \frac{D}{\frac{D}{2\cdot40} + \frac{D}{2\cdot60}} = \frac{2}{\frac{1}{40} + \frac{1}{60}}$

## Insert images
* `.png` or `.webp`
* We recommend to store the embedded images closed to the `.md` file or in a dedicated directory
* Target the images as if you are at the root of the project.
    * Below, in the deck `my_deck`, a directory `md/assets/` host the images.

<p align="center">
<img src="static/my_deck/md/assets/kitten.png" alt="harmonic" width="577"/>
</p>


```










## Heroku Deployment

### Single Deck Deployment

```bash
# Set environment variables
heroku config:set DECK_DISPLAY_NAME="My Flashcards"

# Deploy
git push heroku main
```

### Multi-Deck Deployment

Deploy the same codebase to multiple Heroku apps:

```bash
# App 1: Data Science deck
heroku config:set DECK_ID=datascience DECK_DISPLAY_NAME="Data Science" -a app-datascience
git push heroku main -a app-datascience

# App 2: Rust deck
heroku config:set DECK_ID=rust DECK_DISPLAY_NAME="Rust Programming" -a app-rust
git push heroku main -a app-rust
```

## Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized for size)
cargo build --release
```

### Database Management

```bash
# Rebuild database (deletes and recreates)
cargo run -- --rebuild-deck-id deck

# The database is auto-created from content files on first run
# Subsequent runs reuse the existing database for fast startup
```

### Code Statistics

```bash
tokei --compact --exclude assets --exclude static --exclude flashcards_staging
```

Last update: 2025-12-09

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 HTML                      3          181          161            8           12
 Markdown                  2          347            0          241          106
 Rust                     16         1256          942          110          204
 TOML                      1           62           36           13           13
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                    22         2126         1188          548          390
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Technology Stack

- **Web Framework:** Axum 0.7
- **Database:** SQLite with FTS5 (full-text search)
- **Templates:** Askama
- **Markdown:** pulldown-cmark
- **Syntax Highlighting:** syntect
- **Session Management:** tower-sessions
- **CLI:** clap

## License

MIT License - see [LICENSE](LICENSE) file for details.


## Contributing
This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved.