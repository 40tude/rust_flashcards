// Filter form interaction handler
//
// Key features:
// 1. Dynamic subcategory filtering: only show subcategories for selected categories
// 2. Checkbox state management: disabled checkboxes are not submitted with form (prevents duplicate field errors)
// 3. Client-side validation: ensures at least one subcategory is selected when categories are filtered
// 4. Auto-check behavior: if no categories selected, auto-enable "All categories"
(function() {
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

        // Show/hide subcategories based on their parent category
        subcatCbs.forEach(cb => {
            const parentCat = cb.dataset.category;
            const parentDiv = cb.parentElement;

            if (allCatCb.checked || checkedCategories.includes(parentCat)) {
                // Show subcategory
                parentDiv.style.display = 'block';
                cb.disabled = false;
            } else {
                // Hide, uncheck, and disable subcategory
                // Disabled checkboxes are not submitted with form
                parentDiv.style.display = 'none';
                cb.checked = false;
                cb.disabled = true;
            }
        });
    }

    // Validates form before submission
    function validateForm(e) {
        // If specific categories selected AND "All subcategories" unchecked
        if (!allCatCb.checked && !allSubcatCb.checked) {
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
            catList.style.display = 'none';
            categoryCbs.forEach(cb => {
                cb.checked = false;
                cb.disabled = true;  // Disable to prevent form submission
            });
            // Disable and check all subcategories when all categories selected
            allSubcatCb.disabled = true;
            allSubcatCb.checked = true;
            subcatList.style.display = 'none';
            subcatCbs.forEach(cb => cb.disabled = true);
        } else {
            catList.style.display = 'block';
            categoryCbs.forEach(cb => {
                cb.checked = true;
                cb.disabled = false;  // Re-enable for form submission
            });
            allSubcatCb.disabled = false;
        }
        updateVisibleSubcategories();
    });

    // Category checkboxes change - ensure at least all or some selected
    categoryCbs.forEach(cb => {
        cb.addEventListener('change', function() {
            const anySelected = Array.from(categoryCbs).some(c => c.checked);
            if (!anySelected && !allCatCb.checked) {
                // Auto-check "All categories" if nothing selected
                allCatCb.checked = true;
                catList.style.display = 'none';
                categoryCbs.forEach(cb => cb.disabled = true);
                allSubcatCb.disabled = true;
                allSubcatCb.checked = true;
                subcatList.style.display = 'none';
                subcatCbs.forEach(cb => cb.disabled = true);
            }
            updateVisibleSubcategories();
        });
    });

    // All subcategories checkbox toggle
    allSubcatCb.addEventListener('change', function() {
        if (this.checked) {
            subcatList.style.display = 'none';
        } else {
            subcatList.style.display = 'block';
            subcatCbs.forEach(cb => cb.checked = true);
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
})();
