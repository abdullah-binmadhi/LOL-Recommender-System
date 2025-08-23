"""
Champion test fixtures
Provides consistent champion data for testing
"""

from typing import List, Dict, Any
from models.champion import Champion, Ability

# Sample champion data for testing
SAMPLE_CHAMPIONS_DATA = [
    {
        "id": "malphite",
        "name": "Malphite",
        "title": "Shard of the Monolith",
        "role": "Tank",
        "difficulty": 2,
        "tags": ["Tank", "Fighter"],
        "attributes": {
            "damage": 4,
            "toughness": 9,
            "control": 7,
            "mobility": 5,
            "utility": 7,
            "difficulty": 2
        },
        "range_type": "Melee",
        "position": "Top",
        "abilities": [
            {
                "name": "Granite Shield",
                "type": "passive",
                "description": "Malphite is shielded by a layer of rock which absorbs damage."
            },
            {
                "name": "Seismic Shard",
                "type": "Q",
                "description": "Malphite sends a shard of the earth through the ground at his foe."
            },
            {
                "name": "Thunderclap",
                "type": "W",
                "description": "Malphite attacks with such force that it creates a thunderclap."
            },
            {
                "name": "Ground Slam",
                "type": "E",
                "description": "Malphite slams the ground, sending out a shockwave."
            },
            {
                "name": "Unstoppable Force",
                "type": "R",
                "description": "Malphite launches himself to a location at high speed."
            }
        ]
    },
    {
        "id": "amumu",
        "name": "Amumu",
        "title": "the Sad Mummy",
        "role": "Tank",
        "difficulty": 3,
        "tags": ["Tank", "Mage"],
        "attributes": {
            "damage": 6,
            "toughness": 6,
            "control": 8,
            "mobility": 5,
            "utility": 6,
            "difficulty": 3
        },
        "range_type": "Melee",
        "position": "Jungle",
        "abilities": [
            {
                "name": "Cursed Touch",
                "type": "passive",
                "description": "Amumu's attacks reduce magic resistance."
            },
            {
                "name": "Bandage Toss",
                "type": "Q",
                "description": "Amumu tosses a bandage at a target."
            },
            {
                "name": "Despair",
                "type": "W",
                "description": "Overcome by anguish, nearby enemies lose health."
            },
            {
                "name": "Tantrum",
                "type": "E",
                "description": "Ammu throws a tantrum, dealing damage to surrounding units."
            },
            {
                "name": "Curse of the Sad Mummy",
                "type": "R",
                "description": "Amumu entangles surrounding enemy units in bandages."
            }
        ]
    },
    {
        "id": "jinx",
        "name": "Jinx",
        "title": "the Loose Cannon",
        "role": "Marksman",
        "difficulty": 6,
        "tags": ["Marksman"],
        "attributes": {
            "damage": 9,
            "toughness": 2,
            "control": 7,
            "mobility": 6,
            "utility": 6,
            "difficulty": 6
        },
        "range_type": "Ranged",
        "position": "Bot",
        "abilities": [
            {
                "name": "Get Excited!",
                "type": "passive",
                "description": "Jinx gains movement speed and attack speed when damaging enemy champions."
            },
            {
                "name": "Switcheroo!",
                "type": "Q",
                "description": "Jinx swaps weapons between Pow-Pow and Fishbones."
            },
            {
                "name": "Zap!",
                "type": "W",
                "description": "Jinx uses Zapper to fire a shock blast."
            },
            {
                "name": "Flame Chompers!",
                "type": "E",
                "description": "Jinx tosses out a line of snare grenades."
            },
            {
                "name": "Super Mega Death Rocket!",
                "type": "R",
                "description": "Jinx fires a rocket across the map."
            }
        ]
    },
    {
        "id": "lux",
        "name": "Lux",
        "title": "the Lady of Luminosity",
        "role": "Mage",
        "difficulty": 5,
        "tags": ["Mage", "Support"],
        "attributes": {
            "damage": 7,
            "toughness": 2,
            "control": 8,
            "mobility": 4,
            "utility": 7,
            "difficulty": 5
        },
        "range_type": "Ranged",
        "position": "Mid",
        "abilities": [
            {
                "name": "Illumination",
                "type": "passive",
                "description": "Lux's spells charge enemies with energy for a few seconds."
            },
            {
                "name": "Light Binding",
                "type": "Q",
                "description": "Lux releases a sphere of light that binds enemies."
            },
            {
                "name": "Prismatic Barrier",
                "type": "W",
                "description": "Lux throws her wand and bends the light around allies."
            },
            {
                "name": "Lucent Singularity",
                "type": "E",
                "description": "Creates an area of twisted light that slows enemies."
            },
            {
                "name": "Final Spark",
                "type": "R",
                "description": "Lux fires a laser that deals damage to all enemies in a line."
            }
        ]
    },
    {
        "id": "thresh",
        "name": "Thresh",
        "title": "the Chain Warden",
        "role": "Support",
        "difficulty": 7,
        "tags": ["Support", "Fighter"],
        "attributes": {
            "damage": 6,
            "toughness": 6,
            "control": 9,
            "mobility": 5,
            "utility": 9,
            "difficulty": 7
        },
        "range_type": "Ranged",
        "position": "Bot",
        "abilities": [
            {
                "name": "Damnation",
                "type": "passive",
                "description": "Thresh can harvest souls to permanently increase his armor and ability power."
            },
            {
                "name": "Death Sentence",
                "type": "Q",
                "description": "Thresh binds an enemy in chains and pulls them toward him."
            },
            {
                "name": "Dark Passage",
                "type": "W",
                "description": "Thresh throws his lantern to a target location."
            },
            {
                "name": "Flay",
                "type": "E",
                "description": "Thresh sweeps his chain, knocking back enemies."
            },
            {
                "name": "The Box",
                "type": "R",
                "description": "Thresh creates a prison of walls around himself."
            }
        ]
    }
]

