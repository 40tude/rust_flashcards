# Dynamic Subcategory Display Plan

## Problem Statement

Currently, the subcategory list is static and doesn't update dynamically when users check/uncheck categories. Additionally, there's no validation to ensure at least one subcategory is selected when categories are specified.

### Current Issues

1. **Static subcategory list**: When user changes category selection, subcategories don't update until page reload
2. **No validation**: If categories are selected but no subcategories are checked, the system treats it as "all subcategories" instead of showing an error
3. **Inconsistent state**: Brief moments where no categories are checked before JS auto-corrects

## Solution: Approach 3 - Client-side filtering with data attributes

### Overview

Add `data-category` attribute to each subcategory checkbox, then use JavaScript to show/hide subcategories based on selected categories. Validate that at least one subcategory is checked when categories are specified.

### Implementation Steps

#### 1. Modify `src/db/queries.rs`

**Function:** `get_distinct_subcategories`

**Change:** Return `Vec<(String, String)>` instead of `Vec<String>` where tuple is `(subcategory_name, parent_category_name)`

**Current signature:**
```rust
pub fn get_distinct_subcategories(
    pool: &DbPool,
    categories: Option<&[String]>,
) -> Result<Vec<String>>
```

**New signature:**
```rust
pub fn get_distinct_subcategories(
    pool: &DbPool,
    categories: Option<&[String]>,
) -> Result<Vec<(String, String)>>  // (subcategory, category)
```

**Query changes:**
- Add `category` to SELECT clause
- Return both subcategory and parent category

#### 2. Modify `src/routes/landing.rs`

**Struct:** Add new struct for subcategory with parent category
```rust
struct SubcategoryItem {
    name: String,
    category: String,  // Parent category name
    selected: bool,
}
```

**Update LandingTemplate:**
```rust
struct LandingTemplate {
    categories: Vec<CategoryItem>,
    subcategories: Vec<SubcategoryItem>,  // Changed type
    // ... rest unchanged
}
```

**Update landing() function (lines 123-144):**
- Handle new return type from `get_distinct_subcategories`
- Build `Vec<SubcategoryItem>` with parent category info

#### 3. Modify `templates/landing.html`

**Subcategory checkboxes (lines 83-92):**

Add `data-category` attribute:
```html
<input type="checkbox"
       name="subcategories"
       value="{{ subcat.name }}"
       class="form-check-input subcategory-cb"
       data-category="{{ subcat.category }}"
       {% if subcat.selected %}checked{% endif %}>
```

Also add class `subcategory-cb` for easier selection in JS.

#### 4. Modify `static/js/filters.js`

**Add dynamic filtering logic:**

1. Add selector for subcategory checkboxes at top
2. Add function to update visible subcategories based on selected categories
3. Call update function when:
   - All categories checkbox changes
   - Any individual category checkbox changes
4. Add validation before form submit

**Pseudocode:**
```javascript
function updateVisibleSubcategories() {
    // Get checked categories
    const checkedCategories = getCheckedCategories();

    // For each subcategory checkbox
    subcategoryCbs.forEach(cb => {
        const parentCat = cb.dataset.category;

        if (allCategoriesChecked || checkedCategories.includes(parentCat)) {
            // Show subcategory
            cb.parentElement.style.display = 'block';
        } else {
            // Hide subcategory
            cb.parentElement.style.display = 'none';
            cb.checked = false;  // Uncheck hidden subcategories
        }
    });
}

function validateForm() {
    // If specific categories selected AND "All subcategories" unchecked
    if (!allCatCb.checked && !allSubcatCb.checked) {
        // Check if at least one subcategory is checked
        const anySubcatChecked = Array.from(subcategoryCbs).some(cb => cb.checked);
        if (!anySubcatChecked) {
            alert('Please select at least one subcategory');
            return false;
        }
    }
    return true;
}
```

**Event listeners to add:**
- `allCatCb.addEventListener('change', updateVisibleSubcategories)`
- `categoryCbs.forEach(cb => cb.addEventListener('change', updateVisibleSubcategories))`
- `form.addEventListener('submit', (e) => { if (!validateForm()) e.preventDefault(); })`

#### 5. Modify `src/routes/landing.rs` - `apply_filters` function

**Server-side validation (lines 218-224):**

**Current logic:**
```rust
session_data.filter_subcategories = if form.all_subcategories.is_some() {
    None // All subcategories
} else if form.subcategories.is_empty() {
    None // ❌ PROBLEM: treats empty as "all"
} else {
    Some(form.subcategories)
};
```

**New logic:**
```rust
session_data.filter_subcategories = if form.all_subcategories.is_some() {
    None // All subcategories
} else if form.subcategories.is_empty() {
    // If specific categories selected but no subcategories, return error
    if session_data.filter_categories.is_some() {
        session_data.error_message = Some(
            "Please select at least one subcategory for the selected categories".to_string()
        );
        session.insert("data", &session_data).await?;
        return Ok(Redirect::to("/"));
    } else {
        None // No categories selected, empty subcats is OK
    }
} else {
    Some(form.subcategories)
};
```

## Files to Modify

1. **`src/db/queries.rs`** (~10 lines)
   - Change return type and query logic

2. **`src/routes/landing.rs`** (~25 lines)
   - Add `SubcategoryItem` struct
   - Update template struct
   - Update landing() to handle new tuple format
   - Add validation in apply_filters()

3. **`templates/landing.html`** (~2 lines)
   - Add `data-category` attribute
   - Add `subcategory-cb` class

4. **`static/js/filters.js`** (~40 lines new code)
   - Add `updateVisibleSubcategories()` function
   - Add `validateForm()` function
   - Add event listeners for dynamic updates
   - Add form submit validation

## Edge Cases Handled

- ✅ All categories checked → all subcategories visible
- ✅ No categories checked → auto-check "All categories"
- ✅ Some categories checked → only those subcategories visible
- ✅ User unchecks category → its subcategories hidden and unchecked
- ✅ Specific categories selected but no subcategories → error message
- ✅ "All subcategories" checked → validation passes
- ✅ Form validation both client-side (JS) and server-side (Rust)

## Testing Scenarios

1. Load page → all categories checked → all subcategories visible
2. Uncheck "All categories" → all categories checked, all subcategories visible
3. Uncheck one category → its subcategories disappear
4. Try to submit with categories but no subcategories → validation error
5. Check "All subcategories" → form submits successfully
6. Change category selection → subcategory list updates immediately
