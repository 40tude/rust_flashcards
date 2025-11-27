// Filter form interaction handler
(function() {
    const allCatCb = document.getElementById('all-categories-cb');
    const catList = document.getElementById('category-list');
    const categoryCbs = document.querySelectorAll('.category-cb');
    const allSubcatCb = document.getElementById('all-subcategories-cb');
    const subcatList = document.getElementById('subcategory-list');
    const subcatCbs = document.querySelectorAll('#subcategory-list input[type="checkbox"]');
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
            categoryCbs.forEach(cb => cb.checked = true);
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
            subcatCbs.forEach(cb => cb.checked = true);
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
