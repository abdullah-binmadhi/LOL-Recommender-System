// LoL Champion Recommender - Champion Database
// Comprehensive champion data with abilities, attributes, and matching logic

const championDatabase = {
    // Tank Champions
    'Alistar': {
        name: 'Alistar',
        title: 'the Minotaur',
        role: 'Tank',
        difficulty: 3,
        tags: ['Tank', 'Support'],
        attributes: {
            damage: 6,
            toughness: 9,
            control: 8,
            mobility: 6,
            utility: 8
        },
        description: 'Always a mighty warrior with a fearsome reputation, Alistar seeks revenge for the death of his clan at the hands of the Noxian empire.',
        abilities: [
            {
                key: 'P',
                name: 'Triumphant Roar',
                description: 'Alistar heals himself and nearby allied champions when enemy minions or monsters die nearby.'
            },
            {
                key: 'Q',
                name: 'Pulverize',
                description: 'Alistar smashes the ground, dealing damage and knocking up nearby enemies.'
            },
            {
                key: 'W',
                name: 'Headbutt',
                description: 'Alistar rams a target with his head, dealing damage and knocking them back.'
            },
            {
                key: 'E',
                name: 'Trample',
                description: 'Alistar tramples nearby enemies, dealing damage and stunning them if they remain nearby.'
            },
            {
                key: 'R',
                name: 'Unbreakable Will',
                description: 'Alistar removes all disables and reduces incoming damage for a duration.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Tank': 10, 'Support': 8 },
            difficulty: { 'Easy': 7, 'Moderate': 9, 'Hard': 6 },
            gameplay: { 'Team fights': 10, 'Supporting allies': 9 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'Low': 7, 'Medium': 8, 'High': 5 },
            style: { 'Defensive': 8, 'Balanced': 9, 'Offensive': 7 },
            abilities: { 'Crowd Control': 10, 'Healing/Shielding': 6 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Support team': 10, 'Improving skills': 8 }
        }
    },

    'Braum': {
        name: 'Braum',
        title: 'the Heart of the Freljord',
        role: 'Tank',
        difficulty: 4,
        tags: ['Tank', 'Support'],
        attributes: {
            damage: 4,
            toughness: 9,
            control: 8,
            mobility: 4,
            utility: 9
        },
        description: 'Blessed with massive biceps and an even bigger heart, Braum is a beloved hero of the Freljord.',
        abilities: [
            {
                key: 'P',
                name: 'Concussive Blows',
                description: 'Braum\'s basic attacks apply Concussive Blows. Once the first stack is applied, ally basic attacks also stack Concussive Blows.'
            },
            {
                key: 'Q',
                name: 'Winter\'s Bite',
                description: 'Braum propels freezing ice from his shield, slowing and dealing magic damage.'
            },
            {
                key: 'W',
                name: 'Stand Behind Me',
                description: 'Braum leaps to a target ally, granting both Braum and his ally bonus armor and magic resistance.'
            },
            {
                key: 'E',
                name: 'Unbreakable',
                description: 'Braum raises his shield in a direction, intercepting projectiles and reducing damage from that direction.'
            },
            {
                key: 'R',
                name: 'Glacial Fissure',
                description: 'Braum slams the ground, knocking up enemies in a line and leaving behind a field that slows enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Tank': 9, 'Support': 10 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Supporting allies': 10, 'Team fights': 9 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Very Low': 8, 'Low': 9, 'Medium': 6 },
            style: { 'Defensive': 10, 'Balanced': 8 },
            abilities: { 'Crowd Control': 8, 'Healing/Shielding': 7 },
            experience: { 'Intermediate': 9, 'Advanced': 8 },
            motivation: { 'Support team': 10, 'Having fun': 8 }
        }
    },

    'Cho\'Gath': {
        name: 'Cho\'Gath',
        title: 'the Terror of the Void',
        role: 'Tank',
        difficulty: 5,
        tags: ['Tank', 'Mage'],
        attributes: {
            damage: 7,
            toughness: 7,
            control: 8,
            mobility: 3,
            utility: 6
        },
        description: 'From the moment Cho\'Gath first emerged into the harsh light of Runeterra\'s sun, the beast was driven by the most pure and insatiable hunger.',
        abilities: [
            {
                key: 'P',
                name: 'Carnivore',
                description: 'Whenever Cho\'Gath kills a unit, he recovers health and mana.'
            },
            {
                key: 'Q',
                name: 'Rupture',
                description: 'Cho\'Gath ruptures the ground at a target location, dealing damage and knocking up enemies.'
            },
            {
                key: 'W',
                name: 'Feral Scream',
                description: 'Cho\'Gath unleashes a terrible scream, dealing damage and silencing enemies in a cone.'
            },
            {
                key: 'E',
                name: 'Vorpal Spikes',
                description: 'Cho\'Gath\'s next 3 basic attacks launch spikes that deal damage to all enemies in front of him.'
            },
            {
                key: 'R',
                name: 'Feast',
                description: 'Cho\'Gath devours an enemy unit, dealing true damage and permanently increasing his health.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Tank': 9, 'Mage': 6 },
            difficulty: { 'Moderate': 8, 'Hard': 9 },
            gameplay: { 'Team fights': 8, 'Solo plays': 7, 'Farming': 8 },
            range: { 'Melee': 7, 'No preference': 8 },
            damage: { 'Medium': 8, 'High': 7 },
            style: { 'Balanced': 9, 'Offensive': 7 },
            abilities: { 'Crowd Control': 9, 'Sustained Damage': 7 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Improving skills': 8, 'Having fun': 7 }
        }
    },

    'Malphite': {
        name: 'Malphite',
        title: 'Shard of the Monolith',
        role: 'Tank',
        difficulty: 2,
        tags: ['Tank', 'Fighter'],
        attributes: {
            damage: 4,
            toughness: 9,
            control: 7,
            mobility: 5,
            utility: 7
        },
        description: 'A massive creature of living stone, Malphite struggles to impose blessed order on a chaotic world.',
        abilities: [
            {
                key: 'P',
                name: 'Granite Shield',
                description: 'Malphite is shielded by a layer of rock which absorbs damage. The shield regenerates after not taking damage.'
            },
            {
                key: 'Q',
                name: 'Seismic Shard',
                description: 'Malphite sends a shard of the earth through the ground, dealing damage and stealing movement speed.'
            },
            {
                key: 'W',
                name: 'Thunderclap',
                description: 'Malphite attacks with such force that it creates a thunderclap, dealing damage to nearby enemies.'
            },
            {
                key: 'E',
                name: 'Ground Slam',
                description: 'Malphite slams the ground, sending out a shockwave that reduces enemy attack speed.'
            },
            {
                key: 'R',
                name: 'Unstoppable Force',
                description: 'Malphite launches himself to a location at high speed, dealing damage and knocking up enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Tank': 10 },
            difficulty: { 'Easy': 9, 'Moderate': 6 },
            gameplay: { 'Team fights': 9, 'Supporting allies': 7 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Low': 8, 'Medium': 9, 'High': 4 },
            style: { 'Defensive': 10, 'Balanced': 7 },
            abilities: { 'Crowd Control': 9, 'Sustained Damage': 6 },
            experience: { 'Beginner': 10, 'Intermediate': 8 },
            motivation: { 'Support team': 9, 'Having fun': 7 }
        }
    },

    'Amumu': {
        name: 'Amumu',
        title: 'the Sad Mummy',
        role: 'Tank',
        difficulty: 3,
        tags: ['Tank', 'Mage'],
        attributes: {
            damage: 6,
            toughness: 6,
            control: 8,
            mobility: 5,
            utility: 6
        },
        description: 'A yordle wrapped in bandages, Amumu is the loneliest champion in the League, seeking friends through his powerful crowd control.',
        abilities: [
            {
                key: 'P',
                name: 'Cursed Touch',
                description: 'Amumu\'s attacks reduce magic resistance and deal bonus magic damage.'
            },
            {
                key: 'Q',
                name: 'Bandage Toss',
                description: 'Amumu tosses a bandage at a target, stunning and pulling himself to them.'
            },
            {
                key: 'W',
                name: 'Despair',
                description: 'Overcome by anguish, nearby enemies lose health each second.'
            },
            {
                key: 'E',
                name: 'Tantrum',
                description: 'Amumu throws a tantrum, dealing damage to surrounding units and reducing physical damage taken.'
            },
            {
                key: 'R',
                name: 'Curse of the Sad Mummy',
                description: 'Ammu entangles surrounding enemy units in bandages, stunning them and dealing damage.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Tank': 10 },
            difficulty: { 'Easy': 7, 'Moderate': 9, 'Hard': 6 },
            gameplay: { 'Team fights': 10, 'Supporting allies': 8 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Medium': 8, 'High': 6 },
            style: { 'Defensive': 8, 'Balanced': 9 },
            abilities: { 'Crowd Control': 10, 'Sustained Damage': 7 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Support team': 8, 'Improving skills': 7 }
        }
    },

    // Marksman Champions
    'Jinx': {
        name: 'Jinx',
        title: 'the Loose Cannon',
        role: 'Marksman',
        difficulty: 6,
        tags: ['Marksman'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 7,
            mobility: 6,
            utility: 6
        },
        description: 'A manic and impulsive criminal from Zaun, Jinx lives to wreak havoc without care for the consequences.',
        abilities: [
            {
                key: 'P',
                name: 'Get Excited!',
                description: 'Jinx gains movement speed and attack speed when damaging enemy champions, structures, or killing enemies.'
            },
            {
                key: 'Q',
                name: 'Switcheroo!',
                description: 'Jinx swaps weapons between Pow-Pow (machine gun) and Fishbones (rocket launcher).'
            },
            {
                key: 'W',
                name: 'Zap!',
                description: 'Jinx uses Zapper to fire a shock blast that deals damage and slows the first enemy hit.'
            },
            {
                key: 'E',
                name: 'Flame Chompers!',
                description: 'Jinx tosses out a line of snare grenades that explode after a delay, rooting enemies.'
            },
            {
                key: 'R',
                name: 'Super Mega Death Rocket!',
                description: 'Jinx fires a rocket across the map that gains damage as it travels and explodes on impact.'
            }
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

    'Ashe': {
        name: 'Ashe',
        title: 'the Frost Archer',
        role: 'Marksman',
        difficulty: 4,
        tags: ['Marksman', 'Support'],
        attributes: {
            damage: 7,
            toughness: 3,
            control: 8,
            mobility: 4,
            utility: 8
        },
        description: 'Ashe is a champion of the Freljord, using her ice magic and bow to lead her people and control the battlefield.',
        abilities: [
            {
                key: 'P',
                name: 'Frost Shot',
                description: 'Ashe\'s attacks slow enemies and deal increased damage to slowed targets.'
            },
            {
                key: 'Q',
                name: 'Ranger\'s Focus',
                description: 'Ashe builds Focus by attacking. At maximum Focus, she can cast to gain attack speed and fire multiple arrows.'
            },
            {
                key: 'W',
                name: 'Volley',
                description: 'Ashe fires arrows in a cone, dealing damage and slowing enemies.'
            },
            {
                key: 'E',
                name: 'Hawkshot',
                description: 'Ashe sends a hawk spirit across the map, granting vision of enemies it passes through.'
            },
            {
                key: 'R',
                name: 'Enchanted Crystal Arrow',
                description: 'Ashe fires a crystal arrow that stuns the first enemy champion hit and deals damage.'
            }
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

    // Mage Champions
    'Lux': {
        name: 'Lux',
        title: 'the Lady of Luminosity',
        role: 'Mage',
        difficulty: 5,
        tags: ['Mage', 'Support'],
        attributes: {
            damage: 7,
            toughness: 2,
            control: 8,
            mobility: 4,
            utility: 7
        },
        description: 'Luxanna Crownguard hails from Demacia, wielding light magic to protect her allies and devastate her enemies.',
        abilities: [
            {
                key: 'P',
                name: 'Illumination',
                description: 'Lux\'s spells charge enemies with energy for a few seconds. Her next attack ignites the energy for bonus damage.'
            },
            {
                key: 'Q',
                name: 'Light Binding',
                description: 'Lux releases a sphere of light that binds and deals damage to up to two enemies.'
            },
            {
                key: 'W',
                name: 'Prismatic Barrier',
                description: 'Lux throws her wand and bends the light around any friendly target it touches, protecting them from enemy damage.'
            },
            {
                key: 'E',
                name: 'Lucent Singularity',
                description: 'Creates an area of twisted light that slows enemies and can be detonated for damage.'
            },
            {
                key: 'R',
                name: 'Final Spark',
                description: 'Lux fires a laser that deals damage to all enemies in a line and refreshes Illumination.'
            }
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

    'Annie': {
        name: 'Annie',
        title: 'the Dark Child',
        role: 'Mage',
        difficulty: 3,
        tags: ['Mage'],
        attributes: {
            damage: 8,
            toughness: 2,
            control: 6,
            mobility: 3,
            utility: 5
        },
        description: 'Dangerous, yet disarmingly precocious, Annie is a child mage with immense pyromantic power.',
        abilities: [
            {
                key: 'P',
                name: 'Pyromania',
                description: 'After casting 4 spells, Annie\'s next offensive spell will stun the target.'
            },
            {
                key: 'Q',
                name: 'Disintegrate',
                description: 'Annie hurls a fireball, dealing damage. Mana is refunded if it kills the target.'
            },
            {
                key: 'W',
                name: 'Incinerate',
                description: 'Annie casts a cone of fire, dealing damage to all enemies in the area.'
            },
            {
                key: 'E',
                name: 'Molten Shield',
                description: 'Annie grants herself or an ally a shield and damages enemies who attack the shielded target.'
            },
            {
                key: 'R',
                name: 'Summon: Tibbers',
                description: 'Annie summons her bear Tibbers, dealing damage and stunning enemies in the area.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10 },
            difficulty: { 'Easy': 9, 'Moderate': 8 },
            gameplay: { 'Team fights': 8, 'Solo plays': 7 },
            range: { 'Ranged': 8, 'No preference': 6 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 9, 'Balanced': 6 },
            abilities: { 'Burst Damage': 10, 'Crowd Control': 7 },
            experience: { 'Beginner': 9, 'Intermediate': 8 },
            motivation: { 'Having fun': 8, 'Learning new champions': 9 }
        }
    },

    // Support Champions
    'Thresh': {
        name: 'Thresh',
        title: 'the Chain Warden',
        role: 'Support',
        difficulty: 7,
        tags: ['Support', 'Fighter'],
        attributes: {
            damage: 6,
            toughness: 6,
            control: 9,
            mobility: 5,
            utility: 9
        },
        description: 'Sadistic and cunning, Thresh is an ambitious and restless spirit of the Shadow Isles.',
        abilities: [
            {
                key: 'P',
                name: 'Damnation',
                description: 'Thresh can harvest souls to permanently increase his armor and ability power.'
            },
            {
                key: 'Q',
                name: 'Death Sentence',
                description: 'Thresh binds an enemy in chains and pulls them toward him. Can reactivate to pull himself to the enemy.'
            },
            {
                key: 'W',
                name: 'Dark Passage',
                description: 'Thresh throws his lantern to a target location. Allies can click it to dash to Thresh.'
            },
            {
                key: 'E',
                name: 'Flay',
                description: 'Thresh sweeps his chain, knocking back enemies and dealing damage.'
            },
            {
                key: 'R',
                name: 'The Box',
                description: 'Thresh creates a prison of walls around himself. Enemies who walk through walls are slowed and take damage.'
            }
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
    },

    'Soraka': {
        name: 'Soraka',
        title: 'the Starchild',
        role: 'Support',
        difficulty: 3,
        tags: ['Support', 'Mage'],
        attributes: {
            damage: 3,
            toughness: 2,
            control: 4,
            mobility: 3,
            utility: 10
        },
        description: 'A celestial being, Soraka gave up her immortality to protect the mortal races from their own ignorance.',
        abilities: [
            {
                key: 'P',
                name: 'Salvation',
                description: 'Soraka gains movement speed when moving towards low health allies.'
            },
            {
                key: 'Q',
                name: 'Starcall',
                description: 'Soraka calls down a star, dealing damage and healing herself if she hits an enemy champion.'
            },
            {
                key: 'W',
                name: 'Astral Infusion',
                description: 'Soraka heals another ally at the cost of her own health.'
            },
            {
                key: 'E',
                name: 'Equinox',
                description: 'Soraka creates a zone that silences enemies and can explode to root them.'
            },
            {
                key: 'R',
                name: 'Wish',
                description: 'Soraka instantly heals all allied champions, regardless of their location.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10 },
            difficulty: { 'Easy': 9, 'Moderate': 7 },
            gameplay: { 'Supporting allies': 10, 'Team fights': 7 },
            range: { 'Ranged': 8, 'No preference': 6 },
            damage: { 'Very Low': 8, 'Low': 9 },
            style: { 'Defensive': 10, 'Balanced': 6 },
            abilities: { 'Healing/Shielding': 10, 'Crowd Control': 6 },
            experience: { 'Beginner': 9, 'Intermediate': 8 },
            motivation: { 'Support team': 10, 'Having fun': 7 }
        }
    },

    // Fighter Champions
    'Garen': {
        name: 'Garen',
        title: 'The Might of Demacia',
        role: 'Fighter',
        difficulty: 2,
        tags: ['Fighter', 'Tank'],
        attributes: {
            damage: 6,
            toughness: 7,
            control: 5,
            mobility: 5,
            utility: 5
        },
        description: 'A proud and noble warrior, Garen fights as one of the Dauntless Vanguard.',
        abilities: [
            {
                key: 'P',
                name: 'Perseverance',
                description: 'Garen regenerates health over time when he hasn\'t taken damage recently.'
            },
            {
                key: 'Q',
                name: 'Decisive Strike',
                description: 'Garen gains movement speed and his next attack deals bonus damage and silences the target.'
            },
            {
                key: 'W',
                name: 'Courage',
                description: 'Garen reduces incoming damage and gains tenacity for a short duration.'
            },
            {
                key: 'E',
                name: 'Judgment',
                description: 'Garen spins with his sword, dealing damage to nearby enemies.'
            },
            {
                key: 'R',
                name: 'Demacian Justice',
                description: 'Garen calls upon Demacian justice to deal true damage based on the enemy\'s missing health.'
            }
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

    // More Fighter Champions
    'Darius': {
        name: 'Darius',
        title: 'the Hand of Noxus',
        role: 'Fighter',
        difficulty: 4,
        tags: ['Fighter', 'Tank'],
        attributes: {
            damage: 9,
            toughness: 6,
            control: 5,
            mobility: 4,
            utility: 4
        },
        description: 'There is no greater symbol of Noxian might than Darius, the nation\'s most feared and battle-hardened commander.',
        abilities: [
            {
                key: 'P',
                name: 'Hemorrhage',
                description: 'Darius\'s attacks and damaging abilities cause enemies to bleed, dealing magic damage over time.'
            },
            {
                key: 'Q',
                name: 'Decimate',
                description: 'Darius winds up and swings his axe in a wide circle, dealing damage and healing himself.'
            },
            {
                key: 'W',
                name: 'Crippling Strike',
                description: 'Darius\'s next attack deals bonus damage and slows the target.'
            },
            {
                key: 'E',
                name: 'Apprehend',
                description: 'Darius pulls in all enemies in front of him and gains armor penetration.'
            },
            {
                key: 'R',
                name: 'Noxian Guillotine',
                description: 'Darius leaps to an enemy champion and strikes a lethal blow, dealing true damage.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 10, 'Tank': 6 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Solo plays': 9, 'Team fights': 8 },
            range: { 'Melee': 10, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Sustained Damage': 9, 'Burst Damage': 7 },
            experience: { 'Intermediate': 9, 'Advanced': 8 },
            motivation: { 'Winning games': 9, 'Solo plays': 8 }
        }
    },

    'Fiora': {
        name: 'Fiora',
        title: 'the Grand Duelist',
        role: 'Fighter',
        difficulty: 7,
        tags: ['Fighter', 'Assassin'],
        attributes: {
            damage: 8,
            toughness: 4,
            control: 4,
            mobility: 7,
            utility: 3
        },
        description: 'The most feared duelist in all Valoran, Fiora is as renowned for her brusque manner and political cunning as she is for her skill with a blade.',
        abilities: [
            {
                key: 'P',
                name: 'Duelist\'s Dance',
                description: 'Fiora identifies weak spots on enemy champions. When she attacks a weak spot, she deals true damage and gains movement speed.'
            },
            {
                key: 'Q',
                name: 'Lunge',
                description: 'Fiora lunges forward and stabs a nearby enemy, dealing damage and applying on-hit effects.'
            },
            {
                key: 'W',
                name: 'Riposte',
                description: 'Fiora parries the next attack and then stabs forward, dealing damage and slowing the enemy.'
            },
            {
                key: 'E',
                name: 'Bladework',
                description: 'Fiora gains attack speed and her next two attacks critically strike, with the second attack slowing the target.'
            },
            {
                key: 'R',
                name: 'Grand Challenge',
                description: 'Fiora reveals all weak spots on a target and gains movement speed while near them.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 10, 'Assassin': 7 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Solo plays': 10, 'Roaming': 7 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Sustained Damage': 8, 'Mobility': 9 },
            experience: { 'Advanced': 9, 'Expert': 10 },
            motivation: { 'Improving skills': 10, 'Winning games': 8 }
        }
    },

    'Jax': {
        name: 'Jax',
        title: 'Grandmaster at Arms',
        role: 'Fighter',
        difficulty: 5,
        tags: ['Fighter', 'Assassin'],
        attributes: {
            damage: 7,
            toughness: 5,
            control: 4,
            mobility: 6,
            utility: 4
        },
        description: 'Unmatched in both his skill with unique armaments and his biting sarcasm, Jax is the last known weapons master of Icathia.',
        abilities: [
            {
                key: 'P',
                name: 'Relentless Assault',
                description: 'Jax\'s consecutive basic attacks grant him bonus attack speed.'
            },
            {
                key: 'Q',
                name: 'Leap Strike',
                description: 'Jax leaps to a target unit, dealing damage if it\'s an enemy.'
            },
            {
                key: 'W',
                name: 'Empower',
                description: 'Jax charges his weapon with energy, causing his next basic attack to deal bonus damage.'
            },
            {
                key: 'E',
                name: 'Counter Strike',
                description: 'Jax dodges incoming attacks and then stuns nearby enemies.'
            },
            {
                key: 'R',
                name: 'Grandmaster\'s Might',
                description: 'Jax\'s combat prowess allows him to dodge attacks and deal magic damage when attacking.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 10, 'Assassin': 6 },
            difficulty: { 'Moderate': 8, 'Hard': 9 },
            gameplay: { 'Solo plays': 9, 'Team fights': 7 },
            range: { 'Melee': 9, 'No preference': 7 },
            damage: { 'High': 8, 'Very High': 7 },
            style: { 'Balanced': 8, 'Offensive': 9 },
            abilities: { 'Sustained Damage': 9, 'Mobility': 7 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Having fun': 8, 'Improving skills': 9 }
        }
    },

    // More Marksman Champions
    'Caitlyn': {
        name: 'Caitlyn',
        title: 'the Sheriff of Piltover',
        role: 'Marksman',
        difficulty: 6,
        tags: ['Marksman'],
        attributes: {
            damage: 8,
            toughness: 2,
            control: 6,
            mobility: 4,
            utility: 6
        },
        description: 'Renowned as its finest peacekeeper, Caitlyn is also Piltover\'s best shot at ridding the city of its elusive criminal elements.',
        abilities: [
            {
                key: 'P',
                name: 'Headshot',
                description: 'Every few basic attacks, or against a target she has trapped or netted, Caitlyn will fire a headshot dealing bonus damage.'
            },
            {
                key: 'Q',
                name: 'Piltover Peacemaker',
                description: 'Caitlyn fires a projectile through all enemies in a line, dealing damage.'
            },
            {
                key: 'W',
                name: 'Yordle Snap Trap',
                description: 'Caitlyn sets traps that root and reveal enemies who step on them.'
            },
            {
                key: 'E',
                name: '90 Caliber Net',
                description: 'Caitlyn fires a net, slowing the first enemy hit and knocking herself backwards.'
            },
            {
                key: 'R',
                name: 'Ace in the Hole',
                description: 'Caitlyn takes aim at an enemy champion, dealing massive damage after a delay.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10 },
            difficulty: { 'Moderate': 7, 'Hard': 9 },
            gameplay: { 'Solo plays': 8, 'Team fights': 8, 'Farming': 9 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 8, 'Balanced': 9 },
            abilities: { 'Sustained Damage': 9, 'Burst Damage': 7 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Winning games': 8, 'Improving skills': 7 }
        }
    },

    'Ezreal': {
        name: 'Ezreal',
        title: 'the Prodigal Explorer',
        role: 'Marksman',
        difficulty: 7,
        tags: ['Marksman', 'Mage'],
        attributes: {
            damage: 7,
            toughness: 2,
            control: 4,
            mobility: 7,
            utility: 5
        },
        description: 'A dashing adventurer, unknowingly gifted in the magical arts, Ezreal raids long-lost catacombs, tangles with ancient curses, and overcomes seemingly impossible odds with ease.',
        abilities: [
            {
                key: 'P',
                name: 'Rising Spell Force',
                description: 'Hitting enemies with abilities grants Ezreal bonus attack speed.'
            },
            {
                key: 'Q',
                name: 'Mystic Shot',
                description: 'Ezreal fires a bolt of energy that deals damage and reduces his cooldowns if it hits an enemy.'
            },
            {
                key: 'W',
                name: 'Essence Flux',
                description: 'Ezreal fires an orb that sticks to the first champion or objective hit, detonating after a delay.'
            },
            {
                key: 'E',
                name: 'Arcane Shift',
                description: 'Ezreal teleports to a nearby location and fires a homing bolt at the nearest enemy.'
            },
            {
                key: 'R',
                name: 'Trueshot Barrage',
                description: 'Ezreal channels for a moment before firing a powerful barrage across the map.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10, 'Mage': 6 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Solo plays': 9, 'Roaming': 8, 'Team fights': 7 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 8, 'Very High': 7 },
            style: { 'Offensive': 8, 'Balanced': 9 },
            abilities: { 'Burst Damage': 8, 'Mobility': 9 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Having fun': 9, 'Improving skills': 8 }
        }
    },

    'Vayne': {
        name: 'Vayne',
        title: 'the Night Hunter',
        role: 'Marksman',
        difficulty: 8,
        tags: ['Marksman', 'Assassin'],
        attributes: {
            damage: 10,
            toughness: 1,
            control: 5,
            mobility: 6,
            utility: 4
        },
        description: 'Shauna Vayne is a deadly, remorseless Demacian who has dedicated her life to hunting down those who wield dark magic.',
        abilities: [
            {
                key: 'P',
                name: 'Night Hunter',
                description: 'Vayne gains bonus movement speed when moving toward enemy champions.'
            },
            {
                key: 'Q',
                name: 'Tumble',
                description: 'Vayne tumbles and her next basic attack deals bonus damage.'
            },
            {
                key: 'W',
                name: 'Silver Bolts',
                description: 'Vayne\'s basic attacks and Tumble apply silver bolts. The third consecutive bolt against the same target deals true damage.'
            },
            {
                key: 'E',
                name: 'Condemn',
                description: 'Vayne fires a bolt that knocks back and stuns enemies if they hit a wall.'
            },
            {
                key: 'R',
                name: 'Final Hour',
                description: 'Vayne gains bonus attack damage, invisibility on Tumble, and bonus movement speed when chasing enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10, 'Assassin': 7 },
            difficulty: { 'Hard': 8, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Team fights': 7 },
            range: { 'Ranged': 9, 'No preference': 6 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 5 },
            abilities: { 'Sustained Damage': 10, 'Mobility': 8 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Winning games': 10, 'Improving skills': 9 }
        }
    },

    // More Mage Champions
    'Ahri': {
        name: 'Ahri',
        title: 'the Nine-Tailed Fox',
        role: 'Mage',
        difficulty: 5,
        tags: ['Mage', 'Assassin'],
        attributes: {
            damage: 8,
            toughness: 3,
            control: 6,
            mobility: 8,
            utility: 5
        },
        description: 'Innately connected to the latent power of Runeterra, Ahri is a vastaya who can reshape magic into orbs of raw energy.',
        abilities: [
            {
                key: 'P',
                name: 'Essence Theft',
                description: 'After killing 9 minions or monsters, Ahri heals when damaging enemies with her abilities.'
            },
            {
                key: 'Q',
                name: 'Orb of Deception',
                description: 'Ahri sends out and pulls back her orb, dealing magic damage on the way out and true damage on the way back.'
            },
            {
                key: 'W',
                name: 'Fox-Fire',
                description: 'Ahri releases three fox-fires that seek out nearby enemies.'
            },
            {
                key: 'E',
                name: 'Charm',
                description: 'Ahri blows a kiss that charms and damages the first enemy hit, causing them to walk toward her.'
            },
            {
                key: 'R',
                name: 'Spirit Rush',
                description: 'Ahri dashes forward and fires essence bolts. Can be cast up to three times before going on cooldown.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10, 'Assassin': 7 },
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

    'Brand': {
        name: 'Brand',
        title: 'the Burning Vengeance',
        role: 'Mage',
        difficulty: 4,
        tags: ['Mage'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 6,
            mobility: 2,
            utility: 5
        },
        description: 'Once a tribal shaman of ancient Lokfar, Brand is a lesson in the seductive danger of power.',
        abilities: [
            {
                key: 'P',
                name: 'Blaze',
                description: 'Brand\'s abilities light enemies ablaze, dealing damage over time and making them vulnerable to additional effects.'
            },
            {
                key: 'Q',
                name: 'Sear',
                description: 'Brand launches a fireball that deals damage and stuns enemies already ablaze.'
            },
            {
                key: 'W',
                name: 'Pillar of Flame',
                description: 'Brand creates a pillar of flame at a target area, dealing damage after a delay.'
            },
            {
                key: 'E',
                name: 'Conflagration',
                description: 'Brand conjures a powerful blast that spreads to nearby enemies if they are ablaze.'
            },
            {
                key: 'R',
                name: 'Pyroclasm',
                description: 'Brand unleashes a devastating torrent of fire that bounces between enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Team fights': 9, 'Solo plays': 7 },
            range: { 'Ranged': 9, 'No preference': 6 },
            damage: { 'High': 9, 'Very High': 10 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Burst Damage': 10, 'Sustained Damage': 8 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Winning games': 8, 'Having fun': 9 }
        }
    },

    'Syndra': {
        name: 'Syndra',
        title: 'the Dark Sovereign',
        role: 'Mage',
        difficulty: 8,
        tags: ['Mage'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 8,
            mobility: 3,
            utility: 6
        },
        description: 'Syndra is a fearsome Ionian mage with incredible power at her command.',
        abilities: [
            {
                key: 'P',
                name: 'Transcendent',
                description: 'Syndra\'s abilities gain additional effects at maximum rank.'
            },
            {
                key: 'Q',
                name: 'Dark Sphere',
                description: 'Syndra conjures a Dark Sphere at a target location, dealing damage to enemies.'
            },
            {
                key: 'W',
                name: 'Force of Will',
                description: 'Syndra picks up and throws a Dark Sphere or enemy minion, dealing damage and slowing enemies.'
            },
            {
                key: 'E',
                name: 'Scatter the Weak',
                description: 'Syndra knocks back enemies and Dark Spheres, stunning enemies hit by spheres.'
            },
            {
                key: 'R',
                name: 'Unleashed Power',
                description: 'Syndra bombards an enemy champion with all of her Dark Spheres.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10 },
            difficulty: { 'Hard': 9, 'Expert': 10 },
            gameplay: { 'Solo plays': 9, 'Team fights': 8 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 5 },
            abilities: { 'Burst Damage': 10, 'Crowd Control': 8 },
            experience: { 'Advanced': 9, 'Expert': 10 },
            motivation: { 'Improving skills': 10, 'Winning games': 9 }
        }
    },

    // More Support Champions
    'Janna': {
        name: 'Janna',
        title: 'the Storm\'s Fury',
        role: 'Support',
        difficulty: 5,
        tags: ['Support', 'Mage'],
        attributes: {
            damage: 3,
            toughness: 3,
            control: 8,
            mobility: 6,
            utility: 10
        },
        description: 'Armed with the power of Runeterra\'s gales, Janna is a mysterious elemental wind spirit who protects the dispossessed of Zaun.',
        abilities: [
            {
                key: 'P',
                name: 'Tailwind',
                description: 'Janna\'s movement speed bonus affects nearby allied champions\' movement speed.'
            },
            {
                key: 'Q',
                name: 'Howling Gale',
                description: 'Janna summons a whirlwind that knocks up and damages enemies in its path.'
            },
            {
                key: 'W',
                name: 'Zephyr',
                description: 'Janna summons an air elemental that deals damage and slows enemies.'
            },
            {
                key: 'E',
                name: 'Eye of the Storm',
                description: 'Janna shields an ally and grants them bonus attack damage.'
            },
            {
                key: 'R',
                name: 'Monsoon',
                description: 'Janna knocks back nearby enemies and channels to heal nearby allies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10, 'Mage': 5 },
            difficulty: { 'Moderate': 8, 'Hard': 9 },
            gameplay: { 'Supporting allies': 10, 'Team fights': 8 },
            range: { 'Ranged': 9, 'No preference': 6 },
            damage: { 'Very Low': 9, 'Low': 8 },
            style: { 'Defensive': 10, 'Balanced': 7 },
            abilities: { 'Healing/Shielding': 10, 'Crowd Control': 8 },
            experience: { 'Intermediate': 8, 'Advanced': 9 },
            motivation: { 'Support team': 10, 'Having fun': 8 }
        }
    },

    'Leona': {
        name: 'Leona',
        title: 'the Radiant Dawn',
        role: 'Support',
        difficulty: 4,
        tags: ['Support', 'Tank'],
        attributes: {
            damage: 4,
            toughness: 8,
            control: 9,
            mobility: 4,
            utility: 8
        },
        description: 'Imbued with the fire of the sun, Leona is a holy warrior of the Solari who defends Mount Targon with her Zenith Blade and the Shield of Daybreak.',
        abilities: [
            {
                key: 'P',
                name: 'Sunlight',
                description: 'Leona\'s abilities mark enemies with Sunlight. When allies damage marked enemies, they consume the mark to deal bonus damage.'
            },
            {
                key: 'Q',
                name: 'Shield of Daybreak',
                description: 'Leona\'s next basic attack stuns the target and deals bonus damage.'
            },
            {
                key: 'W',
                name: 'Eclipse',
                description: 'Leona raises her shield, gaining bonus armor and magic resistance, then explodes to deal damage.'
            },
            {
                key: 'E',
                name: 'Zenith Blade',
                description: 'Leona projects a solar image that deals damage and roots the last enemy champion hit.'
            },
            {
                key: 'R',
                name: 'Solar Flare',
                description: 'Leona calls down a beam of solar energy, dealing damage and stunning enemies in the center.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10, 'Tank': 8 },
            difficulty: { 'Moderate': 9, 'Hard': 7 },
            gameplay: { 'Supporting allies': 9, 'Team fights': 10 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Low': 8, 'Medium': 7 },
            style: { 'Offensive': 8, 'Balanced': 9 },
            abilities: { 'Crowd Control': 10, 'Sustained Damage': 5 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Support team': 10, 'Winning games': 8 }
        }
    },

    // Assassin Champions
    'Akali': {
        name: 'Akali',
        title: 'the Rogue Assassin',
        role: 'Assassin',
        difficulty: 7,
        tags: ['Assassin'],
        attributes: {
            damage: 8,
            toughness: 3,
            control: 4,
            mobility: 9,
            utility: 4
        },
        description: 'Abandoning the Kinkou Order and her title of the Fist of Shadow, Akali now strikes alone, ready to be the deadly weapon her people need.',
        abilities: [
            {
                key: 'P',
                name: 'Assassin\'s Mark',
                description: 'Dealing ability damage to a champion creates a ring around them. Exiting the ring empowers Akali\'s next basic attack.'
            },
            {
                key: 'Q',
                name: 'Five Point Strike',
                description: 'Akali throws kunai in an arc, dealing damage and restoring energy if cast at maximum range.'
            },
            {
                key: 'W',
                name: 'Twilight Shroud',
                description: 'Akali drops a smoke bomb, becoming invisible and gaining movement speed.'
            },
            {
                key: 'E',
                name: 'Shuriken Flip',
                description: 'Akali throws a shuriken that marks the first enemy hit. Can reactivate to dash to the marked target.'
            },
            {
                key: 'R',
                name: 'Perfect Execution',
                description: 'Akali dashes through an enemy, dealing damage. Can be cast again to dash again and execute low health enemies.'
            }
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

    'Katarina': {
        name: 'Katarina',
        title: 'the Sinister Blade',
        role: 'Assassin',
        difficulty: 8,
        tags: ['Assassin', 'Mage'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 4,
            mobility: 8,
            utility: 3
        },
        description: 'Decisive in judgment and lethal in combat, Katarina is a Noxian assassin of the highest caliber.',
        abilities: [
            {
                key: 'P',
                name: 'Voracity',
                description: 'Whenever an enemy champion dies within 3 seconds of Katarina damaging them, her ability cooldowns are reduced.'
            },
            {
                key: 'Q',
                name: 'Bouncing Blade',
                description: 'Katarina throws a dagger that bounces between enemies, marking them and dealing damage.'
            },
            {
                key: 'W',
                name: 'Preparation',
                description: 'Katarina tosses a dagger into the air and gains movement speed. The dagger falls after a delay.'
            },
            {
                key: 'E',
                name: 'Shunpo',
                description: 'Katarina instantly moves to a target, dealing damage if it\'s an enemy.'
            },
            {
                key: 'R',
                name: 'Death Lotus',
                description: 'Katarina channels and rapidly throws daggers at nearby enemies, dealing massive damage.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10, 'Mage': 6 },
            difficulty: { 'Hard': 8, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Team fights': 8 },
            range: { 'Melee': 8, 'No preference': 6 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 3 },
            abilities: { 'Burst Damage': 10, 'Mobility': 9 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Winning games': 9, 'Improving skills': 10 }
        }
    },

    'Zed': {
        name: 'Zed',
        title: 'the Master of Shadows',
        role: 'Assassin',
        difficulty: 8,
        tags: ['Assassin'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 5,
            mobility: 8,
            utility: 4
        },
        description: 'Utterly ruthless and without mercy, Zed is the leader of the Order of Shadow.',
        abilities: [
            {
                key: 'P',
                name: 'Contempt for the Weak',
                description: 'Zed\'s basic attacks against low health enemies deal bonus magic damage.'
            },
            {
                key: 'Q',
                name: 'Razor Shuriken',
                description: 'Zed and his shadows throw shurikens that deal damage to enemies they pass through.'
            },
            {
                key: 'W',
                name: 'Living Shadow',
                description: 'Zed creates a shadow that mimics his abilities. Can reactivate to swap places with the shadow.'
            },
            {
                key: 'E',
                name: 'Shadow Slash',
                description: 'Zed and his shadows slash, dealing damage and slowing enemies.'
            },
            {
                key: 'R',
                name: 'Death Mark',
                description: 'Zed becomes untargetable and dashes to an enemy, marking them for death.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Assassin': 10 },
            difficulty: { 'Hard': 8, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Roaming': 8 },
            range: { 'Melee': 8, 'No preference': 5 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 4 },
            abilities: { 'Burst Damage': 10, 'Mobility': 9 },
            experience: { 'Advanced': 9, 'Expert': 10 },
            motivation: { 'Winning games': 9, 'Improving skills': 10 }
        }
    },

    // Additional Popular Champions
    'Yasuo': {
        name: 'Yasuo',
        title: 'the Unforgiven',
        role: 'Fighter',
        difficulty: 9,
        tags: ['Fighter', 'Assassin'],
        attributes: {
            damage: 8,
            toughness: 4,
            control: 6,
            mobility: 9,
            utility: 4
        },
        description: 'An Ionian of deep resolve, Yasuo is an agile swordsman who wields the air itself against his enemies.',
        abilities: [
            {
                key: 'P',
                name: 'Way of the Wanderer',
                description: 'Yasuo\'s critical strike chance is doubled, but his critical strikes deal reduced damage. Moving grants Flow, and at maximum Flow, damage from champions grants a shield.'
            },
            {
                key: 'Q',
                name: 'Steel Tempest',
                description: 'Yasuo thrusts forward, dealing damage. On hit, grants a stack of Gathering Storm. At two stacks, Steel Tempest fires a whirlwind.'
            },
            {
                key: 'W',
                name: 'Wind Wall',
                description: 'Yasuo creates a moving wall that blocks all enemy projectiles.'
            },
            {
                key: 'E',
                name: 'Sweeping Blade',
                description: 'Yasuo dashes through a target enemy, dealing damage. Cannot be used on the same enemy for a few seconds.'
            },
            {
                key: 'R',
                name: 'Last Breath',
                description: 'Yasuo teleports to an airborne enemy champion, dealing damage and keeping them airborne longer.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 9, 'Assassin': 8 },
            difficulty: { 'Expert': 10, 'Hard': 9 },
            gameplay: { 'Solo plays': 10, 'Team fights': 7 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Sustained Damage': 8, 'Mobility': 10 },
            experience: { 'Expert': 10, 'Advanced': 8 },
            motivation: { 'Improving skills': 10, 'Having fun': 8 }
        }
    },

    'Riven': {
        name: 'Riven',
        title: 'the Exile',
        role: 'Fighter',
        difficulty: 8,
        tags: ['Fighter', 'Assassin'],
        attributes: {
            damage: 8,
            toughness: 5,
            control: 6,
            mobility: 8,
            utility: 4
        },
        description: 'Once a swordmaster in the warhosts of Noxus, Riven is an expatriate in a land she previously tried to conquer.',
        abilities: [
            {
                key: 'P',
                name: 'Runic Blade',
                description: 'Riven\'s abilities charge her blade, and her next basic attack expends one charge to deal bonus damage.'
            },
            {
                key: 'Q',
                name: 'Broken Wings',
                description: 'Riven lashes out in a series of strikes. This ability can be reactivated three times in a short period.'
            },
            {
                key: 'W',
                name: 'Ki Burst',
                description: 'Riven emits a ki burst, dealing damage and stunning nearby enemies.'
            },
            {
                key: 'E',
                name: 'Valor',
                description: 'Riven dashes forward and gains a shield.'
            },
            {
                key: 'R',
                name: 'Blade of the Exile',
                description: 'Riven empowers her blade and gains attack damage and range. Can reactivate to fire a shockwave.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 10, 'Assassin': 7 },
            difficulty: { 'Hard': 9, 'Expert': 10 },
            gameplay: { 'Solo plays': 10, 'Team fights': 7 },
            range: { 'Melee': 9, 'No preference': 6 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 10, 'Balanced': 6 },
            abilities: { 'Burst Damage': 9, 'Mobility': 8 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Improving skills': 10, 'Winning games': 8 }
        }
    },

    'Lee Sin': {
        name: 'Lee Sin',
        title: 'the Blind Monk',
        role: 'Fighter',
        difficulty: 6,
        tags: ['Fighter', 'Assassin'],
        attributes: {
            damage: 7,
            toughness: 4,
            control: 6,
            mobility: 9,
            utility: 5
        },
        description: 'A master of Ionia\'s ancient martial arts, Lee Sin is a principled fighter who channels the essence of the dragon spirit to face any challenge.',
        abilities: [
            {
                key: 'P',
                name: 'Flurry',
                description: 'After using an ability, Lee Sin\'s next 2 basic attacks gain attack speed and restore energy.'
            },
            {
                key: 'Q',
                name: 'Sonic Wave / Resonating Strike',
                description: 'Lee Sin projects a discordant wave that deals damage and reveals the first enemy struck. Can reactivate to dash to the target.'
            },
            {
                key: 'W',
                name: 'Safeguard / Iron Will',
                description: 'Lee Sin rushes to a target ally, granting both a shield. Can reactivate for bonus lifesteal and spell vamp.'
            },
            {
                key: 'E',
                name: 'Tempest / Cripple',
                description: 'Lee Sin smashes the ground, dealing damage to nearby enemies. Can reactivate to slow damaged enemies.'
            },
            {
                key: 'R',
                name: 'Dragon\'s Rage',
                description: 'Lee Sin performs a powerful roundhouse kick, dealing damage and knocking the target back.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Fighter': 9, 'Assassin': 7 },
            difficulty: { 'Hard': 9, 'Expert': 7 },
            gameplay: { 'Solo plays': 9, 'Roaming': 8, 'Team fights': 7 },
            range: { 'Melee': 9, 'No preference': 7 },
            damage: { 'High': 8, 'Medium': 7 },
            style: { 'Offensive': 9, 'Balanced': 8 },
            abilities: { 'Mobility': 10, 'Burst Damage': 7 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Improving skills': 9, 'Having fun': 8 }
        }
    },

    'Draven': {
        name: 'Draven',
        title: 'the Glorious Executioner',
        role: 'Marksman',
        difficulty: 8,
        tags: ['Marksman'],
        attributes: {
            damage: 9,
            toughness: 3,
            control: 4,
            mobility: 5,
            utility: 3
        },
        description: 'In Noxus, warriors known as reckoners face one another in arenas where blood and glory are won and lost.',
        abilities: [
            {
                key: 'P',
                name: 'League of Draven',
                description: 'Draven gains bonus gold when catching Spinning Axes or killing minions, monsters, or towers.'
            },
            {
                key: 'Q',
                name: 'Spinning Axe',
                description: 'Draven\'s next attack will deal bonus damage. This axe will ricochet off the target high up into the air.'
            },
            {
                key: 'W',
                name: 'Blood Rush',
                description: 'Draven gains increased movement speed and attack speed. The movement speed bonus decreases rapidly over its duration.'
            },
            {
                key: 'E',
                name: 'Stand Aside',
                description: 'Draven throws his axes, dealing damage and knocking aside all enemies hit.'
            },
            {
                key: 'R',
                name: 'Whirling Death',
                description: 'Draven hurls two massive axes across the map, dealing damage to each unit struck.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10 },
            difficulty: { 'Hard': 9, 'Expert': 10 },
            gameplay: { 'Solo plays': 9, 'Team fights': 8 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'Very High': 10, 'High': 8 },
            style: { 'Offensive': 10, 'Balanced': 5 },
            abilities: { 'Sustained Damage': 10, 'Burst Damage': 7 },
            experience: { 'Advanced': 8, 'Expert': 10 },
            motivation: { 'Winning games': 9, 'Having fun': 8 }
        }
    },

    'Twitch': {
        name: 'Twitch',
        title: 'the Plague Rat',
        role: 'Marksman',
        difficulty: 6,
        tags: ['Marksman', 'Assassin'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 3,
            mobility: 6,
            utility: 5
        },
        description: 'A Zaunite plague rat by birth, but a connoisseur of filth by passion, Twitch is fascinated by the most repugnant, fetid, and diseased things.',
        abilities: [
            {
                key: 'P',
                name: 'Deadly Venom',
                description: 'Twitch\'s basic attacks infect the target, dealing true damage each second.'
            },
            {
                key: 'Q',
                name: 'Ambush',
                description: 'Twitch becomes camouflaged and gains movement speed. When leaving camouflage, he gains attack speed.'
            },
            {
                key: 'W',
                name: 'Venom Cask',
                description: 'Twitch hurls a cask that explodes in a toxic cloud, slowing and applying poison to enemies.'
            },
            {
                key: 'E',
                name: 'Contaminate',
                description: 'Twitch detonates his venom, dealing damage for each stack of poison on nearby enemies.'
            },
            {
                key: 'R',
                name: 'Spray and Pray',
                description: 'Twitch gains attack damage, attack range, and his attacks pierce through all enemies in a line.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Marksman': 10, 'Assassin': 6 },
            difficulty: { 'Moderate': 7, 'Hard': 9 },
            gameplay: { 'Solo plays': 8, 'Team fights': 9, 'Roaming': 7 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 8, 'Very High': 9 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Sustained Damage': 9, 'Burst Damage': 7 },
            experience: { 'Intermediate': 7, 'Advanced': 9 },
            motivation: { 'Winning games': 8, 'Having fun': 9 }
        }
    },

    'Orianna': {
        name: 'Orianna',
        title: 'the Lady of Clockwork',
        role: 'Mage',
        difficulty: 7,
        tags: ['Mage', 'Support'],
        attributes: {
            damage: 7,
            toughness: 3,
            control: 9,
            mobility: 4,
            utility: 8
        },
        description: 'Once a curious girl of flesh and blood, Orianna is now a technological marvel comprised entirely of clockwork.',
        abilities: [
            {
                key: 'P',
                name: 'Clockwork Windup',
                description: 'Orianna\'s basic attacks deal bonus magic damage. This damage increases the more she attacks the same target.'
            },
            {
                key: 'Q',
                name: 'Command: Attack',
                description: 'Orianna commands her Ball to fly toward a target location, dealing damage to enemies it passes through.'
            },
            {
                key: 'W',
                name: 'Command: Dissonance',
                description: 'Orianna commands the Ball to release a pulse of energy, dealing damage and affecting movement speed.'
            },
            {
                key: 'E',
                name: 'Command: Protect',
                description: 'Orianna commands her Ball to attach to an allied champion, shielding them and dealing damage to enemies it passes through.'
            },
            {
                key: 'R',
                name: 'Command: Shockwave',
                description: 'Orianna commands her Ball to unleash a shockwave, dealing damage and pulling enemies toward the Ball.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10, 'Support': 7 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Team fights': 10, 'Supporting allies': 8 },
            range: { 'Ranged': 9, 'No preference': 7 },
            damage: { 'High': 8, 'Medium': 7 },
            style: { 'Balanced': 9, 'Offensive': 7 },
            abilities: { 'Crowd Control': 9, 'Burst Damage': 8 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Improving skills': 9, 'Support team': 8 }
        }
    },

    'Viktor': {
        name: 'Viktor',
        title: 'the Machine Herald',
        role: 'Mage',
        difficulty: 7,
        tags: ['Mage'],
        attributes: {
            damage: 9,
            toughness: 2,
            control: 7,
            mobility: 4,
            utility: 6
        },
        description: 'The herald of a new age of technology, Viktor has devoted his life to the advancement of humankind.',
        abilities: [
            {
                key: 'P',
                name: 'Glorious Evolution',
                description: 'Viktor can upgrade his basic abilities using Hex Cores, gaining additional effects.'
            },
            {
                key: 'Q',
                name: 'Siphon Power',
                description: 'Viktor blasts an enemy, dealing damage and granting himself a shield and movement speed.'
            },
            {
                key: 'W',
                name: 'Gravity Field',
                description: 'Viktor deploys a device that creates a gravity field, slowing enemies and eventually stunning them.'
            },
            {
                key: 'E',
                name: 'Death Ray',
                description: 'Viktor uses his robotic arm to fire a chaos beam across the battlefield.'
            },
            {
                key: 'R',
                name: 'Chaos Storm',
                description: 'Viktor conjures a singularity that deals damage and follows nearby enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Mage': 10 },
            difficulty: { 'Hard': 9, 'Expert': 8 },
            gameplay: { 'Team fights': 8, 'Solo plays': 9, 'Farming': 7 },
            range: { 'Ranged': 10, 'No preference': 7 },
            damage: { 'High': 9, 'Very High': 8 },
            style: { 'Offensive': 9, 'Balanced': 7 },
            abilities: { 'Burst Damage': 9, 'Sustained Damage': 8 },
            experience: { 'Advanced': 9, 'Expert': 8 },
            motivation: { 'Improving skills': 9, 'Winning games': 8 }
        }
    },

    'Blitzcrank': {
        name: 'Blitzcrank',
        title: 'the Great Steam Golem',
        role: 'Support',
        difficulty: 4,
        tags: ['Support', 'Tank'],
        attributes: {
            damage: 5,
            toughness: 7,
            control: 9,
            mobility: 5,
            utility: 8
        },
        description: 'Blitzcrank is an enormous, near-indestructible automaton from Zaun, originally built to dispose of hazardous waste.',
        abilities: [
            {
                key: 'P',
                name: 'Mana Barrier',
                description: 'When Blitzcrank\'s health drops below 20%, he gains a shield equal to 50% of his mana.'
            },
            {
                key: 'Q',
                name: 'Rocket Grab',
                description: 'Blitzcrank fires his right hand, dealing damage and pulling the first enemy struck to him.'
            },
            {
                key: 'W',
                name: 'Overdrive',
                description: 'Blitzcrank supercharges himself, gaining movement speed and attack speed but becoming slowed afterwards.'
            },
            {
                key: 'E',
                name: 'Power Fist',
                description: 'Blitzcrank charges up his fist, making his next attack deal double damage and knock up the target.'
            },
            {
                key: 'R',
                name: 'Static Field',
                description: 'Blitzcrank creates an electric field around himself, dealing damage and silencing nearby enemies.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10, 'Tank': 7 },
            difficulty: { 'Easy': 7, 'Moderate': 9 },
            gameplay: { 'Supporting allies': 9, 'Team fights': 8 },
            range: { 'Melee': 7, 'No preference': 8 },
            damage: { 'Low': 7, 'Medium': 8 },
            style: { 'Offensive': 8, 'Balanced': 9 },
            abilities: { 'Crowd Control': 10, 'Burst Damage': 6 },
            experience: { 'Beginner': 8, 'Intermediate': 9 },
            motivation: { 'Support team': 9, 'Having fun': 10 }
        }
    },

    'Morgana': {
        name: 'Morgana',
        title: 'the Fallen',
        role: 'Support',
        difficulty: 5,
        tags: ['Support', 'Mage'],
        attributes: {
            damage: 6,
            toughness: 3,
            control: 9,
            mobility: 3,
            utility: 9
        },
        description: 'Conflicted between her celestial and mortal natures, Morgana bound her wings to embrace humanity, and inflicts her pain and bitterness upon the dishonest and the corrupt.',
        abilities: [
            {
                key: 'P',
                name: 'Soul Siphon',
                description: 'Morgana heals herself when damaging enemies with her abilities, with increased healing against champions.'
            },
            {
                key: 'Q',
                name: 'Dark Binding',
                description: 'Morgana fires a bolt of dark energy, rooting and dealing damage to the first enemy hit.'
            },
            {
                key: 'W',
                name: 'Tormented Shadow',
                description: 'Morgana casts a pool of shadow that deals damage over time to enemies standing in it.'
            },
            {
                key: 'E',
                name: 'Black Shield',
                description: 'Morgana shields an ally, absorbing magic damage and preventing disables while the shield holds.'
            },
            {
                key: 'R',
                name: 'Soul Shackles',
                description: 'Morgana chains nearby enemy champions, dealing damage and stunning them if they remain nearby.'
            }
        ],
        matchingFactors: {
            rolePreference: { 'Support': 10, 'Mage': 8 },
            difficulty: { 'Moderate': 8, 'Hard': 7 },
            gameplay: { 'Supporting allies': 10, 'Team fights': 9 },
            range: { 'Ranged': 9, 'No preference': 7 },
            damage: { 'Medium': 7, 'High': 6 },
            style: { 'Defensive': 8, 'Balanced': 9 },
            abilities: { 'Crowd Control': 10, 'Healing/Shielding': 8 },
            experience: { 'Intermediate': 9, 'Advanced': 7 },
            motivation: { 'Support team': 10, 'Having fun': 8 }
        }
    }
};

// Questions database
const questionsDatabase = [
    {
        id: 1,
        text: 'What role do you prefer to play?',
        options: [
            { value: 'Tank', text: '🛡️ Tank - Protect your team and initiate fights', description: 'Durable champions who lead the charge and protect allies' },
            { value: 'Fighter', text: '⚔️ Fighter - Balanced damage and durability', description: 'Versatile melee champions with good damage and survivability' },
            { value: 'Assassin', text: '🗡️ Assassin - High burst damage and mobility', description: 'Mobile champions who eliminate key targets quickly' },
            { value: 'Mage', text: '🔮 Mage - Magical damage and crowd control', description: 'Ranged spellcasters with powerful abilities' },
            { value: 'Marksman', text: '🏹 Marksman - Ranged physical damage', description: 'Ranged champions who deal consistent physical damage' },
            { value: 'Support', text: '💚 Support - Help and protect allies', description: 'Champions focused on enabling and protecting teammates' }
        ],
        weight: 0.9,
        category: 'rolePreference'
    },
    {
        id: 2,
        text: 'What difficulty level are you comfortable with?',
        options: [
            { value: 'Easy', text: '🟢 Easy - Simple mechanics, great for beginners', description: 'Champions with straightforward abilities and gameplay' },
            { value: 'Moderate', text: '🟡 Moderate - Some complexity, good for learning', description: 'Champions with moderate complexity that reward practice' },
            { value: 'Hard', text: '🟠 Hard - Complex mechanics, requires practice', description: 'Champions with intricate mechanics requiring dedication' },
            { value: 'Expert', text: '🔴 Expert - Very challenging, for experienced players', description: 'The most complex champions requiring mastery' }
        ],
        weight: 0.8,
        category: 'difficulty'
    },
    {
        id: 3,
        text: 'What type of gameplay do you enjoy most?',
        options: [
            { value: 'Team fights', text: '👥 Team fights - Big group battles', description: 'Large-scale battles with multiple champions' },
            { value: 'Solo plays', text: '🎯 Solo plays - 1v1 outplays and carries', description: 'Individual skill expression and carrying games' },
            { value: 'Supporting allies', text: '🤝 Supporting allies - Enable your team', description: 'Helping teammates succeed and coordinating plays' },
            { value: 'Farming', text: '🌾 Farming - Focus on gold and items', description: 'Patient gameplay focused on resource accumulation' },
            { value: 'Roaming', text: '🗺️ Roaming - Move around the map', description: 'Mobile gameplay affecting multiple areas of the map' }
        ],
        weight: 0.7,
        category: 'gameplay'
    },
    {
        id: 4,
        text: 'Do you prefer melee or ranged champions?',
        options: [
            { value: 'Melee', text: '⚔️ Melee - Close combat champions', description: 'Champions who fight up close and personal' },
            { value: 'Ranged', text: '🏹 Ranged - Attack from a distance', description: 'Champions who fight from safety at range' },
            { value: 'No preference', text: '🤷 No preference - Either works', description: 'Comfortable with both melee and ranged champions' }
        ],
        weight: 0.6,
        category: 'range'
    },
    {
        id: 5,
        text: 'How important is high damage output to you?',
        options: [
            { value: 'Very Low', text: '1️⃣ Very Low - Damage isn\'t important', description: 'Focus on utility and support rather than damage' },
            { value: 'Low', text: '2️⃣ Low - Some damage is fine', description: 'Moderate damage with focus on other aspects' },
            { value: 'Medium', text: '3️⃣ Medium - Balanced damage output', description: 'Good damage balanced with other strengths' },
            { value: 'High', text: '4️⃣ High - Strong damage is important', description: 'High damage output is a priority' },
            { value: 'Very High', text: '5️⃣ Very High - Maximum damage potential', description: 'Highest possible damage output is essential' }
        ],
        weight: 0.7,
        category: 'damage'
    },
    {
        id: 6,
        text: 'Do you prefer defensive or offensive playstyles?',
        options: [
            { value: 'Defensive', text: '🛡️ Defensive - Protect and react', description: 'Focus on protection, healing, and reactive gameplay' },
            { value: 'Offensive', text: '⚡ Offensive - Aggressive and proactive', description: 'Aggressive playstyle focused on making plays' },
            { value: 'Balanced', text: '⚖️ Balanced - Mix of both approaches', description: 'Flexible approach adapting to situations' }
        ],
        weight: 0.6,
        category: 'style'
    },
    {
        id: 7,
        text: 'What type of abilities do you prefer?',
        options: [
            { value: 'Crowd Control', text: '🕸️ Crowd Control - Stuns, slows, roots', description: 'Abilities that control enemy movement and actions' },
            { value: 'Burst Damage', text: '💥 Burst Damage - High damage combos', description: 'Abilities that deal massive damage quickly' },
            { value: 'Sustained Damage', text: '🔄 Sustained Damage - Consistent DPS', description: 'Abilities that provide steady damage over time' },
            { value: 'Healing/Shielding', text: '💚 Healing/Shielding - Protect allies', description: 'Abilities that heal or shield teammates' },
            { value: 'Mobility', text: '💨 Mobility - Dashes and movement', description: 'Abilities that provide movement and positioning' }
        ],
        weight: 0.8,
        category: 'abilities'
    },
    {
        id: 8,
        text: 'How experienced are you with League of Legends?',
        options: [
            { value: 'Beginner', text: '🌱 Beginner - New to the game', description: 'Just starting your League of Legends journey' },
            { value: 'Intermediate', text: '🌿 Intermediate - Know the basics', description: 'Understand core mechanics and some champions' },
            { value: 'Advanced', text: '🌳 Advanced - Experienced player', description: 'Strong understanding of game mechanics and strategy' },
            { value: 'Expert', text: '🏆 Expert - Highly skilled veteran', description: 'Master-level understanding of the game' }
        ],
        weight: 0.5,
        category: 'experience'
    },
    {
        id: 9,
        text: 'What motivates you most when playing?',
        options: [
            { value: 'Winning games', text: '🏆 Winning games - Victory above all', description: 'Focused on climbing ranks and winning matches' },
            { value: 'Learning new champions', text: '📚 Learning new champions - Expanding skills', description: 'Enjoying the process of mastering new champions' },
            { value: 'Having fun', text: '🎉 Having fun - Enjoyment first', description: 'Playing for entertainment and enjoyable experiences' },
            { value: 'Improving skills', text: '📈 Improving skills - Personal growth', description: 'Focused on becoming a better player overall' },
            { value: 'Support team', text: '🤝 Support team - Help others succeed', description: 'Satisfaction from enabling teammates to succeed' }
        ],
        weight: 0.4,
        category: 'motivation'
    }
];

// Recommendation algorithm
const RecommendationEngine = {
    calculateChampionScore: function(champion, answers) {
        let totalScore = 0;
        let totalWeight = 0;

        questionsDatabase.forEach(question => {
            const answer = answers[question.id];
            if (!answer) return;

            const matchingFactor = champion.matchingFactors[question.category];
            if (!matchingFactor) return;

            const score = matchingFactor[answer] || 0;
            totalScore += score * question.weight;
            totalWeight += question.weight;
        });

        return totalWeight > 0 ? (totalScore / totalWeight) / 10 : 0;
    },

    getRecommendations: function(answers, excludeChampions = []) {
        const scores = [];

        Object.values(championDatabase).forEach(champion => {
            if (excludeChampions.includes(champion.name)) return;

            const score = this.calculateChampionScore(champion, answers);
            if (score > 0) {
                scores.push({
                    champion: champion,
                    score: score,
                    confidence: Math.min(95, Math.round(score * 100))
                });
            }
        });

        return scores.sort((a, b) => b.score - a.score);
    },

    getBestRecommendation: function(answers) {
        const recommendations = this.getRecommendations(answers);
        return recommendations.length > 0 ? recommendations[0] : null;
    },

    getAlternatives: function(answers, mainChampion, count = 3) {
        const recommendations = this.getRecommendations(answers, [mainChampion.name]);
        return recommendations.slice(0, count);
    },

    generateExplanation: function(champion, answers, confidence) {
        const explanations = [];
        
        // Role match
        const roleAnswer = answers[1];
        if (roleAnswer && champion.matchingFactors.rolePreference[roleAnswer] >= 8) {
            explanations.push(`Perfect role alignment with ${roleAnswer} champions`);
        }

        // Difficulty match
        const difficultyAnswer = answers[2];
        if (difficultyAnswer && champion.matchingFactors.difficulty[difficultyAnswer] >= 8) {
            explanations.push(`Matches your preference for ${difficultyAnswer.toLowerCase()} champions`);
        }

        // Gameplay style
        const gameplayAnswer = answers[3];
        if (gameplayAnswer && champion.matchingFactors.gameplay[gameplayAnswer] >= 8) {
            explanations.push(`Excels in ${gameplayAnswer.toLowerCase()}`);
        }

        // Abilities preference
        const abilitiesAnswer = answers[7];
        if (abilitiesAnswer && champion.matchingFactors.abilities[abilitiesAnswer] >= 8) {
            explanations.push(`Strong ${abilitiesAnswer.toLowerCase()} capabilities`);
        }

        let explanation = `Based on your preferences, ${champion.name} is an excellent match! `;
        
        if (explanations.length > 0) {
            explanation += `Key strengths: ${explanations.join(', ')}.`;
        }

        explanation += ` ${champion.description}`;

        return explanation;
    }
};