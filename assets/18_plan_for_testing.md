# Testing Strategy Deployment Plan - Rust Flashcards

## Overview

Pragmatic testing strategy targeting **60-70% coverage** focusing on critical and complex logic.

**Configuration:**
- ✅ Advanced frameworks (rstest, proptest, fixtures)
- ✅ Mixed organization (unit tests inline, integration tests in `tests/`)
- ✅ Local testing only (no CI/CD)
- ✅ Microsoft Rust Guidelines compliant

---

## Phase 1: Infrastructure & Setup

### Dependencies to Add

**File:** `Cargo.toml`

```toml
[dev-dependencies]
rstest = "0.23"           # Parametrized testing
proptest = "1.6"          # Property-based testing
tempfile = "3"            # Temporary directories
axum-test = "15"          # HTTP testing
tokio-test = "0.4"        # Async test utilities
```

### Test Utilities to Create

**New file:** `tests/common/mod.rs`

```rust
// Test setup utilities
pub fn setup_in_memory_db() -> DbPool { /* ... */ }
pub fn create_test_pool(db_path: &str) -> DbPool { /* ... */ }
pub fn insert_test_flashcards(pool: &DbPool, cards: Vec<TestCard>) { /* ... */ }
```

**New file:** `tests/common/fixtures.rs`

```rust
// Sample markdown files, filter criteria, expected HTML patterns
pub const VALID_MARKDOWN: &str = "Question : Cat - Sub - Q?\nAnswer : A!";
pub const MALFORMED_MARKDOWN: &str = "Question without proper format\nAnswer : A";
// ... additional fixtures
```

**Complexity:** LOW-MEDIUM (in-memory SQLite setup with FTS5)

---

## Phase 2: Unit Tests - Tier 1 (Critical)

### 2.1: Tests for `src/db/queries.rs`

**Priority:** CRITICAL (complex dynamic SQL logic)

**Add at end of file:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tempfile::TempDir;

    // 20-25 tests covering:
    // - insert_flashcard: basic insertion, NULL handling
    // - clear_flashcards: verify both tables cleared
    // - populate_fts_table: FTS sync verification
    // - count_filtered_flashcards: 8-10 parametrized tests
    // - get_filtered_random_flashcard: 8-10 parametrized tests
    // - get_distinct_categories: sorting, NULL handling
    // - get_distinct_subcategories: with/without filter
}
```

**Critical test scenarios (rstest):**

```rust
#[rstest]
#[case(vec![], None, None, true, 10)]  // No filters
#[case(vec!["keyword"], None, None, true, 5)]  // Keyword only
#[case(vec![], Some(vec!["Cat1"]), None, true, 7)]  // Category only
#[case(vec![], Some(vec![]), None, true, 2)]  // Empty cats = images only
#[case(vec![], Some(vec!["Cat1"]), Some(vec!["Sub1"]), true, 3)]  // Cat + subcat
#[case(vec![], None, None, false, 8)]  // Exclude images
fn test_count_filtered(
    #[case] keywords: Vec<&str>,
    #[case] categories: Option<Vec<&str>>,
    #[case] subcategories: Option<Vec<&str>>,
    #[case] include_images: bool,
    #[case] expected_count: i64
) { /* ... */ }
```

**Expected coverage:** 75-80%

---

### 2.2: Tests for `src/content/markdown.rs`

**Priority:** CRITICAL (complex regex parsing, syntax highlighting)

**Add at end of file:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tempfile::TempDir;

    // 15-20 tests covering:
    // - markdown_to_html: basic conversion, code blocks, tables
    // - process_markdown_file: category extraction, regex edge cases
    // - load_markdown: recursive traversal, error handling
}
```

**Critical test scenarios:**

```rust
#[rstest]
#[case("Question : Cat - Sub - Q?\nAnswer  : A!", 1, Some("Cat"), Some("Sub"))]
#[case("Question : Q without category\nAnswer : A", 1, None, None)]
#[case("<!-- Comment -->\nQuestion : Cat - Sub - Q\nAnswer : A", 1, _, _)]
#[case("Question : Machine-Learning - Supervised - Q\nAnswer : A", 1, Some("Machine-Learning"), _)]
fn test_process_markdown_file(
    #[case] content: &str,
    #[case] expected_count: usize,
    #[case] expected_category: Option<&str>,
    #[case] expected_subcategory: Option<&str>
) { /* ... */ }
```

