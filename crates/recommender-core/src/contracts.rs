use crate::{CatalogEntry, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

/// Ordered, typed view of the captured effective catalog. Extra catalog fields are retained
/// verbatim so this model can be used without enriching or normalizing legacy champion data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChampionRecord {
    pub name: String,
    pub data: Champion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Champion {
    pub role: String,
    pub positions: Vec<String>,
    pub difficulty: f64,
    pub damage: f64,
    pub toughness: f64,
    pub mobility: f64,
    pub control: f64,
    pub utility: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_stats: Option<Value>,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct EffectiveCatalog(pub Vec<ChampionRecord>);

impl EffectiveCatalog {
    pub fn from_json(source: &str) -> Result<Self, Error> {
        let catalog: Self = serde_json::from_str(source)?;
        let mut names = HashSet::with_capacity(catalog.0.len());
        if catalog.0.is_empty()
            || catalog
                .0
                .iter()
                .any(|champion| champion.name.is_empty() || !names.insert(champion.name.as_str()))
        {
            return Err(Error::new(
                "Effective catalog names must be nonempty and unique",
            ));
        }
        Ok(catalog)
    }

    /// This is the effective inline production catalog, extracted once with its JS object order
    /// and duplicate-key overwrite behavior. It is production data, not an expected-score fixture.
    pub fn current() -> Result<Self, Error> {
        Self::from_json(include_str!("../data/catalog.json"))
    }

    /// Modern catalog containing the complete League of Legends roster (168+ champions)
    /// up to 2026, including Ambessa, Aurora, Naafiri, Milio, K'Sante, and Renata Glasc.
    pub fn modern() -> Result<Self, Error> {
        Self::from_json(include_str!("../data/catalog_modern.json"))
    }

    pub fn into_engine_entries(self) -> Result<Vec<CatalogEntry>, Error> {
        self.0
            .into_iter()
            .map(|champion| {
                Ok(CatalogEntry {
                    name: champion.name,
                    data: serde_json::to_value(champion.data)?,
                })
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionnaireDefinition {
    pub questions: Vec<QuestionDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionDefinition {
    pub id: u32,
    pub text: String,
    #[serde(rename = "type")]
    pub question_type: String,
    pub options: Vec<String>,
    pub weight: f64,
    pub feature_mapping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EffectiveQuestion {
    pub id: u32,
    pub text: String,
    pub options: Vec<QuestionOption>,
    pub dimension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionOption {
    pub value: String,
    pub text: String,
    pub description: String,
}

impl QuestionnaireDefinition {
    pub fn from_json(source: &str) -> Result<Self, Error> {
        let questionnaire: Self = serde_json::from_str(source)?;
        let mut ids = HashSet::with_capacity(questionnaire.questions.len());
        if questionnaire.questions.is_empty()
            || questionnaire.questions.iter().any(|question| {
                question.id == 0
                    || question.text.is_empty()
                    || question.options.is_empty()
                    || !ids.insert(question.id)
            })
        {
            return Err(Error::new(
                "Questionnaire IDs, text, and options must be valid",
            ));
        }
        Ok(questionnaire)
    }

    /// Read the historical questionnaire unchanged; its JSON remains the canonical source during
    /// parity checks against legacy migration fixtures.
    pub fn current() -> Result<Self, Error> {
        Self::from_json(include_str!("../data/questions_legacy.json"))
    }

    /// Modern 25-question comprehensive assessment across 5 tactical stages.
    pub fn modern() -> Result<Self, Error> {
        Self::from_json(include_str!("../../../src/data/questions.json"))
    }

    /// Match the production successful-load projection used by the current quiz UI.
    pub fn effective_questions(&self) -> Vec<EffectiveQuestion> {
        self.questions
            .iter()
            .map(|question| EffectiveQuestion {
                id: question.id,
                text: question.text.clone(),
                options: question
                    .options
                    .iter()
                    .map(|option| QuestionOption {
                        value: option.clone(),
                        text: option.clone(),
                        description: String::new(),
                    })
                    .collect(),
                dimension: question.dimension.clone().unwrap_or_default(),
            })
            .collect()
    }
}

/// Sparse answer object keyed by the legacy question IDs. Unknown IDs and null values remain
/// representable so saved/historical answer objects do not need to be rewritten.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Answers(pub BTreeMap<String, Option<String>>);

impl Answers {
    pub fn from_choices(choices: BTreeMap<String, String>) -> Self {
        Self(
            choices
                .into_iter()
                .map(|(id, value)| (id, Some(value)))
                .collect(),
        )
    }

    pub fn insert(&mut self, question_id: impl Into<String>, answer: impl Into<String>) {
        self.0.insert(question_id.into(), Some(answer.into()));
    }

    pub fn to_json(&self) -> Result<Value, Error> {
        Ok(serde_json::to_value(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserFeatures {
    pub role: String,
    pub position: String,
    pub difficulty: f64,
    pub damage: f64,
    pub toughness: f64,
    pub mobility: f64,
    pub control: f64,
    pub utility: f64,
    pub playstyle: String,
    pub range: String,
    pub pressure_response: String,
    pub aesthetic_preference: String,
    pub team_contribution: String,
    pub character_identity: String,
    pub problem_solving: String,
    pub burst: f64,
    pub sustain: f64,
    pub cc: f64,
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlgorithmScore {
    pub score: f64,
    pub raw_score: f64,
    pub details: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncertainty: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlgorithmScores {
    pub random_forest: BTreeMap<String, AlgorithmScore>,
    pub decision_tree: BTreeMap<String, AlgorithmScore>,
    pub knn: BTreeMap<String, AlgorithmScore>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub naive_bayes: BTreeMap<String, AlgorithmScore>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub cosine_similarity: BTreeMap<String, AlgorithmScore>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub gbdt: BTreeMap<String, AlgorithmScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedChampionScore {
    pub champion_name: String,
    pub random_forest: f64,
    pub decision_tree: f64,
    pub knn: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub naive_bayes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cosine_similarity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gbdt: Option<f64>,
    pub average: f64,
    pub weighted: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationResult {
    pub relevant_champions: Vec<String>,
    pub precision_at1: f64,
    pub precision_at3: f64,
    pub precision_at10: f64,
    pub recall_at10: f64,
    pub f1_at10: f64,
    pub mrr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecommendationResult {
    pub features: UserFeatures,
    pub scores: AlgorithmScores,
    pub aggregated: Vec<AggregatedChampionScore>,
    pub top10: Vec<AggregatedChampionScore>,
    #[serde(default)]
    pub evaluation: Option<EvaluationResult>,
}

/// Local records have changed shape over time. Known registration/result fields are typed,
/// unknown fields survive round-tripping, and age/userAnswers retain historical JSON types.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StoredUserRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_champion: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_answers: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Engine;
    use serde_json::json;

    fn assert_json_close(actual: &Value, expected: &Value, path: &str) {
        if let (Some(actual), Some(expected)) = (actual.as_f64(), expected.as_f64()) {
            assert!(
                (actual - expected).abs() <= 1e-9,
                "{path}: {actual} != {expected}"
            );
        } else if let (Some(actual), Some(expected)) = (actual.as_array(), expected.as_array()) {
            assert_eq!(actual.len(), expected.len(), "{path}: array length");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_json_close(actual, expected, &format!("{path}[{index}]"));
            }
        } else if let (Some(actual), Some(expected)) = (actual.as_object(), expected.as_object()) {
            let mut actual_keys: Vec<_> = actual.keys().collect();
            let mut expected_keys: Vec<_> = expected.keys().collect();
            actual_keys.sort_unstable();
            expected_keys.sort_unstable();
            assert_eq!(actual_keys, expected_keys, "{path}: object keys");
            for key in expected_keys {
                assert_json_close(&actual[key], &expected[key], &format!("{path}.{key}"));
            }
        } else {
            assert_eq!(actual, expected, "{path}");
        }
    }

    #[test]
    fn questionnaire_projection_matches_the_captured_production_projection() {
        let questionnaire = QuestionnaireDefinition::current().unwrap();
        let actual = questionnaire.effective_questions();
        let expected: Vec<EffectiveQuestion> = serde_json::from_str(include_str!(
            "../../../tests/fixtures/migration/questions.json"
        ))
        .unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 12);
    }

    #[test]
    fn modern_questionnaire_contains_all_25_questions_across_5_stages() {
        let questionnaire = QuestionnaireDefinition::modern().unwrap();
        let actual = questionnaire.effective_questions();
        assert_eq!(actual.len(), 25);
        assert_eq!(actual[0].id, 1);
        assert_eq!(actual[24].id, 25);
    }

    #[test]
    fn typed_effective_catalog_keeps_catalog_order_and_values() {
        let catalog = EffectiveCatalog::current().unwrap();
        let expected: Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/migration/catalog.json"
        ))
        .unwrap();
        assert_json_close(
            &serde_json::to_value(&catalog).unwrap(),
            &expected,
            "$catalog",
        );
        assert_eq!(catalog.len(), 162);
    }

    #[test]
    fn modern_catalog_contains_all_172_champions_including_new_additions() {
        let catalog = EffectiveCatalog::modern().unwrap();
        assert_eq!(catalog.len(), 172);
        assert!(catalog.0.iter().any(|c| c.name == "Ambessa"));
        assert!(catalog.0.iter().any(|c| c.name == "Aurora"));
        assert!(catalog.0.iter().any(|c| c.name == "Naafiri"));
        assert!(catalog.0.iter().any(|c| c.name == "Milio"));
        assert!(catalog.0.iter().any(|c| c.name == "K'Sante"));
        assert!(catalog.0.iter().any(|c| c.name == "Renata Glasc"));
        assert!(catalog.0.iter().any(|c| c.name == "Locke"));
        assert!(catalog.0.iter().any(|c| c.name == "Zaahen"));
        assert!(catalog.0.iter().any(|c| c.name == "Mel"));
        assert!(catalog.0.iter().any(|c| c.name == "Yunara"));
    }

    #[test]
    fn answer_conversion_matches_every_reference_profile() {
        let profiles = [
            include_str!("../../../tests/fixtures/migration/profile-mage-mid.json"),
            include_str!("../../../tests/fixtures/migration/profile-marksman-bot.json"),
            include_str!("../../../tests/fixtures/migration/profile-tank-top.json"),
            include_str!("../../../tests/fixtures/migration/profile-support-team.json"),
            include_str!("../../../tests/fixtures/migration/profile-empty-defaults.json"),
            include_str!("../../../tests/fixtures/migration/profile-unknown-values.json"),
            include_str!("../../../tests/fixtures/migration/profile-no-preference-adc.json"),
            include_str!("../../../tests/fixtures/migration/profile-legacy-psychological-10.json"),
        ];
        for profile in profiles {
            let profile: Value = serde_json::from_str(profile).unwrap();
            let answers: Answers = serde_json::from_value(profile["answers"].clone()).unwrap();
            let features = crate::convert_answers(&answers).unwrap();
            assert_json_close(
                &serde_json::to_value(features).unwrap(),
                &profile["features"],
                "$features",
            );
        }
    }

    #[test]
    fn recommendation_result_has_a_typed_lossless_json_boundary() {
        let engine = Engine::reference().unwrap();
        let answers = Answers::from_choices(BTreeMap::from([
            ("1".into(), "Medium (4-6)".into()),
            ("2".into(), "Mage".into()),
            ("3".into(), "Mid".into()),
        ]));
        let raw = engine.recommend(&answers.to_json().unwrap()).unwrap();
        let typed: RecommendationResult = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(typed.top10.len(), 10);
        assert_eq!(serde_json::to_value(typed).unwrap(), raw);
    }

    #[test]
    fn historical_storage_values_and_unknown_fields_round_trip() {
        let old_record = json!({
            "fullName": "Synthetic User",
            "age": "24",
            "registrationDate": "2025-01-01T00:00:00.000Z",
            "userAnswers": "{\"1\":\"Medium (4-6)\"}",
            "legacyExtra": {"preserve": true}
        });
        let parsed: StoredUserRecord = serde_json::from_value(old_record.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), old_record);
    }
}
