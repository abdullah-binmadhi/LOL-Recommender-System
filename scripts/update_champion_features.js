const fs = require('fs');
const path = require('path');
const FeatureExtractor = require('../src/analytics/FeatureExtractor.js');

const DATA_FILE = path.join(__dirname, '../src/data/champions.json');

// Heuristics
function getDamageType(role, herotype) {
    const type = herotype || role;
    if (['Mage', 'Support'].includes(type)) return 'Magic';
    if (['Fighter', 'Marksman', 'Assassin', 'Tank'].includes(type)) return 'Physical';
    return 'Mixed';
}

function getPowerSpike(role, herotype) {
    const type = herotype || role;
    if (['Assassin'].includes(type)) return 'Early';
    if (['Marksman', 'Mage'].includes(type)) return 'Late';
    return 'Mid';
}

function getSkillCap(difficulty) {
    if (difficulty === 1) return 'Low';
    if (difficulty === 3) return 'High';
    return 'Medium';
}

function updateChampions() {
    if (!fs.existsSync(DATA_FILE)) {
        console.error(`Error: File not found at ${DATA_FILE}`);
        process.exit(1);
    }

    const rawData = fs.readFileSync(DATA_FILE, 'utf8');
    const data = JSON.parse(rawData);
    
    if (!data.champions) {
        console.error("Error: Invalid JSON structure (missing 'champions' key)");
        process.exit(1);
    }

    let updatedCount = 0;

    data.champions = data.champions.map(champion => {
        const attributes = champion.attributes || {};
        
        // Calculate Derived Stats
        const burst = FeatureExtractor.calculateBurst(attributes);
        const sustain = FeatureExtractor.calculateSustain(attributes);
        const cc = FeatureExtractor.calculateCC(attributes);

        // Add Derived Stats
        champion.derived_stats = {
            burst: parseFloat(burst.toFixed(2)),
            sustain: parseFloat(sustain.toFixed(2)),
            cc: parseFloat(cc.toFixed(2))
        };

        // Add New Features (if missing)
        if (!champion.damage_type) {
            champion.damage_type = getDamageType(champion.role, champion.herotype);
        }
        if (!champion.power_spike) {
            champion.power_spike = getPowerSpike(champion.role, champion.herotype);
        }
        if (!champion.skill_cap) {
            champion.skill_cap = getSkillCap(champion.difficulty);
        }

        updatedCount++;
        return champion;
    });

    // Write back to file
    fs.writeFileSync(DATA_FILE, JSON.stringify(data, null, 2), 'utf8');
    console.log(`Successfully updated ${updatedCount} champions with new features.`);
}

updateChampions();