**Edge cases to test:**
- Category names with hyphens ("Machine-Learning")
- Multiple spaces in "Answer  :"
- Multiline HTML comments
- CRLF vs LF line endings

**Expected coverage:** 70-75%

---

### 2.3: Tests for `src/config.rs`

**Priority:** HIGH (critical for multi-deck support)

**Add at end of file:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Helper to manage env vars in tests
    struct EnvGuard { /* ... */ }

    // 12-15 parametrized tests for all priority combinations
}
```

**Critical test scenarios:**

```rust
#[rstest]
// (cli_deck, cli_name, env_deck, env_name, expected_deck, expected_name)
#[case(Some("rust"), Some("Rust Cards"), None, None, "rust", "Rust Cards")]
#[case(Some("rust"), None, None, None, "rust", "rust")]  // CLI deck → name = deck_id
#[case(None, None, Some("py"), Some("Python"), "py", "Python")]
#[case(Some("test"), None, Some("py"), Some("Python"), "test", "test")]  // CLI override
#[case(None, None, None, None, "deck", "deck")]  // Defaults
fn test_config_priority(
    #[case] cli_deck: Option<&str>,
    #[case] cli_name: Option<&str>,
    #[case] env_deck: Option<&str>,
    #[case] env_name: Option<&str>,
    #[case] expected_deck: &str,
    #[case] expected_name: &str
) { /* ... */ }
```

**Additional tests:**
- Port parsing (valid/invalid)
- Database URL: local .db vs external (Heroku Postgres)
- Path generation: `./static/{deck_id}/md` and `./static/{deck_id}/img`
- Backward compatibility `DECK_NAME`

**Expected coverage:** 80-85%

---

## Phase 3: Unit Tests - Tier 2 (High Value)

### 3.1: Tests for `src/routes/landing.rs`

**Priority:** HIGH (server-side validation, custom form parsing)

**Add:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Tests for:
    // - has_active_filters(): detection logic
    // - Manual form parsing (repeated field names)
    // - URL decoding ('+' → space)
    // - Validation: specific categories + no subcategories → error
    // - Validation: empty categories (images-only) + no subcategories → OK
}
```

**Expected coverage:** 60-65%

---

### 3.2: Tests for `src/content/images.rs`

**Priority:** MEDIUM (multi-deck path handling)

**Add:**

```rust
#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("./static/deck/img", "test.png", "deck", "/static/deck/img/test.png")]
    #[case("./static/rust/img", "foo/bar.webp", "rust", "/static/rust/img/foo/bar.webp")]
    fn test_process_image_file_path_generation(...) { /* ... */ }

    // Tests for:
    // - PNG and WEBP detection (case-insensitive)
    // - Recursive directory traversal
    // - Deck ID extraction
    // - Path separator conversion (Windows \ → /)
}
```

**Expected coverage:** 70-75%

---

### 3.3: Tests for `src/session/mod.rs`

**Priority:** MEDIUM (state management)

**Add:**

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_default_values() { /* ... */ }

    #[test]
    fn test_serialization_roundtrip() {
        let mut session = SessionData::default();
        session.seen_ids = vec![1, 2, 3];

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: SessionData = serde_json::from_string(&json).unwrap();

        assert_eq!(session.seen_ids, deserialized.seen_ids);
    }
}
```

**Expected coverage:** 90%+

---

## Phase 4: Integration Tests

### 4.1: Content Loading Pipeline

**New file:** `tests/integration_content_loading.rs`

```rust
use common::{setup_in_memory_db, create_sample_markdown};
use tempfile::TempDir;

#[tokio::test]
async fn test_markdown_to_database_full_pipeline() {
    // Create temp dir with markdown files
    // Load markdown → database
    // Query database → verify HTML, categories, subcategories
    // Verify FTS table populated
}

#[tokio::test]
async fn test_multi_deck_isolation() {
    // Load two decks into separate databases
    // Verify no cross-contamination
    // Verify paths use correct deck_id
}

