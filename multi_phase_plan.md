# Multi-Phase Implementation Plan: Python Flask → Rust Axum

## STATUS: Phases 1-4 COMPLETED ✅, Ready for Phase 5

---

## Project Goal
Réécrire `py-flashcards-2/` (Python Flask) en Rust avec Axum. Full feature parity. Déploiement Heroku. Supprimer Python à la fin.

## User Requirements
- Full feature parity (toutes features d'un coup)
- SQLite + FTS5 pour DB
- Axum web framework
- .env pour config
- Juste match Python (pas d'améliorations pour l'instant)

---

## PHASE 1: Foundation & Setup ✅ COMPLETED

### Files Created
1. ✅ `Cargo.toml` - Edition "2021", toutes dépendances ajoutées
2. ✅ `.env` - FLASHCARDS_SECRET_KEY, PORT, DATABASE_URL, RUST_LOG
3. ✅ `src/config.rs` - Load env vars
4. ✅ `src/db/mod.rs` - Module exports
5. ✅ `src/db/models.rs` - Flashcard struct
6. ✅ `src/db/connection.rs` - r2d2 pool
7. ✅ `src/db/schema.rs` - CREATE TABLE flashcards + flashcards_fts

### Success Criteria Met
- ✅ Project compiles
- ✅ DB connection pool créé
- ✅ Tables créées (flashcards + flashcards_fts)
- ✅ Config loads from .env

### Test Result
```
cargo run
INFO Starting rust-flashcards application
INFO Configuration loaded: port=8080, database=./flashcards.db
INFO Database connection pool created
INFO Created flashcards table
INFO Created flashcards_fts virtual table
INFO Phase 1 complete: Foundation & Setup successful!
```

---

## PHASE 2: Content Loading System ✅ COMPLETED

### Goal
Parse markdown files + PNG images, populate database

### Files to Create
1. `src/content/mod.rs` - Module exports
2. `src/content/markdown.rs` - Parse markdown avec pulldown-cmark + syntect
3. `src/content/images.rs` - Scan PNG avec walkdir
4. `src/db/queries.rs` - Insert functions
5. **ACTION**: Copier `py-flashcards-2/static/` → `./static/`

### Implementation Details

**markdown.rs** doit:
1. Scan récursif `./static/md/**/*.md` avec walkdir
2. Strip HTML comments: regex `<!--.*?-->`
3. Parse Q&A: regex `Question\s*:\s*(.*?)\nAnswer\s*:\s*(.*?)(?=\nQuestion|\Z)`
4. Convert markdown→HTML: pulldown-cmark avec Options:
   - `ENABLE_TABLES | ENABLE_STRIKETHROUGH | ENABLE_FOOTNOTES`
5. Syntax highlighting: syntect avec theme "InspiredGitHub"
6. Prepend "###Question :\n" et "###Answer :\n"

**images.rs** doit:
1. Scan récursif `./static/png/**/*.png`
2. Pour chaque PNG:
   - question_html = "###Question :\n" (vide)
   - answer_html = "###Answer :\n<img src='/static/png/...' class='img-fluid'>"

**queries.rs** doit avoir:
```rust
pub fn insert_flashcard(pool: &DbPool, q: &str, a: &str) -> Result<i64>
pub fn clear_flashcards(pool: &DbPool) -> Result<()>
```

### Success Criteria Met
- ✅ Scan ./static/md/**/*.md récursif
- ✅ Parse format "Question: / Answer:" avec regex (split-based parsing)
- ✅ Markdown→HTML avec syntax highlighting (syntect)
- ✅ PNG files scannés, paths stockés
- ✅ DB populated au startup (405 markdown + 300 PNG = 705 total)

### Files: ~3 nouveaux, ~300-400 lignes

---

## PHASE 3: Web Server & Static Files ✅ COMPLETED

### Goal
Setup Axum server avec static file serving

### Files to Create/Modify
1. `src/main.rs` - Axum server, routing skeleton
2. `src/routes/mod.rs` - Route exports
3. `Procfile` - `web: ./target/release/rust-flashcards`
4. `.gitignore` - Add `target/`, `.env`, `flashcards.db`

### main.rs Structure
```rust
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let config = Config::from_env()?;
    let pool = db::create_pool(&config.database_url)?;
    db::init_database(&pool)?;

    // CALL content loading here:
    // content::load_markdown(&pool, "./static/md")?;
    // content::load_images(&pool, "./static/png")?;
    // db::populate_fts_table(&pool)?;

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

### Success Criteria Met
- ✅ Server starts, bind to PORT (0.0.0.0:8080)
- ✅ Static files served at /static/* (tested favicon.png, default.css)
- ✅ Logging/tracing configured (tracing_subscriber)
- ✅ DB initializes au startup (705 flashcards loaded)
- ✅ Access http://localhost:8080/static/favicon.png → 200 OK
- ✅ Procfile created for Heroku
- ✅ .gitignore updated (target/, .env, *.db)

### Files: ~4, ~150-200 lignes

---

## PHASE 4: Templates & Basic Route ✅ COMPLETED

### Goal
Askama templates + GET / route

### Files to Create
1. `templates/index.html` - Main flashcard template
2. `templates/search.html` - Search form
3. `templates/search_results.html` - Search results
4. `src/routes/index.rs` - GET / handler (sans session d'abord)
5. `src/routes/next.rs` - GET /next redirect
6. `src/db/queries.rs` - Add `get_random_flashcard()`, `get_total_count()`

### Template Structure (index.html)
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" rel="stylesheet">
    <title>rust-flashcards</title>
    <link rel='shortcut icon' href="/static/favicon.png" />
    <link rel="stylesheet" href="/static/css/default.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.7/MathJax.js?config=TeX-MML-AM_CHTML" async></script>
</head>
<body>
    <div class="container">
        <h1 class="mt-5">Data Science</h1>
        <p><small>{{ nb_cards }} cards</small></p>
        <div class="mt-3">{{ q_html|safe }}</div>
        <div class="mt-3">{{ a_html|safe }}</div>
        <div class="mt-3">
            <a href="/next" class="btn btn-primary">Next</a>
        </div>
    </div>
</body>
</html>
```

### Askama Template Struct
```rust
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    q_html: String,
    a_html: String,
    nb_cards: i64,
}
```

### queries.rs Functions
```rust
pub fn get_random_flashcard(pool: &DbPool, exclude: &[i64]) -> Result<Option<Flashcard>>
pub fn get_total_count(pool: &DbPool) -> Result<i64>
```

### Success Criteria Met
- ✅ Templates compile avec Askama
- ✅ GET / affiche random flashcard (705 cards)
- ✅ HTML renders avec Bootstrap + MathJax
- ✅ Syntax highlighting CSS works (default.css loaded)
- ✅ "Next" button redirect (303 → /) + shows new card
- ✅ Templates: index.html, search.html, search_results.html
- ✅ Routes: GET /, GET /next

### Files: ~5, ~250-300 lignes

---

## PHASE 5: Session Management 🔄 NEXT

### Goal
tower-sessions + seen_ids tracking

### Files to Create/Modify
1. `src/session/mod.rs` - Session helpers
2. `src/routes/index.rs` - Add session logic
3. `src/routes/debug.rs` - GET /reset_session
4. `src/main.rs` - Add session middleware

### Session Data
```rust
#[derive(Serialize, Deserialize, Default)]
struct SessionData {
    seen_ids: Vec<i64>,
    searched_ids: Vec<i64>,
    keywords: Vec<String>,
    nb_cards: Option<i64>,
}
```

### main.rs Session Setup
```rust
let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_name("flashcards_session");

let app = Router::new()
    .route("/", get(routes::index))
    .route("/next", get(routes::next))
    .route("/reset_session", get(routes::reset_session))
    .nest_service("/static", ServeDir::new("static"))
    .layer(session_layer);
```

### index.rs Logic
```rust
pub async fn index(
    State(pool): State<DbPool>,
    session: Session,
) -> Result<IndexTemplate> {
    let mut seen_ids: Vec<i64> = session.get("seen_ids")
        .await?.unwrap_or_default();

    let nb_cards: i64 = session.get("nb_cards").await?
        .unwrap_or_else(|| get_total_count(&pool).unwrap());

    if seen_ids.len() >= nb_cards as usize {
        seen_ids.clear();
    }

    let card = get_random_flashcard(&pool, &seen_ids)?
        .ok_or(anyhow!("No cards"))?;

    seen_ids.push(card.id);
    session.insert("seen_ids", &seen_ids).await?;
    session.insert("nb_cards", nb_cards).await?;

    Ok(IndexTemplate {
        q_html: card.question_html,
        a_html: card.answer_html,
        nb_cards,
    })
}
```

### Success Criteria
- [ ] Session persists across requests
- [ ] seen_ids prevents repeats
- [ ] Quand all cards seen, list resets
- [ ] nb_cards cached in session
- [ ] /reset_session clears session

### Files: ~4, ~150-200 lignes

---

## PHASE 6: Search Functionality

### Goal
FTS5 full-text search avec session tracking

### Files to Create/Modify
1. `src/routes/search.rs` - GET/POST /search
2. `src/routes/search_results.rs` - GET /search_results
3. `src/db/queries.rs` - Add `get_random_searched_flashcard()`
4. `src/db/schema.rs` - FTS5 population (déjà fait phase 1)

### FTS5 Query Function
```rust
pub fn get_random_searched_flashcard(
    pool: &DbPool,
    exclude: &[i64],
    keywords: &[String],
) -> Result<(Option<Flashcard>, i64)> {
    let conn = pool.get()?;

    // Build FTS5 query: "keyword1 AND keyword2"
    let match_query = keywords.join(" AND ");

    let where_clause = if exclude.is_empty() {
        format!("flashcards_fts MATCH '{}'", match_query)
    } else {
        format!("flashcards_fts MATCH '{}' AND id NOT IN ({})",
            match_query,
            exclude.iter().map(|_| "?").collect::<Vec<_>>().join(","))
    };

    // Count
    let count: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM flashcards_fts WHERE {}", where_clause),
        rusqlite::params_from_iter(exclude),
        |row| row.get(0),
    )?;

    // Random
    let card: Option<Flashcard> = conn.query_row(
        &format!("SELECT id, question_html, answer_html FROM flashcards_fts
                  WHERE {} ORDER BY RANDOM() LIMIT 1", where_clause),
        rusqlite::params_from_iter(exclude),
        |row| Ok(Flashcard {
            id: row.get(0)?,
            question_html: row.get(1)?,
            answer_html: row.get(2)?,
        }),
    ).optional()?;

    Ok((card, count))
}
```

### Search Routes
```rust
// GET /search
pub async fn search_form() -> SearchTemplate {
    SearchTemplate {}
}

// POST /search
pub async fn search_submit(
    session: Session,
    Form(form): Form<SearchForm>,
) -> Redirect {
    let keywords: Vec<String> = form.keywords.split_whitespace()
        .map(|s| s.to_string())
        .collect();

    session.insert("keywords", &keywords).await?;
    session.insert("searched_ids", &Vec::<i64>::new()).await?;

    Redirect::to("/search_results")
}
```

### Success Criteria
- [ ] Search form displays at /search
- [ ] POST /search parse keywords + redirect
- [ ] /search_results displays random matching card
- [ ] FTS5 query avec multiple keywords (AND logic)
- [ ] searched_ids prevents repeats
- [ ] "Home" et "Next" buttons work

### Files: ~4, ~200-250 lignes

---

## PHASE 7-9: Testing, Heroku, Cleanup

### Phase 7: Testing
- Test all routes
- Verify session persistence
- Test search avec various keywords
- Check markdown rendering (code blocks, math)
- Test PNG display
- Verify reset logic

### Phase 8: Heroku
```bash
heroku create rust-flashcards
heroku buildpacks:set emk/rust
heroku config:set FLASHCARDS_SECRET_KEY=$(openssl rand -hex 32)
git push heroku main
heroku logs --tail
```

### Phase 9: Cleanup
- Verify Rust app fully functional
- Delete `py-flashcards-2/` directory
- Update CLAUDE.md (Rust-only project)
- Final commit

---

## Critical Technical Details

### Markdown Extensions Mapping
- Python `extra` → pulldown-cmark: `ENABLE_TABLES | ENABLE_STRIKETHROUGH | ENABLE_FOOTNOTES`
- Python `codehilite` → syntect: theme "InspiredGitHub"
- Python `sane_lists` → pulldown-cmark default

### Session Quirk
Axum requires explicit `session.insert()` et `session.get()`, pas auto-serialization comme Flask

### FTS5 Query Format
```sql
SELECT * FROM flashcards_fts WHERE flashcards_fts MATCH 'keyword1 AND keyword2'
```

### Reset Logic
When `seen_ids.len() >= nb_cards`, reset to `Vec::new()`

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "trace"] }
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"
tower-sessions = "0.12"
askama = { version = "0.12", features = ["with-axum"] }
pulldown-cmark = "0.11"
syntect = "5.2"
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

