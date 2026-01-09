const fs = require('fs');
const path = require('path');
const FeatureExtractor = require('./FeatureExtractor.js');

// Mock data and classes for the test environment
const DATA_FILE = path.join(__dirname, '../data/champions.json');
const rawData = JSON.parse(fs.readFileSync(DATA_FILE, 'utf8'));
const allChampions = {};
rawData.champions.forEach(c => {
    allChampions[c.name] = c;
});

// Ported classes from src/index.html (simplified for validation)
class SimpleRandomForest {
    predictAll(features) {
        const scores = {};
        for (const [name, champion] of Object.entries(allChampions)) {
            let score = 0;
            let max = 0;
            // Match role OR herotype
            if (features.role === 'No Preference' || features.role === champion.role || features.role === champion.herotype) { 
                score += 40; 
            }
            max += 40;
            const diffDist = Math.abs(features.difficulty - (champion.difficulty || 0));
            score += Math.max(0, 20 - diffDist * 2); max += 20;
            const damageDist = Math.abs(features.damage - (champion.attributes?.damage || 0));
            score += Math.max(0, 15 - damageDist * 1.5); max += 15;
            
            scores[name] = { score: (score / max) * 100 };
        }
        return scores;
    }
}

class SimpleDecisionTree {
    predictAll(features) {
        const scores = {};
        for (const [name, champion] of Object.entries(allChampions)) {
            let score = 0;
            if (features.role === 'No Preference' || features.role === champion.role || features.role === champion.herotype) score += 40;
            const diffMatch = Math.max(0, 15 - Math.abs(features.difficulty - (champion.difficulty || 0)) * 2);
            score += diffMatch;
            const damageMatch = Math.max(0, 15 - Math.abs(features.damage - (champion.attributes?.damage || 0)) * 1.5);
            score += damageMatch;
            
            // Normalized based on 190 max as in index.html
            const normalized = ((score - (-20)) / (190 - (-20))) * 100;
            scores[name] = { score: Math.max(0, Math.min(100, normalized)) };
        }
        return scores;
    }
}

class SimpleKNN {
    normalize(value) { return (value) / (3); } // Data is 0-3 in attributes
    calculateDistance(features, champion) {
        let sumSquares = 0;
        const numericFeatures = ['difficulty', 'damage', 'toughness', 'mobility', 'control', 'utility'];
        for (const f of numericFeatures) {
            const userVal = this.normalize(features[f] / 3.33); // Map 0-10 to 0-3
            const champVal = this.normalize(champion.attributes?.[f] || 0);
            sumSquares += Math.pow(userVal - champVal, 2);
        }
        const roleMatch = (features.role === 'No Preference' || features.role === champion.role || features.role === champion.herotype) ? 0 : 1;
        sumSquares += roleMatch * roleMatch * 2;
        return Math.sqrt(sumSquares);
    }
    predictAll(features) {
        const scores = {};
        for (const [name, champion] of Object.entries(allChampions)) {
            const dist = this.calculateDistance(features, champion);
            // maxDistance approx 3.5
            scores[name] = { score: Math.max(0, 100 - (dist / 3.5) * 100) };
        }
        return scores;
    }
}

class ScoreAggregator {
    static aggregateScores(rf, dt, knn) {
        const aggregated = {};
        for (const name of Object.keys(allChampions)) {
            const r = rf[name]?.score || 0;
            const d = dt[name]?.score || 0;
            const k = knn[name]?.score || 0;
            aggregated[name] = (r * 0.4) + (d * 0.3) + (k * 0.3);
        }
        return aggregated;
    }
}

// Ground Truth Profiles
const validationProfiles = [
    {
        name: "Aggressive Carry",
        features: { role: 'Marksman', difficulty: 5, damage: 9, toughness: 2, mobility: 5, control: 3, utility: 3, position: 'Bottom' },
        expectedRelevant: ['Jinx', 'Caitlyn', 'Draven', 'Vayne', 'Ashe']
    },
    {
        name: "Strategic Tank",
        features: { role: 'Tank', difficulty: 3, damage: 3, toughness: 9, mobility: 2, control: 8, utility: 5, position: 'Top' },
        expectedRelevant: ['Malphite', 'Leona', 'Alistar', 'Braum', 'Maokai']
    },
    {
        name: "Utility Support",
        features: { role: 'Support', difficulty: 4, damage: 2, toughness: 3, mobility: 4, control: 7, utility: 9, position: 'Support' },
        expectedRelevant: ['Soraka', 'Nami', 'Sona', 'Janna', 'Lulu']
    }
];

function runValidation() {
    console.log("=== Ensemble Model Validation Suite ===\n");
    const rf = new SimpleRandomForest();
    const dt = new SimpleDecisionTree();
    const knn = new SimpleKNN();

    let totalScore = 0;

    validationProfiles.forEach(profile => {
        console.log(`Testing Profile: ${profile.name}`);
        const rfResults = rf.predictAll(profile.features);
        const dtResults = dt.predictAll(profile.features);
        const knnResults = knn.predictAll(profile.features);
        
        const aggregated = ScoreAggregator.aggregateScores(rfResults, dtResults, knnResults);
        
        // Sort and get top 5
        const recommendations = Object.entries(aggregated)
            .sort((a, b) => b[1] - a[1])
            .slice(0, 5)
            .map(e => e[0]);

        console.log(`  Top 5: ${recommendations.join(', ')}`);
        
        console.log(`  Scores for expected champions:`);
        profile.expectedRelevant.forEach(exp => {
            console.log(`    - ${exp}: ${aggregated[exp] ? aggregated[exp].toFixed(2) : 'N/A'}%`);
        });
        
        // Calculate Precision@5 against expected
        const hits = recommendations.filter(r => profile.expectedRelevant.includes(r)).length;
        const precision = (hits / 5) * 100;
        totalScore += precision;
        
        console.log(`  Precision@5: ${precision}% (${hits}/5 hits)\n`);
    });

    console.log(`Overall System Precision: ${(totalScore / validationProfiles.length).toFixed(2)}%`);
}

runValidation();
