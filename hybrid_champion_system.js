// Hybrid Champion System - Detailed data for popular champions, compact for all 173
// This approach balances functionality with file size

// Detailed champion data (top 30 most popular champions)
const detailedChampions = {
    // Popular champions with full data structure from your existing system
    'Jinx': {
        name: 'Jinx',
        title: 'the Loose Cannon',
        role: 'Marksman',
        difficulty: 6,
        tags: ['Marksman'],
        attributes: { damage: 9, toughness: 2, control: 7, mobility: 6, utility: 6 },
        description: 'A manic and impulsive criminal from Zaun, Jinx lives to wreak havoc without care for the consequences.',
        abilities: [
            { key: 'P', name: 'Get Excited!', description: 'Jinx gains movement speed and attack speed when damaging enemy champions, structures, or killing enemies.' },
            { key: 'Q', name: 'Switcheroo!', description: 'Jinx swaps weapons between Pow-Pow (machine gun) and Fishbones (rocket launcher).' },
            { key: 'W', name: 'Zap!', description: 'Jinx uses Zapper to fire a shock blast that deals damage and slows the first enemy hit.' },
            { key: 'E', name: 'Flame Chompers!', description: 'Jinx tosses out a line of snare grenades that explode after a delay, rooting enemies.' },
            { key: 'R', name: 'Super Mega Death Rocket!', description: 'Jinx fires a rocket across the map that gains damage as it travels and explodes on impact.' }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10 },
            difficulty: { 'Moderate': 6, 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Solo plays': 8, 'Team fights': 9, 'Farming': 7 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 10 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Burst Damage': 8, 'Sustained Damage': 9 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Winning games': 9, 'Having fun': 10 }
        }
    },

    'Yasuo': {
        name: 'Yasuo',
        title: 'the Unforgiven',
        role: 'Fighter',
        difficulty: 10,
        tags: ['Fighter', 'Assassin'],
        attributes: { damage: 8, toughness: 4, control: 6, mobility: 8, utility: 4 },
        description: 'An Ionian of deep resolve, Yasuo is an agile swordsman who wields the air itself against his enemies.',
        abilities: [
            { key: 'P', name: 'Way of the Wanderer', description: 'Yasuo\'s critical strike chance is doubled, but his critical strikes deal reduced damage.' },
            { key: 'Q', name: 'Steel Tempest', description: 'Yasuo thrusts forward, dealing damage. On hit, grants a stack of Gathering Storm.' },
            { key: 'W', name: 'Wind Wall', description: 'Yasuo creates a wall of wind that blocks all enemy projectiles.' },
            { key: 'E', name: 'Sweeping Blade', description: 'Yasuo dashes through a target, dealing damage.' },
            { key: 'R', name: 'Last Breath', description: 'Yasuo teleports to an airborne enemy champion, dealing damage and suspending them longer.' }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 9, 'Assassin': 8 },
            difficulty: { 'Expert': 10, 'Hard': 8 },
            gameplay: { 'Solo plays': 10, 'Outplaying opponents': 10 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Sustained Damage': 9, 'Mobility': 10 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Improving skills': 10, 'Outplaying opponents': 10 }
        }
    },

    'Lux': {
        name: 'Lux',
        title: 'the Lady of Luminosity',
        role: 'Mage',
        difficulty: 5,
        tags: ['Mage', 'Support'],
        attributes: { damage: 7, toughness: 2, control: 8, mobility: 4, utility: 7 },
        description: 'Luxanna Crownguard hails from Demacia, wielding light magic to protect her allies and devastate her enemies.',
        abilities: [
            { key: 'P', name: 'Illumination', description: 'Lux\'s spells charge enemies with energy for a few seconds. Her next attack ignites the energy for bonus damage.' },
            { key: 'Q', name: 'Light Binding', description: 'Lux releases a sphere of light that binds and deals damage to up to two enemies.' },
            { key: 'W', name: 'Prismatic Barrier', description: 'Lux throws her wand and bends the light around any friendly target it touches, protecting them from enemy damage.' },
            { key: 'E', name: 'Lucent Singularity', description: 'Creates an area of twisted light that slows enemies and can be detonated for damage.' },
            { key: 'R', name: 'Final Spark', description: 'Lux fires a laser that deals damage to all enemies in a line and refreshes Illumination.' }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10, 'Support': 7 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Team fights': 9, 'Supporting allies': 8, 'Solo plays': 6 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 8, 'Very High': 7 },
            style: { 'Offensive': 8, 'Balanced': 9 },
            abilities: { 'Burst Damage': 9, 'Crowd Control': 8 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Having fun': 9, 'Learning new champions': 8 }
        }
    },

    'Garen': {
        name: 'Garen',
        title: 'The Might of Demacia',
        role: 'Fighter',
        difficulty: 2,
        tags: ['Fighter', 'Tank'],
        attributes: { damage: 6, toughness: 7, control: 5, mobility: 5, utility: 5 },
        description: 'A proud and noble warrior, Garen fights as one of the Dauntless Vanguard.',
        abilities: [
            { key: 'P', name: 'Perseverance', description: 'Garen regenerates health over time when he hasn\'t taken damage recently.' },
            { key: 'Q', name: 'Decisive Strike', description: 'Garen gains movement speed and his next attack deals bonus damage and silences the target.' },
            { key: 'W', name: 'Courage', description: 'Garen reduces incoming damage and gains tenacity for a short duration.' },
            { key: 'E', name: 'Judgment', description: 'Garen spins with his sword, dealing damage to nearby enemies.' },
            { key: 'R', name: 'Demacian Justice', description: 'Garen calls upon Demacian justice to deal true damage based on the enemy\'s missing health.' }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 10, 'Tank': 6 },
            difficulty: { 'Easy': 10, 'Moderate': 7 },
            gameplay: { 'Solo plays': 8, 'Team fights': 7 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'Medium': 8, 'High': 7 },
            style: { 'Balanced': 9, 'Offensive': 7 },
            abilities: { 'Sustained Damage': 8, 'Burst Damage': 6 },
            experience: { 'Beginner': 10, 'Intermediate': 8 },
            motivation: { 'Learning new champions': 8, 'Having fun': 9 }
        }
    },

    'Ashe': {
        name: 'Ashe',
        title: 'the Frost Archer',
        role: 'Marksman',
        difficulty: 4,
        tags: ['Marksman', 'Support'],
        attributes: { damage: 7, toughness: 3, control: 8, mobility: 4, utility: 8 },
        description: 'Ashe is a champion of the Freljord, using her ice magic and bow to lead her people and control the battlefield.',
        abilities: [
            { key: 'P', name: 'Frost Shot', description: 'Ashe\'s attacks slow enemies and deal increased damage to slowed targets.' },
            { key: 'Q', name: 'Ranger\'s Focus', description: 'Ashe builds Focus by attacking. At maximum Focus, she can cast to gain attack speed and fire multiple arrows.' },
            { key: 'W', name: 'Volley', description: 'Ashe fires arrows in a cone, dealing damage and slowing enemies.' },
            { key: 'E', name: 'Hawkshot', description: 'Ashe sends a hawk spirit across the map, granting vision of enemies it passes through.' },
            { key: 'R', name: 'Enchanted Crystal Arrow', description: 'Ashe fires a crystal arrow that stuns the first enemy champion hit and deals damage.' }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10, 'Support': 6 },
            difficulty: { 'Easy': 8, 'Moderate': 9 },
            gameplay: { 'Team fights': 8, 'Supporting allies': 9, 'Farming': 7 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'Medium': 7, 'High': 8 },
            style: { 'Balanced': 9, 'Offensive': 7 },
            abilities: { 'Crowd Control': 8, 'Sustained Damage': 8 },
            experience: { 'Beginner': 8, 'Intermediate': 9 },
            motivation: { 'Support team': 8, 'Learning new champions': 7 }
        }
    },

    'Thresh': {
        name: 'Thresh',
        title: 'the Chain Warden',
        role: 'Support',
        difficulty: 7,
        tags: ['Support', 'Fighter'],
        attributes: { damage: 6, toughness: 6, control: 9, mobility: 5, utility: 9 },
        description: 'Sadistic and cunning, Thresh is an ambitious and restless spirit of the Shadow Isles.',
        abilities: [
            { key: 'P', name: 'Damnation', description: 'Thresh can harvest souls to permanently increase his armor and ability power.' },
            { key: 'Q', name: 'Death Sentence', description: 'Thresh binds an enemy in chains and pulls them toward him. Can reactivate to pull himself to the enemy.' },
            { key: 'W', name: 'Dark Passage', description: 'Thresh throws his lantern to a target location. Allies can click it to dash to Thresh.' },
            { key: 'E', name: 'Flay', description: 'Thresh sweeps his chain, knocking back enemies and dealing damage.' },
            { key: 'R', name: 'The Box', description: 'Thresh creates a prison of walls around himself. Enemies who walk through walls are slowed and take damage.' }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Supporting allies': 10, 'Team fights': 9 },
            range: { 'Ranged': 8, 'No preference': 6 },
            damage: { 'Low': 6, 'Medium': 8 },
            style: { 'Defensive': 8, 'Balanced': 9 },
            abilities: { 'Crowd Control': 10, 'Healing/Shielding': 6 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Support team': 10, 'Improving skills': 8 }
        }
    }
};

