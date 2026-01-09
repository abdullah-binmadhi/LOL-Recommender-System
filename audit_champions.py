import json
import sys
import os

REQUIRED_FIELDS = [
    "id", "name", "title", "role", "difficulty", "tags", 
    "attributes", "range_type", "position", "herotype"
]

REQUIRED_ATTRIBUTES = [
    "damage", "toughness", "control", "mobility", "utility", "difficulty"
]

NEW_FEATURES = [
    "damage_type", "power_spike", "skill_cap"
]

def audit_champions():
    file_path = 'src/data/champions.json'
    if not os.path.exists(file_path):
        print(f"Error: {file_path} not found.")
        sys.exit(1)

    with open(file_path, 'r') as f:
        data = json.load(f)

    champions = data.get('champions', [])
    total_champions = len(champions)
    errors = []
    missing_new_features = {feat: 0 for feat in NEW_FEATURES}

    print(f"Auditing {total_champions} champions...")

    for champion in champions:
        champ_name = champion.get('name', 'Unknown')
        
        # Check required fields
        for field in REQUIRED_FIELDS:
            if field not in champion:
                errors.append(f"Champion '{champ_name}' missing required field: '{field}'")
            elif champion[field] is None or (isinstance(champion[field], str) and not champion[field]):
                errors.append(f"Champion '{champ_name}' has empty value for field: '{field}'")

        # Check attributes
        attributes = champion.get('attributes')
        if attributes:
            for attr in REQUIRED_ATTRIBUTES:
                if attr not in attributes:
                    errors.append(f"Champion '{champ_name}' missing attribute: '{attr}'")
        else:
             errors.append(f"Champion '{champ_name}' missing 'attributes' object")

        # Check for new features (to quantify what needs to be added)
        for feat in NEW_FEATURES:
            if feat not in champion:
                missing_new_features[feat] += 1

    # Report results
    if errors:
        print("\nDATA INTEGRITY ERRORS FOUND:")
        for error in errors:
            print(f"  - {error}")
        print(f"\nTotal Errors: {len(errors)}")
    else:
        print("\nNo data integrity errors found in existing fields.")

    print("\nNEW FEATURE ANALYSIS:")
    for feat, count in missing_new_features.items():
        if count == total_champions:
             print(f"  - '{feat}' is missing for ALL champions (Implementation needed).")
        elif count > 0:
             print(f"  - '{feat}' is missing for {count}/{total_champions} champions.")
        else:
             print(f"  - '{feat}' is present for all champions.")

    if errors:
        sys.exit(1)
    else:
        sys.exit(0)

if __name__ == "__main__":
    audit_champions()
