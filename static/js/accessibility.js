// Accessibility enhancements for LoL Champion Recommender

document.addEventListener('DOMContentLoaded', function() {
    // Initialize accessibility features
    initializeAccessibility();
    initializeKeyboardNavigation();
    initializeFocusManagement();
    initializeScreenReaderSupport();
    initializeColorContrastToggle();
    initializeReducedMotionSupport();
});

function initializeAccessibility() {
    // Add skip links for screen readers
    addSkipLinks();
    
    // Enhance form accessibility
    enhanceFormAccessibility();
    
    // Add ARIA labels and descriptions
    addAriaLabels();
    
    // Initialize focus indicators
    initializeFocusIndicators();
}

function addSkipLinks() {
    const skipLinks = document.createElement('div');
    skipLinks.className = 'skip-links';
    skipLinks.innerHTML = `
        <a href="#main-content" class="skip-link">Skip to main content</a>
        <a href="#navigation" class="skip-link">Skip to navigation</a>
        <a href="#questionnaire-form" class="skip-link">Skip to questionnaire</a>
    `;
    document.body.insertBefore(skipLinks, document.body.firstChild);
}

function enhanceFormAccessibility() {
    // Add proper labels and descriptions to form elements
    const radioButtons = document.querySelectorAll('input[type="radio"]');
    radioButtons.forEach((radio, index) => {
        const label = radio.nextElementSibling;
        if (label && label.tagName === 'LABEL') {
            // Add keyboard navigation hints
            label.setAttribute('tabindex', '0');
            label.setAttribute('role', 'radio');
            label.setAttribute('aria-checked', radio.checked ? 'true' : 'false');
            
            // Update aria-checked when selection changes
            radio.addEventListener('change', function() {
                radioButtons.forEach(r => {
                    const l = r.nextElementSibling;
                    if (l) l.setAttribute('aria-checked', 'false');
                });
                label.setAttribute('aria-checked', 'true');
            });
            
            // Allow keyboard selection
            label.addEventListener('keydown', function(e) {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    radio.checked = true;
                    radio.dispatchEvent(new Event('change'));
                }
            });
        }
    });
    
    // Add fieldset and legend for question groups
    const questionForms = document.querySelectorAll('form');
    questionForms.forEach(form => {
        const questionTitle = form.querySelector('h2');
        if (questionTitle) {
            const fieldset = document.createElement('fieldset');
            const legend = document.createElement('legend');
            legend.textContent = questionTitle.textContent;
            legend.className = 'visually-hidden';
            
            fieldset.appendChild(legend);
            
            // Move form contents into fieldset
            const formContents = Array.from(form.children);
            formContents.forEach(child => {
                if (child !== questionTitle) {
                    fieldset.appendChild(child);
                }
            });
            
            form.appendChild(fieldset);
        }
    });
}

function addAriaLabels() {
    // Add ARIA labels to interactive elements
    const buttons = document.querySelectorAll('button, .btn');
    buttons.forEach(button => {
        if (!button.getAttribute('aria-label') && !button.getAttribute('aria-labelledby')) {
            const text = button.textContent.trim();
            if (text) {
                button.setAttribute('aria-label', text);
            }
        }
    });
    
    // Add ARIA descriptions to progress bars
    const progressBars = document.querySelectorAll('.progress-bar');
    progressBars.forEach(bar => {
        const progress = bar.getAttribute('aria-valuenow');
        const max = bar.getAttribute('aria-valuemax');
        if (progress && max) {
            bar.setAttribute('aria-label', `Progress: ${progress} out of ${max}`);
        }
    });
    
    // Add ARIA labels to champion cards
    const championCards = document.querySelectorAll('.champion-card');
    championCards.forEach(card => {
        const championName = card.querySelector('.champion-name, h5, h6');
        const role = card.querySelector('.badge');
        if (championName) {
            const label = `Champion: ${championName.textContent}${role ? `, Role: ${role.textContent}` : ''}`;
            card.setAttribute('aria-label', label);
            card.setAttribute('role', 'article');
        }
    });
}

