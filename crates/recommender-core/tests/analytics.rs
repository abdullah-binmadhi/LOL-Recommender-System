use recommender_core::{
    analytics::{
        compute_analytics, difficulty_distribution, hero_type_distribution,
        range_type_distribution, release_year_distribution, resource_type_distribution,
    },
    CatalogEntry,
};
use serde_json::Value;

fn load_test_catalog() -> Vec<CatalogEntry> {
    let raw = include_str!("../../../src/data/champions.json");
    let val: Value = serde_json::from_str(raw).expect("valid champions json");
    let arr = if let Value::Array(items) = val {
        items
    } else if let Some(Value::Array(items)) = val.get("champions") {
        items.clone()
    } else {
        panic!("unexpected format");
    };

    arr.into_iter()
        .map(|data| {
            let name = data
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            CatalogEntry { name, data }
        })
        .collect()
}

#[test]
fn test_hero_type_distribution() {
    let catalog = load_test_catalog();
    assert_eq!(catalog.len(), 171);

    let hero_types = hero_type_distribution(&catalog);
    let total: usize = hero_types.iter().map(|s| s.count).sum();
    assert_eq!(total, 171);

    // Expected exact counts from dataset:
    // Fighter: 48, Mage: 37, Marksman: 28, Tank: 24, Assassin: 17, Support: 17
    let map: std::collections::HashMap<_, _> =
        hero_types.into_iter().map(|s| (s.label, s.count)).collect();
    assert_eq!(map.get("Fighter"), Some(&48));
    assert_eq!(map.get("Mage"), Some(&37));
    assert_eq!(map.get("Marksman"), Some(&28));
    assert_eq!(map.get("Tank"), Some(&24));
    assert_eq!(map.get("Assassin"), Some(&17));
    assert_eq!(map.get("Support"), Some(&17));
}

#[test]
fn test_range_type_distribution() {
    let catalog = load_test_catalog();
    let ranges = range_type_distribution(&catalog);
    let total: usize = ranges.iter().map(|s| s.count).sum();
    assert_eq!(total, 171);

    let map: std::collections::HashMap<_, _> =
        ranges.into_iter().map(|s| (s.label, s.count)).collect();
    assert_eq!(map.get("Melee"), Some(&90));
    assert_eq!(map.get("Ranged"), Some(&81));
}

#[test]
fn test_difficulty_distribution() {
    let catalog = load_test_catalog();
    let diffs = difficulty_distribution(&catalog);
    let total: usize = diffs.iter().map(|s| s.count).sum();
    assert_eq!(total, 171);

    assert_eq!(diffs[0].label, "Low (1)");
    assert_eq!(diffs[0].count, 51);
    assert_eq!(diffs[1].label, "Medium (2)");
    assert_eq!(diffs[1].count, 92);
    assert_eq!(diffs[2].label, "High (3)");
    assert_eq!(diffs[2].count, 28);
}

#[test]
fn test_resource_type_deterministic() {
    let catalog = load_test_catalog();
    let r1 = resource_type_distribution(&catalog);
    let r2 = resource_type_distribution(&catalog);
    assert_eq!(r1, r2, "deterministic allocation across runs");

    let total: usize = r1.iter().map(|s| s.count).sum();
    assert_eq!(total, 171);
    assert_eq!(r1.len(), 7);
}

#[test]
fn test_release_year_distribution() {
    let releases = release_year_distribution();
    assert_eq!(releases.len(), 18);
    assert_eq!(releases[0].label, "2009");
    assert_eq!(releases[0].count, 40);
    assert_eq!(releases[releases.len() - 1].label, "2026");
    assert_eq!(releases[releases.len() - 1].count, 1);
}

#[test]
fn test_compute_analytics_summary() {
    let catalog = load_test_catalog();
    let summary = compute_analytics(&catalog);
    assert_eq!(summary.total_champions, 171);
    assert_eq!(summary.hero_types.len(), 6);
    assert_eq!(summary.range_types.len(), 2);
    assert_eq!(summary.difficulties.len(), 3);
    assert_eq!(summary.resource_types.len(), 7);
    assert_eq!(summary.release_years.len(), 18);
    assert!(!summary.resource_disclosure.is_empty());
}
