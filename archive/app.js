// LoL Champion Recommender - Main Application Logic
// Full-featured version with comprehensive questionnaire and recommendations

class ChampionRecommenderApp {
    constructor() {
        this.answers = {};
        this.currentQuestion = 1;
        this.totalQuestions = questionsDatabase.length;
        this.currentRecommendation = null;
        this.alternatives = [];
        
        this.init();
    }

    init() {
        this.generateQuestionCards();
        this.setupEventListeners();
    }

    generateQuestionCards() {
        const container = document.getElementById('questions-container');
        
        questionsDatabase.forEach((question, index) => {
            const questionCard = this.createQuestionCard(question, index + 1);
            container.appendChild(questionCard);
        });
    }

    createQuestionCard(question, questionNum) {
        const card = document.createElement('div');
        card.id = `question-${questionNum}`;
        card.className = 'card question-card';
        
        card.innerHTML = `
            <div class="progress-container">
                <div class="progress-info">
                    <span class="progress-text">Question ${questionNum} of ${this.totalQuestions}</span>
                    <span class="progress-text">${Math.round((questionNum - 1) / this.totalQuestions * 100)}% Complete</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: ${(questionNum - 1) / this.totalQuestions * 100}%"></div>
                </div>
            </div>
            
            <div class="question">
                <h2>${question.text}</h2>
                <div class="options">
                    ${question.options.map(option => `
                        <div class="option" 
                             onclick="app.selectOption(${questionNum}, '${option.value}', this)"
                             title="${option.description}">
                            ${option.text}
                        </div>
                    `).join('')}
                </div>
            </div>
            
            <div class="buttons">
                <button class="btn btn-secondary" 
                        onclick="app.previousQuestion(${questionNum})"
                        ${questionNum === 1 ? 'style="visibility: hidden;"' : ''}>
                    Previous
                </button>
                <button class="btn btn-primary" 
                        id="next-${questionNum}" 
                        onclick="app.nextQuestion(${questionNum})" 
                        disabled>
                    ${questionNum === this.totalQuestions ? 'Get Recommendation' : 'Next'}
                </button>
            </div>
        `;
        
        return card;
    }

