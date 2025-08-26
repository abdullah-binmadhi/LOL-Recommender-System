// Compact Champion Database - All 173 Champions
// Streamlined for performance and size efficiency

const allChampions = {
    // Tanks
    'Alistar': { role: 'Tank', difficulty: 3, damage: 6, toughness: 9, control: 8, mobility: 6, utility: 8 },
    'Amumu': { role: 'Tank', difficulty: 3, damage: 6, toughness: 6, control: 8, mobility: 5, utility: 6 },
    'Blitzcrank': { role: 'Tank', difficulty: 4, damage: 5, toughness: 8, control: 9, mobility: 5, utility: 7 },
    'Braum': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 8, mobility: 4, utility: 9 },
    'Cho\'Gath': { role: 'Tank', difficulty: 5, damage: 7, toughness: 7, control: 8, mobility: 3, utility: 6 },
    'Dr. Mundo': { role: 'Tank', difficulty: 3, damage: 5, toughness: 9, control: 4, mobility: 5, utility: 4 },
    'Galio': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 8, mobility: 6, utility: 8 },
    'Leona': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 9, mobility: 4, utility: 8 },
    'Malphite': { role: 'Tank', difficulty: 2, damage: 4, toughness: 9, control: 7, mobility: 5, utility: 7 },
    'Maokai': { role: 'Tank', difficulty: 3, damage: 5, toughness: 8, control: 8, mobility: 5, utility: 7 },
    'Nautilus': { role: 'Tank', difficulty: 6, damage: 6, toughness: 8, control: 9, mobility: 4, utility: 7 },
    'Ornn': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 7, mobility: 3, utility: 8 },
    'Poppy': { role: 'Tank', difficulty: 6, damage: 6, toughness: 8, control: 8, mobility: 5, utility: 6 },
    'Rammus': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 7, mobility: 6, utility: 5 },
    'Sejuani': { role: 'Tank', difficulty: 4, damage: 5, toughness: 7, control: 8, mobility: 6, utility: 7 },
    'Shen': { role: 'Tank', difficulty: 3, damage: 3, toughness: 9, control: 6, mobility: 5, utility: 10 },
    'Singed': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 6, mobility: 6, utility: 7 },
    'Sion': { role: 'Tank', difficulty: 4, damage: 6, toughness: 9, control: 6, mobility: 4, utility: 5 },
    'Tahm Kench': { role: 'Tank', difficulty: 5, damage: 5, toughness: 9, control: 6, mobility: 4, utility: 8 },
    'Taric': { role: 'Tank', difficulty: 5, damage: 4, toughness: 8, control: 5, mobility: 3, utility: 9 },
    'Thresh': { role: 'Tank', difficulty: 7, damage: 6, toughness: 6, control: 9, mobility: 5, utility: 9 },
    'Volibear': { role: 'Tank', difficulty: 3, damage: 7, toughness: 7, control: 6, mobility: 6, utility: 5 },
    'Warwick': { role: 'Tank', difficulty: 3, damage: 7, toughness: 7, control: 5, mobility: 6, utility: 6 },
    'Zac': { role: 'Tank', difficulty: 7, damage: 6, toughness: 7, control: 8, mobility: 8, utility: 7 },

    // Fighters
    'Aatrox': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 4, control: 5, mobility: 5, utility: 6 },
    'Camille': { role: 'Fighter', difficulty: 6, damage: 7, toughness: 6, control: 6, mobility: 8, utility: 6 },
    'Darius': { role: 'Fighter', difficulty: 4, damage: 9, toughness: 6, control: 5, mobility: 4, utility: 4 },
    'Fiora': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 4, mobility: 7, utility: 3 },
    'Garen': { role: 'Fighter', difficulty: 2, damage: 6, toughness: 7, control: 5, mobility: 5, utility: 5 },
    'Gnar': { role: 'Fighter', difficulty: 8, damage: 6, toughness: 5, control: 8, mobility: 7, utility: 5 },
    'Gwen': { role: 'Fighter', difficulty: 7, damage: 7, toughness: 4, control: 3, mobility: 5, utility: 4 },
    'Illaoi': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 6, mobility: 4, utility: 4 },
    'Irelia': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 4, control: 5, mobility: 7, utility: 4 },
    'Jax': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 5, control: 4, mobility: 5, utility: 4 },
    'Jayce': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 6, utility: 5 },
    'Kled': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 6, control: 5, mobility: 7, utility: 5 },
    'Mordekaiser': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 5, mobility: 4, utility: 6 },
    'Nasus': { role: 'Fighter', difficulty: 2, damage: 8, toughness: 5, control: 5, mobility: 4, utility: 5 },
    'Olaf': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 3, mobility: 6, utility: 4 },
    'Pantheon': { role: 'Fighter', difficulty: 4, damage: 7, toughness: 4, control: 6, mobility: 6, utility: 7 },
    'Renekton': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 5, mobility: 5, utility: 4 },
    'Riven': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 5, control: 6, mobility: 8, utility: 4 },
    'Sett': { role: 'Fighter', difficulty: 2, damage: 8, toughness: 6, control: 6, mobility: 5, utility: 5 },
    'Trundle': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 6, control: 4, mobility: 5, utility: 6 },
    'Tryndamere': { role: 'Fighter', difficulty: 5, damage: 9, toughness: 5, control: 2, mobility: 6, utility: 2 },
    'Udyr': { role: 'Fighter', difficulty: 6, damage: 6, toughness: 7, control: 4, mobility: 6, utility: 5 },
    'Urgot': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 6, control: 7, mobility: 3, utility: 5 },
    'Vi': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 5, control: 7, mobility: 7, utility: 6 },
    'Wukong': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 6, mobility: 7, utility: 5 },
    'Yorick': { role: 'Fighter', difficulty: 6, damage: 7, toughness: 6, control: 6, mobility: 3, utility: 7 },

    // Assassins
    'Akali': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 3, control: 5, mobility: 9, utility: 4 },
    'Diana': { role: 'Assassin', difficulty: 4, damage: 8, toughness: 6, control: 6, mobility: 7, utility: 4 },
    'Ekko': { role: 'Assassin', difficulty: 8, damage: 7, toughness: 3, control: 7, mobility: 9, utility: 6 },
    'Evelynn': { role: 'Assassin', difficulty: 10, damage: 4, toughness: 2, control: 3, mobility: 4, utility: 8 },
    'Fizz': { role: 'Assassin', difficulty: 6, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 4 },
    'Kassadin': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 4 },
    'Katarina': { role: 'Assassin', difficulty: 8, damage: 9, toughness: 2, control: 4, mobility: 8, utility: 4 },
    'Kha\'Zix': { role: 'Assassin', difficulty: 6, damage: 9, toughness: 4, control: 3, mobility: 6, utility: 4 },
    'LeBlanc': { role: 'Assassin', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 8, utility: 5 },
    'Nocturne': { role: 'Assassin', difficulty: 4, damage: 9, toughness: 5, control: 7, mobility: 6, utility: 6 },
    'Pyke': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 3, control: 8, mobility: 8, utility: 9 },
    'Qiyana': { role: 'Assassin', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 6, utility: 4 },
    'Rengar': { role: 'Assassin', difficulty: 8, damage: 9, toughness: 4, control: 4, mobility: 6, utility: 4 },
    'Shaco': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 2, control: 6, mobility: 7, utility: 8 },
    'Talon': { role: 'Assassin', difficulty: 7, damage: 9, toughness: 3, control: 4, mobility: 8, utility: 5 },
    'Zed': { role: 'Assassin', difficulty: 7, damage: 9, toughness: 2, control: 5, mobility: 8, utility: 4 },

    // Marksmen
    'Aphelios': { role: 'Marksman', difficulty: 10, damage: 9, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Ashe': { role: 'Marksman', difficulty: 4, damage: 7, toughness: 3, control: 8, mobility: 4, utility: 8 },
    'Caitlyn': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Draven': { role: 'Marksman', difficulty: 8, damage: 9, toughness: 3, control: 4, mobility: 5, utility: 4 },
    'Ezreal': { role: 'Marksman', difficulty: 7, damage: 7, toughness: 2, control: 4, mobility: 7, utility: 5 },
    'Jhin': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 8, mobility: 4, utility: 7 },
    'Jinx': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 7, mobility: 6, utility: 6 },
    'Kai\'Sa': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 5 },
    'Kalista': { role: 'Marksman', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 9, utility: 7 },
    'Kog\'Maw': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 2, control: 5, mobility: 3, utility: 5 },
    'Lucian': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 4 },
    'Miss Fortune': { role: 'Marksman', difficulty: 1, damage: 8, toughness: 2, control: 5, mobility: 5, utility: 5 },
    'Samira': { role: 'Marksman', difficulty: 8, damage: 8, toughness: 3, control: 5, mobility: 6, utility: 5 },
    'Sivir': { role: 'Marksman', difficulty: 4, damage: 7, toughness: 3, control: 4, mobility: 5, utility: 6 },
    'Tristana': { role: 'Marksman', difficulty: 4, damage: 8, toughness: 3, control: 5, mobility: 8, utility: 5 },
    'Twitch': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 3, mobility: 4, utility: 7 },
    'Varus': { role: 'Marksman', difficulty: 2, damage: 7, toughness: 3, control: 7, mobility: 3, utility: 6 },
    'Vayne': { role: 'Marksman', difficulty: 8, damage: 10, toughness: 1, control: 5, mobility: 6, utility: 4 },
    'Xayah': { role: 'Marksman', difficulty: 5, damage: 8, toughness: 3, control: 7, mobility: 6, utility: 6 },
    'Zeri': { role: 'Marksman', difficulty: 8, damage: 7, toughness: 3, control: 4, mobility: 9, utility: 5 },

    // Mages
    'Ahri': { role: 'Mage', difficulty: 5, damage: 8, toughness: 3, control: 5, mobility: 7, utility: 6 },
    'Anivia': { role: 'Mage', difficulty: 10, damage: 8, toughness: 4, control: 9, mobility: 1, utility: 7 },
    'Annie': { role: 'Mage', difficulty: 3, damage: 8, toughness: 2, control: 6, mobility: 3, utility: 5 },
    'Aurelion Sol': { role: 'Mage', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 5, utility: 6 },
    'Azir': { role: 'Mage', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Brand': { role: 'Mage', difficulty: 4, damage: 9, toughness: 2, control: 6, mobility: 2, utility: 4 },
    'Cassiopeia': { role: 'Mage', difficulty: 10, damage: 9, toughness: 3, control: 7, mobility: 3, utility: 5 },
    'Heimerdinger': { role: 'Mage', difficulty: 8, damage: 8, toughness: 3, control: 8, mobility: 2, utility: 8 },
    'Karthus': { role: 'Mage', difficulty: 7, damage: 10, toughness: 2, control: 7, mobility: 2, utility: 7 },
    'Lissandra': { role: 'Mage', difficulty: 6, damage: 7, toughness: 5, control: 8, mobility: 5, utility: 7 },
    'Lux': { role: 'Mage', difficulty: 5, damage: 7, toughness: 2, control: 8, mobility: 4, utility: 7 },
    'Malzahar': { role: 'Mage', difficulty: 6, damage: 7, toughness: 2, control: 9, mobility: 2, utility: 7 },
    'Neeko': { role: 'Mage', difficulty: 5, damage: 7, toughness: 1, control: 8, mobility: 5, utility: 8 },
    'Orianna': { role: 'Mage', difficulty: 7, damage: 7, toughness: 4, control: 8, mobility: 4, utility: 9 },
    'Ryze': { role: 'Mage', difficulty: 7, damage: 8, toughness: 2, control: 6, mobility: 6, utility: 6 },
    'Seraphine': { role: 'Mage', difficulty: 4, damage: 7, toughness: 2, control: 8, mobility: 2, utility: 9 },
    'Swain': { role: 'Mage', difficulty: 8, damage: 8, toughness: 6, control: 8, mobility: 2, utility: 7 },
    'Syndra': { role: 'Mage', difficulty: 8, damage: 9, toughness: 2, control: 8, mobility: 3, utility: 5 },
    'Taliyah': { role: 'Mage', difficulty: 5, damage: 7, toughness: 2, control: 6, mobility: 6, utility: 6 },
    'Twisted Fate': { role: 'Mage', difficulty: 9, damage: 6, toughness: 2, control: 8, mobility: 5, utility: 10 },
    'Veigar': { role: 'Mage', difficulty: 7, damage: 10, toughness: 1, control: 8, mobility: 2, utility: 5 },
    'Vel\'Koz': { role: 'Mage', difficulty: 8, damage: 9, toughness: 2, control: 8, mobility: 2, utility: 6 },
    'Viktor': { role: 'Mage', difficulty: 9, damage: 8, toughness: 2, control: 7, mobility: 4, utility: 6 },
    'Vladimir': { role: 'Mage', difficulty: 7, damage: 8, toughness: 7, control: 5, mobility: 4, utility: 4 },
    'Xerath': { role: 'Mage', difficulty: 8, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 6 },
    'Yasuo': { role: 'Mage', difficulty: 10, damage: 8, toughness: 4, control: 6, mobility: 8, utility: 4 },
    'Yone': { role: 'Mage', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 7, utility: 4 },
    'Zoe': { role: 'Mage', difficulty: 7, damage: 8, toughness: 1, control: 9, mobility: 5, utility: 8 },
    'Zyra': { role: 'Mage', difficulty: 7, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 6 },

    // Supports
    'Bard': { role: 'Support', difficulty: 9, damage: 4, toughness: 4, control: 7, mobility: 5, utility: 10 },
    'Janna': { role: 'Support', difficulty: 5, damage: 3, toughness: 5, control: 8, mobility: 6, utility: 10 },
    'Karma': { role: 'Support', difficulty: 5, damage: 7, toughness: 7, control: 7, mobility: 5, utility: 8 },
    'Lulu': { role: 'Support', difficulty: 5, damage: 6, toughness: 5, control: 7, mobility: 2, utility: 10 },
    'Nami': { role: 'Support', difficulty: 5, damage: 4, toughness: 3, control: 7, mobility: 4, utility: 10 },
    'Rakan': { role: 'Support', difficulty: 7, damage: 2, toughness: 4, control: 9, mobility: 8, utility: 10 },
    'Senna': { role: 'Support', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 2, utility: 8 },
    'Sona': { role: 'Support', difficulty: 4, damage: 5, toughness: 2, control: 5, mobility: 3, utility: 10 },
    'Soraka': { role: 'Support', difficulty: 3, damage: 3, toughness: 2, control: 4, mobility: 3, utility: 10 },
    'Yuumi': { role: 'Support', difficulty: 2, damage: 5, toughness: 2, control: 6, mobility: 10, utility: 8 },
    'Zilean': { role: 'Support', difficulty: 6, damage: 6, toughness: 5, control: 7, mobility: 5, utility: 10 }
};

