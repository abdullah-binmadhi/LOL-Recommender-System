"""
Enhanced champion details service with comprehensive information,
tips, guides, and external resources.
"""

import json
import os
from typing import Dict, List, Optional, Any
from models.champion import Champion
from services.champion_service import ChampionService

class ChampionDetailsService:
    """Service for providing detailed champion information and guidance"""
    
    def __init__(self, data_folder: str = "data"):
        self.data_folder = data_folder
        self.champion_service = ChampionService(data_folder)
        self.tips_file = os.path.join(data_folder, "champion_tips.json")
        self.guides_file = os.path.join(data_folder, "champion_guides.json")
        self.counters_file = os.path.join(data_folder, "champion_counters.json")
        
        # Load additional data
        self._tips_data = self._load_tips_data()
        self._guides_data = self._load_guides_data()
        self._counters_data = self._load_counters_data()
    
    def get_champion_details(self, champion_name: str) -> Dict[str, Any]:
        """Get comprehensive champion details"""
        champion = self.champion_service.get_champion_by_name(champion_name)
        if not champion:
            return {}
        
        return {
            'basic_info': self._get_basic_info(champion),
            'abilities': self._get_abilities_info(champion),
            'playstyle_tips': self._get_playstyle_tips(champion),
            'build_recommendations': self._get_build_recommendations(champion),
            'matchup_info': self._get_matchup_info(champion),
            'learning_resources': self._get_learning_resources(champion),
            'statistics': self._get_champion_statistics(champion),
            'lore_snippet': self._get_lore_snippet(champion)
        }
    
    def _get_basic_info(self, champion: Champion) -> Dict[str, Any]:
        """Get basic champion information"""
        return {
            'name': champion.name,
            'title': champion.title,
            'role': champion.role,
            'difficulty': champion.difficulty,
            'tags': champion.tags,
            'range_type': getattr(champion, 'range_type', 'Unknown'),
            'position': getattr(champion, 'position', 'Any'),
            'resource_type': self._get_resource_type(champion),
            'damage_type': self._get_damage_type(champion),
            'attributes': getattr(champion, 'attributes', {})
        }
    
    def _get_abilities_info(self, champion: Champion) -> Dict[str, Any]:
        """Get detailed abilities information"""
        abilities = getattr(champion, 'abilities', [])
        
        # If no abilities in data, generate generic ones based on role
        if not abilities:
            abilities = self._generate_generic_abilities(champion)
        
        return {
            'passive': self._find_ability(abilities, 'passive'),
            'q_ability': self._find_ability(abilities, 'Q'),
            'w_ability': self._find_ability(abilities, 'W'),
            'e_ability': self._find_ability(abilities, 'E'),
            'ultimate': self._find_ability(abilities, 'R'),
            'ability_tips': self._get_ability_tips(champion)
        }
    
    def _get_playstyle_tips(self, champion: Champion) -> List[str]:
        """Get playstyle tips for the champion"""
        tips = []
        
        # Role-based tips
        role_tips = {
            'Tank': [
                "Focus on engaging fights and protecting your team",
                "Build defensive items to maximize survivability",
                "Use your crowd control abilities to set up plays"
            ],
            'Fighter': [
                "Balance offense and defense in your item builds",
                "Look for opportunities to split push",
                "Excel in extended trades and duels"
            ],
            'Assassin': [
                "Target enemy carries in team fights",
                "Use mobility to enter and exit fights quickly",
                "Build lethality items for maximum burst damage"
            ],
            'Mage': [
                "Position safely in team fights",
                "Manage your mana efficiently",
                "Build ability power and magic penetration"
            ],
            'Marksman': [
                "Focus on farming and scaling into late game",
                "Stay behind your frontline in team fights",
                "Build critical strike and attack speed items"
            ],
            'Support': [
                "Ward key areas to provide vision for your team",
                "Protect your ADC during the laning phase",
                "Build utility items to help your team"
            ]
        }
        
        tips.extend(role_tips.get(champion.role, []))
        
        # Difficulty-based tips
        if champion.difficulty <= 3:
            tips.append("Great champion for beginners to learn the game")
            tips.append("Focus on mastering basic mechanics first")
        elif champion.difficulty >= 7:
            tips.append("Requires practice to master effectively")
            tips.append("Watch high-level gameplay to learn advanced techniques")
        
        # Champion-specific tips from data
        champion_tips = self._tips_data.get(champion.name.lower(), [])
        tips.extend(champion_tips)
        
        return tips[:6]  # Limit to 6 tips
    
    def _get_build_recommendations(self, champion: Champion) -> Dict[str, List[str]]:
        """Get item build recommendations"""
        builds = {
            'starter_items': [],
            'core_items': [],
            'situational_items': [],
            'boots_options': []
        }
        
        # Role-based build recommendations
        role_builds = {
            'Tank': {
                'starter_items': ["Doran's Shield", "Health Potion"],
                'core_items': ["Sunfire Aegis", "Thornmail", "Spirit Visage"],
                'situational_items': ["Randuin's Omen", "Force of Nature", "Gargoyle Stoneplate"],
                'boots_options': ["Plated Steelcaps", "Mercury's Treads"]
            },
            'Fighter': {
                'starter_items': ["Doran's Blade", "Health Potion"],
                'core_items': ["Trinity Force", "Sterak's Gage", "Death's Dance"],
                'situational_items': ["Black Cleaver", "Guardian Angel", "Maw of Malmortius"],
                'boots_options': ["Plated Steelcaps", "Mercury's Treads"]
            },
            'Assassin': {
                'starter_items': ["Doran's Blade", "Long Sword"],
                'core_items': ["Duskblade of Draktharr", "Youmuu's Ghostblade", "Edge of Night"],
                'situational_items': ["Serpent's Fang", "Guardian Angel", "Maw of Malmortius"],
                'boots_options': ["Ionian Boots of Lucidity", "Mercury's Treads"]
            },
            'Mage': {
                'starter_items': ["Doran's Ring", "Health Potion"],
                'core_items': ["Luden's Tempest", "Zhonya's Hourglass", "Void Staff"],
                'situational_items': ["Banshee's Veil", "Morellonomicon", "Cosmic Drive"],
                'boots_options': ["Sorcerer's Shoes", "Ionian Boots of Lucidity"]
            },
            'Marksman': {
                'starter_items': ["Doran's Blade", "Health Potion"],
                'core_items': ["Kraken Slayer", "Phantom Dancer", "Infinity Edge"],
                'situational_items': ["Lord Dominik's Regards", "Guardian Angel", "Mercurial Scimitar"],
                'boots_options': ["Berserker's Greaves", "Mercury's Treads"]
            },
            'Support': {
                'starter_items': ["Spectral Sickle", "Health Potion"],
                'core_items': ["Imperial Mandate", "Staff of Flowing Water", "Redemption"],
                'situational_items': ["Locket of the Iron Solari", "Mikael's Blessing", "Zhonya's Hourglass"],
                'boots_options': ["Ionian Boots of Lucidity", "Mercury's Treads"]
            }
        }
        
        return role_builds.get(champion.role, builds)
    
    def _get_matchup_info(self, champion: Champion) -> Dict[str, List[str]]:
        """Get matchup information"""
        matchups = {
            'strong_against': [],
            'weak_against': [],
            'synergizes_with': []
        }
        
        # Get from counters data if available
        champion_counters = self._counters_data.get(champion.name.lower(), {})
        matchups.update(champion_counters)
        
        # If no specific data, provide role-based matchups
        if not any(matchups.values()):
            role_matchups = {
                'Tank': {
                    'strong_against': ['Assassins', 'Squishy champions'],
                    'weak_against': ['Percent health damage', 'Sustained DPS'],
                    'synergizes_with': ['ADCs', 'Mages']
                },
                'Fighter': {
                    'strong_against': ['Squishy champions', 'Immobile mages'],
                    'weak_against': ['Kiting champions', 'Heavy crowd control'],
                    'synergizes_with': ['Supports', 'Junglers']
                },
                'Assassin': {
                    'strong_against': ['Squishy carries', 'Immobile champions'],
                    'weak_against': ['Tanks', 'Heavy crowd control'],
                    'synergizes_with': ['Engage supports', 'Divers']
                },
                'Mage': {
                    'strong_against': ['Melee champions', 'Grouped enemies'],
                    'weak_against': ['Assassins', 'Long-range poke'],
                    'synergizes_with': ['Tanks', 'Supports']
                },
                'Marksman': {
                    'strong_against': ['Tanks', 'Melee champions'],
                    'weak_against': ['Assassins', 'Divers'],
                    'synergizes_with': ['Supports', 'Peeling champions']
                },
                'Support': {
                    'strong_against': ['Varies by type'],
                    'weak_against': ['Varies by type'],
                    'synergizes_with': ['ADCs', 'Team compositions']
                }
            }
            matchups = role_matchups.get(champion.role, matchups)
        
        return matchups
    
    def _get_learning_resources(self, champion: Champion) -> Dict[str, List[Dict[str, str]]]:
        """Get learning resources and external links"""
        resources = {
            'guides': [],
            'videos': [],
            'pro_builds': [],
            'community': []
        }
        
        # Champion-specific guides from data
        champion_guides = self._guides_data.get(champion.name.lower(), {})
        resources.update(champion_guides)
        
        # Default resources if none specified
        if not any(resources.values()):
            champion_url_name = champion.name.lower().replace(' ', '').replace("'", "")
            
            resources = {
                'guides': [
                    {
                        'title': f'{champion.name} Guide - Mobafire',
                        'url': f'https://www.mobafire.com/league-of-legends/{champion_url_name}-guide',
                        'description': 'Comprehensive champion guide with builds and strategies'
                    },
                    {
                        'title': f'{champion.name} Guide - OP.GG',
                        'url': f'https://op.gg/champions/{champion_url_name}',
                        'description': 'Statistics, builds, and matchup data'
                    }
                ],
                'videos': [
                    {
                        'title': f'{champion.name} Gameplay - YouTube',
                        'url': f'https://www.youtube.com/results?search_query={champion.name}+league+of+legends+guide',
                        'description': 'Video guides and gameplay examples'
                    }
                ],
                'pro_builds': [
                    {
                        'title': f'{champion.name} Pro Builds',
                        'url': f'https://probuildstats.com/champion/{champion_url_name}',
                        'description': 'Professional player builds and statistics'
                    }
                ],
                'community': [
                    {
                        'title': f'{champion.name} Mains Subreddit',
                        'url': f'https://www.reddit.com/r/{champion_url_name}mains',
                        'description': 'Community discussions and tips'
                    }
                ]
            }
        
        return resources
    
    def _get_champion_statistics(self, champion: Champion) -> Dict[str, Any]:
        """Get champion statistics and ratings"""
        attributes = getattr(champion, 'attributes', {})
        
        return {
            'difficulty_rating': champion.difficulty,
            'damage_rating': attributes.get('damage', 5),
            'toughness_rating': attributes.get('toughness', 5),
            'control_rating': attributes.get('control', 5),
            'mobility_rating': attributes.get('mobility', 5),
            'utility_rating': attributes.get('utility', 5),
            'overall_rating': sum(attributes.values()) / len(attributes) if attributes else 5,
            'popularity_tier': self._calculate_popularity_tier(champion),
            'skill_floor': self._calculate_skill_floor(champion),
            'skill_ceiling': self._calculate_skill_ceiling(champion)
        }
    
    def _get_lore_snippet(self, champion: Champion) -> str:
        """Get a brief lore snippet for the champion"""
        # This would ideally come from a lore database
        # For now, generate based on role and title
        lore_templates = {
            'Tank': f"{champion.name}, {champion.title}, stands as an unwavering guardian, protecting allies with unbreakable resolve.",
            'Fighter': f"{champion.name}, {champion.title}, combines strength and skill in battle, adapting to any combat situation.",
            'Assassin': f"{champion.name}, {champion.title}, strikes from the shadows with deadly precision, eliminating key targets.",
            'Mage': f"{champion.name}, {champion.title}, wields arcane powers to devastate enemies from afar.",
            'Marksman': f"{champion.name}, {champion.title}, delivers precise ranged attacks to control the battlefield.",
            'Support': f"{champion.name}, {champion.title}, empowers allies and controls the flow of battle through strategic support."
        }
        
        return lore_templates.get(champion.role, f"{champion.name}, {champion.title}, brings unique abilities to the Rift.")
    
    def _load_tips_data(self) -> Dict[str, List[str]]:
        """Load champion tips data"""
        if os.path.exists(self.tips_file):
            try:
                with open(self.tips_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception:
                pass
        return {}
    
    def _load_guides_data(self) -> Dict[str, Dict]:
        """Load champion guides data"""
        if os.path.exists(self.guides_file):
            try:
                with open(self.guides_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception:
                pass
        return {}
    
    def _load_counters_data(self) -> Dict[str, Dict]:
        """Load champion counters data"""
        if os.path.exists(self.counters_file):
            try:
                with open(self.counters_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception:
                pass
        return {}
    
    def _get_resource_type(self, champion: Champion) -> str:
        """Determine champion resource type"""
        # This would ideally come from champion data
        role_resources = {
            'Mage': 'Mana',
            'Support': 'Mana',
            'Tank': 'Mana',
            'Fighter': 'Mana',
            'Assassin': 'Energy/Mana',
            'Marksman': 'Mana'
        }
        return role_resources.get(champion.role, 'Mana')
    
    def _get_damage_type(self, champion: Champion) -> str:
        """Determine primary damage type"""
        role_damage = {
            'Mage': 'Magic',
            'Support': 'Magic',
            'Tank': 'Mixed',
            'Fighter': 'Physical',
            'Assassin': 'Physical',
            'Marksman': 'Physical'
        }
        return role_damage.get(champion.role, 'Mixed')
    
    def _generate_generic_abilities(self, champion: Champion) -> List[Dict]:
        """Generate generic abilities based on role"""
        role_abilities = {
            'Tank': [
                {'name': 'Defensive Stance', 'type': 'passive', 'description': 'Gains defensive bonuses'},
                {'name': 'Shield Slam', 'type': 'Q', 'description': 'Damages and slows enemies'},
                {'name': 'Protective Barrier', 'type': 'W', 'description': 'Shields nearby allies'},
                {'name': 'Crowd Control', 'type': 'E', 'description': 'Stuns or roots enemies'},
                {'name': 'Ultimate Defense', 'type': 'R', 'description': 'Massive defensive boost'}
            ],
            'Fighter': [
                {'name': 'Combat Prowess', 'type': 'passive', 'description': 'Enhanced combat abilities'},
                {'name': 'Strike', 'type': 'Q', 'description': 'Powerful melee attack'},
                {'name': 'Defensive Stance', 'type': 'W', 'description': 'Reduces incoming damage'},
                {'name': 'Gap Closer', 'type': 'E', 'description': 'Dashes to enemies'},
                {'name': 'Devastating Combo', 'type': 'R', 'description': 'Powerful finishing move'}
            ]
            # Add more roles as needed
        }
        
        return role_abilities.get(champion.role, [])
    
    def _find_ability(self, abilities: List, ability_type: str) -> Optional[Dict]:
        """Find ability by type"""
        for ability in abilities:
            if ability.get('type', '').lower() == ability_type.lower():
                return ability
        return None
    
    def _get_ability_tips(self, champion: Champion) -> List[str]:
        """Get ability usage tips"""
        role_tips = {
            'Tank': [
                "Use your crowd control abilities to engage fights",
                "Save defensive abilities for protecting carries",
                "Your ultimate is best used to initiate team fights"
            ],
            'Fighter': [
                "Combo your abilities for maximum damage",
                "Use mobility abilities to stick to targets",
                "Time your defensive abilities to survive burst"
            ],
            'Assassin': [
                "Use abilities in quick succession for burst",
                "Save escape abilities for after eliminating targets",
                "Your ultimate is your primary assassination tool"
            ]
        }
        return role_tips.get(champion.role, [])
    
    def _calculate_popularity_tier(self, champion: Champion) -> str:
        """Calculate popularity tier based on difficulty and role"""
        if champion.difficulty <= 3:
            return "Popular"
        elif champion.difficulty >= 8:
            return "Niche"
        else:
            return "Moderate"
    
    def _calculate_skill_floor(self, champion: Champion) -> str:
        """Calculate skill floor (minimum skill to be effective)"""
        if champion.difficulty <= 3:
            return "Low"
        elif champion.difficulty <= 6:
            return "Medium"
        else:
            return "High"
    
    def _calculate_skill_ceiling(self, champion: Champion) -> str:
        """Calculate skill ceiling (maximum potential)"""
        if champion.difficulty <= 4:
            return "Medium"
        elif champion.difficulty <= 7:
            return "High"
        else:
            return "Very High"