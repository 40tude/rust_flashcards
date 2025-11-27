# Landing Page Redesign with Advanced Filtering

## Overview

Transform landing page from direct card display to filter selection form. Users configure filters (keywords, categories, subcategories, images) then start practice session with filtered card set.

**User Flow:**
1. Visit `/` → see filter form with all checkboxes checked by default (or previous session state)
2. Configure filters: enter keywords, select categories/subcategories, toggle image inclusion
3. Press "Practice" button or ENTER in keywords field → start filtered practice session
4. View filtered cards at `/practice` with "Next" navigation
5. Return to `/` to modify filters (state persists)

**Filter Behavior (per user requirements):**
- Keywords search scoped to selected categories/subcategories
- "All categories" unchecked → show scrollable category list
- "All subcategories" unchecked when "All categories" checked → disable subcategory list
- "All subcategories" unchecked when specific categories selected → show only their subcategories
- All filters combinable (AND logic)
- State persists in session across visits

---

## Important: Database Loading Behavior

**Current system loads content on every startup. Keep this behavior unchanged.**
- The plan assumes existing startup logic remains (clears + reloads content)
- Categories/subcategories queried dynamically from current DB state
- If user deletes DB and restarts → content reloads from markdown files
- Landing page always queries live DB state for categories/subcategories

**Note:** Conditional loading (only load if DB empty) is separate optimization, not part of this plan.

---

## Implementation Steps

### 1. Database Layer - New Filter Queries

**File:** `src/db/models.rs`

Add `FilterCriteria` struct:
```rust
pub struct FilterCriteria {
    pub keywords: Vec<String>,
    pub categories: Option<Vec<String>>,  // None = all
    pub subcategories: Option<Vec<String>>, // None = all
    pub include_images: bool,
}
```

**File:** `src/db/queries.rs`

Add 4 new functions:

**1. `get_distinct_categories(pool: &DbPool) -> Result<Vec<String>>`**
- Query: `SELECT DISTINCT category FROM flashcards WHERE category IS NOT NULL ORDER BY category`
- Returns sorted list of unique categories

**2. `get_distinct_subcategories(pool: &DbPool, categories: Option<&[String]>) -> Result<Vec<String>>`**
- If categories None: `SELECT DISTINCT subcategory FROM flashcards WHERE subcategory IS NOT NULL ORDER BY subcategory`
- If categories Some: Add `AND category IN (?, ?, ...)` to WHERE clause
- Returns subcategories filtered by selected categories

**3. `get_filtered_random_flashcard(pool: &DbPool, exclude: &[i64], filters: &FilterCriteria) -> Result<Option<Flashcard>>`**
- Core query combining all filter criteria
- Use FTS5 subquery for keywords: `id IN (SELECT id FROM flashcards_fts WHERE flashcards_fts MATCH 'keyword1 AND keyword2')`
- Add category filter: `category IN (?, ?, ...)` when specific categories selected
- Add subcategory filter: `subcategory IN (?, ?, ...)` when specific subcategories selected
- Add image filter: `question_html != '<h3>Question :</h3>'` when include_images=false
- Exclude seen IDs: `id NOT IN (?, ?, ...)`
- Build WHERE clause dynamically based on FilterCriteria fields
- End with `ORDER BY RANDOM() LIMIT 1`

**4. `count_filtered_flashcards(pool: &DbPool, filters: &FilterCriteria) -> Result<i64>`**
- Same WHERE clause as get_filtered_random_flashcard but use COUNT(*)
- Returns total matching cards for display

**SQL Construction Pattern:**
```sql
SELECT ... FROM flashcards
WHERE 1=1
  AND (id IN (SELECT id FROM flashcards_fts WHERE flashcards_fts MATCH ?) OR no_keywords)
  AND (category IN (?, ...) OR all_categories)
  AND (subcategory IN (?, ...) OR all_subcategories)
  AND (question_html != '<h3>Question :</h3>' OR include_images)
  AND id NOT IN (?, ...)
ORDER BY RANDOM() LIMIT 1
```

---

### 2. Session Management - Extended State

**File:** `src/session/mod.rs`

