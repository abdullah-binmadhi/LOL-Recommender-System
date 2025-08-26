// Enhanced JavaScript for recommendation interface

document.addEventListener('DOMContentLoaded', function() {
    // Initialize tooltips
    var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
    var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
        return new bootstrap.Tooltip(tooltipTriggerEl);
    });
    
    // Animate progress bars
    animateProgressBars();
    
    // Add hover effects to champion cards
    addChampionCardEffects();
    
    // Initialize copy to clipboard functionality
    initializeCopyFeatures();
    
    // Add smooth scrolling for navigation
    addSmoothScrolling();
});

function animateProgressBars() {
    const progressBars = document.querySelectorAll('.progress-bar');
    progressBars.forEach(bar => {
        const width = bar.style.width;
        bar.style.width = '0%';
        setTimeout(() => {
            bar.style.transition = 'width 1s ease-in-out';
            bar.style.width = width;
        }, 100);
    });
}

function addChampionCardEffects() {
    const championCards = document.querySelectorAll('.champion-card');
    championCards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-8px) scale(1.02)';
            this.style.boxShadow = '0 15px 40px rgba(200, 155, 60, 0.3)';
        });
        
        card.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0) scale(1)';
            this.style.boxShadow = '0 8px 32px rgba(0, 0, 0, 0.4)';
        });
    });
}

function initializeCopyFeatures() {
    // Add copy buttons for champion names and details
    const copyButtons = document.querySelectorAll('.copy-btn');
    copyButtons.forEach(button => {
        button.addEventListener('click', function() {
            const textToCopy = this.getAttribute('data-copy');
            navigator.clipboard.writeText(textToCopy).then(() => {
                showToast('Copied to clipboard!', 'success');
            });
        });
    });
}

function addSmoothScrolling() {
    const scrollLinks = document.querySelectorAll('a[href^="#"]');
    scrollLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

function showToast(message, type = 'info') {
    // Create toast element
    const toast = document.createElement('div');
    toast.className = `toast align-items-center text-white bg-${type} border-0`;
    toast.setAttribute('role', 'alert');
    toast.innerHTML = `
        <div class="d-flex">
            <div class="toast-body">${message}</div>
            <button type="button" class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast"></button>
        </div>
    `;
    
    // Add to toast container or create one
    let toastContainer = document.querySelector('.toast-container');
    if (!toastContainer) {
        toastContainer = document.createElement('div');
        toastContainer.className = 'toast-container position-fixed top-0 end-0 p-3';
        document.body.appendChild(toastContainer);
    }
    
    toastContainer.appendChild(toast);
    
    // Show toast
    const bsToast = new bootstrap.Toast(toast);
    bsToast.show();
    
    // Remove after hiding
    toast.addEventListener('hidden.bs.toast', () => {
        toast.remove();
    });
}

// Champion comparison functionality
function compareChampions() {
    const currentChampion = document.querySelector('.current-champion').textContent;
    window.location.href = `/compare-models?current=${encodeURIComponent(currentChampion)}`;
}

// Share recommendation functionality
function shareRecommendation() {
    const championName = document.querySelector('.champion-name').textContent;
    const confidence = document.querySelector('.confidence-score').textContent;
    
    const shareText = `I got ${championName} as my League of Legends champion recommendation with ${confidence} confidence! Try the LoL Champion Recommender.`;
    
    if (navigator.share) {
        navigator.share({
            title: 'My LoL Champion Recommendation',
            text: shareText,
            url: window.location.href
        });
    } else {
        // Fallback to clipboard
        navigator.clipboard.writeText(shareText).then(() => {
            showToast('Recommendation copied to clipboard!', 'success');
        });
    }
}

// Advanced filtering for alternatives
function filterAlternatives(filterType, filterValue) {
    const alternatives = document.querySelectorAll('.alternative-champion');
    
    alternatives.forEach(alt => {
        const shouldShow = checkAlternativeFilter(alt, filterType, filterValue);
        alt.style.display = shouldShow ? 'block' : 'none';
    });
}

function checkAlternativeFilter(element, filterType, filterValue) {
    switch(filterType) {
        case 'role':
            return element.querySelector('.champion-role').textContent === filterValue;
        case 'difficulty':
            const difficulty = parseInt(element.querySelector('.champion-difficulty').textContent);
            return difficulty <= filterValue;
        case 'confidence':
            const confidence = parseFloat(element.querySelector('.confidence-score').textContent);
            return confidence >= filterValue;
        default:
            return true;
    }
}

// Keyboard shortcuts
document.addEventListener('keydown', function(e) {
    // Alt + A for alternatives
    if (e.altKey && e.key === 'a') {
        e.preventDefault();
        const altButton = document.querySelector('a[href*="alternatives"]');
        if (altButton) altButton.click();
    }
    
    // Alt + C for compare models
    if (e.altKey && e.key === 'c') {
        e.preventDefault();
        const compareButton = document.querySelector('a[href*="compare-models"]');
        if (compareButton) compareButton.click();
    }
    
    // Alt + R for retake
    if (e.altKey && e.key === 'r') {
        e.preventDefault();
        const retakeButton = document.querySelector('a[href*="retake"]');
        if (retakeButton) retakeButton.click();
    }
});

// Add loading states for navigation
function addLoadingState(button) {
    const originalText = button.innerHTML;
    button.innerHTML = '<span class="spinner-border spinner-border-sm me-2"></span>Loading...';
    button.disabled = true;
    
    // Restore after navigation (fallback)
    setTimeout(() => {
        button.innerHTML = originalText;
        button.disabled = false;
    }, 3000);
}