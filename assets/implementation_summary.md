# Summary: Dynamic Subcategory Filtering Implementation

## Problem Solved

Users wanted subcategories to dynamically filter based on selected categories, with validation to prevent empty subcategory selection when categories are filtered.

## Files Modified

### 1. `src/db/queries.rs`
**Change:** Modified `get_distinct_subcategories()` return type from `Vec<String>` to `Vec<(String, String)>`

**Why:** Need to return both subcategory name AND parent category name to enable client-side filtering.

**Code:**
```rust
// Before: SELECT DISTINCT subcategory FROM...
// After:  SELECT DISTINCT subcategory, category FROM...

pub fn get_distinct_subcategories(
    pool: &DbPool,
    categories: Option<&[String]>,
) -> Result<Vec<(String, String)>> {  // Returns (subcategory, category) tuples
```

**Necessary:** ✅ Yes - Without parent category info, JavaScript cannot filter subcategories dynamically.

---

### 2. `src/routes/landing.rs`

#### Change A: New `SubcategoryItem` struct
**Why:** Template needs both subcategory name and parent category to render `data-category` attribute.

```rust
struct SubcategoryItem {
    name: String,
    category: String,  // Parent category for JS filtering
    selected: bool,
}
```

**Necessary:** ✅ Yes - Enables passing parent category to template.

#### Change B: Updated `landing()` function
**Why:** Build `SubcategoryItem` from tuples returned by database.

```rust
let subcategories: Vec<SubcategoryItem> = all_subcategories_list
    .into_iter()
    .map(|(name, category)| SubcategoryItem {
        selected: session_data.filter_subcategories...contains(&name),
        name,
        category,  // Now includes parent category
    })
    .collect();
```

**Necessary:** ✅ Yes - Adapts to new database return type.

#### Change C: Manual form parsing in `apply_filters()`
**Why:** `serde_urlencoded` has bugs with repeated field names (categories[], subcategories[]) when using `untagged` enums. Causes "duplicate field" errors.

**Before (broken):**
```rust
pub async fn apply_filters(
    session: Session,
    Form(form): Form<FilterForm>,  // ❌ Deserialization fails with repeated fields
) -> Result<...>
```

**After (working):**
```rust
pub async fn apply_filters(
    session: Session,
    body: String,  // Raw form data
) -> Result<...> {
    // Manual parsing: split on '&' and '='
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            match key.as_str() {
                "categories" => form.categories.push(value),  // Collect multiples
                "subcategories" => form.subcategories.push(value),
                ...
            }
        }
    }
}
```

**Necessary:** ✅ Yes - Only way to reliably handle repeated field names without serde_urlencoded bugs.

#### Change D: Server-side validation
**Why:** Ensure users select at least one subcategory when categories are filtered.

```rust
if form.subcategories.is_empty() {
    if session_data.filter_categories.is_some() {
        // Error: categories filtered but no subcategories selected
        session_data.error_message = Some("Please select at least one subcategory...");
        return Ok(Redirect::to("/"));
    }
}
```

**Necessary:** ✅ Yes - Prevents invalid filter state (categories without subcategories).

---

### 3. `templates/landing.html`

**Changes:**
- Added `data-category="{{ subcat.category }}"` attribute
- Added `subcategory-cb` CSS class

```html
<input type="checkbox"
       name="subcategories"
       value="{{ subcat.name }}"
       class="form-check-input subcategory-cb"
       data-category="{{ subcat.category }}"
       {% if subcat.selected %}checked{% endif %}>
```

**Why:**
- `data-category`: JavaScript reads this to determine parent category
- `subcategory-cb`: JavaScript uses this class to select all subcategory checkboxes

**Necessary:** ✅ Yes - Without these, JavaScript cannot identify which subcategories belong to which categories.

---

### 4. `static/js/filters.js`

#### Change A: `updateVisibleSubcategories()` function
**Why:** Dynamically show/hide subcategories based on selected categories.

```javascript
function updateVisibleSubcategories() {
    const checkedCategories = Array.from(categoryCbs)
        .filter(cb => cb.checked)
        .map(cb => cb.value);

    subcatCbs.forEach(cb => {
        const parentCat = cb.dataset.category;  // Read from HTML attribute

        if (allCatCb.checked || checkedCategories.includes(parentCat)) {
            cb.parentElement.style.display = 'block';
            cb.disabled = false;  // Enable for form submission
        } else {
            cb.parentElement.style.display = 'none';
            cb.checked = false;
            cb.disabled = true;  // CRITICAL: disabled = not submitted with form
        }
    });
}
```

