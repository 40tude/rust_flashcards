// Filter form interaction handler
//
// Architecture: Server renders ALL categories/subcategories
// JavaScript handles ALL visibility and state logic
// Session stores ONLY selection state (checked/unchecked)
document.addEventListener('DOMContentLoaded', function() {
    const allCatCb = document.getElementById('all-categories-cb');
    const catList = document.getElementById('category-list');
    const categoryCbs = document.querySelectorAll('.category-cb');
    const allSubcatCb = document.getElementById('all-subcategories-cb');
    const subcatList = document.getElementById('subcategory-list');
    const subcatCbs = document.querySelectorAll('.subcategory-cb');
    const form = document.getElementById('filter-form');
    const keywordsInput = document.getElementById('keywords-input');

    // Updates visible subcategories based on selected categories
    function updateVisibleSubcategories() {
        // Get list of checked categories
        const checkedCategories = Array.from(categoryCbs)
            .filter(cb => cb.checked)
            .map(cb => cb.value);

        const allCategoriesMode = allCatCb.checked;
        const noCategoriesSelected = !allCategoriesMode && checkedCategories.length === 0;

        // Show/hide subcategories based on parent category selection
        subcatCbs.forEach(cb => {
            const parentCat = cb.dataset.category;
            const parentDiv = cb.parentElement;

            if (allCategoriesMode || checkedCategories.includes(parentCat)) {
                // Show subcategory (parent category is selected)
                parentDiv.style.display = 'block';
                cb.disabled = false;
            } else {
                // Hide subcategory (parent category not selected)
                parentDiv.style.display = 'none';
                cb.checked = false;  // Uncheck hidden items
                cb.disabled = true;  // Disable so they don't submit
            }
        });

        // Auto-manage "All Subcategories" checkbox state
        if (noCategoriesSelected) {
            // Images-only mode: disable subcategory selection
            allSubcatCb.disabled = true;
            allSubcatCb.checked = true;
            subcatList.style.display = 'none';
        } else {
            // Categories selected: enable subcategory selection
            allSubcatCb.disabled = false;
            if (allSubcatCb.checked) {
                subcatList.style.display = 'none';
            } else {
                subcatList.style.display = 'block';
            }
        }
    }

    // Validates form before submission
    function validateForm(e) {
        const anyCategorySelected = Array.from(categoryCbs).some(cb => cb.checked);

        // If specific categories selected AND "All subcategories" unchecked
        if (!allCatCb.checked && anyCategorySelected && !allSubcatCb.checked) {
            // Check if at least one enabled subcategory is checked
            const anyEnabledSubcatChecked = Array.from(subcatCbs).some(cb => {
                return cb.checked && !cb.disabled;
            });

            if (!anyEnabledSubcatChecked) {
                alert('Please select at least one subcategory for the selected categories');
                e.preventDefault();
                return false;
            }
        }
        return true;
    }

    // All categories checkbox toggle
    allCatCb.addEventListener('change', function() {
        if (this.checked) {
            // Hide and disable individual category selection
            catList.style.display = 'none';
            categoryCbs.forEach(cb => {
                cb.checked = false;
                cb.disabled = true;
            });
        } else {
            // Show and enable individual category selection
            catList.style.display = 'block';
            categoryCbs.forEach(cb => {
                cb.checked = true;  // Auto-check all when expanding
                cb.disabled = false;
            });
        }
        updateVisibleSubcategories();
    });

    // Category checkboxes change
    categoryCbs.forEach(cb => {
        cb.addEventListener('change', function() {
            updateVisibleSubcategories();
        });
    });

    // All subcategories checkbox toggle
    allSubcatCb.addEventListener('change', function() {
        if (this.checked) {
            subcatList.style.display = 'none';
        } else {
            subcatList.style.display = 'block';
            // Only check ENABLED subcategories (visible ones)
            subcatCbs.forEach(cb => {
                if (!cb.disabled) {
                    cb.checked = true;
                }
            });
        }
    });

    // Form submit validation
    form.addEventListener('submit', validateForm);

    // ENTER key in keywords field submits form
    keywordsInput.addEventListener('keydown', function(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            if (validateForm(e)) {
                form.submit();
            }
        }
    });

    // Initialize subcategory visibility on page load
    updateVisibleSubcategories();
});