// Compact data for all 173 champions (including the detailed ones for consistency)
const allChampionsCompact = {
    // A-D
    'Aatrox': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 4, control: 5, mobility: 5, utility: 6 },
    'Ahri': { role: 'Mage', difficulty: 5, damage: 8, toughness: 3, control: 5, mobility: 7, utility: 6 },
    'Akali': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 3, control: 5, mobility: 9, utility: 4 },
    'Akshan': { role: 'Marksman', difficulty: 7, damage: 8, toughness: 3, control: 5, mobility: 8, utility: 7 },
    'Alistar': { role: 'Tank', difficulty: 3, damage: 6, toughness: 9, control: 8, mobility: 6, utility: 8 },
    'Ambessa': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 5, control: 6, mobility: 8, utility: 5 },
    'Amumu': { role: 'Tank', difficulty: 3, damage: 6, toughness: 6, control: 8, mobility: 5, utility: 6 },
    'Anivia': { role: 'Mage', difficulty: 10, damage: 8, toughness: 4, control: 9, mobility: 1, utility: 7 },
    'Annie': { role: 'Mage', difficulty: 3, damage: 8, toughness: 2, control: 6, mobility: 3, utility: 5 },
    'Aphelios': { role: 'Marksman', difficulty: 10, damage: 9, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Ashe': { role: 'Marksman', difficulty: 4, damage: 7, toughness: 3, control: 8, mobility: 4, utility: 8 },
    'Aurelion Sol': { role: 'Mage', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 5, utility: 6 },
    'Aurora': { role: 'Mage', difficulty: 7, damage: 7, toughness: 3, control: 7, mobility: 6, utility: 6 },
    'Azir': { role: 'Mage', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Bard': { role: 'Support', difficulty: 9, damage: 4, toughness: 4, control: 7, mobility: 5, utility: 10 },
    'Bel\'Veth': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 8, utility: 5 },
    'Blitzcrank': { role: 'Tank', difficulty: 4, damage: 5, toughness: 8, control: 9, mobility: 5, utility: 7 },
    'Brand': { role: 'Mage', difficulty: 4, damage: 9, toughness: 2, control: 6, mobility: 2, utility: 4 },
    'Braum': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 8, mobility: 4, utility: 9 },
    'Briar': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 4, mobility: 7, utility: 4 },
    'Caitlyn': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 2, control: 6, mobility: 4, utility: 6 },
    'Camille': { role: 'Fighter', difficulty: 6, damage: 7, toughness: 6, control: 6, mobility: 8, utility: 6 },
    'Cassiopeia': { role: 'Mage', difficulty: 10, damage: 9, toughness: 3, control: 7, mobility: 3, utility: 5 },
    'Cho\'Gath': { role: 'Tank', difficulty: 5, damage: 7, toughness: 7, control: 8, mobility: 3, utility: 6 },
    'Corki': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 6, mobility: 6, utility: 5 },
    'Darius': { role: 'Fighter', difficulty: 4, damage: 9, toughness: 6, control: 5, mobility: 4, utility: 4 },
    'Diana': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 6, mobility: 7, utility: 4 },
    'Dr. Mundo': { role: 'Tank', difficulty: 3, damage: 5, toughness: 9, control: 4, mobility: 5, utility: 4 },
    'Draven': { role: 'Marksman', difficulty: 8, damage: 9, toughness: 3, control: 4, mobility: 5, utility: 4 },

    // E-H
    'Ekko': { role: 'Assassin', difficulty: 8, damage: 7, toughness: 3, control: 7, mobility: 9, utility: 6 },
    'Elise': { role: 'Mage', difficulty: 9, damage: 6, toughness: 3, control: 8, mobility: 7, utility: 6 },
    'Evelynn': { role: 'Assassin', difficulty: 10, damage: 4, toughness: 2, control: 3, mobility: 4, utility: 8 },
    'Ezreal': { role: 'Marksman', difficulty: 7, damage: 7, toughness: 2, control: 4, mobility: 7, utility: 5 },
    'Fiddlesticks': { role: 'Mage', difficulty: 9, damage: 8, toughness: 3, control: 8, mobility: 2, utility: 7 },
    'Fiora': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 4, mobility: 7, utility: 3 },
    'Fizz': { role: 'Assassin', difficulty: 6, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 4 },
    'Galio': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 8, mobility: 6, utility: 8 },
    'Gangplank': { role: 'Fighter', difficulty: 9, damage: 8, toughness: 4, control: 7, mobility: 5, utility: 6 },
    'Garen': { role: 'Fighter', difficulty: 2, damage: 6, toughness: 7, control: 5, mobility: 5, utility: 5 },
    'Gnar': { role: 'Fighter', difficulty: 8, damage: 6, toughness: 5, control: 8, mobility: 7, utility: 5 },
    'Gragas': { role: 'Fighter', difficulty: 4, damage: 7, toughness: 7, control: 8, mobility: 6, utility: 6 },
    'Graves': { role: 'Marksman', difficulty: 3, damage: 8, toughness: 5, control: 4, mobility: 6, utility: 4 },
    'Gwen': { role: 'Fighter', difficulty: 7, damage: 7, toughness: 4, control: 3, mobility: 5, utility: 4 },
    'Hecarim': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 6, control: 6, mobility: 8, utility: 5 },
    'Heimerdinger': { role: 'Mage', difficulty: 8, damage: 8, toughness: 3, control: 8, mobility: 2, utility: 8 },
    'Hwei': { role: 'Mage', difficulty: 8, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 7 },

    // I-M
    'Illaoi': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 6, mobility: 4, utility: 4 },
    'Irelia': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 4, control: 5, mobility: 7, utility: 4 },
    'Ivern': { role: 'Support', difficulty: 7, damage: 3, toughness: 5, control: 7, mobility: 4, utility: 10 },
    'Janna': { role: 'Support', difficulty: 5, damage: 3, toughness: 5, control: 8, mobility: 6, utility: 10 },
    'Jarvan IV': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 8, mobility: 6, utility: 7 },
    'Jax': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 5, control: 4, mobility: 5, utility: 4 },
    'Jayce': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 6, utility: 5 },
    'Jhin': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 8, mobility: 4, utility: 7 },
    'Jinx': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 7, mobility: 6, utility: 6 },
    'K\'Sante': { role: 'Tank', difficulty: 8, damage: 6, toughness: 8, control: 7, mobility: 6, utility: 6 },
    'Kai\'Sa': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 5 },
    'Kalista': { role: 'Marksman', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 9, utility: 7 },
    'Karma': { role: 'Support', difficulty: 5, damage: 7, toughness: 7, control: 7, mobility: 5, utility: 8 },
    'Karthus': { role: 'Mage', difficulty: 7, damage: 10, toughness: 2, control: 7, mobility: 2, utility: 7 },
    'Kassadin': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 4 },
    'Katarina': { role: 'Assassin', difficulty: 8, damage: 9, toughness: 2, control: 4, mobility: 8, utility: 4 },
    'Kayle': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 4, mobility: 4, utility: 7 },
    'Kayn': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 4, control: 5, mobility: 8, utility: 5 },
    'Kennen': { role: 'Mage', difficulty: 4, damage: 7, toughness: 2, control: 8, mobility: 8, utility: 5 },
    'Kha\'Zix': { role: 'Assassin', difficulty: 6, damage: 9, toughness: 4, control: 3, mobility: 6, utility: 4 },
    'Kindred': { role: 'Marksman', difficulty: 4, damage: 8, toughness: 3, control: 7, mobility: 7, utility: 7 },
    'Kled': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 6, control: 5, mobility: 7, utility: 5 },
    'Kog\'Maw': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 2, control: 5, mobility: 3, utility: 5 },
    'LeBlanc': { role: 'Assassin', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 8, utility: 5 },
    'Lee Sin': { role: 'Fighter', difficulty: 6, damage: 8, toughness: 4, control: 6, mobility: 9, utility: 6 },
    'Leona': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 9, mobility: 4, utility: 8 },
    'Lillia': { role: 'Fighter', difficulty: 8, damage: 7, toughness: 4, control: 7, mobility: 7, utility: 6 },
    'Lissandra': { role: 'Mage', difficulty: 6, damage: 7, toughness: 5, control: 8, mobility: 5, utility: 7 },
    'Lucian': { role: 'Marksman', difficulty: 6, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 4 },
    'Lulu': { role: 'Support', difficulty: 5, damage: 6, toughness: 5, control: 7, mobility: 2, utility: 10 },
    'Lux': { role: 'Mage', difficulty: 5, damage: 7, toughness: 2, control: 8, mobility: 4, utility: 7 },
    'Malphite': { role: 'Tank', difficulty: 2, damage: 4, toughness: 9, control: 7, mobility: 5, utility: 7 },
    'Malzahar': { role: 'Mage', difficulty: 6, damage: 7, toughness: 2, control: 9, mobility: 2, utility: 7 },
    'Maokai': { role: 'Tank', difficulty: 3, damage: 5, toughness: 8, control: 8, mobility: 5, utility: 7 },
    'Master Yi': { role: 'Assassin', difficulty: 4, damage: 10, toughness: 4, control: 2, mobility: 7, utility: 2 },
    'Miss Fortune': { role: 'Marksman', difficulty: 1, damage: 8, toughness: 2, control: 5, mobility: 5, utility: 5 },
    'Mordekaiser': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 5, mobility: 4, utility: 6 },

    // N-R
    'Nami': { role: 'Support', difficulty: 5, damage: 4, toughness: 3, control: 7, mobility: 4, utility: 10 },
    'Nasus': { role: 'Fighter', difficulty: 2, damage: 8, toughness: 5, control: 5, mobility: 4, utility: 5 },
    'Nautilus': { role: 'Tank', difficulty: 6, damage: 6, toughness: 8, control: 9, mobility: 4, utility: 7 },
    'Neeko': { role: 'Mage', difficulty: 5, damage: 7, toughness: 1, control: 8, mobility: 5, utility: 8 },
    'Nidalee': { role: 'Assassin', difficulty: 8, damage: 7, toughness: 4, control: 4, mobility: 8, utility: 7 },
    'Nilah': { role: 'Marksman', difficulty: 9, damage: 8, toughness: 3, control: 4, mobility: 7, utility: 6 },
    'Nocturne': { role: 'Assassin', difficulty: 4, damage: 9, toughness: 5, control: 7, mobility: 6, utility: 6 },
    'Nunu & Willump': { role: 'Tank', difficulty: 4, damage: 5, toughness: 6, control: 8, mobility: 5, utility: 7 },
    'Olaf': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 3, mobility: 6, utility: 4 },
    'Orianna': { role: 'Mage', difficulty: 7, damage: 7, toughness: 4, control: 8, mobility: 4, utility: 9 },
    'Ornn': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 7, mobility: 3, utility: 8 },
    'Pantheon': { role: 'Fighter', difficulty: 4, damage: 7, toughness: 4, control: 6, mobility: 6, utility: 7 },
    'Poppy': { role: 'Tank', difficulty: 6, damage: 6, toughness: 8, control: 8, mobility: 5, utility: 6 },
    'Pyke': { role: 'Support', difficulty: 7, damage: 8, toughness: 3, control: 8, mobility: 8, utility: 9 },
    'Qiyana': { role: 'Assassin', difficulty: 9, damage: 8, toughness: 2, control: 6, mobility: 6, utility: 4 },
    'Quinn': { role: 'Marksman', difficulty: 5, damage: 8, toughness: 4, control: 5, mobility: 9, utility: 6 },
    'Rakan': { role: 'Support', difficulty: 7, damage: 2, toughness: 4, control: 9, mobility: 8, utility: 10 },
    'Rammus': { role: 'Tank', difficulty: 4, damage: 4, toughness: 9, control: 7, mobility: 6, utility: 5 },
    'Rek\'Sai': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 5, control: 7, mobility: 7, utility: 6 },
    'Rell': { role: 'Tank', difficulty: 7, damage: 4, toughness: 8, control: 9, mobility: 2, utility: 8 },
    'Renekton': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 5, mobility: 5, utility: 4 },
    'Rengar': { role: 'Assassin', difficulty: 8, damage: 9, toughness: 4, control: 4, mobility: 6, utility: 4 },
    'Riven': { role: 'Fighter', difficulty: 8, damage: 8, toughness: 5, control: 6, mobility: 8, utility: 4 },
    'Rumble': { role: 'Fighter', difficulty: 10, damage: 8, toughness: 4, control: 6, mobility: 3, utility: 6 },
    'Ryze': { role: 'Mage', difficulty: 7, damage: 8, toughness: 2, control: 6, mobility: 6, utility: 6 },

    // S-Z
    'Samira': { role: 'Marksman', difficulty: 8, damage: 8, toughness: 3, control: 5, mobility: 6, utility: 5 },
    'Sejuani': { role: 'Tank', difficulty: 4, damage: 5, toughness: 7, control: 8, mobility: 6, utility: 7 },
    'Senna': { role: 'Support', difficulty: 7, damage: 8, toughness: 2, control: 7, mobility: 2, utility: 8 },
    'Seraphine': { role: 'Support', difficulty: 4, damage: 7, toughness: 2, control: 8, mobility: 2, utility: 9 },
    'Sett': { role: 'Fighter', difficulty: 2, damage: 8, toughness: 6, control: 6, mobility: 5, utility: 5 },
    'Shaco': { role: 'Assassin', difficulty: 8, damage: 8, toughness: 2, control: 6, mobility: 7, utility: 8 },
    'Shen': { role: 'Tank', difficulty: 3, damage: 3, toughness: 9, control: 6, mobility: 5, utility: 10 },
    'Shyvana': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 6, control: 5, mobility: 6, utility: 4 },
    'Singed': { role: 'Tank', difficulty: 5, damage: 6, toughness: 8, control: 6, mobility: 6, utility: 7 },
    'Sion': { role: 'Tank', difficulty: 4, damage: 6, toughness: 9, control: 6, mobility: 4, utility: 5 },
    'Sivir': { role: 'Marksman', difficulty: 4, damage: 7, toughness: 3, control: 4, mobility: 5, utility: 6 },
    'Skarner': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 6, control: 8, mobility: 5, utility: 6 },
    'Smolder': { role: 'Marksman', difficulty: 5, damage: 7, toughness: 3, control: 5, mobility: 5, utility: 5 },
    'Sona': { role: 'Support', difficulty: 4, damage: 5, toughness: 2, control: 5, mobility: 3, utility: 10 },
    'Soraka': { role: 'Support', difficulty: 3, damage: 3, toughness: 2, control: 4, mobility: 3, utility: 10 },
    'Swain': { role: 'Mage', difficulty: 8, damage: 8, toughness: 6, control: 8, mobility: 2, utility: 7 },
    'Sylas': { role: 'Mage', difficulty: 5, damage: 7, toughness: 4, control: 6, mobility: 4, utility: 7 },
    'Syndra': { role: 'Mage', difficulty: 8, damage: 9, toughness: 2, control: 8, mobility: 3, utility: 5 },
    'Tahm Kench': { role: 'Tank', difficulty: 5, damage: 5, toughness: 9, control: 6, mobility: 4, utility: 8 },
    'Taliyah': { role: 'Mage', difficulty: 5, damage: 7, toughness: 2, control: 6, mobility: 6, utility: 6 },
    'Talon': { role: 'Assassin', difficulty: 7, damage: 9, toughness: 3, control: 4, mobility: 8, utility: 5 },
    'Taric': { role: 'Support', difficulty: 5, damage: 4, toughness: 8, control: 5, mobility: 3, utility: 9 },
    'Teemo': { role: 'Marksman', difficulty: 6, damage: 5, toughness: 3, control: 6, mobility: 5, utility: 7 },
    'Thresh': { role: 'Support', difficulty: 7, damage: 6, toughness: 6, control: 9, mobility: 5, utility: 9 },
    'Tristana': { role: 'Marksman', difficulty: 4, damage: 8, toughness: 3, control: 5, mobility: 8, utility: 5 },
    'Trundle': { role: 'Fighter', difficulty: 5, damage: 7, toughness: 6, control: 4, mobility: 5, utility: 6 },
    'Tryndamere': { role: 'Fighter', difficulty: 5, damage: 9, toughness: 5, control: 2, mobility: 6, utility: 2 },
    'Twisted Fate': { role: 'Mage', difficulty: 9, damage: 6, toughness: 2, control: 8, mobility: 5, utility: 10 },
    'Twitch': { role: 'Marksman', difficulty: 6, damage: 9, toughness: 2, control: 3, mobility: 4, utility: 7 },
    'Udyr': { role: 'Fighter', difficulty: 6, damage: 6, toughness: 7, control: 4, mobility: 6, utility: 5 },
    'Urgot': { role: 'Fighter', difficulty: 7, damage: 8, toughness: 6, control: 7, mobility: 3, utility: 5 },
    'Varus': { role: 'Marksman', difficulty: 2, damage: 7, toughness: 3, control: 7, mobility: 3, utility: 6 },
    'Vayne': { role: 'Marksman', difficulty: 8, damage: 10, toughness: 1, control: 5, mobility: 6, utility: 4 },
    'Veigar': { role: 'Mage', difficulty: 7, damage: 10, toughness: 1, control: 8, mobility: 2, utility: 5 },
    'Vel\'Koz': { role: 'Mage', difficulty: 8, damage: 9, toughness: 2, control: 8, mobility: 2, utility: 6 },
    'Vex': { role: 'Mage', difficulty: 6, damage: 7, toughness: 3, control: 7, mobility: 4, utility: 5 },
    'Vi': { role: 'Fighter', difficulty: 4, damage: 8, toughness: 5, control: 7, mobility: 7, utility: 6 },
    'Viego': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 7, utility: 6 },
    'Viktor': { role: 'Mage', difficulty: 9, damage: 8, toughness: 2, control: 7, mobility: 4, utility: 6 },
    'Vladimir': { role: 'Mage', difficulty: 7, damage: 8, toughness: 7, control: 5, mobility: 4, utility: 4 },
    'Volibear': { role: 'Fighter', difficulty: 3, damage: 7, toughness: 7, control: 6, mobility: 6, utility: 5 },
    'Warwick': { role: 'Fighter', difficulty: 3, damage: 7, toughness: 7, control: 5, mobility: 6, utility: 6 },
    'Wukong': { role: 'Fighter', difficulty: 3, damage: 8, toughness: 5, control: 6, mobility: 7, utility: 5 },
    'Xayah': { role: 'Marksman', difficulty: 5, damage: 8, toughness: 3, control: 7, mobility: 6, utility: 6 },
    'Xerath': { role: 'Mage', difficulty: 8, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 6 },
    'Xin Zhao': { role: 'Fighter', difficulty: 2, damage: 8, toughness: 6, control: 5, mobility: 6, utility: 4 },
    'Yasuo': { role: 'Fighter', difficulty: 10, damage: 8, toughness: 4, control: 6, mobility: 8, utility: 4 },
    'Yone': { role: 'Assassin', difficulty: 7, damage: 8, toughness: 4, control: 6, mobility: 7, utility: 4 },
    'Yorick': { role: 'Fighter', difficulty: 6, damage: 7, toughness: 6, control: 6, mobility: 3, utility: 7 },
    'Yuumi': { role: 'Support', difficulty: 2, damage: 5, toughness: 2, control: 6, mobility: 10, utility: 8 },
    'Zac': { role: 'Tank', difficulty: 7, damage: 6, toughness: 7, control: 8, mobility: 8, utility: 7 },
    'Zed': { role: 'Assassin', difficulty: 7, damage: 9, toughness: 2, control: 5, mobility: 8, utility: 4 },
    'Zeri': { role: 'Marksman', difficulty: 8, damage: 7, toughness: 3, control: 4, mobility: 9, utility: 5 },
    'Ziggs': { role: 'Mage', difficulty: 4, damage: 9, toughness: 2, control: 6, mobility: 3, utility: 5 },
    'Zilean': { role: 'Support', difficulty: 6, damage: 6, toughness: 5, control: 7, mobility: 5, utility: 10 },
    'Zoe': { role: 'Mage', difficulty: 7, damage: 8, toughness: 1, control: 9, mobility: 5, utility: 8 },
    'Zyra': { role: 'Support', difficulty: 7, damage: 8, toughness: 2, control: 8, mobility: 3, utility: 6 }
};