def create_test_champion(champion_data: Dict[str, Any]) -> Champion:
    """Create a Champion object from test data"""
    abilities = []
    for ability_data in champion_data.get("abilities", []):
        ability = Ability(
            name=ability_data["name"],
            ability_type=ability_data["type"],
            description=ability_data["description"]
        )
        abilities.append(ability)
    
    return Champion(
        id=champion_data["id"],
        name=champion_data["name"],
        title=champion_data["title"],
        role=champion_data["role"],
        difficulty=champion_data["difficulty"],
        tags=champion_data["tags"],
        attributes=champion_data["attributes"],
        abilities=abilities
    )

def get_test_champions() -> List[Champion]:
    """Get all test champions as Champion objects"""
    return [create_test_champion(data) for data in SAMPLE_CHAMPIONS_DATA]

def get_test_champion_by_name(name: str) -> Champion:
    """Get a specific test champion by name"""
    for champion_data in SAMPLE_CHAMPIONS_DATA:
        if champion_data["name"].lower() == name.lower():
            return create_test_champion(champion_data)
    raise ValueError(f"Test champion '{name}' not found")

def get_test_champions_by_role(role: str) -> List[Champion]:
    """Get test champions filtered by role"""
    champions = []
    for champion_data in SAMPLE_CHAMPIONS_DATA:
        if champion_data["role"].lower() == role.lower():
            champions.append(create_test_champion(champion_data))
    return champions

def get_test_champions_by_difficulty(min_diff: int, max_diff: int) -> List[Champion]:
    """Get test champions filtered by difficulty range"""
    champions = []
    for champion_data in SAMPLE_CHAMPIONS_DATA:
        difficulty = champion_data["difficulty"]
        if min_diff <= difficulty <= max_diff:
            champions.append(create_test_champion(champion_data))
    return champions

# Champion data for specific test scenarios
TANK_CHAMPIONS = ["Malphite", "Ammu"]
EASY_CHAMPIONS = ["Malphite"]  # Difficulty <= 3
HARD_CHAMPIONS = ["Jinx", "Thresh"]  # Difficulty >= 6
RANGED_CHAMPIONS = ["Jinx", "Lux", "Thresh"]
MELEE_CHAMPIONS = ["Malphite", "Amumu"]

# Expected champion recommendations for common response patterns
EXPECTED_RECOMMENDATIONS = {
    "tank_easy": "Malphite",
    "tank_moderate": "Amumu", 
    "damage_ranged": "Jinx",
    "support_utility": "Thresh",
    "mage_control": "Lux"
}

# Champion attribute ranges for validation
ATTRIBUTE_RANGES = {
    "damage": (1, 10),
    "toughness": (1, 10),
    "control": (1, 10),
    "mobility": (1, 10),
    "utility": (1, 10),
    "difficulty": (1, 10)
}

# Role mappings for testing
ROLE_MAPPINGS = {
    "Tank": ["Malphite", "Amumu"],
    "Marksman": ["Jinx"],
    "Mage": ["Lux"],
    "Support": ["Thresh"],
    "Fighter": [],
    "Assassin": []
}