**Why `disabled = true`:** Prevents disabled checkboxes from being submitted. Without this, hidden checkboxes cause "duplicate field" errors.

**Necessary:** ✅ Yes - Core feature. Without this, subcategories don't filter dynamically.

#### Change B: `validateForm()` function
**Why:** Client-side validation prevents form submission if no subcategories selected.

```javascript
function validateForm(e) {
    if (!allCatCb.checked && !allSubcatCb.checked) {
        const anyEnabledSubcatChecked = Array.from(subcatCbs).some(cb => {
            return cb.checked && !cb.disabled;
        });

        if (!anyEnabledSubcatChecked) {
            alert('Please select at least one subcategory...');
            e.preventDefault();
            return false;
        }
    }
    return true;
}
```

**Necessary:** ✅ Yes - Provides immediate feedback before server validation.

#### Change C: Disable hidden category/subcategory checkboxes
**Why:** Prevent duplicate field errors from hidden-but-enabled checkboxes.

```javascript
allCatCb.addEventListener('change', function() {
    if (this.checked) {
        categoryCbs.forEach(cb => cb.disabled = true);  // Disable hidden categories
        subcatCbs.forEach(cb => cb.disabled = true);    // Disable hidden subcategories
    } else {
        categoryCbs.forEach(cb => cb.disabled = false); // Re-enable when visible
    }
    updateVisibleSubcategories();
});
```

**Necessary:** ✅ Yes - Without this, form submission sends hidden checkbox values → "duplicate field" errors.

#### Change D: Event listeners
**Why:** Call `updateVisibleSubcategories()` whenever categories change.

```javascript
// When "All categories" checkbox changes
allCatCb.addEventListener('change', updateVisibleSubcategories);

// When any individual category checkbox changes
categoryCbs.forEach(cb => {
    cb.addEventListener('change', updateVisibleSubcategories);
});

// On page load
updateVisibleSubcategories();
```

**Necessary:** ✅ Yes - Keeps UI in sync with selection state.

---

## Key Technical Decisions

### Why Manual Form Parsing?
`serde_urlencoded` (used by Axum's `Form` extractor) has fundamental issues with:
1. **Repeated field names:** `categories=A&categories=B` confuses the deserializer
2. **Untagged enums:** The workaround (`#[serde(untagged)]`) causes "duplicate field" errors

**Solution:** Parse raw `String` body manually by splitting on `&` and `=`.

### Why `disabled` instead of just hiding?
- HTML forms submit ALL `<input>` elements, even if `display: none`
- `disabled` inputs are NOT submitted
- This prevents duplicate field errors

### Why both client AND server validation?
- **Client-side:** Immediate user feedback (alert popup)
- **Server-side:** Security - never trust client, enforce business rules

---

## Testing Checklist

✅ All categories checked → all subcategories visible
✅ Uncheck "All categories" → all categories checked, all subcategories visible
✅ Select specific categories → only their subcategories visible
✅ Uncheck a category → its subcategories hidden and unchecked
✅ Try submit with categories but no subcategories → validation error
✅ Check "All subcategories" → validation passes
✅ Form data contains only visible checkbox values (no duplicates)

---

## Diagram: Data Flow

```
User clicks category
    ↓
JS: updateVisibleSubcategories()
    ↓
- Read data-category from each subcategory checkbox
- Show if parent category checked
- Hide + disable if parent unchecked
    ↓
User clicks "Practice"
    ↓
JS: validateForm()
    ↓
- Check if at least one enabled subcategory checked
- Show alert if validation fails
    ↓
Form submits (disabled checkboxes NOT included)
    ↓
Server: apply_filters() manually parses body
    ↓
- Split on '&' to get key=value pairs
- Collect multiple values into Vec
- Validate: categories without subcategories = error
    ↓
Redirect to /practice or back to / with error
```

---

## Conclusion

All modifications are **necessary** and **minimal**. Each change addresses a specific requirement:

1. **Database:** Return parent category with subcategories
2. **Backend:** Handle repeated form fields + validate logic
3. **Template:** Add data attributes for JS filtering
4. **JavaScript:** Dynamic filtering + disable hidden checkboxes

The manual form parsing is unfortunate but required due to `serde_urlencoded` limitations with repeated fields.