// Smart recommendation function that uses detailed data when available
function getSmartRecommendations(answers, topN = 5) {
    const scores = {};

    for (const [name, champion] of Object.entries(allChampionsCompact)) {
        let score = 0;

        // Use detailed matching if available
        if (detailedChampions[name]) {
            const detailed = detailedChampions[name];
            // Complex matching logic using detailed.matchingFactors
            for (const [category, preferences] of Object.entries(detailed.matchingFactors)) {
                if (answers[category] && preferences[answers[category]]) {
                    score += preferences[answers[category]];
                }
            }
        } else {
            // Fallback to simple matching for compact data
            if (answers.rolePreference === champion.role) score += 25;

            const difficultyScore = Math.max(0, 10 - Math.abs(answers.difficulty - champion.difficulty));
            score += difficultyScore * 2;

            score += champion.damage * (answers.damagePreference / 10);
            score += champion.toughness * (answers.toughnessPreference / 10);
            score += champion.control * (answers.controlPreference / 10);
            score += champion.mobility * (answers.mobilityPreference / 10);
            score += champion.utility * (answers.utilityPreference / 10);
        }

        scores[name] = score;
    }

    return Object.entries(scores)
        .sort(([, a], [, b]) => b - a)
        .slice(0, topN)
        .map(([name, score]) => ({
            name,
            score,
            ...allChampionsCompact[name],
            detailed: !!detailedChampions[name]
        }));
}