function initializeFocusIndicators() {
    // Enhanced focus indicators for better visibility
    const focusableElements = document.querySelectorAll(
        'a, button, input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    
    focusableElements.forEach(element => {
        element.addEventListener('focus', function() {
            this.classList.add('focus-visible');
        });
        
        element.addEventListener('blur', function() {
            this.classList.remove('focus-visible');
        });
    });
}

function initializeKeyboardNavigation() {
    // Enhanced keyboard navigation
    document.addEventListener('keydown', function(e) {
        // Escape key to close modals or return to previous page
        if (e.key === 'Escape') {
            const modals = document.querySelectorAll('.modal.show');
            if (modals.length > 0) {
                modals[modals.length - 1].querySelector('.btn-close')?.click();
            } else {
                // Go back if no modals
                history.back();
            }
        }
        
        // Tab navigation enhancements
        if (e.key === 'Tab') {
            document.body.classList.add('keyboard-navigation');
        }
        
        // Arrow key navigation for radio buttons
        if (e.key.startsWith('Arrow')) {
            const activeElement = document.activeElement;
            if (activeElement && activeElement.type === 'radio') {
                handleRadioArrowNavigation(e, activeElement);
            }
        }
        
        // Number keys for quick question answers (1-9)
        if (e.key >= '1' && e.key <= '9' && !e.ctrlKey && !e.altKey) {
            const radioButtons = document.querySelectorAll('input[type="radio"]');
            const index = parseInt(e.key) - 1;
            if (radioButtons[index]) {
                radioButtons[index].checked = true;
                radioButtons[index].focus();
                radioButtons[index].dispatchEvent(new Event('change'));
            }
        }
    });
    
    // Remove keyboard navigation class on mouse use
    document.addEventListener('mousedown', function() {
        document.body.classList.remove('keyboard-navigation');
    });
}

function handleRadioArrowNavigation(e, activeRadio) {
    e.preventDefault();
    const radioGroup = document.querySelectorAll(`input[name="${activeRadio.name}"]`);
    const currentIndex = Array.from(radioGroup).indexOf(activeRadio);
    let nextIndex;
    
    switch(e.key) {
        case 'ArrowDown':
        case 'ArrowRight':
            nextIndex = (currentIndex + 1) % radioGroup.length;
            break;
        case 'ArrowUp':
        case 'ArrowLeft':
            nextIndex = (currentIndex - 1 + radioGroup.length) % radioGroup.length;
            break;
        default:
            return;
    }
    
    radioGroup[nextIndex].checked = true;
    radioGroup[nextIndex].focus();
    radioGroup[nextIndex].dispatchEvent(new Event('change'));
}

function initializeFocusManagement() {
    // Focus management for dynamic content
    const observer = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList') {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType === Node.ELEMENT_NODE) {
                        // Focus first interactive element in new content
                        const firstFocusable = node.querySelector(
                            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
                        );
                        if (firstFocusable) {
                            setTimeout(() => firstFocusable.focus(), 100);
                        }
                    }
                });
            }
        });
    });
    
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
}

function initializeScreenReaderSupport() {
    // Live regions for dynamic content updates
    const liveRegion = document.createElement('div');
    liveRegion.setAttribute('aria-live', 'polite');
    liveRegion.setAttribute('aria-atomic', 'true');
    liveRegion.className = 'visually-hidden';
    liveRegion.id = 'live-region';
    document.body.appendChild(liveRegion);
    
    // Announce page changes
    const originalTitle = document.title;
    const titleObserver = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList' && mutation.target.tagName === 'TITLE') {
                announceToScreenReader(`Page changed to: ${document.title}`);
            }
        });
    });
    
    if (document.querySelector('title')) {
        titleObserver.observe(document.querySelector('title'), {
            childList: true
        });
    }
    
    // Announce form validation errors
    document.addEventListener('invalid', function(e) {
        const message = e.target.validationMessage || 'Please check your input';
        announceToScreenReader(`Error: ${message}`);
    }, true);
    
    // Announce successful form submissions
    document.addEventListener('submit', function(e) {
        announceToScreenReader('Form submitted successfully');
    });
}

function announceToScreenReader(message) {
    const liveRegion = document.getElementById('live-region');
    if (liveRegion) {
        liveRegion.textContent = message;
        setTimeout(() => {
            liveRegion.textContent = '';
        }, 1000);
    }
}

function initializeColorContrastToggle() {
    // High contrast mode toggle
    const contrastToggle = document.createElement('button');
    contrastToggle.className = 'contrast-toggle';
    contrastToggle.innerHTML = '<i class="fas fa-adjust"></i>';
    contrastToggle.setAttribute('aria-label', 'Toggle high contrast mode');
    contrastToggle.setAttribute('title', 'Toggle high contrast mode');
    
    contrastToggle.addEventListener('click', function() {
        document.body.classList.toggle('high-contrast');
        const isHighContrast = document.body.classList.contains('high-contrast');
        this.setAttribute('aria-pressed', isHighContrast);
        announceToScreenReader(`High contrast mode ${isHighContrast ? 'enabled' : 'disabled'}`);
        
        // Save preference
        localStorage.setItem('highContrast', isHighContrast);
    });
    
    // Restore saved preference
    if (localStorage.getItem('highContrast') === 'true') {
        document.body.classList.add('high-contrast');
        contrastToggle.setAttribute('aria-pressed', 'true');
    }
    
    document.body.appendChild(contrastToggle);
}

function initializeReducedMotionSupport() {
    // Respect user's motion preferences
    const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
    
    function handleReducedMotion(e) {
        if (e.matches) {
            document.body.classList.add('reduced-motion');
            // Disable animations
            const style = document.createElement('style');
            style.textContent = `
                *, *::before, *::after {
                    animation-duration: 0.01ms !important;
                    animation-iteration-count: 1 !important;
                    transition-duration: 0.01ms !important;
                }
            `;
            document.head.appendChild(style);
        } else {
            document.body.classList.remove('reduced-motion');
        }
    }
    
    prefersReducedMotion.addListener(handleReducedMotion);
    handleReducedMotion(prefersReducedMotion);
}

// Utility functions for accessibility
window.accessibilityUtils = {
    announceToScreenReader: announceToScreenReader,
    
    focusElement: function(selector) {
        const element = document.querySelector(selector);
        if (element) {
            element.focus();
            element.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    },
    
    addAriaLabel: function(selector, label) {
        const elements = document.querySelectorAll(selector);
        elements.forEach(el => el.setAttribute('aria-label', label));
    },
    
    toggleAriaExpanded: function(element) {
        const expanded = element.getAttribute('aria-expanded') === 'true';
        element.setAttribute('aria-expanded', !expanded);
    }
};