use crate::{
    compat::{num, text},
    CatalogEntry,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn aggregate(name: &str, rf: &Value, dt: &Value, knn: &Value) -> Value {
    let rf = num(rf, "score");
    let dt = num(dt, "score");
    let knn = num(knn, "score");
    let average = rf * 0.4 + dt * 0.3 + knn * 0.3;
    // Duplicate details stay in scores, matching the reference fixture encoding.
    json!({"championName": name, "randomForest": rf, "decisionTree": dt,
        "knn": knn, "average": average, "weighted": average.clamp(0.0, 100.0)})
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