Extend `SessionData` struct:
```rust
pub struct SessionData {
    pub seen_ids: Vec<i64>,
    pub searched_ids: Vec<i64>,  // Keep for compatibility
    pub keywords: Vec<String>,   // Keep for compatibility
    pub nb_cards: Option<i64>,

    // NEW: Filter state (persists across visits)
    pub filter_keywords: Vec<String>,
    pub filter_categories: Option<Vec<String>>,  // None = all
    pub filter_subcategories: Option<Vec<String>>, // None = all
    pub filter_include_images: bool,
    pub filtered_card_count: Option<i64>,
}
```

Update `Default` impl to initialize new fields:
- `filter_keywords`: empty vec
- `filter_categories`: None (all categories)
- `filter_subcategories`: None (all subcategories)
- `filter_include_images`: true (include images by default)
- `filtered_card_count`: None

---

### 3. Landing Page Route - Filter Form

**File:** `src/routes/landing.rs` (rename from index.rs)

**GET `/` handler:**
```rust
pub async fn landing(
    State(pool): State<DbPool>,
    session: Session,
) -> Result<impl IntoResponse, String> {
    let session_data: SessionData = session.get("data").await?.unwrap_or_default();

    // Query available categories and subcategories
    let categories = queries::get_distinct_categories(&pool)?;

    // Get subcategories based on current filter state
    let subcategories = if let Some(ref cats) = session_data.filter_categories {
        queries::get_distinct_subcategories(&pool, Some(cats))?
    } else {
        queries::get_distinct_subcategories(&pool, None)?
    };

    let total_count = queries::get_total_count(&pool)?;

    // Count filtered cards if filters active
    let filtered_count = if has_active_filters(&session_data) {
        let criteria = FilterCriteria::from_session(&session_data);
        Some(queries::count_filtered_flashcards(&pool, &criteria)?)
    } else {
        None
    };

    let template = LandingTemplate {
        categories,
        subcategories,
        total_count,
        filtered_count,
        filter_keywords: session_data.filter_keywords.join(" "),
        filter_categories: session_data.filter_categories,
        filter_subcategories: session_data.filter_subcategories,
        filter_include_images: session_data.filter_include_images,
        error_message: None,
    };

    Ok(Html(template.render()?))
}
```

**POST `/apply_filters` handler:**
```rust
#[derive(Deserialize)]
pub struct FilterForm {
    pub keywords: String,
    pub all_categories: Option<String>,  // "on" if checked
    pub categories: Option<Vec<String>>,
    pub all_subcategories: Option<String>,
    pub subcategories: Option<Vec<String>>,
    pub all_images: Option<String>,
}

pub async fn apply_filters(
    State(pool): State<DbPool>,
    session: Session,
    Form(form): Form<FilterForm>,
) -> Result<impl IntoResponse, String> {
    let mut session_data: SessionData = session.get("data").await?.unwrap_or_default();

    // Parse form data into session state
    session_data.filter_keywords = form.keywords
        .split_whitespace()
        .map(String::from)
        .collect();

    session_data.filter_categories = if form.all_categories.is_some() {
        None
    } else {
        form.categories
    };

    session_data.filter_subcategories = if form.all_subcategories.is_some() {
        None
    } else {
        form.subcategories
    };

    session_data.filter_include_images = form.all_images.is_some();

    // Reset seen cards for new practice session
    session_data.seen_ids.clear();
    session_data.filtered_card_count = None;

    session.insert("data", &session_data).await?;

    Ok(Redirect::to("/practice"))
}
```

Helper function:
```rust
fn has_active_filters(session: &SessionData) -> bool {
    !session.filter_keywords.is_empty()
        || session.filter_categories.is_some()
        || session.filter_subcategories.is_some()
        || !session.filter_include_images
}
```

---

### 4. Practice Route - Filtered Card Display

**File:** `src/routes/practice.rs` (NEW)

