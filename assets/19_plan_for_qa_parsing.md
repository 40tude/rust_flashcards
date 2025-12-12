# Plan: Fix Markdown Parsing Bug (Keywords in Text)

## Problem

Flashcard parsing fails when question/answer text contains keywords "Question" or "Answer".

**Failing case:** `static\deck\md\00_test.md:53`
```markdown
Question : Culture - The Hitchhiker's Guide to the Galaxy - The Answer to the Ultimate Question of Life, the Universe, and Everything is?
Answer  :
42
```

Error: "Non-compliant question format" + card not displayed.

## Root Cause

`src\content\markdown.rs`:
- **Line 56**: `cleaned.split("Question")` splits on "Question" ANYWHERE in text (not line-anchored)
- **Line 69**: `Regex::new(r"\nAnswer\s+:")` matches "Answer" after ANY newline (not line-anchored)
- Both are case-sensitive
- Result: "The Answer" in question text causes incorrect split

## Solution

Replace simple string split and `\n` regex with proper line-anchored patterns.

### Changes to `src\content\markdown.rs`

**1. Line 56 - Question Split**
```rust
// OLD
let parts: Vec<&str> = cleaned.split("Question").collect();

// NEW
let question_regex = Regex::new(r"(?mi)^\s*Question\s+:").unwrap();
let parts: Vec<&str> = question_regex.split(&cleaned).collect();
```

**2. Line 69 - Answer Detection**
```rust
// OLD
let answer_re = Regex::new(r"\nAnswer\s+:").unwrap();

// NEW
let answer_re = Regex::new(r"(?mi)^\s*Answer\s+:").unwrap();
```

**Pattern Details:**
- `(?m)` = Multiline mode (^ matches line start)
- `(?i)` = Case-insensitive (matches "Question"/"question"/"QUESTION")
- `^\s*` = Line start + optional leading whitespace
- `Question\s+:` / `Answer\s+:` = Keyword + spaces + colon

**3. Optimization - Move Regex Outside Loop**
Compile all regexes once at function start (lines 52-62 area).

### Test Cases to Add (after line 367)

**New test function: `test_process_markdown_file_keywords_in_text`**

8 parametrized test cases:
1. "Answer" word in question text (Hitchhiker's Guide case)
2. "Question" word in question text
3. "answer"/"question" lowercase in answer text
4. Lowercase keywords "question :" / "answer :"
5. Uppercase keywords "QUESTION :" / "ANSWER :"
6. "Answer" at line start within question block (before actual Answer marker)
7. Leading whitespace before keywords
8. Keywords mid-line should NOT match

**Specific test: `test_exact_hitchhiker_case`**
- Exact content from `00_test.md:53-56`
- Verify: count=1, category="Culture", subcategory="The Hitchhiker's Guide to the Galaxy"
- Assert: question HTML contains "The Answer to" phrase

**Regex unit tests: `test_question_regex_pattern` + `test_answer_regex_pattern`**
- Verify: matches at line start (with/without whitespace)
- Verify: case-insensitive matching
- Verify: does NOT match mid-line

## Critical Files

1. **`src\content\markdown.rs`** (lines 56, 69, 367+)
   - Fix: Replace split/regex patterns
   - Add: ~100 lines of test cases

2. **`static\deck\md\00_test.md`** (line 53)
   - Integration test: Verify fix works with actual failing case

3. **`Cargo.toml`** (line 35)
   - Verified: `regex = "1"` supports required features

## Validation Steps

1. Run existing 24 tests → must pass (backward compatibility)
2. Run new edge case tests → must pass
3. Rebuild deck: `cargo run -- --rebuild-deck deck`
4. Verify Hitchhiker's Guide card displays correctly in browser
5. Test search with "Answer" keyword still works

## Notes

- More restrictive patterns = safer (prevents false matches)
- All existing markdown files continue to work (uppercase, line-start format)
- Case-insensitive support = future-proof
- Whitespace flexibility = handles indented markdown
- No performance impact (regex is O(n), compiled once)
