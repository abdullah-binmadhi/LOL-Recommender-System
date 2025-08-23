import os
from typing import Dict, List, Any
from services.question_service import QuestionService
from services.champion_service import ChampionService

class DataValidator:
    """Utility class for validating all data sources"""
    
    def __init__(self, data_folder: str = "data"):
        self.data_folder = data_folder
        self.question_service = QuestionService(data_folder)
        self.champion_service = ChampionService(data_folder)
    
    def validate_all_data(self) -> Dict[str, Any]:
        """Validate all data sources and return validation results"""
        results = {
            "questions": {"valid": False, "errors": [], "count": 0},
            "champions": {"valid": False, "errors": [], "count": 0},
            "overall": {"valid": False, "errors": []}
        }
        
        # Validate questions
        try:
            questions_valid = self.question_service.validate_questions_file()
            if questions_valid:
                questions = self.question_service.get_all_questions()
                results["questions"]["valid"] = True
                results["questions"]["count"] = len(questions)
            else:
                results["questions"]["errors"].append("Questions validation failed")
        except Exception as e:
            results["questions"]["errors"].append(str(e))
        
        # Validate champions
        try:
            champions_valid = self.champion_service.validate_champions_data()
            if champions_valid:
                champions = self.champion_service.get_all_champions()
                results["champions"]["valid"] = True
                results["champions"]["count"] = len(champions)
            else:
                results["champions"]["errors"].append("Champions validation failed")
        except Exception as e:
            results["champions"]["errors"].append(str(e))
        
        # Overall validation
        if results["questions"]["valid"] and results["champions"]["valid"]:
            results["overall"]["valid"] = True
        else:
            if not results["questions"]["valid"]:
                results["overall"]["errors"].extend(results["questions"]["errors"])
            if not results["champions"]["valid"]:
                results["overall"]["errors"].extend(results["champions"]["errors"])
        
        return results
    
    def print_validation_report(self) -> None:
        """Print a detailed validation report"""
        results = self.validate_all_data()
        
        print("=" * 50)
        print("DATA VALIDATION REPORT")
        print("=" * 50)
        
        # Questions report
        print(f"\nQUESTIONS: {'✅ VALID' if results['questions']['valid'] else '❌ INVALID'}")
        if results['questions']['valid']:
            print(f"  - Loaded {results['questions']['count']} questions successfully")
        else:
            print("  - Errors:")
            for error in results['questions']['errors']:
                print(f"    • {error}")
        
        # Champions report
        print(f"\nCHAMPIONS: {'✅ VALID' if results['champions']['valid'] else '❌ INVALID'}")
        if results['champions']['valid']:
            print(f"  - Loaded {results['champions']['count']} champions successfully")
        else:
            print("  - Errors:")
            for error in results['champions']['errors']:
                print(f"    • {error}")
        
        # Overall report
        print(f"\nOVERALL: {'✅ READY' if results['overall']['valid'] else '❌ NOT READY'}")
        if not results['overall']['valid']:
            print("  - Fix the above errors before proceeding")
        
        print("=" * 50)
        
        return results['overall']['valid']