---

## Project Structure (Final)

```
rust-flashcards/
├── Cargo.toml ✅
├── .env ✅
├── Procfile
├── src/
│   ├── main.rs ✅ (à modifier phases 3-6)
│   ├── config.rs ✅
│   ├── db/
│   │   ├── mod.rs ✅
│   │   ├── models.rs ✅
│   │   ├── schema.rs ✅
│   │   ├── connection.rs ✅
│   │   └── queries.rs (phase 2+)
│   ├── content/
│   │   ├── mod.rs (phase 2)
│   │   ├── markdown.rs (phase 2)
│   │   └── images.rs (phase 2)
│   ├── routes/
│   │   ├── mod.rs (phase 3)
│   │   ├── index.rs (phase 4-5)
│   │   ├── next.rs (phase 4)
│   │   ├── search.rs (phase 6)
│   │   ├── search_results.rs (phase 6)
│   │   └── debug.rs (phase 5)
│   └── session/
│       └── mod.rs (phase 5)
├── templates/
│   ├── index.html (phase 4)
│   ├── search.html (phase 4)
│   └── search_results.html (phase 4)
└── static/ (copier de py-flashcards-2/)
    ├── css/default.css
    ├── favicon.png
    ├── md/**/*.md
    └── png/**/*.png
```

---

## Quick Start Resume

Pour reprendre:

```bash
cd C:\Users\phili\OneDrive\Documents\Programmation\rust\15_rust_flashcards

# Phase 1 déjà complétée, commencer Phase 2:
# 1. Copier static/ de Python
# 2. Créer src/content/mod.rs, markdown.rs, images.rs
# 3. Créer src/db/queries.rs
# 4. Modifier src/main.rs pour appeler content loading
# 5. cargo run pour tester

cargo run  # Should work (Phase 1)
```

---

## Notes Importantes

- **Context window**: Plan multi-phase pour éviter dépassement
- **Commit après chaque phase**: Sauvegarder progrès
- **Test incrémental**: Valider avant next phase
- **CLAUDE.md updated**: Messages concis, sacrifice grammar pour concision
- **Python app**: py-flashcards-2/ reste jusqu'à Phase 9

Fin du plan.