#[tokio::test]
async fn test_content_validation_errors() {
    // Missing directories, permission denied, corrupted markdown
}
```

**Total:** 8-10 content loading integration tests

---

### 4.2: Routes & Session Management

**New file:** `tests/integration_routes.rs`

```rust
use axum_test::TestServer;

#[tokio::test]
async fn test_landing_page_renders() { /* ... */ }

#[tokio::test]
async fn test_apply_filters_redirect() {
    // POST /apply_filters → redirects to /practice
    // Verify session updated
}

#[tokio::test]
async fn test_practice_avoids_seen_cards() {
    // Mark cards as seen
    // GET /practice multiple times → verify no repeats
}

#[tokio::test]
async fn test_filter_validation_error_flow() {
    // Submit invalid filters
    // Verify redirect with error message
}

#[tokio::test]
async fn test_session_persistence_across_requests() {
    // Apply filters → navigate → return → filters preserved
}
```

**Total:** 10-12 routes/session integration tests

---

## Phase 5: Property-Based Tests

### 5.1: Config Resolution Invariants

**Add to `src/config.rs`:**

```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cli_always_overrides_env(
            cli_deck in "[a-z]{3,10}",
            env_deck in "[a-z]{3,10}"
        ) {
            // Set env var, call from_env with CLI arg
            // Assert result.deck_id == cli_deck
        }

        #[test]
        fn test_paths_match_deck_id(deck_id in "[a-z0-9_]{1,20}") {
            let config = Config::from_env(Some(deck_id.clone()), None).unwrap();
            assert_eq!(config.md_path, format!("./static/{}/md", deck_id));
        }
    }
}
```

---

### 5.2: Filter Combination Invariants

**Add to `src/db/queries.rs`:**

```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_filtered_count_le_total_count(filters in arb_filter_criteria()) {
            // Assert: count_filtered <= get_total_count
        }

        #[test]
        fn test_random_flashcard_matches_filters(filters in arb_filter_criteria()) {
            // If Some(card), verify card matches all filter criteria
        }

        #[test]
        fn test_exclude_list_respected(exclude_ids in vec(0i64..100, 0..10)) {
            // Assert returned card.id NOT IN exclude_ids
        }
    }
}
```

---

### 5.3: Path Handling Edge Cases

**Add to `src/content/images.rs`:**

```rust
#[cfg(test)]
mod proptests {
    proptest! {
        #[test]
        fn test_no_backslashes_in_html(
            filename in "[a-z0-9_/\\\\]{5,30}\\.(png|webp)"
        ) {
            // Assert HTML contains only forward slashes
        }
    }
}
```

---

## Test Execution

### PowerShell Commands

```powershell
# All tests
cargo test

# Unit tests only (inline #[cfg(test)])
cargo test --lib

# Integration tests only
cargo test --test integration_*

# Specific module
cargo test db::queries::tests

# Property-based tests (1000 cases)
$env:PROPTEST_CASES=1000; cargo test proptests

# With output
cargo test -- --nocapture --test-threads=1

# Coverage (optional, requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

---

## Implementation Priority

### Recommended order (optimal):

1. ✅ **Phase 1** (infrastructure) - CRITICAL for everything else
2. ✅ **Phase 2.3** (`config.rs`) - Quick win, relatively simple
3. ✅ **Phase 3.3** (`session/mod.rs`) - Quick win, builds confidence
4. ✅ **Phase 2.1** (`db/queries.rs`) - Complex but maximum ROI
5. ✅ **Phase 2.2** (`content/markdown.rs`) - Complex, critical
6. ✅ **Phase 3.1 + 3.2** (routes, images) - High value
7. ✅ **Phase 4** (integration) - Validates complete system
8. ✅ **Phase 5** (property-based) - Catches edge cases

---

## Coverage Targets

### By Module

| Module | Target Coverage | Justification |
|--------|-----------------|---------------|
| `db/queries.rs` | 75-80% | Critical dynamic SQL logic |
| `content/markdown.rs` | 70-75% | Complex parsing, edge cases |
| `config.rs` | 80-85% | All priority paths |
| `routes/landing.rs` | 60-65% | Validation + form parsing |
| `content/images.rs` | 70-75% | Multi-deck path handling |
| `session/mod.rs` | 90%+ | Simple structure |
| Other modules | 40-50% | Selective testing |