```rust
pub async fn practice(
    State(pool): State<DbPool>,
    session: Session,
) -> Result<impl IntoResponse, String> {
    let mut session_data: SessionData = session.get("data").await?.unwrap_or_default();

    // Build filter criteria from session
    let criteria = FilterCriteria {
        keywords: session_data.filter_keywords.clone(),
        categories: session_data.filter_categories.clone(),
        subcategories: session_data.filter_subcategories.clone(),
        include_images: session_data.filter_include_images,
    };

    // Get or calculate filtered card count
    let nb_cards = if let Some(cached) = session_data.filtered_card_count {
        cached
    } else {
        let count = queries::count_filtered_flashcards(&pool, &criteria)?;
        session_data.filtered_card_count = Some(count);
        count
    };

    if nb_cards == 0 {
        return Err("No cards match your filters".to_string());
    }

    // Reset seen_ids if all filtered cards seen
    if session_data.seen_ids.len() >= nb_cards as usize {
        session_data.seen_ids.clear();
    }

    // Get random filtered flashcard
    let card = queries::get_filtered_random_flashcard(&pool, &session_data.seen_ids, &criteria)?
        .ok_or("No cards available")?;

    session_data.seen_ids.push(card.id);
    session.insert("data", &session_data).await?;

    let template = PracticeTemplate {
        category: card.category.clone(),
        subcategory: card.subcategory.clone(),
        q_html: card.question_html.clone(),
        a_html: card.answer_html,
        nb_cards,
        is_png_only: is_png_only_card(&card.question_html),
    };

    Ok(Html(template.render()?))
}

fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}
```

---

### 5. Frontend Templates

**File:** `templates/landing.html` (NEW)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" rel="stylesheet">
    <title>Flashcards - Practice Setup</title>
    <link rel='shortcut icon' href="/static/favicon.png" />
    <link rel="stylesheet" href="/static/css/default.css">
</head>
<body>
    <div class="container">
        <h1 class="mt-5">Data Science Flashcards</h1>
        <p class="text-muted">
            {% if let Some(fc) = filtered_count %}
                {{ fc }} matching cards
            {% else %}
                {{ total_count }} total cards
            {% endif %}
        </p>

        {% if let Some(err) = error_message %}
        <div class="alert alert-danger">{{ err }}</div>
        {% endif %}

        <form method="post" action="/apply_filters" id="filter-form" class="mt-4">
            <!-- Keywords -->
            <div class="form-group">
                <label for="keywords-input"><strong>Keywords (space separated):</strong></label>
                <input type="text"
                       name="keywords"
                       id="keywords-input"
                       class="form-control"
                       value="{{ filter_keywords }}"
                       placeholder="Enter keywords..."
                       autofocus>
            </div>

            <!-- Categories -->
            <div class="form-group">
                <div class="form-check">
                    <input type="checkbox"
                           name="all_categories"
                           id="all-categories-cb"
                           class="form-check-input"
                           {% if filter_categories is none %}checked{% endif %}>
                    <label class="form-check-label" for="all-categories-cb">
                        <strong>All categories</strong>
                    </label>
                </div>
                <div id="category-list"
                     class="scrollable-list mt-2"
                     style="{% if filter_categories is none %}display:none{% endif %}">
                    {% for cat in categories %}
                    <div class="form-check">
                        <input type="checkbox"
                               name="categories"
                               value="{{ cat }}"
                               class="form-check-input category-cb"
                               {% if let Some(ref cats) = filter_categories %}
                                   {% if cats.contains(&cat) %}checked{% endif %}
                               {% endif %}>
                        <label class="form-check-label">{{ cat }}</label>
                    </div>
                    {% endfor %}
                </div>
            </div>

            <!-- Subcategories -->
            <div class="form-group">
                <div class="form-check">
                    <input type="checkbox"
                           name="all_subcategories"
                           id="all-subcategories-cb"
                           class="form-check-input"
                           {% if filter_subcategories is none %}checked{% endif %}
                           {% if filter_categories is none %}disabled{% endif %}>
                    <label class="form-check-label" for="all-subcategories-cb">
                        <strong>All subcategories</strong>
                    </label>
                </div>
                <div id="subcategory-list"
                     class="scrollable-list mt-2"
                     style="{% if filter_subcategories is none or filter_categories is none %}display:none{% endif %}">
                    {% for subcat in subcategories %}
                    <div class="form-check">
                        <input type="checkbox"
                               name="subcategories"
                               value="{{ subcat }}"
                               class="form-check-input"
                               {% if let Some(ref subcats) = filter_subcategories %}
                                   {% if subcats.contains(&subcat) %}checked{% endif %}
                               {% endif %}>
                        <label class="form-check-label">{{ subcat }}</label>
                    </div>
                    {% endfor %}
                </div>
            </div>

            <!-- Images -->
            <div class="form-check">
                <input type="checkbox"
                       name="all_images"
                       id="all-images-cb"
                       class="form-check-input"
                       {% if filter_include_images %}checked{% endif %}>
                <label class="form-check-label" for="all-images-cb">
                    <strong>All images</strong> (include image-only cards)
                </label>
            </div>

            <!-- Submit Button -->
            <div class="mt-4">
                <button type="submit" class="btn btn-primary btn-lg">Practice</button>
            </div>
        </form>
    </div>
    <script src="/static/js/filters.js"></script>
