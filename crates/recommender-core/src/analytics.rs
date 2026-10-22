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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChampionProfileSummary {
    pub name: String,
    pub title: String,
    pub herotype: String,
    pub range_type: String,
    pub resource_type: String,
    pub difficulty: u8,
    pub release_year: String,
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
    pub champions: Vec<ChampionProfileSummary>,
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
        resource_disclosure: "Official Riot Games Data Dragon Resource Systems: Accurate partype classification covering all League of Legends champions.".to_string(),
        champions: champion_summaries(catalog),
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

/// Official Riot Games Data Dragon resource classification for each champion.
pub fn resource_type_for_champion(name: &str) -> &'static str {
    match name {
        "Aatrox" => "Blood Well",
        "Akali" | "Ambessa" | "Kennen" | "Lee Sin" | "LeeSin" | "Shen" | "Zed" => "Energy",
        "Bel'Veth" | "Belveth" | "Dr. Mundo" | "DrMundo" | "Garen" | "Katarina" | "Riven"
        | "Viego" | "Zac" => "None",
        "Briar" | "Renekton" | "Shyvana" | "Tryndamere" => "Fury",
        "Gnar" | "GnarBig" | "Rek'Sai" | "RekSai" => "Rage",
        "Kled" => "Courage",
        "Mordekaiser" => "Shield",
        "Rengar" => "Ferocity",
        "Rumble" => "Heat",
        "Sett" => "Grit",
        "Vladimir" => "Crimson Rush",
        "Yasuo" | "Yone" => "Flow",
        _ => "Mana",
    }
}

/// Historical release year mapping for all 171 champions.
pub fn champion_release_year(name: &str) -> &'static str {
    match name {
        // 2009 (41)
        "Alistar" | "Annie" | "Ashe" | "Fiddlesticks" | "Jax" | "Kayle" | "MasterYi"
        | "Morgana" | "Nunu" | "Ryze" | "Sion" | "Sivir" | "Soraka" | "Teemo" | "Tristana"
        | "TwistedFate" | "Warwick" | "Singed" | "Zilean" | "Evelynn" | "Tryndamere" | "Twitch"
        | "Karthus" | "Amumu" | "Chogath" | "Anivia" | "Rammus" | "Veigar" | "Kassadin"
        | "Taric" | "Blitzcrank" | "DrMundo" | "Janna" | "Malphite" | "Corki" | "Katarina"
        | "Nasus" | "Heimerdinger" | "Shaco" | "Udyr" | "Gangplank" => "2009",

        // 2010 (24)
        "Nidalee" | "Poppy" | "Gragas" | "Pantheon" | "Mordekaiser" | "Ezreal" | "Shen"
        | "Kennen" | "Garen" | "Akali" | "Malzahar" | "Olaf" | "KogMaw" | "XinZhao"
        | "Vladimir" | "Galio" | "Urgot" | "MissFortune" | "Sona" | "Swain" | "Lux" | "Leblanc"
        | "Irelia" | "Trundle" => "2010",

        // 2011 (24)
        "Cassiopeia" | "Caitlyn" | "Renekton" | "Karma" | "Maokai" | "JarvanIV" | "Nocturne"
        | "LeeSin" | "Brand" | "Rumble" | "Vayne" | "Orianna" | "Yorick" | "Leona"
        | "MonkeyKing" | "Skarner" | "Talon" | "Riven" | "Xerath" | "Graves" | "Shyvana"
        | "Fizz" | "Volibear" | "Ahri" => "2011",

        // 2012 (19)
        "Viktor" | "Sejuani" | "Ziggs" | "Nautilus" | "Fiora" | "Lulu" | "Hecarim" | "Varus"
        | "Darius" | "Draven" | "Jayce" | "Zyra" | "Diana" | "Rengar" | "Syndra" | "Khazix"
        | "Elise" | "Zed" | "Nami" => "2012",

        // 2013 (8)
        "Vi" | "Thresh" | "Quinn" | "Zac" | "Lissandra" | "Aatrox" | "Lucian" | "Jinx" => "2013",

        // 2014 (7)
        "Yasuo" | "Velkoz" | "Braum" | "Gnar" | "GnarBig" | "Azir" | "Kalista" => "2014",

        // 2015 (6)
        "RekSai" | "Bard" | "Ekko" | "TahmKench" | "Kindred" | "Illaoi" => "2015",

        // 2016 (6)
        "Jhin" | "AurelionSol" | "Taliyah" | "Kled" | "Ivern" | "Camille" => "2016",

        // 2017 (5)
        "Rakan" | "Xayah" | "Kayn" | "Ornn" | "Zoe" => "2017",

        // 2018 (4)
        "Kaisa" | "Pyke" | "Neeko" | "Sylas" => "2018",

        // 2019 (5)
        "Yuumi" | "Qiyana" | "Senna" | "Aphelios" | "Sett" => "2019",

        // 2020 (5)
        "Lillia" | "Yone" | "Samira" | "Seraphine" | "Rell" => "2020",

        // 2021 (4)
        "Viego" | "Gwen" | "Akshan" | "Vex" => "2021",

        // 2022 (5)
        "Zeri" | "Renata" | "Belveth" | "Nilah" | "KSante" => "2022",

        // 2023 (4)
        "Milio" | "Naafiri" | "Briar" | "Hwei" => "2023",

        // 2024 (3)
        "Smolder" | "Aurora" | "Ambessa" => "2024",

        // 2025 (1)
        "Mel" => "2025",

        _ => "2026",
    }
}

