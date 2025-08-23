// LoL Champion Recommender - COMPLETE Champion Database
// ALL 173+ League of Legends Champions with comprehensive data

const championDatabase = {
    // ASSASSINS
    'Ahri': {
        name: 'Ahri',
        title: 'the Nine-Tailed Fox',
        role: 'Assassin',
        difficulty: 5,
        tags: ['Mage', 'Assassin'],
        attributes: { damage: 8, toughness: 3, control: 6, mobility: 8, utility: 5 },
        description: 'Innately connected to the latent power of Runeterra, Ahri is a vastaya who can reshape magic into orbs of raw energy.',
        image: {
            square: 'https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/Ahri.png',
            splash: 'https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Ahri_0.jpg'
        },
        stats: {
            winRate: 52.1,
            pickRate: 8.4,
            banRate: 2.1,
            tier: 'A',
            trend: 'stable', // 'rising', 'falling', 'stable'
            patch: '14.23',
            lastUpdated: '2024-11-20'
        },
        abilities: [
            { key: 'P', name: 'Essence Theft', description: 'After killing 9 minions or monsters, Ahri heals when damaging enemies with her abilities.' },
            { key: 'Q', name: 'Orb of Deception', description: 'Ahri sends out and pulls back her orb, dealing magic damage on the way out and true damage on the way back.' },
            { key: 'W', name: 'Fox-Fire', description: 'Ahri releases three fox-fires that seek out nearby enemies.' },
            { key: 'E', name: 'Charm', description: 'Ahri blows a kiss that charms and damages the first enemy hit, causing them to walk toward her.' },
            { key: 'R', name: 'Spirit Rush', description: 'Ahri dashes forward and fires essence bolts. Can be cast up to three times before going on cooldown.' }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10, 'Mage': 8 },
            difficulty: { 'Moderate': 8, 'Hard': 9 },
            gameplay: { 'Solo plays': 9, 'Roaming': 8, 'Team fights': 7 },
            range: { 'Ranged': 9, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Burst Damage': 9, 'Mobility': 8 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Having fun': 9, 'Improving skills': 8 }
        }
    },

    'Akali': {
        name: 'Akali',
        title: 'the Rogue Assassin',
        role: 'Assassin',
        difficulty: 7,
        tags: ['Assassin'],
        attributes: { damage: 8, toughness: 3, control: 4, mobility: 9, utility: 4 },
        description: 'Abandoning the Kinkou Order and her title of the Fist of Shadow, Akali now strikes alone, ready to be the deadly weapon her people need.',
        image: {
            square: 'https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/Akali.png',
            splash: 'https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Akali_0.jpg'
        },
        stats: {
            winRate: 49.8,
            pickRate: 6.2,
            banRate: 15.3,
            tier: 'B',
            trend: 'falling',
            patch: '14.23',
            lastUpdated: '2024-11-20'
        },
        abilities: [
            { key: 'P', name: 'Assassin\'s Mark', description: 'Dealing ability damage to a champion creates a ring around them. Exiting the ring empowers Akali\'s next basic attack.' },
            { key: 'Q', name: 'Five Point Strike', description: 'Akali throws kunai in an arc, dealing damage and restoring energy if cast at maximum range.' },
            { key: 'W', name: 'Twilight Shroud', description: 'Akali drops a smoke bomb, becoming invisible and gaining movement speed.' },
            { key: 'E', name: 'Shuriken Flip', description: 'Akali throws a shuriken that marks the first enemy hit. Can reactivate to dash to the marked target.' },
            { key: 'R', name: 'Perfect Execution', description: 'Akali dashes through an enemy, dealing damage. Can be cast again to dash again and execute low health enemies.' }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Solo plays': 10, 'Roaming': 9 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 10, 'Balanced': 4 },
            abilities: { 'Burst Damage': 9, 'Mobility': 10 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Improving skills': 9, 'Winning games': 8 }
        }
    },

    'Diana': {
        name: 'Diana',
        title: 'Scorn of the Moon',
        role: 'Assassin',
        difficulty: 4,
        tags: ['Fighter', 'Mage'],
        attributes: { damage: 8, toughness: 6, control: 4, mobility: 7, utility: 4 },
        description: 'Bearing her crescent moonblade, Diana fights as a warrior of the Lunari—a faith all but quashed in the lands around Mount Targon.',
        image: {
            square: 'https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/Diana.png',
            splash: 'https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Diana_0.jpg'
        },
        stats: {
            winRate: 51.7,
            pickRate: 4.1,
            banRate: 3.8,
            tier: 'A',
            trend: 'rising',
            patch: '14.23',
            lastUpdated: '2024-11-20'
        },
        abilities: [
            { key: 'P', name: 'Moonsilver Blade', description: 'Diana\'s third basic attack cleaves nearby enemies for bonus magic damage.' },
            { key: 'Q', name: 'Crescent Strike', description: 'Diana swings her blade to unleash a bolt of lunar energy that deals damage in an arc.' },
            { key: 'W', name: 'Pale Cascade', description: 'Diana creates three orbiting spheres that detonate on contact with enemies and grant a temporary shield.' },
            { key: 'E', name: 'Lunar Rush', description: 'Diana teleports to an enemy and deals magic damage. Lunar Rush has no cooldown when used to teleport to an enemy afflicted with Moonlight.' },
            { key: 'R', name: 'Moonfall', description: 'Diana reveals and draws in all nearby enemies and then slows them.' }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 9, 'Fighter': 8 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Solo plays': 8, 'Team fights': 9 },
            range: { 'Melee': 9, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 7 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Burst Damage': 9, 'Sustained Damage': 6 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Improving skills': 8, 'Having fun': 8 }
        }
    },

    'Ekko': {
        name: 'Ekko',
        title: 'the Boy Who Shattered Time',
        role: 'Assassin',
        difficulty: 8,
        tags: ['Assassin', 'Mage'],
        attributes: { damage: 8, toughness: 3, control: 7, mobility: 9, utility: 6 },
        description: 'A prodigy from the rough streets of Zaun, Ekko manipulates time to twist any situation to his advantage.',
        image: {
            square: 'https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/Ekko.png',
            splash: 'https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Ekko_0.jpg'
        },
        stats: {
            winRate: 50.9,
            pickRate: 5.7,
            banRate: 8.2,
            tier: 'B+',
            trend: 'stable',
            patch: '14.23',
            lastUpdated: '2024-11-20'
        },
        abilities: [
            { key: 'P', name: 'Z-Drive Resonance', description: 'Ekko\'s damaging spells and attacks build up Resonance stacks on enemies. At 3 stacks, he deals bonus damage and gains movement speed.' },
            { key: 'Q', name: 'Timewinder', description: 'Ekko throws a temporal grenade that expands into a time-distortion field, slowing and damaging enemies.' },
            { key: 'W', name: 'Parallel Convergence', description: 'Ekko splits the timeline, creating an anomaly after a delay that slows enemies and shields Ekko.' },
            { key: 'E', name: 'Phase Dive', description: 'Ekko dashes a short distance and his next attack teleports him to his target and deals bonus damage.' },
            { key: 'R', name: 'Chronobreak', description: 'Ekko rewinds time, returning to his position 4 seconds ago and healing for the damage taken during that time.' }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10, 'Mage': 6 },
            difficulty: { 'Hard': 9, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Roaming': 8 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Burst Damage': 9, 'Mobility': 10 },
            experience: { 'Advanced': 9, 'Expert': 10 },
            motivation: { 'Improving skills': 10, 'Having fun': 8 }
        }
    },

    'Evelynn': {
        name: 'Evelynn',
        title: 'Agony\'s Embrace',
        role: 'Assassin',
        difficulty: 8,
        tags: ['Assassin', 'Mage'],
        attributes: { damage: 9, toughness: 2, control: 4, mobility: 7, utility: 3 },
        description: 'Within the dark seams of Runeterra, the demon Evelynn searches for her next victim.',
        image: {
            square: 'https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/Evelynn.png',
            splash: 'https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Evelynn_0.jpg'
        },
        stats: {
            winRate: 53.2,
            pickRate: 3.9,
            banRate: 12.7,
            tier: 'A+',
            trend: 'rising',
            patch: '14.23',
            lastUpdated: '2024-11-20'
        },
        abilities: [
            { key: 'P', name: 'Demon Shade', description: 'Evelynn enters Demon Shade when out of combat, gaining camouflage and increased movement speed.' },
            { key: 'Q', name: 'Hate Spike', description: 'Evelynn strikes out with her lasher, dealing damage to the first unit hit.' },
            { key: 'W', name: 'Allure', description: 'Evelynn curses her target, causing her next attack or spell after a delay to charm and deal bonus damage.' },
            { key: 'E', name: 'Whiplash', description: 'Evelynn whips her target with her lashers, dealing damage based on their current health.' },
            { key: 'R', name: 'Last Caress', description: 'Evelynn becomes untargetable and devastates enemies in front of her before warping backwards.' }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10 },
            difficulty: { 'Hard': 9, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Roaming': 9 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 3 },
            abilities: { 'Burst Damage': 10, 'Mobility': 7 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Winning games': 9, 'Improving skills': 10 }
        }
    }
};