console.log(`Hybrid system loaded: ${Object.keys(allChampionsCompact).length} champions total`);
console.log(`Detailed data available for: ${Object.keys(detailedChampions).length} champions`);

// Export for use
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        allChampionsCompact,
        detailedChampions,
        getSmartRecommendations
    };
}
// Qu
estions Database for the questionnaire
const questionsDatabase = [
        {
            id: 1,
            text: "What role do you prefer to play?",
            options: [
                { value: "Tank", text: "Tank", description: "Absorb damage and protect allies" },
                { value: "Fighter", text: "Fighter", description: "Balanced damage and survivability" },
                { value: "Assassin", text: "Assassin", description: "High damage, eliminate key targets" },
                { value: "Marksman", text: "Marksman", description: "Ranged physical damage dealer" },
                { value: "Mage", text: "Mage", description: "Magical damage and utility" },
                { value: "Support", text: "Support", description: "Help and protect teammates" }
            ]
        },
        {
            id: 2,
            text: "What difficulty level are you comfortable with?",
            options: [
                { value: 1, text: "Beginner (1-3)", description: "Simple mechanics, easy to learn" },
                { value: 5, text: "Intermediate (4-6)", description: "Moderate complexity" },
                { value: 8, text: "Advanced (7-8)", description: "Complex mechanics and combos" },
                { value: 10, text: "Expert (9-10)", description: "Extremely challenging champions" }
            ]
        },
        {
            id: 3,
            text: "How much damage do you want to deal?",
            options: [
                { value: 3, text: "Low", description: "Focus on utility and support" },
                { value: 5, text: "Medium", description: "Balanced damage output" },
                { value: 8, text: "High", description: "Strong damage dealer" },
                { value: 10, text: "Very High", description: "Maximum damage potential" }
            ]
        },
        {
            id: 4,
            text: "How tanky do you want to be?",
            options: [
                { value: 2, text: "Squishy", description: "Low health, high risk/reward" },
                { value: 4, text: "Moderate", description: "Some survivability" },
                { value: 7, text: "Durable", description: "Good survivability" },
                { value: 10, text: "Very Tanky", description: "Maximum survivability" }
            ]
        },
        {
            id: 5,
            text: "How important is crowd control to you?",
            options: [
                { value: 2, text: "Not Important", description: "Prefer direct damage" },
                { value: 5, text: "Somewhat", description: "Some CC is nice" },
                { value: 8, text: "Important", description: "Good CC abilities" },
                { value: 10, text: "Very Important", description: "Lots of CC and utility" }
            ]
        },
        {
            id: 6,
            text: "How mobile do you want to be?",
            options: [
                { value: 2, text: "Immobile", description: "Stationary, powerful abilities" },
                { value: 4, text: "Low Mobility", description: "Limited movement options" },
                { value: 7, text: "Mobile", description: "Good mobility options" },
                { value: 10, text: "Very Mobile", description: "High mobility and repositioning" }
            ]
        },
        {
            id: 7,
            text: "How much utility do you want to provide?",
            options: [
                { value: 2, text: "Low Utility", description: "Focus on personal performance" },
                { value: 5, text: "Some Utility", description: "Moderate team support" },
                { value: 8, text: "High Utility", description: "Strong team support" },
                { value: 10, text: "Maximum Utility", description: "Ultimate team player" }
            ]
        },
        {
            id: 8,
            text: "What's your preferred playstyle?",
            options: [
                { value: "aggressive", text: "Aggressive", description: "Take fights and make plays" },
                { value: "balanced", text: "Balanced", description: "Adapt to the situation" },
                { value: "defensive", text: "Defensive", description: "Play safe and scale" },
                { value: "supportive", text: "Supportive", description: "Enable teammates" }
            ]
        },
        {
            id: 9,
            text: "What motivates you most in games?",
            options: [
                { value: "winning", text: "Winning Games", description: "Focus on victory" },
                { value: "fun", text: "Having Fun", description: "Enjoy the gameplay" },
                { value: "improvement", text: "Skill Improvement", description: "Get better at the game" },
                { value: "teamwork", text: "Team Play", description: "Work well with others" }
            ]
        }
    ];