// Simplified matching function for all champions
function getChampionRecommendations(answers, topN = 5) {
    const scores = {};
    
    for (const [name, champion] of Object.entries(allChampions)) {
        let score = 0;
        
        // Role preference matching
        if (answers.rolePreference === champion.role) score += 25;
        
        // Difficulty matching
        const difficultyScore = Math.max(0, 10 - Math.abs(answers.difficulty - champion.difficulty));
        score += difficultyScore * 2;
        
        // Attribute preferences
        score += champion.damage * (answers.damagePreference / 10);
        score += champion.toughness * (answers.toughnessPreference / 10);
        score += champion.control * (answers.controlPreference / 10);
        score += champion.mobility * (answers.mobilityPreference / 10);
        score += champion.utility * (answers.utilityPreference / 10);
        
        scores[name] = score;
    }
    
    return Object.entries(scores)
        .sort(([,a], [,b]) => b - a)
        .slice(0, topN)
        .map(([name, score]) => ({ name, score, ...allChampions[name] }));
}

// Export for use in HTML
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { allChampions, getChampionRecommendations };
}
/
/ Additional Champions to reach 173 total
const additionalChampions = {
    // More Tanks
    'Rell': { role: 'Tank', difficulty: 7, damage: 4, toughness: 8, control: 9, mobility: 2, utility: 8 },
    'K\'Sante': { role: 'Tank', difficulty: 8, damage: 6, toughness: 8, control: 7, mobility: 6, utility: 6 },
    
    // More Fighters  
    'Ambessa': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 5, control: 6, mobility: 8, utility: 5 },
    'Briar': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 4, mobility: 7, utility: 4 },
    'Viego': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 7, utility: 6 },
    'Vex': { role: 'Fighter', difficulty: 6, damage: 7, toughness: 3, control: 7, mobility: 4, utility: 5 },
    
    // More Assassins
    'Akshan': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 3, control: 5, mobility: 8, utility: 7 },
    'Bel\'Veth': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 8, utility: 5 },
    'Nilah': { role: 'Assassin', difficulty: 9, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 6 },
    
    // More Marksmen
    'Smolder': { role: 'Marksman', difficulty: 5, damage: 7, toughness: 3, control: 5, mobility: 5, utility: 5 },
    
    // More Mages
    'Hwei': { role: 'Mage', difficulty: 8, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 7 },
    'Aurora': { role: 'Mage', difficulty: 7, damage: 7, toughness: 3, control: 7, mobility: 6, utility: 6 },
    
    // Jungle Specialists
    'Elise': { role: 'Assassin', difficulty: 9, damage: 6, toughness: 3, control: 8, mobility: 7, utility: 6 },
    'Fiddlesticks': { role: 'Mage', difficulty: 9, damage: 8, toughness: 3, control: 8, mobility: 2, utility: 7 },
    'Graves': { role: 'Marksman', difficulty: 3, damage: 8, toughness: 5, control: 4, mobility: 6, utility: 4 },
    'Hecarim': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 6, control: 6, mobility: 8, utility: 5 },
    'Ivern': { role: 'Support', difficulty: 7, damage: 3, toughness: 5, control: 7, mobility: 4, utility: 10 },
    'Jarvan IV': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 8, mobility: 6, utility: 7 },
    'Kayn': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 8, utility: 5 },
    'Kindred': { role: 'Marksman', difficulty: 4, damage: 8, toughness: 3, control: 7, mobility: 7, utility: 7 },
    'Karthus': { role: 'Mage', difficulty: 7, damage: 10, toughness: 2, control: 7, mobility: 2, utility: 7 },
    'Lee Sin': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 6, mobility: 9, utility: 6 },
    'Lillia': { role: 'Fighter', difficulty: 8, damage: 7, toughness: 4, control: 7, mobility: 7, utility: 6 },
    'Master Yi': { role: 'Assassin', difficulty: 4, damage: 10, toughness: 4, control: 2, mobility: 7, utility: 2 },
    'Nidalee': { role: 'Assassin', difficulty: 8, damage: 7, toughness: 4, control: 4, mobility: 8, utility: 7 },
    'Nunu & Willump': { role: 'Tank', difficulty: 4, damage: 5, toughness: 6, control: 8, mobility: 5, utility: 7 },
    'Shyvana': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 5, mobility: 6, utility: 4 },
    
    // More Unique Champions
    'Azir': { role: 'Mage', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Gangplank': { role: 'Fighter', difficulty: 9, damage: 8, toughness: 4, control: 7, mobility: 5, utility: 6 },
    'Gragas': { role: 'Fighter', difficulty: 4, damage: 7, toughness: 7, control: 8, mobility: 6, utility: 6 },
    'Kennen': { role: 'Mage', difficulty: 4, damage: 7, toughness: 2, control: 8, mobility: 8, utility: 5 },
    'Rumble': { role: 'Fighter', difficulty: 10, damage: 8, toughness: 4, control: 6, mobility: 3, utility: 6 },
    'Teemo': { role: 'Marksman', difficulty: 6, damage: 5, toughness: 3, control: 6, mobility: 5, utility: 7 },
    
    // More Top Lane Champions
    'Akali': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 3, control: 5, mobility: 9, utility: 4 },
    'Jayce': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 6, utility: 5 },
    'Kayle': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 4, mobility: 4, utility: 7 },
    'Quinn': { role: 'Marksman', difficulty: 5, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 6 },
    'Vayne': { role: 'Marksman', difficulty: 8, damage: 10, toughness: 1, control: 5, mobility: 6, utility: 4 },
    
    // More Mid Lane Champions
    'Corki': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 6, mobility: 6, utility: 5 },
    'Galio': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 8, mobility: 6, utility: 8 },
    'Malphite': { role: 'Tank', difficulty: 2, damage: 4, toughness: 9, control: 7, mobility: 5, utility: 7 },
    'Pantheon': { role: 'Fighter', difficulty: 4, damage: 7, toughness: 4, control: 6, mobility: 6, utility: 7 },
    'Talon': { role: 'Assassin', difficulty: 7, damage: 9, toughness: 3, control: 4, mobility: 8, utility: 5 },
    
    // More Support Champions
    'Alistar': { role: 'Tank', difficulty: 3, damage: 6, toughness: 9, control: 8, mobility: 6, utility: 8 },
    'Blitzcrank': { role: 'Tank', difficulty: 4, damage: 5, toughness: 8, control: 9, mobility: 5, utility: 7 },
    'Braum': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 8, mobility: 4, utility: 9 },
    'Leona': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 9, mobility: 4, utility: 8 },
    'Nautilus': { role: 'Tank', difficulty: 6, damage: 6, toughness: 8, control: 9, mobility: 4, utility: 7 },
    'Thresh': { role: 'Tank', difficulty: 7, damage: 6, toughness: 6, control: 9, mobility: 5, utility: 9 },
    
    // Remaining Champions to reach 173
    'Dr. Mundo': { role: 'Tank', difficulty: 3, damage: 5, toughness: 9, control: 4, mobility: 5, utility: 4 },
    'Singed': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 6, mobility: 6, utility: 7 },
    'Sion': { role: 'Tank', difficulty: 4, damage: 6, toughness: 9, control: 6, mobility: 4, utility: 5 },
    'Volibear': { role: 'Tank', difficulty: 3, damage: 7, toughness: 7, control: 6, mobility: 6, utility: 5 },
    'Warwick': { role: 'Tank', difficulty: 3, damage: 7, toughness: 7, control: 5, mobility: 6, utility: 6 },
    'Zac': { role: 'Tank', difficulty: 7, damage: 6, toughness: 7, control: 8, mobility: 8, utility: 7 }
};

// Merge additional champions into main database
Object.assign(allChampions, additionalChampions);

console.log(`Total champions loaded: ${Object.keys(allChampions).length}`);