//! Offline deterministic scoring, with a narrow JSON boundary for native/Wasm callers.
//! No browser, filesystem, networking, clocks, or JavaScript runtime in the library.
mod aggregation;
pub mod analytics;
pub mod player_analytics;
mod compat;
pub mod contracts;
mod decision_tree;
pub mod evaluation;
pub mod export;
mod features;
mod knn;
mod random_forest;
#[cfg(target_arch = "wasm32")]
mod wasm_abi;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogEntry {
    pub name: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Tree {
    pub id: usize,
    pub feature_indices: Vec<String>,
    pub bootstrap_champion_names: Vec<String>,
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Error {}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

/// Immutable reusable engine. Catalog array order is part of the ranking contract.
pub struct Engine {
    catalog: Vec<CatalogEntry>,
    forest: Vec<Tree>,
}

impl Engine {
    pub fn new(catalog: Vec<CatalogEntry>, forest: Vec<Tree>) -> Result<Self, Error> {
        if catalog.is_empty() || forest.is_empty() {
            return Err(Error("Catalog and forest must not be empty".into()));
        }
        let mut names = HashSet::new();
        for entry in &catalog {
            if entry.name.is_empty() || !names.insert(entry.name.as_str()) {
                return Err(Error("Catalog names must be nonempty and unique".into()));
            }
            if !entry.data["role"].is_string()
                || !entry.data["positions"]
                    .as_array()
                    .is_some_and(|a| a.iter().all(Value::is_string))
            {
                return Err(Error(format!("Invalid role/positions for {}", entry.name)));
            }
            validate_numbers(
                &entry.data,
                &[
                    "difficulty",
                    "damage",
                    "toughness",
                    "mobility",
                    "control",
                    "utility",
                ],
            )?;
            if let Some(derived) = entry.data.get("derived_stats") {
                validate_numbers(derived, &["burst", "sustain", "cc"])?;
            }
            for field in ["herotype", "title"] {
                if entry.data.get(field).is_some_and(|v| !v.is_string()) {
                    return Err(Error(format!("Invalid {field} for {}", entry.name)));
                }
            }
        }
        let mut ids = HashSet::new();
        for tree in &forest {
            let mut subset = HashSet::new();
            if !ids.insert(tree.id)
                || tree.feature_indices.is_empty()
                || tree.feature_indices.iter().any(|f| {
                    ![
                        "role",
                        "difficulty",
                        "damage",
                        "toughness",
                        "mobility",
                        "control",
                        "utility",
                    ]
                    .contains(&f.as_str())
                        || !subset.insert(f)
                })
                || tree
                    .bootstrap_champion_names
                    .iter()
                    .any(|name| !names.contains(name.as_str()))
            {
                return Err(Error(
                    "Invalid forest IDs, feature subsets, or bootstrap names".into(),
                ));
            }
        }
        Ok(Self { catalog, forest })
    }

    pub fn from_json(catalog: &str, forest: &str) -> Result<Self, Error> {
        Self::new(
            serde_json::from_str(catalog)?,
            serde_json::from_str(forest)?,
        )
    }

    /// Load the captured effective production catalog and actual forest choices. Reference
    /// profile scores remain test-only; runtime data is kept separate from parity fixtures.
    pub fn reference() -> Result<Self, Error> {
        let catalog = contracts::EffectiveCatalog::current()?.into_engine_entries()?;
        let forest = serde_json::from_str(include_str!("../data/forest.json"))?;
        Self::new(catalog, forest)
    }

    pub fn champion_count(&self) -> usize {
        self.catalog.len()
    }

    /// String/null answer values are the supported questionnaire boundary. Unknown keys
    /// and option strings are intentionally allowed, matching legacy default handling.
    pub fn recommend(&self, answers: &Value) -> Result<Value, Error> {
        let object = answers
            .as_object()
            .ok_or_else(|| Error("Answers must be a JSON object".into()))?;
        if object.values().any(|v| !v.is_string() && !v.is_null()) {
            return Err(Error("Answer values must be strings or null".into()));
        }
        let features = features::from_answers(answers);
        let mut result = self.score_features(&features)?;
        let top10 = result["top10"].as_array().expect("internal top10 array");
        let metrics = evaluation::evaluate(answers, &self.catalog, top10);
        result["evaluation"] = metrics;
        Ok(result)
    }

    /// Typed boundary for Rust callers; the JSON API remains available for the existing Wasm ABI.
    pub fn recommend_typed(
        &self,
        answers: &contracts::Answers,
    ) -> Result<contracts::RecommendationResult, Error> {
        let result = self.recommend(&answers.to_json()?)?;
        Ok(serde_json::from_value(result)?)
    }

    /// Score precomputed numeric features without pretending they are raw quiz answers.
    /// Relevance evaluation is deliberately only included by recommend().
    pub fn score_features(&self, features: &Value) -> Result<Value, Error> {
        validate_numbers(
            features,
            &[
                "difficulty",
                "damage",
                "toughness",
                "mobility",
                "control",
                "utility",
                "burst",
                "sustain",
                "cc",
            ],
        )?;
        for key in [
            "role",
            "position",
            "playstyle",
            "range",
            "pressure_response",
            "aesthetic_preference",
            "team_contribution",
            "character_identity",
            "problem_solving",
        ] {
            if !features[key].is_string() {
                return Err(Error(format!("Feature {key} must be a string")));
            }
        }
        let mut rf_scores = Map::new();
        let mut dt_scores = Map::new();
        let mut knn_scores = Map::new();
        let mut aggregated = Vec::with_capacity(self.catalog.len());
        for entry in &self.catalog {
            let rf = random_forest::score(features, &entry.data, &self.forest);
            let dt = decision_tree::score(features, &entry.data);
            let knn = knn::score(features, &entry.data);
            // Reject overflow rather than serialize non-finite scores silently as JSON null.
            for value in [&rf, &dt, &knn] {
                for key in ["score", "rawScore"] {
                    if !value[key].is_number() {
                        return Err(Error("Scoring produced a non-finite value".into()));
                    }
                }
            }
            aggregated.push(aggregation::aggregate(&entry.name, &rf, &dt, &knn));
            rf_scores.insert(entry.name.clone(), rf);
            dt_scores.insert(entry.name.clone(), dt);
            knn_scores.insert(entry.name.clone(), knn);
        }
        let top10 = aggregation::top10(&aggregated, &self.catalog, true);
        Ok(
            json!({"features": features, "scores": {"randomForest": rf_scores,
            "decisionTree": dt_scores, "knn": knn_scores}, "aggregated": aggregated, "top10": top10}),
        )
    }
}

/// Convert the sparse legacy question-ID map into the exact production feature contract.
pub fn convert_answers(answers: &contracts::Answers) -> Result<contracts::UserFeatures, Error> {
    let value = features::from_answers(&answers.to_json()?);
    Ok(serde_json::from_value(value)?)
}

fn validate_numbers(value: &Value, keys: &[&str]) -> Result<(), Error> {
    for &key in keys {
        // Bounds keep intermediate distance/variance computations finite while allowing
        // real scoring inputs and custom catalogs beyond the nominal feature scales.
        if !value[key]
            .as_f64()
            .is_some_and(|n| n.is_finite() && n.abs() <= 1_000_000.0)
        {
            return Err(Error(format!(
                "{key} must be a finite number within ±1000000"
            )));
        }
    }
    Ok(())
}
