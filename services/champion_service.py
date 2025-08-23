import pandas as pd
import json
import os
from typing import List, Optional, Dict, Any
from models.champion import Champion, Ability
from utils.cache_manager import cached, timed, cache_manager

class ChampionService:
    def __init__(self, data_folder: str = "data"):
        self.data_folder = data_folder
        self.excel_file = os.path.join(os.path.dirname(data_folder), "LoL_champion_data.xlsx")
        self.json_file = os.path.join(data_folder, "champions.json")
        self._champions = None
    
    def _convert_excel_to_json(self) -> None:
        """Convert Excel file to JSON format for easier processing"""
        if not os.path.exists(self.excel_file):
            raise FileNotFoundError(f"Excel file not found: {self.excel_file}")
        
        try:
            # Read Excel file
            df = pd.read_excel(self.excel_file)
            
            # Convert DataFrame to list of dictionaries
            champions_data = []
            seen_names = set()
            for _, row in df.iterrows():
                # Skip rows with empty or invalid names
                name = str(row.get('Champion_name', '')).strip()
                if not name or name == 'nan' or pd.isna(row.get('Champion_name')):
                    continue
                
                # Skip duplicates
                if name in seen_names:
                    continue
                seen_names.add(name)
                
                # Clean up role data
                role = str(row.get('role', 'Fighter')).strip()
                if not role or role == 'nan':
                    role = 'Fighter'
                
                # Ensure role is valid
                valid_roles = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support']
                if role not in valid_roles:
                    # Map common variations
                    role_mapping = {
                        'ADC': 'Marksman',
                        'Carry': 'Marksman',
                        'Jungler': 'Fighter',
                        'Mid': 'Mage',
                        'Top': 'Fighter',
                        'Slayer': 'Assassin',
                        'Specialist': 'Mage',
                        'Controller': 'Support'
                    }
                    role = role_mapping.get(role, 'Fighter')
                
                champion_data = {
                    "id": name.lower().replace(' ', '').replace("'", "").replace('.', ''),
                    "name": name,
                    "title": str(row.get('title', 'The Champion')).strip(),
                    "role": role,
                    "difficulty": int(row.get('difficulty', 5)) if pd.notna(row.get('difficulty')) else 5,
                    "tags": [role],  # Use role as primary tag
                    "attributes": {
                        "damage": float(row.get('damage', 5)) if pd.notna(row.get('damage')) else 5,
                        "toughness": float(row.get('toughness', 5)) if pd.notna(row.get('toughness')) else 5,
                        "control": float(row.get('control', 5)) if pd.notna(row.get('control')) else 5,
                        "mobility": float(row.get('mobility', 5)) if pd.notna(row.get('mobility')) else 5,
                        "utility": float(row.get('utility', 5)) if pd.notna(row.get('utility')) else 5,
                        "difficulty": float(row.get('difficulty', 5)) if pd.notna(row.get('difficulty')) else 5
                    },
                    "range_type": str(row.get('rangetype', 'Melee')).strip(),
                    "position": str(row.get('positions', 'Any')).strip(),
                    "herotype": str(row.get('herotype', 'Balanced')).strip()
                }
                champions_data.append(champion_data)
            
            # Save to JSON file
            os.makedirs(self.data_folder, exist_ok=True)
            with open(self.json_file, 'w', encoding='utf-8') as file:
                json.dump({"champions": champions_data}, file, indent=2, ensure_ascii=False)
            
            print(f"Successfully converted {len(champions_data)} champions from Excel to JSON")
            
        except Exception as e:
            raise RuntimeError(f"Error converting Excel to JSON: {e}")
    
    def _load_champions(self) -> List[Champion]:
        """Load champions from JSON file, converting from Excel if needed"""
        # Convert Excel to JSON if JSON doesn't exist
        if not os.path.exists(self.json_file):
            self._convert_excel_to_json()
        
        try:
            with open(self.json_file, 'r', encoding='utf-8') as file:
                data = json.load(file)
            
            champions = []
            for c_data in data.get('champions', []):
                # Create basic champion without abilities for now
                champion = Champion(
                    id=c_data['id'],
                    name=c_data['name'],
                    title=c_data.get('title', ''),
                    role=c_data['role'],
                    difficulty=c_data['difficulty'],
                    tags=c_data.get('tags', [c_data['role']]),
                    attributes=c_data.get('attributes', {}),
                    abilities=[]  # Will be populated later if needed
                )
                # Add custom attributes for ML processing
                champion.range_type = c_data.get('range_type', 'Melee')
                champion.position = c_data.get('position', 'Any')
                
                champions.append(champion)
            
            return champions
            
        except json.JSONDecodeError as e:
            raise ValueError(f"Invalid JSON in champions file: {e}")
        except Exception as e:
            raise RuntimeError(f"Error loading champions: {e}")
    
    @cached(ttl=7200, namespace="champions", disk_cache=True)  # Cache for 2 hours
    @timed("champion_data_loading")
    def get_all_champions(self) -> List[Champion]:
        """Get all champions, loading them if not already loaded"""
        if self._champions is None:
            self._champions = self._load_champions()
        return self._champions
    
    @cached(ttl=3600, namespace="champion_lookup")
    @timed("champion_lookup")
    def get_champion_by_name(self, name: str) -> Optional[Champion]:
        """Get a champion by name (case insensitive)"""
        champions = self.get_all_champions()
        name_lower = name.lower()
        for champion in champions:
            if champion.name.lower() == name_lower:
                return champion
        return None
    
    def get_champion_by_id(self, champion_id: str) -> Optional[Champion]:
        """Get a champion by ID"""
        champions = self.get_all_champions()
        for champion in champions:
            if champion.id == champion_id:
                return champion
        return None
    
    @cached(ttl=3600, namespace="champion_role")
    @timed("champion_role_filter")
    def get_champions_by_role(self, role: str) -> List[Champion]:
        """Get all champions with a specific role"""
        champions = self.get_all_champions()
        return [c for c in champions if c.role.lower() == role.lower()]
    
    def get_champions_by_difficulty(self, min_diff: int, max_diff: int) -> List[Champion]:
        """Get champions within a difficulty range"""
        champions = self.get_all_champions()
        return [c for c in champions if min_diff <= c.difficulty <= max_diff]
    
    def get_champions_by_range_type(self, range_type: str) -> List[Champion]:
        """Get champions by range type (Ranged/Melee)"""
        champions = self.get_all_champions()
        return [c for c in champions if hasattr(c, 'range_type') and c.range_type.lower() == range_type.lower()]
    
    def validate_champions_data(self) -> bool:
        """Validate that champions data is properly loaded"""
        try:
            champions = self.get_all_champions()
            
            if len(champions) == 0:
                raise ValueError("No champions loaded")
            
            # Check for duplicate names
            names = [c.name for c in champions]
            if len(names) != len(set(names)):
                raise ValueError("Duplicate champion names found")
            
            # Validate each champion
            for champion in champions:
                champion.validate()
            
            print(f"Successfully validated {len(champions)} champions")
            return True
            
        except Exception as e:
            print(f"Champions validation failed: {e}")
            return False