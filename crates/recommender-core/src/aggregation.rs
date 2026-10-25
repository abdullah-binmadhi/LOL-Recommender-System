use crate::{
    compat::{num, text},
    contracts::AggregatedChampionScore,
    CatalogEntry,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Configurable weights for the 6 machine learning scoring models.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelWeights {
    pub random_forest: f64,
    pub decision_tree: f64,
    pub knn: f64,
    pub naive_bayes: f64,
    pub cosine_similarity: f64,
    pub gbdt: f64,
}

impl Default for ModelWeights {
    fn default() -> Self {
        Self {
            random_forest: 0.25,
            decision_tree: 0.15,
            knn: 0.15,
            naive_bayes: 0.15,
            cosine_similarity: 0.15,
            gbdt: 0.15,
        }
    }
}

impl ModelWeights {
    pub fn normalized(&self) -> Self {
        let sum = self.random_forest
            + self.decision_tree
            + self.knn
            + self.naive_bayes
            + self.cosine_similarity
            + self.gbdt;
        if sum.abs() < 1e-6 {
            return Self::default();
        }
        Self {
            random_forest: self.random_forest / sum,
            decision_tree: self.decision_tree / sum,
            knn: self.knn / sum,
            naive_bayes: self.naive_bayes / sum,
            cosine_similarity: self.cosine_similarity / sum,
            gbdt: self.gbdt / sum,
        }
    }

    pub fn balanced() -> Self {
        Self {
            random_forest: 1.0 / 6.0,
            decision_tree: 1.0 / 6.0,
            knn: 1.0 / 6.0,
            naive_bayes: 1.0 / 6.0,
            cosine_similarity: 1.0 / 6.0,
            gbdt: 1.0 / 6.0,
        }
    }

    pub fn mechanics_and_kit() -> Self {
        Self {
            knn: 0.40,
            cosine_similarity: 0.30,
            random_forest: 0.15,
            decision_tree: 0.05,
            naive_bayes: 0.05,
            gbdt: 0.05,
        }
    }

    pub fn pro_meta() -> Self {
        Self {
            random_forest: 0.35,
            gbdt: 0.35,
            decision_tree: 0.10,
            knn: 0.10,
            naive_bayes: 0.05,
            cosine_similarity: 0.05,
        }
    }

    pub fn rule_clarity() -> Self {
        Self {
            decision_tree: 0.40,
            naive_bayes: 0.25,
            random_forest: 0.15,
            knn: 0.10,
            gbdt: 0.05,
            cosine_similarity: 0.05,
        }
    }
}

pub fn aggregate(
    name: &str,
    rf: &Value,
    dt: &Value,
    knn: &Value,
    nb: Option<&Value>,
    cos: Option<&Value>,
    gbdt: Option<&Value>,
) -> Value {
    aggregate_with_weights(name, rf, dt, knn, nb, cos, gbdt, &ModelWeights::default())
}

pub fn aggregate_with_weights(
    name: &str,
    rf: &Value,
    dt: &Value,
    knn: &Value,
    nb: Option<&Value>,
    cos: Option<&Value>,
    gbdt: Option<&Value>,
    weights: &ModelWeights,
) -> Value {
    let rf_val = num(rf, "score");
    let dt_val = num(dt, "score");
    let knn_val = num(knn, "score");

    if let (Some(nb), Some(cos), Some(gbdt)) = (nb, cos, gbdt) {
        let nb_val = num(nb, "score");
        let cos_val = num(cos, "score");
        let gbdt_val = num(gbdt, "score");

        let w = weights.normalized();
        let avg = rf_val * w.random_forest
            + dt_val * w.decision_tree
            + knn_val * w.knn
            + nb_val * w.naive_bayes
            + cos_val * w.cosine_similarity
            + gbdt_val * w.gbdt;

        let mut map = serde_json::Map::new();
        map.insert("championName".into(), json!(name));
        map.insert("randomForest".into(), json!(rf_val));
        map.insert("decisionTree".into(), json!(dt_val));
        map.insert("knn".into(), json!(knn_val));
        map.insert("naiveBayes".into(), json!(nb_val));
        map.insert("cosineSimilarity".into(), json!(cos_val));
        map.insert("gbdt".into(), json!(gbdt_val));
        map.insert("average".into(), json!(avg));
        map.insert("weighted".into(), json!(avg.clamp(0.0, 100.0)));
        Value::Object(map)
    } else {
        let average = rf_val * 0.4 + dt_val * 0.3 + knn_val * 0.3;
        json!({"championName": name, "randomForest": rf_val, "decisionTree": dt_val,
            "knn": knn_val, "average": average, "weighted": average.clamp(0.0, 100.0)})
    }
}

/// Dynamically re-weights scored champions and re-ranks the top 10 list with optional role diversity cap.
pub fn reweight_top10(
    aggregated: &[AggregatedChampionScore],
    catalog: &[CatalogEntry],
    weights: &ModelWeights,
    diversity: bool,
) -> Vec<AggregatedChampionScore> {
    let w = weights.normalized();
    let reweighted: Vec<AggregatedChampionScore> = aggregated
        .iter()
        .map(|item| {
            let rf = item.random_forest;
            let dt = item.decision_tree;
            let knn = item.knn;
            let nb = item.naive_bayes.unwrap_or(item.average);
            let cos = item.cosine_similarity.unwrap_or(item.average);
            let gbdt = item.gbdt.unwrap_or(item.average);

            let avg = rf * w.random_forest
                + dt * w.decision_tree
                + knn * w.knn
                + nb * w.naive_bayes
                + cos * w.cosine_similarity
                + gbdt * w.gbdt;

            AggregatedChampionScore {
                champion_name: item.champion_name.clone(),
                random_forest: rf,
                decision_tree: dt,
                knn,
                naive_bayes: item.naive_bayes,
                cosine_similarity: item.cosine_similarity,
                gbdt: item.gbdt,
                average: avg,
                weighted: avg.clamp(0.0, 100.0),
            }
        })
        .collect();

    // Stable sort preserving original catalog rank for ties
    let mut ranked: Vec<usize> = (0..reweighted.len()).collect();
    ranked.sort_by(|&a, &b| {
        reweighted[b]
            .average
            .total_cmp(&reweighted[a].average)
    });

    if !diversity {
        return ranked.into_iter().take(10).map(|i| reweighted[i].clone()).collect();
    }

    let mut selected = Vec::new();
    let mut counts = HashMap::new();
    for &index in &ranked {
        let name = &reweighted[index].champion_name;
        let role = catalog
            .iter()
            .find(|c| &c.name == name)
            .and_then(|c| c.data.get("role"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown");
        let count = counts.entry(role).or_insert(0);
        if *count < 3 {
            selected.push(index);
            *count += 1;
        }
        if selected.len() >= 10 {
            break;
        }
    }
    for index in ranked {
        if selected.len() >= 10 {
            break;
        }
        if !selected.contains(&index) {
            selected.push(index);
        }
    }

    selected.into_iter().map(|i| reweighted[i].clone()).collect()
}

pub fn top10(aggregated: &[Value], catalog: &[CatalogEntry], diversity: bool) -> Vec<Value> {
    let mut ranked: Vec<usize> = (0..aggregated.len()).collect();
    // Stable sorting preserves effective catalog order for exact score ties.
    ranked.sort_by(|&a, &b| {
        num(&aggregated[b], "average").total_cmp(&num(&aggregated[a], "average"))
    });
    if !diversity {
        return ranked
            .iter()
            .take(10)
            .map(|&i| aggregated[i].clone())
            .collect();
    }
    let mut selected = Vec::new();
    let mut counts = HashMap::new();
    for &index in &ranked {
        let role = text(&catalog[index].data, "role");
        let count = counts.entry(role).or_insert(0);
        if *count < 3 {
            selected.push(index);
            *count += 1;
        }
        if selected.len() >= 10 {
            break;
        }
    }
    for index in ranked {
        if selected.len() >= 10 {
            break;
        }
        if !selected.contains(&index) {
            selected.push(index);
        }
    }
    selected.iter().map(|&i| aggregated[i].clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_ties_and_small_single_role_fallback() {
        let catalog: Vec<_> = (0..12)
            .map(|i| CatalogEntry {
                name: format!("C{i}"),
                data: json!({"role":"Mage"}),
            })
            .collect();
        let rows: Vec<_> = catalog
            .iter()
            .map(|c| json!({"championName":c.name,"average":50}))
            .collect();
        assert_eq!(top10(&rows, &catalog, true), rows[..10]);
        assert_eq!(top10(&[], &[], true), Vec::<Value>::new());
    }

    #[test]
    fn test_model_weights_normalization_and_presets() {
        let default_w = ModelWeights::default();
        let norm = default_w.normalized();
        let sum = norm.random_forest
            + norm.decision_tree
            + norm.knn
            + norm.naive_bayes
            + norm.cosine_similarity
            + norm.gbdt;
        assert!((sum - 1.0).abs() < 1e-6);

        let balanced = ModelWeights::balanced().normalized();
        assert!((balanced.random_forest - 1.0 / 6.0).abs() < 1e-6);

        let pro = ModelWeights::pro_meta().normalized();
        assert!(pro.random_forest > pro.naive_bayes);
    }

    #[test]
    fn test_reweight_top10_dynamic_rerank() {
        let item1 = AggregatedChampionScore {
            champion_name: "ChampA".to_string(),
            random_forest: 90.0,
            decision_tree: 10.0,
            knn: 50.0,
            naive_bayes: Some(50.0),
            cosine_similarity: Some(50.0),
            gbdt: Some(90.0),
            average: 60.0,
            weighted: 60.0,
        };
        let item2 = AggregatedChampionScore {
            champion_name: "ChampB".to_string(),
            random_forest: 10.0,
            decision_tree: 95.0,
            knn: 50.0,
            naive_bayes: Some(95.0),
            cosine_similarity: Some(50.0),
            gbdt: Some(10.0),
            average: 50.0,
            weighted: 50.0,
        };
        let items = vec![item1, item2];
        let catalog = vec![
            CatalogEntry { name: "ChampA".into(), data: json!({"role": "Fighter"}) },
            CatalogEntry { name: "ChampB".into(), data: json!({"role": "Mage"}) },
        ];

        let ranked_pro = reweight_top10(&items, &catalog, &ModelWeights::pro_meta(), false);
        assert_eq!(ranked_pro[0].champion_name, "ChampA");

        let ranked_rules = reweight_top10(&items, &catalog, &ModelWeights::rule_clarity(), false);
        assert_eq!(ranked_rules[0].champion_name, "ChampB");
    }
}