**Overall Target:** 65-70% (pragmatic coverage)

---

## Microsoft Rust Guidelines Compliance

### Guidelines to follow:

- ✅ **M-CANONICAL-DOCS**: Document non-trivial tests
- ✅ **M-PUBLIC-DEBUG**: Use Debug trait for assertions
- ✅ **M-PANIC-IS-STOP**: No panics in test code (except assertions)
- ✅ **M-DOCUMENTED-MAGIC**: Document magic values in tests
- ✅ **M-LINT-OVERRIDE-EXPECT**: Use `#[expect]` instead of `#[allow]`

### Test documentation format:

```rust
/// Tests config priority resolution.
///
/// # Examples
/// Verifies CLI args override environment variables.
#[rstest]
#[case(Some("rust"), None, "rust", "rust")]
fn test_config_priority(...) { /* ... */ }
```

---

## Critical Files to Modify

### Phase 1: Infrastructure

1. ✏️ `Cargo.toml` - Add test dependencies
2. ✏️ `tests/common/mod.rs` (new) - Test utilities
3. ✏️ `tests/common/fixtures.rs` (new) - Fixtures

### Phase 2-3: Unit Tests

4. ✏️ `src/db/queries.rs` - 20-25 tests (inline)
5. ✏️ `src/content/markdown.rs` - 15-20 tests (inline)
6. ✏️ `src/config.rs` - 12-15 tests (inline)
7. ✏️ `src/routes/landing.rs` - 10-12 tests (inline)
8. ✏️ `src/content/images.rs` - 6-8 tests (inline)
9. ✏️ `src/session/mod.rs` - 4-5 tests (inline)

### Phase 4: Integration Tests

10. ✏️ `tests/integration_content_loading.rs` (new) - 8-10 tests
11. ✏️ `tests/integration_routes.rs` (new) - 10-12 tests

### Phase 5: Property-Based Tests

12. ✏️ Add `mod proptests` in `config.rs`, `queries.rs`, `images.rs`

**Total:** 12 files (3 new, 9 modified)

---

## Complexity Warnings

### Expected challenges:

⚠️ **db/queries.rs**: Dynamic SQL with `Box<dyn ToSql>` - test carefully
⚠️ **content/markdown.rs**: Regex patterns - many edge cases
⚠️ **Integration tests**: Axum-test server setup - follow examples closely
⚠️ **Env vars**: Managing env vars in tests - use EnvGuard or similar-tests crate
⚠️ **FTS5**: SQLite FTS5 may not be available everywhere - document requirements

---

## Effort Estimation

| Phase | Complexity | Estimated Time | Tests Created |
|-------|------------|----------------|---------------|
| Phase 1 | MEDIUM | 2-3h | Infrastructure |
| Phase 2 | HIGH | 6-8h | 50-60 tests |
| Phase 3 | MEDIUM | 3-4h | 20-25 tests |
| Phase 4 | MEDIUM-HIGH | 4-5h | 18-22 tests |
| Phase 5 | MEDIUM | 2-3h | 6-8 tests |

**Total:** 17-23h for ~100-120 tests achieving 65-70% coverage

---

## Quick Wins (rapid start)

To begin immediately:

1. ✅ Add dependencies to `Cargo.toml`
2. ✅ Create `tests/common/mod.rs` with `setup_in_memory_db()`
3. ✅ Implement `config.rs` tests (straightforward, high value)
4. ✅ Implement `session/mod.rs` tests (simple, builds confidence)
5. ✅ Attack `db/queries.rs` (complex but maximum ROI)

---

## Conclusion

This plan provides comprehensive strategy for deploying robust tests across all critical modules. Pragmatic approach (60-70% coverage) focuses on complex logic and critical paths, maximizing ROI of time invested in testing.

Advanced frameworks (rstest, proptest) enable writing maintainable and expressive tests. Mixed organization (inline + tests/) follows Rust best practices while maintaining necessary flexibility.
