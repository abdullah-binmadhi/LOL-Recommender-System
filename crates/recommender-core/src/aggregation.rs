use crate::{
    compat::{num, text},
    CatalogEntry,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn aggregate(
    name: &str,
    rf: &Value,
    dt: &Value,
    knn: &Value,
    nb: Option<&Value>,
    cos: Option<&Value>,
    gbdt: Option<&Value>,
) -> Value {
    let rf_val = num(rf, "score");
    let dt_val = num(dt, "score");
    let knn_val = num(knn, "score");

    if let (Some(nb), Some(cos), Some(gbdt)) = (nb, cos, gbdt) {
        let nb_val = num(nb, "score");
        let cos_val = num(cos, "score");
        let gbdt_val = num(gbdt, "score");

        // 6-Model Balanced Ensemble Weighted Average
        let avg = rf_val * 0.25
            + dt_val * 0.15
            + knn_val * 0.15
            + nb_val * 0.15
            + cos_val * 0.15
            + gbdt_val * 0.15;
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
}