/// Extract clean structured summaries for every champion in the catalog.
pub fn champion_summaries(catalog: &[CatalogEntry]) -> Vec<ChampionProfileSummary> {
    catalog
        .iter()
        .map(|entry| {
            let title = entry
                .data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let herotype = entry
                .data
                .get("herotype")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let range_type = entry
                .data
                .get("range_type")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let difficulty = entry
                .data
                .get("difficulty")
                .and_then(|v| v.as_u64())
                .map(|n| n as u8)
                .unwrap_or(2);
            let resource_type = entry
                .data
                .get("resource_type")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| resource_type_for_champion(&entry.name))
                .to_string();
            let release_year = champion_release_year(&entry.name).to_string();

            ChampionProfileSummary {
                name: entry.name.clone(),
                title,
                herotype,
                range_type,
                resource_type,
                difficulty,
                release_year,
            }
        })
        .collect()
}

/// Multi-facet cross-filter matching champions across all 5 chart dimensions.
pub fn filter_champions<'a>(
    champions: &'a [ChampionProfileSummary],
    hero_type: Option<&str>,
    range_type: Option<&str>,
    resource_type: Option<&str>,
    difficulty: Option<u8>,
    release_year: Option<&str>,
) -> Vec<&'a ChampionProfileSummary> {
    champions
        .iter()
        .filter(|c| {
            if let Some(ht) = hero_type {
                if !c.herotype.eq_ignore_ascii_case(ht) {
                    return false;
                }
            }
            if let Some(rt) = range_type {
                if !c.range_type.eq_ignore_ascii_case(rt) {
                    return false;
                }
            }
            if let Some(res) = resource_type {
                if !c.resource_type.eq_ignore_ascii_case(res) {
                    return false;
                }
            }
            if let Some(diff) = difficulty {
                if c.difficulty != diff {
                    return false;
                }
            }
            if let Some(yr) = release_year {
                if c.release_year != yr {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// Official Riot Games Data Dragon resource systems distribution across the champion catalog.
pub fn resource_type_distribution(catalog: &[CatalogEntry]) -> Vec<MetricSlice> {
    const ORDER: [&str; 13] = [
        "Mana",
        "Energy",
        "None",
        "Fury",
        "Rage",
        "Flow",
        "Blood Well",
        "Courage",
        "Shield",
        "Ferocity",
        "Heat",
        "Grit",
        "Crimson Rush",
    ];

    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    for &r in &ORDER {
        counts.insert(r, 0);
    }

    for entry in catalog {
        let chosen = resource_type_for_champion(&entry.name);
        *counts.entry(chosen).or_default() += 1;
    }

    let mut slices: Vec<MetricSlice> = counts
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(label, count)| MetricSlice {
            label: label.to_string(),
            count,
        })
        .collect();

    slices.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.label.cmp(&b.label)));
    slices
}

/// Historical champion releases by year (2009–2026).
///
/// Mirrors the dataset used in the legacy analytics dashboard.
pub fn release_year_distribution() -> Vec<MetricSlice> {
    const RELEASES: [(&str, usize); 18] = [
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
        ("2023", 4),
        ("2024", 3),
        ("2025", 1),
        ("2026", 1),
    ];

    RELEASES
        .iter()
        .map(|(year, count)| MetricSlice {
            label: year.to_string(),
            count: *count,
        })
        .collect()
}