// Recommendation Engine that works with the hybrid system
const RecommendationEngine = {
    getBestRecommendation: function (answers) {
        // Convert answers to the format expected by getSmartRecommendations
        const processedAnswers = this.processAnswers(answers);

        // Get recommendations using the hybrid system
        const recommendations = getSmartRecommendations(processedAnswers, 1);

        if (recommendations.length === 0) {
            return null;
        }

        const topRecommendation = recommendations[0];

        // Get detailed champion data if available
        const championData = detailedChampions[topRecommendation.name] || {
            name: topRecommendation.name,
            title: `the ${topRecommendation.role}`,
            role: topRecommendation.role,
            difficulty: topRecommendation.difficulty,
            tags: [topRecommendation.role],
            attributes: {
                damage: topRecommendation.damage,
                toughness: topRecommendation.toughness,
                control: topRecommendation.control,
                mobility: topRecommendation.mobility,
                utility: topRecommendation.utility
            },
            description: `A ${topRecommendation.role.toLowerCase()} champion perfect for your playstyle.`,
            abilities: [
                { key: 'P', name: 'Passive Ability', description: 'Unique passive ability.' },
                { key: 'Q', name: 'First Ability', description: 'Primary ability.' },
                { key: 'W', name: 'Second Ability', description: 'Secondary ability.' },
                { key: 'E', name: 'Third Ability', description: 'Utility ability.' },
                { key: 'R', name: 'Ultimate', description: 'Powerful ultimate ability.' }
            ]
        };

        return {
            champion: championData,
            confidence: Math.min(95, Math.max(60, Math.round(topRecommendation.score * 2)))
        };
    },

    getAlternatives: function (answers, currentChampion, count = 4) {
        const processedAnswers = this.processAnswers(answers);

        // Get more recommendations and filter out the current one
        const recommendations = getSmartRecommendations(processedAnswers, count + 5)
            .filter(rec => rec.name !== currentChampion.name)
            .slice(0, count);

        return recommendations.map(rec => {
            const championData = detailedChampions[rec.name] || {
                name: rec.name,
                title: `the ${rec.role}`,
                role: rec.role,
                difficulty: rec.difficulty,
                tags: [rec.role],
                attributes: {
                    damage: rec.damage,
                    toughness: rec.toughness,
                    control: rec.control,
                    mobility: rec.mobility,
                    utility: rec.utility
                },
                description: `A ${rec.role.toLowerCase()} champion that matches your preferences.`,
                abilities: [
                    { key: 'P', name: 'Passive Ability', description: 'Unique passive ability.' },
                    { key: 'Q', name: 'First Ability', description: 'Primary ability.' },
                    { key: 'W', name: 'Second Ability', description: 'Secondary ability.' },
                    { key: 'E', name: 'Third Ability', description: 'Utility ability.' },
                    { key: 'R', name: 'Ultimate', description: 'Powerful ultimate ability.' }
                ]
            };

            return {
                champion: championData,
                confidence: Math.min(90, Math.max(55, Math.round(rec.score * 1.8)))
            };
        });
    },

    generateExplanation: function (champion, answers, confidence) {
        const explanations = [];

        // Role match explanation
        if (answers[1] === champion.role) {
            explanations.push(`Perfect role match - you wanted ${champion.role} and ${champion.name} is exactly that!`);
        }

        // Difficulty explanation
        const difficultyAnswer = answers[2];
        const champDifficulty = champion.difficulty;
        if (Math.abs(difficultyAnswer - champDifficulty) <= 2) {
            explanations.push(`Great difficulty match - ${champion.name} fits your skill level perfectly.`);
        }

        // Attribute explanations
        const damageAnswer = answers[3];
        const champDamage = champion.attributes.damage;
        if (Math.abs(damageAnswer - champDamage) <= 2) {
            explanations.push(`${champion.name} delivers the damage output you're looking for.`);
        }

        const toughnessAnswer = answers[4];
        const champToughness = champion.attributes.toughness;
        if (Math.abs(toughnessAnswer - champToughness) <= 2) {
            explanations.push(`Perfect survivability match for your playstyle.`);
        }

        // Playstyle explanation
        const playstyle = answers[8];
        if (playstyle === 'aggressive' && champion.attributes.damage >= 7) {
            explanations.push(`${champion.name} excels at aggressive plays with high damage potential.`);
        } else if (playstyle === 'defensive' && champion.attributes.toughness >= 7) {
            explanations.push(`${champion.name} offers the defensive capabilities you prefer.`);
        } else if (playstyle === 'supportive' && champion.attributes.utility >= 7) {
            explanations.push(`${champion.name} provides excellent utility for supporting your team.`);
        }

        // Confidence-based explanation
        if (confidence >= 85) {
            explanations.push(`This is an excellent match based on your preferences!`);
        } else if (confidence >= 70) {
            explanations.push(`This champion should work well for your playstyle.`);
        } else {
            explanations.push(`This champion might be worth trying based on your answers.`);
        }

        return explanations.length > 0 ? explanations.join(' ') :
            `${champion.name} is recommended based on your questionnaire responses.`;
    },

    processAnswers: function (answers) {
        return {
            rolePreference: answers[1],
            difficulty: answers[2] || 5,
            damagePreference: answers[3] || 5,
            toughnessPreference: answers[4] || 5,
            controlPreference: answers[5] || 5,
            mobilityPreference: answers[6] || 5,
            utilityPreference: answers[7] || 5,
            playstyle: answers[8],
            motivation: answers[9]
        };
    }
};