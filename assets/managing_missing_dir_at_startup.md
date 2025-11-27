# Error Handling Plan for Content Directory Validation

## User Requirements
- Exit at startup with error message if both directories missing/empty
- Allow startup with partial content (md-only OR png-only)
- Include setup instructions in error messages

## Current Issues
1. `WalkDir::new()` panics if directory doesn't exist (unhandled)
2. No validation before attempting to load content
3. Silent filtering of directory-level IO errors via `.filter_map(|e| e.ok())`
4. Empty directories allowed (returns `Ok(())` with 0 cards loaded)

## Proposed Solution

### 1. Add Directory Validation Helper Function in `src/content/mod.rs`

Create validation function that checks:
- Directory existence using `std::path::Path::exists()`
- Directory is actually a directory (not a file) using `Path::is_dir()`
- Directory read permissions (attempt `std::fs::read_dir()`)

Returns enum representing state:
```rust
pub enum ContentDirStatus {
    Valid,           // Exists, is directory, readable
    Missing,         // Path doesn't exist
    NotADirectory,   // Path exists but is a file
    PermissionDenied // Exists but can't read
}
```

### 2. Modify `main.rs` Startup Logic (lines 38-46)

**Before loading content:**
1. Check both `./static/md` and `./static/png` directory status
2. Validate at least ONE directory is valid
3. If both invalid → exit with helpful error message
4. If at least one valid → proceed with loading (skip invalid ones)

**Validation logic:**
```
md_status = validate_directory("./static/md")
png_status = validate_directory("./static/png")

if both invalid:
    exit with error + setup instructions
else:
    if md_status valid: load_markdown()
    if png_status valid: load_images()
    populate_fts_table()
```

### 3. Error Message Format

**When both directories invalid:**
```
ERROR: Cannot start application - no content directories found

The application requires at least one of the following directories:
  - ./static/md/  (for markdown flashcards)
  - ./static/png/ (for image-only flashcards)

Current status:
  ./static/md  -> [Missing/NotADirectory/PermissionDenied]
  ./static/png -> [Missing/NotADirectory/PermissionDenied]

Setup instructions:
  1. Create at least one directory:
     mkdir static/md
     OR
     mkdir static/png

  2. Add content files:
     - For markdown: place .md files in static/md/
     - For images: place .png files in static/png/

For more information, see the README.md file.
```

**When one directory invalid but app starting:**
```
WARNING: Content directory unavailable: ./static/md
Reason: [Missing/NotADirectory/PermissionDenied]
Continuing with image-only flashcards from ./static/png
```

### 4. Enhance `load_markdown()` and `load_images()` (Optional)

Add context to errors:
- Remove silent `.filter_map(|e| e.ok())` filtering
- Log directory-level errors explicitly
- Add `.with_context()` to WalkDir initialization

## Files to Modify

1. **`src/content/mod.rs`** (4 lines, 2 exports)
   - Add directory validation function
   - Export validation function and status enum

2. **`src/main.rs`** (lines 38-46, ~20 lines new code)
   - Add directory validation before content loading
   - Add conditional loading based on validation results
   - Add error message formatting and exit logic

3. **`src/content/markdown.rs`** (optional enhancement)
   - Add error context to WalkDir initialization
   - Log explicit warnings for directory-level errors

4. **`src/content/images.rs`** (optional enhancement)
   - Add error context to WalkDir initialization
   - Log explicit warnings for directory-level errors

## Implementation Order

1. Create validation function in `src/content/mod.rs`
2. Update `main.rs` with validation logic and error messages
3. Test scenarios: both missing, one missing, both present, permission errors
4. (Optional) Enhance content loaders with better error context

## Edge Cases Handled

- Both directories missing → Exit with instructions
- One directory missing → Start with partial content, log warning
- Directory is actually a file → Exit with clear error
- Permission denied → Exit with clear error
- Empty directories → Allow (existing behavior, loads 0 cards)
- Database already populated → Skip validation (existing fast-startup optimization)
