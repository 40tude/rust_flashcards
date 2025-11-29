# CLI Arguments Implementation Plan

## Objective
Add CLI argument parsing to rust-flashcards app with `--rebuild-db` flag while establishing foundation for future args (--database, --port, etc.)

## Approach: Option 2 Adapted
Standard CLI with essential args, starting with rebuild-db only but structured for easy extension.

## Implementation Steps

### 1. Add clap dependency to Cargo.toml
- Add `clap = { version = "4.5", features = ["derive"] }`
- Use derive API for clean, declarative CLI definition

### 2. Create src/cli.rs module
Define CLI structure:
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-flashcards")]
#[command(version)]
#[command(about = "Flashcard web application with full-text search", long_about = None)]
pub struct Cli {
    /// Rebuild database by deleting existing DB file before startup
    #[arg(short = 'r', long = "rebuild-db")]
    pub rebuild_db: bool,

    // Future args (commented out for now):
    // /// Database file path (overrides DATABASE_URL)
    // #[arg(long, value_name = "PATH")]
    // pub database: Option<String>,
    //
    // /// Server port (overrides PORT env var)
    // #[arg(short, long, value_name = "PORT")]
    // pub port: Option<u16>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
```

### 3. Modify src/main.rs

**Add module declaration** (after existing use statements):
```rust
mod cli;
use cli::Cli;
```

**Parse CLI args early** (before dotenvy::dotenv()):
```rust
let cli_args = Cli::parse_args();
```

**Handle --rebuild-db flag** (after config loading, before pool creation):
```rust
// Handle database rebuild if requested
if cli_args.rebuild_db {
    let db_path = std::path::Path::new(&config.database_url);
    if db_path.exists() {
        info!("Deleting existing database: {}", config.database_url);
        std::fs::remove_file(db_path)
            .unwrap_or_else(|e| {
                error!("Failed to delete database: {}", e);
                std::process::exit(1);
            });
        info!("Database deleted, will rebuild from content");
    } else {
        warn!("Database file not found, nothing to delete");
    }
}
```

### 4. Update src/lib.rs (if exists) or main.rs
Add `mod cli;` to module declarations

### 5. Update CLAUDE.md project docs
Add CLI usage examples:
```bash
# Rebuild database
cargo run -- --rebuild-db
cargo run -- -r

# Show help
cargo run -- --help

# Show version
cargo run -- --version
```

## Files to Modify

1. **Cargo.toml** - Add clap dependency
2. **src/cli.rs** - NEW file, CLI arg definitions
3. **src/main.rs** - Parse args, handle rebuild logic
4. **CLAUDE.md** - Update "Build and Run Commands" section

## Key Design Decisions

### Why Option 2 (Standard CLI)?
- Professional foundation without over-engineering
- Easy to add --database, --port later
- clap's derive API gives --help/--version for free
- Follows Rust CLI best practices

### Rebuild Implementation: File Deletion
- Simple: delete .db file before pool creation
- Triggers natural flow: pool creation → schema init → empty check → content load
- No need to modify db queries or add new functions
- Clear log messages for user feedback

### Priority Order (for future args)
CLI args > Environment vars > Defaults in config.rs
- Maintains backward compatibility (env vars still work)
- CLI overrides for dev/test flexibility

### Error Handling
- Failed deletion → log error + exit(1)
- File not found → warning (not error, user may want fresh start)

## Testing Checklist (Post-Implementation)

- [ ] `cargo run -- --help` displays usage
- [ ] `cargo run -- --version` shows version from Cargo.toml
- [ ] `cargo run -- --rebuild-db` deletes DB and recreates
- [ ] `cargo run -- -r` (short form) works
- [ ] Normal `cargo run` works without args (backward compatibility)
- [ ] Rebuild with non-existent DB shows warning, continues normally
- [ ] Invalid args show helpful error + usage
- [ ] Production build: `cargo build --release` preserves CLI functionality

## Future Extensions (Not Implemented Now)

Ready to add when needed:
```rust
// In Cli struct:
#[arg(long, value_name = "PATH")]
pub database: Option<String>,

#[arg(short, long, value_name = "PORT")]
pub port: Option<u16>,

// In main.rs, override config:
if let Some(db_path) = cli_args.database {
    config.database_url = db_path;
}
if let Some(port) = cli_args.port {
    config.port = port;
}
```
