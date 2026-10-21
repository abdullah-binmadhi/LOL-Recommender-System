use recommender_core::{CatalogEntry, Engine, Tree};
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

fn fixture_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/migration")
}

fn assert_parity(actual: &Value, expected: &Value, path: &str) {
    match (actual, expected) {
        (Value::Number(a), Value::Number(b)) => {
            let (a, b) = (a.as_f64().unwrap(), b.as_f64().unwrap());
            assert!((a - b).abs() <= 1e-9, "{path}: {a} != {b}");
        }
        (Value::Object(a), Value::Object(b)) => {
            assert_eq!(
                a.keys().collect::<Vec<_>>(),
                b.keys().collect::<Vec<_>>(),
                "{path}: keys"
            );
            for (key, value) in b {
                assert_parity(&a[key], value, &format!("{path}.{key}"));
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len(), "{path}: length");
            for (index, (a, b)) in a.iter().zip(b).enumerate() {
                assert_parity(a, b, &format!("{path}[{index}]"));
            }
        }
        _ => assert_eq!(actual, expected, "{path}"),
    }
}

#[test]
fn all_profiles_match_end_to_end_reference_including_exact_ranking_and_details() {
    let engine = Engine::reference().unwrap();
    let mut count = 0;
    for file in fs::read_dir(fixture_directory()).unwrap() {
        let file = file.unwrap().path();
        if !file
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("profile-")
        {
            continue;
        }
        let reference: Value = serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap();
        let actual = engine.recommend(&reference["answers"]).unwrap();
        for key in ["features", "scores", "aggregated", "top10", "evaluation"] {
            assert_parity(
                &actual[key],
                &reference[key],
                &format!("{}.{}", reference["id"], key),
            );
        }
        assert_eq!(
            engine.champion_count(),
            actual["aggregated"].as_array().unwrap().len()
        );
        assert_eq!(actual, engine.recommend(&reference["answers"]).unwrap());
        count += 1;
    }
    assert_eq!(count, 8);
}

#[test]
fn engines_validate_assets_and_inputs() {
    let catalog: Vec<CatalogEntry> = serde_json::from_str(
        &fs::read_to_string(fixture_directory().join("catalog.json")).unwrap(),
    )
    .unwrap();
    let forest: Vec<Tree> =
        serde_json::from_str(&fs::read_to_string(fixture_directory().join("forest.json")).unwrap())
            .unwrap();
    assert!(Engine::new(Vec::new(), forest.clone()).is_err());
    assert!(Engine::new(catalog.clone(), Vec::new()).is_err());
    let mut duplicate = catalog.clone();
    duplicate.push(duplicate[0].clone());
    assert!(Engine::new(duplicate, forest.clone()).is_err());
    let mut broken = forest.clone();
    broken[0].feature_indices.push("unknown".into());
    assert!(Engine::new(catalog.clone(), broken).is_err());
    let mut broken = forest.clone();
    let repeated_feature = broken[0].feature_indices[0].clone();
    broken[0].feature_indices.push(repeated_feature);
    assert!(Engine::new(catalog.clone(), broken).is_err());
    let mut missing = catalog.clone();
    missing[0].data.as_object_mut().unwrap().remove("damage");
    assert!(Engine::new(missing, forest.clone()).is_err());
    let mut invalid_title = catalog.clone();
    invalid_title[0].data["title"] = json!(["the"]);
    assert!(Engine::new(invalid_title, forest.clone()).is_err());
    let mut derived = catalog.clone();
    derived[0].data["derived_stats"] = json!({"burst":10});
    assert!(Engine::new(derived, forest.clone()).is_err());
    let engine = Engine::new(catalog, forest).unwrap();
    for answers in [Value::Null, json!([]), json!({"1":5}), json!({"2":{}})] {
        assert!(engine.recommend(&answers).is_err());
    }
    assert!(engine
        .recommend(&json!({"1":null,"2":"","999":"unused"}))
        .is_ok());
    let valid = engine.recommend(&json!({})).unwrap();
    let mut invalid = valid["features"].clone();
    invalid["damage"] = json!(1e100);
    assert!(engine.score_features(&invalid).is_err());
    assert_parity(
        &engine.score_features(&valid["features"]).unwrap()["scores"],
        &valid["scores"],
        "feature API",
    );
}

#[test]
fn scoring_responds_to_new_answers_and_keeps_known_mapping_quirks() {
    let engine = Engine::reference().unwrap();
    let a = engine
        .recommend(&json!({"2":"Mage","3":"Mid","4":"High Damage Output","5":"No Preference"}))
        .unwrap();
    let b = engine
        .recommend(&json!({"2":"Tank","3":"Top","4":"Tanky and Durable"}))
        .unwrap();
    assert_ne!(a["scores"], b["scores"]);
    assert_eq!(a["features"]["range"], "no preference");
    assert_eq!(b["features"]["range"], "no_preference");
    let alternate = engine.recommend(&json!({"2":"Mage","3":"Mid","4":"High Damage Output","5":"No Preference","11":"ignored","12":"ignored"})).unwrap();
    assert_eq!(a, alternate);
}

#[test]
fn cli_outputs_json_and_rejects_invalid_input_without_stdout() {
    for (input, success) in [
        ("{}", true),
        ("{", false),
        ("[]", false),
        ("{\"1\": 2}", false),
    ] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_lol-scoring-core"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.success(), success);
        if success {
            let value: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(value["top10"].as_array().unwrap().len(), 10);
            assert!(output.stderr.is_empty());
        } else {
            assert!(output.stdout.is_empty());
            assert!(String::from_utf8_lossy(&output.stderr).contains("Scoring failed:"));
        }
    }
}