</body>
</html>
```

**File:** `templates/practice.html` (NEW - copy of current index.html with modifications)

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
        {% if let Some(cat) = category %}
        <div class="mt-2">
            <p class="text-muted">
                <small>
                    <strong>{{ cat }}{% if let Some(subcat) = subcategory %} - {{ subcat }}{% endif %}</strong>
                </small>
            </p>
        </div>
        {% endif %}
        {% if !is_png_only %}
        <div class="mt-3">{{ q_html|safe }}</div>
        {% endif %}
        <div id="answer-content" class="mt-3">{{ a_html|safe }}</div>
        <div class="mt-3">
            <a href="/practice"
               id="action-btn"
               class="btn btn-primary"
               data-png-only="{{ is_png_only }}"
               autofocus>{% if is_png_only %}Next{% else %}Show Answer{% endif %}</a>
            <a href="/" class="btn btn-secondary ml-2">Back to Filters</a>
        </div>
    </div>
    <script src="/static/js/flashcard.js"></script>
</body>
</html>
```

---

### 6. Frontend JavaScript

**File:** `static/js/filters.js` (NEW)

```javascript
(function() {
    const allCatCb = document.getElementById('all-categories-cb');
    const catList = document.getElementById('category-list');
    const categoryCbs = document.querySelectorAll('.category-cb');
    const allSubcatCb = document.getElementById('all-subcategories-cb');
    const subcatList = document.getElementById('subcategory-list');
    const form = document.getElementById('filter-form');
    const keywordsInput = document.getElementById('keywords-input');

    // All categories checkbox toggle
    allCatCb.addEventListener('change', function() {
        if (this.checked) {
            catList.style.display = 'none';
            categoryCbs.forEach(cb => cb.checked = false);
            // Disable and check all subcategories when all categories selected
            allSubcatCb.disabled = true;
            allSubcatCb.checked = true;
            subcatList.style.display = 'none';
        } else {
            catList.style.display = 'block';
            allSubcatCb.disabled = false;
        }
    });

    // Category checkboxes change - ensure at least all or some selected
    categoryCbs.forEach(cb => {
        cb.addEventListener('change', function() {
            const anySelected = Array.from(categoryCbs).some(c => c.checked);
            if (!anySelected && !allCatCb.checked) {
                // Auto-check "All categories" if nothing selected
                allCatCb.checked = true;
                catList.style.display = 'none';
                allSubcatCb.disabled = true;
                allSubcatCb.checked = true;
                subcatList.style.display = 'none';
            }
        });
    });

    // All subcategories checkbox toggle
    allSubcatCb.addEventListener('change', function() {
        if (this.checked) {
            subcatList.style.display = 'none';
        } else {
            subcatList.style.display = 'block';
        }
    });

    // ENTER key in keywords field submits form
    keywordsInput.addEventListener('keydown', function(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            form.submit();
        }
    });
})();
```

**File:** `static/css/default.css` (ADD)

```css
.scrollable-list {
    max-height: 250px;
    overflow-y: auto;
    border: 1px solid #dee2e6;
    border-radius: 0.25rem;
    padding: 0.75rem;
    background-color: #f8f9fa;
}

.scrollable-list .form-check {
    margin-bottom: 0.5rem;
}
```

---

### 7. Database Conditional Loading (Optimization)

**File:** `src/db/queries.rs`

Add helper function:
```rust
pub fn is_database_empty(pool: &DbPool) -> Result<bool> {
    let count = get_total_count(pool)?;
    Ok(count == 0)
}
```

**File:** `src/main.rs`

