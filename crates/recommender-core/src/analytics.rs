//! Analytics calculations for League of Legends champion data.
//!
//! Provides deterministic aggregations of champion attributes, combat range,
//! difficulty distributions, historical release statistics, and clearly disclosed
//! illustrative resource types.

use crate::CatalogEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricSlice {
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DifficultyMetric {
    pub level: u8,
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalyticsSummary {
    pub total_champions: usize,
    pub hero_types: Vec<MetricSlice>,
    pub range_types: Vec<MetricSlice>,
    pub difficulties: Vec<DifficultyMetric>,
    pub resource_types: Vec<MetricSlice>,
    pub release_years: Vec<MetricSlice>,
    pub resource_disclosure: String,
}

/// Compute complete analytics summary from the loaded catalog.
pub fn compute_analytics(catalog: &[CatalogEntry]) -> AnalyticsSummary {
    AnalyticsSummary {
        total_champions: catalog.len(),
        hero_types: hero_type_distribution(catalog),
        range_types: range_type_distribution(catalog),
        difficulties: difficulty_distribution(catalog),
        resource_types: resource_type_distribution(catalog),
        release_years: release_year_distribution(),
        resource_disclosure: "Illustrative Distribution: The champion dataset does not specify resource types; this distribution is generated deterministically for visualization demonstration.".to_string(),
    }
}

/// Load the production champions dataset used by the legacy analytics dashboard.
pub fn default_analytics_catalog() -> Vec<CatalogEntry> {
    let raw = include_str!("../../../src/data/champions.json");
    let val: serde_json::Value = serde_json::from_str(raw).expect("valid champions json");
    let arr = if let serde_json::Value::Array(items) = val {
        items
    } else if let Some(serde_json::Value::Array(items)) = val.get("champions") {
        items.clone()
    } else {
        vec![]
    };

    arr.into_iter()
        .map(|data| {
            let name = data
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string();
            CatalogEntry { name, data }
        })
        .collect()
}

/// Compute analytics summary from the bundled production champions dataset.
pub fn default_analytics_summary() -> AnalyticsSummary {
    compute_analytics(&default_analytics_catalog())
}

/// Calculate distribution of champions per hero type.
pub fn hero_type_distribution(catalog: &[CatalogEntry]) -> Vec<MetricSlice> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in catalog {
        let hero_type = entry
            .data
            .get("herotype")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        *counts.entry(hero_type.to_string()).or_default() += 1;
    }

    let mut slices: Vec<MetricSlice> = counts
        .into_iter()
        .map(|(label, count)| MetricSlice { label, count })
        .collect();

    // Sort descending by count, then alphabetically
    slices.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.label.cmp(&b.label)));
    slices
}

/// Calculate combat range balance (Melee vs Ranged).
pub fn range_type_distribution(catalog: &[CatalogEntry]) -> Vec<MetricSlice> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in catalog {
        let range_type = entry
            .data
            .get("range_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        *counts.entry(range_type.to_string()).or_default() += 1;
    }

    let mut slices: Vec<MetricSlice> = counts
        .into_iter()
        .map(|(label, count)| MetricSlice { label, count })
        .collect();

    slices.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.label.cmp(&b.label)));
    slices
}

/// Calculate champion difficulty distribution (1: Low, 2: Medium, 3: High).
pub fn difficulty_distribution(catalog: &[CatalogEntry]) -> Vec<DifficultyMetric> {
    let mut counts: BTreeMap<u8, usize> = BTreeMap::new();
    // Pre-populate 1, 2, 3 so all levels exist
    counts.insert(1, 0);
    counts.insert(2, 0);
    counts.insert(3, 0);

    for entry in catalog {
        let diff = entry
            .data
            .get("difficulty")
            .and_then(|v| v.as_u64())
            .map(|n| n as u8)
            .unwrap_or(2);
        *counts.entry(diff).or_default() += 1;
    }

    vec![
        DifficultyMetric {
            level: 1,
            label: "Low (1)".to_string(),
            count: counts.get(&1).copied().unwrap_or(0),
        },
        DifficultyMetric {
            level: 2,
            label: "Medium (2)".to_string(),
            count: counts.get(&2).copied().unwrap_or(0),
        },
        DifficultyMetric {
            level: 3,
            label: "High (3)".to_string(),
            count: counts.get(&3).copied().unwrap_or(0),
        },
    ]
}

/// Deterministic assignment of champions to resource categories.
///
/// NOTE: The legacy JS code picked randomly from
/// `['Mana', 'Fury', 'Energy', 'Ferocity', 'Rage', 'Heat', 'Other']` on every page load.
/// To preserve deterministic behavior in Rust, this function assigns champions
/// via a deterministic polynomial hash of their name.
pub fn resource_type_distribution(catalog: &[CatalogEntry]) -> Vec<MetricSlice> {
    const RESOURCES: [&str; 7] = [
        "Mana", "Fury", "Energy", "Ferocity", "Rage", "Heat", "Other",
    ];

    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    for r in &RESOURCES {
        counts.insert(*r, 0);
    }

    for entry in catalog {
        // Deterministic hash based on champion name
        let mut hash: usize = 0;
        for b in entry.name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as usize);
        }
        let chosen = RESOURCES[hash % RESOURCES.len()];
        *counts.entry(chosen).or_default() += 1;
    }

    RESOURCES
        .iter()
        .map(|&label| MetricSlice {
            label: label.to_string(),
            count: counts.get(label).copied().unwrap_or(0),
        })
        .collect()
}

/// Historical champion releases by year (2009–2024).
///
/// Mirrors the dataset used in the legacy analytics dashboard.
pub fn release_year_distribution() -> Vec<MetricSlice> {
    const RELEASES: [(&str, usize); 16] = [
        ("2009", 40),
        ("2010", 24),
        ("2011", 24),
        ("2012", 19),
        ("2013", 6),
        ("2014", 6),
        ("2015", 6),
        ("2016", 6),
        ("2017", 6),
        ("2018", 4),
        ("2019", 6),
        ("2020", 5),
        ("2021", 4),
        ("2022", 6),
        ("2023", 5),
        ("2024", 3),
    ];

    RELEASES
        .iter()
        .map(|(year, count)| MetricSlice {
            label: year.to_string(),
            count: *count,
        })
        .collect()
}
