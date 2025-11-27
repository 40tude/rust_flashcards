# Hide/Reveal Answer Implementation Plan

## Overview

Add hide/reveal functionality for flashcard answers with different behavior for image-only cards (PNG) vs text cards.

## Requirements

**Case 1 - PNG-only cards (empty question):**
- Display answer immediately
- Single ENTER/click → next card

**Case 2 - Text cards (question exists):**
- Hide answer initially
- First ENTER/click → reveal answer + button text changes "Show Answer" → "Next"
- Second ENTER/click → next card

**Case 3 - Search results:**
- Same behavior as regular cards
- Button navigates to `/search_results` instead of `/next`

**Constraints:**
- Answer state resets on each card load (always start hidden)
- Only ENTER key for keyboard shortcuts
- Hidden answer displays as blank space (no placeholder)
- PNG-only detection: `question_html == "<h3>Question :</h3>\n"` (from images.rs:52)

## Implementation Approach

**Client-side JavaScript** for hide/reveal logic to avoid unnecessary server round-trips and enable instant reveal without page reload. Backend only adds boolean flag to indicate PNG-only cards.

## File Changes

### 1. Backend Changes

#### `src/routes/index.rs`

**Add helper function before `IndexTemplate`:**
```rust
fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}
```

**Modify `IndexTemplate` struct (lines 11-19):**
```rust
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_cards: i64,
    is_png_only: bool,  // ADD THIS
}
```

**Update template instantiation (lines 61-67):**
```rust
let template = IndexTemplate {
    category: card.category.clone(),
    subcategory: card.subcategory.clone(),
    q_html: card.question_html.clone(),
    a_html: card.answer_html,
    nb_cards,
    is_png_only: is_png_only_card(&card.question_html),  // ADD THIS
};
```

#### `src/routes/search_results.rs`

**Add same helper function before `SearchResultsTemplate`:**
```rust
fn is_png_only_card(question_html: &str) -> bool {
    question_html.trim() == "<h3>Question :</h3>"
}
```

**Modify `SearchResultsTemplate` struct (lines 7-15):**
```rust
#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_results: i64,
    is_png_only: bool,  // ADD THIS
}
```

**Update template instantiation (lines 58-64 and 79-85):**
```rust
// Both occurrences need this field added:
let template = SearchResultsTemplate {
    category: card.category.clone(),
    subcategory: card.subcategory.clone(),
    q_html: card.question_html.clone(),
    a_html: card.answer_html,
    nb_results: count,
    is_png_only: is_png_only_card(&card.question_html),  // ADD THIS
};
```

### 2. Frontend Changes

#### `static/js/flashcard.js` (NEW FILE)

Create new JavaScript file:

```javascript
(function() {
    const answerDiv = document.getElementById('answer-content');
    const actionBtn = document.getElementById('action-btn');
    const isPngOnly = actionBtn.dataset.pngOnly === 'true';

    let isRevealed = isPngOnly;

    // Initialize UI state
    if (!isPngOnly) {
        answerDiv.style.display = 'none';
        actionBtn.textContent = 'Show Answer';
    }

    function handleAction() {
        if (!isRevealed) {
            // First action: reveal answer
            answerDiv.style.display = 'block';
            actionBtn.textContent = 'Next';
            isRevealed = true;
        } else {
            // Second action: navigate
            window.location.href = actionBtn.href;
        }
    }

    // Button click
    actionBtn.addEventListener('click', function(e) {
        if (!isRevealed) {
            e.preventDefault();
            handleAction();
        }
    });

    // ENTER key
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleAction();
        }
    });
})();
```

#### `templates/index.html`

**Modify line 25 - conditionally show question:**
```html
{% if !is_png_only %}
<div class="mt-3">{{ q_html|safe }}</div>
{% endif %}
```

**Modify line 26 - add ID to answer:**
```html
<div id="answer-content" class="mt-3">{{ a_html|safe }}</div>
```

**Modify lines 27-29 - update button:**
```html
<div class="mt-3">
    <a href="/next"
       id="action-btn"
       class="btn btn-primary"
       data-png-only="{{ is_png_only }}"
       autofocus>{% if is_png_only %}Next{% else %}Show Answer{% endif %}</a>
</div>
```

**Add before `</body>` (after line 30):**
```html
<script src="/static/js/flashcard.js"></script>
```

#### `templates/search_results.html`

**Same changes as index.html:**

**Modify line 25:**
```html
{% if !is_png_only %}
<div class="mt-3">{{ q_html|safe }}</div>
{% endif %}
```

**Modify line 26:**
```html
<div id="answer-content" class="mt-3">{{ a_html|safe }}</div>
```

**Modify lines 27-30:**
```html
<div class="mt-3">
    <a href="/search_results"
       id="action-btn"
       class="btn btn-primary"
       data-png-only="{{ is_png_only }}"
       autofocus>{% if is_png_only %}Next{% else %}Show Answer{% endif %}</a>
    <a href="/" class="btn btn-secondary">Home</a>
</div>
```

**Add before `</body>` (after line 31):**
```html
<script src="/static/js/flashcard.js"></script>
```

## Implementation Steps

1. **Backend - PNG detection:** Add `is_png_only_card()` helper and modify template structs in `index.rs` and `search_results.rs`
2. **Backend - template data:** Add `is_png_only` field calculation when instantiating templates
3. **Frontend - JavaScript:** Create `static/js/flashcard.js` with reveal logic
4. **Frontend - index template:** Update `templates/index.html` with conditional question, IDs, button attributes, and script include
5. **Frontend - search template:** Update `templates/search_results.html` with same changes (adjust button href to `/search_results`)

## Testing Checklist

- [ ] PNG card displays answer immediately, button shows "Next", single click/ENTER → next card
- [ ] Text card hides answer, button shows "Show Answer", first click/ENTER reveals, second → next card
- [ ] Search results same behavior as regular cards, button navigates to `/search_results`
- [ ] ENTER key works consistently
- [ ] Button text changes correctly after reveal
- [ ] State resets when navigating to new card
- [ ] No JavaScript errors in console

## Notes

- Static files already served via `/static` route (main.rs:60)
- JavaScript will be cached by browser
- No session state needed (client-side only)
- Progressive enhancement: without JS, buttons still work as links