Update startup sequence to only load content if DB empty:
```rust
// Initialize database schema
db::init_database(&pool)?;

// Only load content if database is empty
if db::queries::is_database_empty(&pool)? {
    tracing::info!("Empty database, loading content...");
    content::load_markdown(&pool, "./static/md")?;
    content::load_images(&pool, "./static/png")?;
    db::queries::populate_fts_table(&pool)?;
} else {
    tracing::info!("Database exists with {} cards, skipping content load",
                   db::queries::get_total_count(&pool)?);
}
```

**Benefits:**
- Fast startup when DB exists
- Only reload when DB missing/empty
- Categories/subcategories still dynamic (queried from DB on each landing page visit)

---

### 8. Router Configuration

**File:** `src/main.rs`

Update router:
```rust
let app = Router::new()
    .route("/", get(routes::landing))  // CHANGED: filter form
    .route("/apply_filters", post(routes::apply_filters))  // NEW
    .route("/practice", get(routes::practice))  // NEW: filtered cards
    .route("/reset_session", get(routes::reset_session))
    // REMOVE: .route("/next", get(routes::next))
    // REMOVE: .route("/search", get(routes::search_form).post(routes::search_submit))
    // REMOVE: .route("/search_results", get(routes::search_results))
    .nest_service("/static", ServeDir::new("static"))
    .layer(session_layer)
    .with_state(pool);
```

**File:** `src/routes/mod.rs`

```rust
pub mod debug;
pub mod landing;  // RENAMED from index
pub mod practice; // NEW

pub use debug::reset_session;
pub use landing::{landing, apply_filters};
pub use practice::practice;

// REMOVE: pub mod index, next, search, search_results
```

---

## Implementation Order

1. **Database Layer** (queries.rs, models.rs)
   - Add FilterCriteria struct
   - Add is_database_empty() helper
   - Add 4 new query functions (get_distinct_categories, get_distinct_subcategories, get_filtered_random_flashcard, count_filtered_flashcards)
   - Test queries independently

2. **Session Management** (session/mod.rs)
   - Extend SessionData with filter fields
   - Update Default impl

3. **Startup Optimization** (main.rs)
   - Add conditional content loading (only if DB empty)
   - Ensures fast startup when DB exists

4. **Practice Route** (routes/practice.rs, templates/practice.html)
   - Create new practice route using filtered queries
   - Copy index.html → practice.html with minor changes

5. **Landing Page Backend** (routes/landing.rs)
   - Rename index.rs → landing.rs
   - Implement GET handler (display filter form)
   - Implement POST handler (save filters, redirect)

6. **Landing Page Frontend** (templates/landing.html, static/js/filters.js, static/css/default.css)
   - Create landing.html with filter form
   - Create filters.js for checkbox interactions
   - Add CSS for scrollable lists

7. **Router Updates** (main.rs, routes/mod.rs)
   - Update route configuration
   - Remove deprecated routes (next, search, search_results)

8. **Testing**
   - Test filter combinations
   - Test edge cases (no matches, empty categories)
   - Test session persistence
   - Test DB empty vs exists startup behavior

---

## Edge Cases

1. **No cards match filters:** Show error message on landing page after POST, don't redirect
2. **Empty categories/subcategories:** Show "No categories available" message
3. **All filtered cards seen:** Reset seen_ids automatically
4. **Invalid session data:** Validate and clean session state on landing page load
5. **"All categories" + specific subcategories:** Disable subcategory list per requirements

---

## Critical Files to Modify

1. `src/db/queries.rs` - Add 4 new query functions
2. `src/db/models.rs` - Add FilterCriteria struct
3. `src/session/mod.rs` - Extend SessionData
4. `src/routes/landing.rs` - Rename from index.rs, rewrite handlers
5. `src/routes/practice.rs` - NEW file, filtered card display
6. `templates/landing.html` - NEW file, filter form
7. `templates/practice.html` - NEW file, copy of index.html
8. `static/js/filters.js` - NEW file, checkbox interactions
9. `static/css/default.css` - Add scrollable-list styles
10. `src/main.rs` - Update router configuration
11. `src/routes/mod.rs` - Update module exports

Files to DELETE:
- `src/routes/next.rs`
- `src/routes/search.rs`
- `src/routes/search_results.rs`
- `templates/search.html`
- `templates/search_results.html`