// Flashcard hide/reveal logic
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