    setupEventListeners() {
        // Keyboard navigation
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const nextBtn = document.querySelector(`#next-${this.currentQuestion}`);
                if (nextBtn && !nextBtn.disabled) {
                    nextBtn.click();
                }
            } else if (e.key === 'Escape') {
                this.closeModal();
            }
        });

        // Modal click outside to close
        document.getElementById('champion-modal').addEventListener('click', (e) => {
            if (e.target.id === 'champion-modal') {
                this.closeModal();
            }
        });
    }   
 startQuestionnaire() {
        document.getElementById('start-screen').style.display = 'none';
        document.getElementById(`question-${this.currentQuestion}`).classList.add('active');
        
        // Add entrance animation
        setTimeout(() => {
            document.getElementById(`question-${this.currentQuestion}`).style.opacity = '1';
        }, 100);
    }

    selectOption(questionNum, answer, element) {
        // Remove selection from other options
        const options = element.parentNode.querySelectorAll('.option');
        options.forEach(opt => {
            opt.classList.remove('selected');
            opt.style.transform = '';
        });
        
        // Select this option with animation
        element.classList.add('selected');
        element.style.transform = 'scale(1.02)';
        
        // Store answer
        this.answers[questionNum] = answer;
        
        // Enable next button with animation
        const nextBtn = document.getElementById(`next-${questionNum}`);
        nextBtn.disabled = false;
        nextBtn.style.transform = 'scale(1.05)';
        setTimeout(() => {
            nextBtn.style.transform = '';
        }, 200);

        // Auto-advance after a short delay (optional UX enhancement)
        if (questionNum < this.totalQuestions) {
            setTimeout(() => {
                if (this.answers[questionNum] === answer) { // Still the same answer
                    nextBtn.click();
                }
            }, 1500);
        }
    }

    nextQuestion(currentNum) {
        if (currentNum === this.totalQuestions) {
            this.showRecommendation();
            return;
        }

        // Hide current question
        const currentCard = document.getElementById(`question-${currentNum}`);
        currentCard.classList.remove('active');
        
        // Show next question
        this.currentQuestion = currentNum + 1;
        const nextCard = document.getElementById(`question-${this.currentQuestion}`);
        nextCard.classList.add('active');
        
        // Scroll to top smoothly
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    previousQuestion(currentNum) {
        if (currentNum === 1) return;

        // Hide current question
        const currentCard = document.getElementById(`question-${currentNum}`);
        currentCard.classList.remove('active');
        
        // Show previous question
        this.currentQuestion = currentNum - 1;
        const prevCard = document.getElementById(`question-${this.currentQuestion}`);
        prevCard.classList.add('active');
        
        // Scroll to top smoothly
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    showRecommendation() {
        // Hide current question
        document.querySelector('.question-card.active').classList.remove('active');
        
        // Get recommendation
        const recommendation = RecommendationEngine.getBestRecommendation(this.answers);
        
        if (!recommendation) {
            this.showError('Unable to generate recommendation. Please try again.');
            return;
        }

        this.currentRecommendation = recommendation;
        
        // Generate explanation
        const explanation = RecommendationEngine.generateExplanation(
            recommendation.champion, 
            this.answers, 
            recommendation.confidence
        );
        
        // Update recommendation display
        this.updateRecommendationDisplay(recommendation.champion, recommendation.confidence, explanation);
        
        // Show recommendation screen
        document.getElementById('recommendation').style.display = 'block';
        
        // Animate confidence bar
        setTimeout(() => {
            const confidenceFill = document.querySelector('.confidence-fill');
            if (confidenceFill) {
                confidenceFill.style.width = `${recommendation.confidence}%`;
            }
        }, 500);
        
        // Scroll to top
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }   
 updateRecommendationDisplay(champion, confidence, explanation) {
        const championCard = document.getElementById('main-champion');
        
        championCard.innerHTML = `
            <div class="champion-content">
                <div class="champion-name">${champion.name}</div>
                <div class="champion-title">${champion.title}</div>
                <div class="champion-role">${champion.role}</div>
                
                <div class="confidence">
                    <strong>Match Confidence: ${confidence}%</strong>
                    <div class="confidence-bar">
                        <div class="confidence-fill" style="width: 0%"></div>
                    </div>
                </div>
                
                <div class="champion-stats">
                    <div class="stat-grid">
                        <div class="stat">
                            <span class="stat-label">Damage</span>
                            <div class="stat-bar">
                                <div class="stat-fill" style="width: ${champion.attributes.damage * 10}%"></div>
                            </div>
                        </div>
                        <div class="stat">
                            <span class="stat-label">Toughness</span>
                            <div class="stat-bar">
                                <div class="stat-fill" style="width: ${champion.attributes.toughness * 10}%"></div>
                            </div>
                        </div>
                        <div class="stat">
                            <span class="stat-label">Control</span>
                            <div class="stat-bar">
                                <div class="stat-fill" style="width: ${champion.attributes.control * 10}%"></div>
                            </div>
                        </div>
                        <div class="stat">
                            <span class="stat-label">Mobility</span>
                            <div class="stat-bar">
                                <div class="stat-fill" style="width: ${champion.attributes.mobility * 10}%"></div>
                            </div>
                        </div>
                        <div class="stat">
                            <span class="stat-label">Utility</span>
                            <div class="stat-bar">
                                <div class="stat-fill" style="width: ${champion.attributes.utility * 10}%"></div>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="explanation">${explanation}</div>
            </div>
        `;
    }

    showAlternatives() {
        if (!this.currentRecommendation) return;
        
        // Get alternatives
        this.alternatives = RecommendationEngine.getAlternatives(
            this.answers, 
            this.currentRecommendation.champion, 
            4
        );
        
        // Update alternatives display
        const container = document.getElementById('alternatives-container');
        container.innerHTML = '';
        
        this.alternatives.forEach(alt => {
            const altElement = this.createAlternativeElement(alt);
            container.appendChild(altElement);
        });
        
        // Hide recommendation, show alternatives
        document.getElementById('recommendation').style.display = 'none';
        document.getElementById('alternatives').style.display = 'block';
        
        // Scroll to top
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    createAlternativeElement(alternative) {
        const element = document.createElement('div');
        element.className = 'alternative-champion';
        element.onclick = () => this.selectAlternative(alternative);
        
        const explanation = RecommendationEngine.generateExplanation(
            alternative.champion, 
            this.answers, 
            alternative.confidence
        );
        
        element.innerHTML = `
            <div class="alternative-header">
                <div class="alternative-name">${alternative.champion.name}</div>
                <div class="alternative-confidence">${alternative.confidence}%</div>
            </div>
            <div class="alternative-role">${alternative.champion.role}</div>
            <div class="alternative-title">${alternative.champion.title}</div>
            <div class="alternative-explanation">${explanation}</div>
        `;
        
        return element;
    }    
selectAlternative(alternative) {
        this.currentRecommendation = alternative;
        
        const explanation = RecommendationEngine.generateExplanation(
            alternative.champion, 
            this.answers, 
            alternative.confidence
        );
        
        this.updateRecommendationDisplay(
            alternative.champion, 
            alternative.confidence, 
            explanation
        );
        
        // Hide alternatives, show recommendation
        document.getElementById('alternatives').style.display = 'none';
        document.getElementById('recommendation').style.display = 'block';
        
        // Animate confidence bar
        setTimeout(() => {
            const confidenceFill = document.querySelector('.confidence-fill');
            if (confidenceFill) {
                confidenceFill.style.width = `${alternative.confidence}%`;
            }
        }, 500);
        
        // Scroll to top
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    backToRecommendation() {
        document.getElementById('alternatives').style.display = 'none';
        document.getElementById('recommendation').style.display = 'block';
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    showChampionDetails() {
        if (!this.currentRecommendation) return;
        
        const champion = this.currentRecommendation.champion;
        const modal = document.getElementById('champion-modal');
        const detailsContainer = document.getElementById('champion-details');
        
        detailsContainer.innerHTML = `
            <div class="champion-details">
                <div class="champion-header">
                    <h2>${champion.name}</h2>
                    <h3>${champion.title}</h3>
                    <div class="champion-role">${champion.role}</div>
                    <div class="champion-difficulty">Difficulty: ${champion.difficulty}/10</div>
                </div>
                
                <div class="champion-description">
                    <p>${champion.description}</p>
                </div>
                
                <div class="champion-tags">
                    <h4>Tags:</h4>
                    <div class="tags-list">
                        ${champion.tags.map(tag => `<span class="tag">${tag}</span>`).join('')}
                    </div>
                </div>
                
                <div class="abilities-section">
                    <h3>Champion Abilities</h3>
                    ${champion.abilities.map(ability => `
                        <div class="ability">
                            <div class="ability-header">
                                <div class="ability-key">${ability.key}</div>
                                <div class="ability-name">${ability.name}</div>
                            </div>
                            <div class="ability-description">${ability.description}</div>
                        </div>
                    `).join('')}
                </div>
                
                <div class="champion-attributes">
                    <h4>Champion Attributes</h4>
                    <div class="attributes-grid">
                        ${Object.entries(champion.attributes).map(([attr, value]) => `
                            <div class="attribute">
                                <span class="attribute-name">${attr.charAt(0).toUpperCase() + attr.slice(1)}</span>
                                <div class="attribute-bar">
                                    <div class="attribute-fill" style="width: ${value * 10}%"></div>
                                </div>
                                <span class="attribute-value">${value}/10</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
            </div>
        `;
        
        modal.style.display = 'block';
        document.body.style.overflow = 'hidden'; // Prevent background scrolling
    }

    closeModal() {
        document.getElementById('champion-modal').style.display = 'none';
        document.body.style.overflow = 'auto'; // Restore scrolling
    }    
restartQuestionnaire() {
        // Reset state
        this.answers = {};
        this.currentQuestion = 1;
        this.currentRecommendation = null;
        this.alternatives = [];
        
        // Hide all screens
        document.getElementById('recommendation').style.display = 'none';
        document.getElementById('alternatives').style.display = 'none';
        document.querySelectorAll('.question-card').forEach(card => {
            card.classList.remove('active');
        });
        
        // Reset all selections
        document.querySelectorAll('.option').forEach(opt => {
            opt.classList.remove('selected');
            opt.style.transform = '';
        });
        
        // Reset all next buttons
        document.querySelectorAll('[id^="next-"]').forEach(btn => {
            btn.disabled = true;
            btn.style.transform = '';
        });
        
        // Show start screen
        document.getElementById('start-screen').style.display = 'block';
        
        // Scroll to top
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }

    showError(message) {
        alert(`Error: ${message}`);
        this.restartQuestionnaire();
    }
}

// Global functions for HTML onclick handlers
function startQuestionnaire() {
    app.startQuestionnaire();
}

function restartQuestionnaire() {
    app.restartQuestionnaire();
}

function showAlternatives() {
    app.showAlternatives();
}

function showChampionDetails() {
    app.showChampionDetails();
}

function backToRecommendation() {
    app.backToRecommendation();
}

function closeModal() {
    app.closeModal();
}

// Initialize app when DOM is loaded
let app;
document.addEventListener('DOMContentLoaded', function() {
    app = new ChampionRecommenderApp();
});

// Add CSS for new elements
const additionalStyles = `
    .stat-grid, .attributes-grid {
        display: grid;
        gap: 15px;
        margin: 20px 0;
    }
    
    .stat, .attribute {
        display: flex;
        align-items: center;
        gap: 10px;
    }
    
    .stat-label, .attribute-name {
        min-width: 80px;
        font-weight: 600;
        color: white;
        font-size: 0.9rem;
    }
    
    .stat-bar, .attribute-bar {
        flex: 1;
        height: 8px;
        background: rgba(255,255,255,0.2);
        border-radius: 4px;
        overflow: hidden;
    }
    
    .stat-fill, .attribute-fill {
        height: 100%;
        background: linear-gradient(90deg, var(--success-color), var(--warning-color));
        border-radius: 4px;
        transition: width 1s ease;
    }
    
    .attribute-value {
        min-width: 40px;
        font-weight: 600;
        color: var(--text-primary);
        font-size: 0.9rem;
    }
    
    .tags-list {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin: 10px 0;
    }
    
    .tag {
        background: var(--primary-color);
        color: white;
        padding: 4px 12px;
        border-radius: 15px;
        font-size: 0.85rem;
        font-weight: 500;
    }
    
    .champion-difficulty {
        background: var(--warning-color);
        color: white;
        padding: 5px 15px;
        border-radius: 15px;
        display: inline-block;
        margin: 10px 0;
        font-weight: 600;
    }
    
    .champion-description {
        background: var(--bg-secondary);
        padding: 20px;
        border-radius: 10px;
        margin: 20px 0;
        font-style: italic;
        line-height: 1.6;
    }
`;

// Inject additional styles
const styleSheet = document.createElement('style');
styleSheet.textContent = additionalStyles;
document.head.appendChild(styleSheet);