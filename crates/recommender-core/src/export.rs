use serde_json::Value;

pub const CSV_HEADERS: [&str; 26] = [
    "ID",
    "Full Name",
    "Email",
    "Phone",
    "Age",
    "Gender",
    "LoL Experience",
    "Registration Date",
    "Session ID",
    "Recommended Champion",
    "Winning Algorithm",
    "Confidence Score",
    "Random Forest Champion",
    "Random Forest Confidence",
    "Decision Tree Champion",
    "Decision Tree Confidence",
    "KNN Champion",
    "KNN Confidence",
    "Consensus Level",
    "User Answers",
    "Completion Date",
    "Pressure Response",
    "Aesthetic Preference",
    "Team Contribution",
    "Character Identity",
    "Problem Solving Approach",
];

pub const TABLE_CSV_HEADERS: [&str; 8] = [
    "ID",
    "Name",
    "Email",
    "Age",
    "Gender",
    "Experience",
    "Recommended Champion",
    "Registration Date",
];

fn field_to_str(user: &Value, key: &str) -> String {
    match user.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Array(_) | Value::Object(_)) => {
            serde_json::to_string(user.get(key).unwrap()).unwrap_or_default()
        }
        Some(Value::Null) | None => String::new(),
    }
}

fn escape_csv_field(field: &str) -> String {
    format!("\"{}\"", field.replace('"', "\"\""))
}

/// Generates the standard 26-column user export CSV matching the legacy exportToCSV().
pub fn export_users_to_csv(users: &[Value]) -> String {
    let mut csv = CSV_HEADERS.join(",") + "\n";
    for (index, user) in users.iter().enumerate() {
        let row = [
            (index + 1).to_string(),
            field_to_str(user, "fullName"),
            field_to_str(user, "email"),
            field_to_str(user, "phone"),
            field_to_str(user, "age"),
            field_to_str(user, "gender"),
            field_to_str(user, "experience"),
            field_to_str(user, "registrationDate"),
            field_to_str(user, "sessionId"),
            field_to_str(user, "recommendedChampion"),
            field_to_str(user, "winningAlgorithm"),
            field_to_str(user, "confidence"),
            field_to_str(user, "randomForestChampion"),
            field_to_str(user, "randomForestConfidence"),
            field_to_str(user, "decisionTreeChampion"),
            field_to_str(user, "decisionTreeConfidence"),
            field_to_str(user, "knnChampion"),
            field_to_str(user, "knnConfidence"),
            field_to_str(user, "consensusLevel"),
            field_to_str(user, "userAnswers"),
            field_to_str(user, "completionDate"),
            field_to_str(user, "pressureResponse"),
            field_to_str(user, "aestheticPreference"),
            field_to_str(user, "teamContribution"),
            field_to_str(user, "characterIdentity"),
            field_to_str(user, "problemSolving"),
        ];

        let escaped_row: Vec<String> = row.iter().map(|f| escape_csv_field(f)).collect();
        csv.push_str(&escaped_row.join(","));
        csv.push('\n');
    }
    csv
}

/// Generates the table CSV export matching legacy downloadTableAsCSV().
pub fn export_table_to_csv(users: &[Value]) -> String {
    let mut csv = TABLE_CSV_HEADERS.join(",") + "\n";
    for (index, user) in users.iter().enumerate() {
        let row = [
            (index + 1).to_string(),
            field_to_str(user, "fullName"),
            field_to_str(user, "email"),
            field_to_str(user, "age"),
            field_to_str(user, "gender"),
            field_to_str(user, "experience"),
            field_to_str(user, "recommendedChampion"),
            field_to_str(user, "registrationDate"),
        ];

        let escaped_row: Vec<String> = row.iter().map(|f| escape_csv_field(f)).collect();
        csv.push_str(&escaped_row.join(","));
        csv.push('\n');
    }
    csv
}

/// Updates the recent champions list with newly recommended champions, preserving
/// the legacy deduplicating unshift and truncation to 15.
pub fn update_recent_champions(existing: &[String], new_recommendations: &[String]) -> Vec<String> {
    let mut recent = existing.to_vec();
    for champion in new_recommendations {
        if !recent.contains(champion) {
            recent.insert(0, champion.clone());
        }
    }
    recent.truncate(15);
    recent
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn exports_empty_users_with_header_only() {
        let csv = export_users_to_csv(&[]);
        assert!(csv.starts_with("ID,Full Name,Email"));
        assert_eq!(csv.lines().count(), 1);
    }

    #[test]
    fn exports_users_with_exact_headers_and_quotes() {
        let users = vec![json!({
            "fullName": "Test Summoner",
            "email": "test@example.com",
            "phone": "Not provided",
            "age": "24",
            "gender": "Not specified",
            "experience": "Intermediate",
            "registrationDate": "2026-10-21T00:00:00.000Z",
            "sessionId": "session_123_abc",
            "recommendedChampion": "Ahri",
            "winningAlgorithm": "Random Forest",
            "confidence": 0.85
        })];
        let csv = export_users_to_csv(&users);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], CSV_HEADERS.join(","));
        assert!(lines[1].starts_with("\"1\",\"Test Summoner\",\"test@example.com\""));
        assert!(lines[1].contains("\"Ahri\""));
        assert!(lines[1].contains("\"0.85\""));
    }

    #[test]
    fn handles_quotes_commas_newlines_and_unicode() {
        let users = vec![json!({
            "fullName": "Ahri, \"The Nine-Tailed Fox\" & 狐",
            "email": "fox@ionia.runeterra\nsecond line",
            "age": 21
        })];
        let csv = export_users_to_csv(&users);
        assert!(csv.contains("\"Ahri, \"\"The Nine-Tailed Fox\"\" & 狐\""));
        assert!(csv.contains("\"fox@ionia.runeterra\nsecond line\""));
    }

    #[test]
    fn table_csv_has_8_columns() {
        let users = vec![json!({
            "fullName": "Summoner One",
            "email": "s1@example.com",
            "age": "20",
            "gender": "Female",
            "experience": "Advanced",
            "recommendedChampion": "Jinx",
            "registrationDate": "2026-10-21"
        })];
        let csv = export_table_to_csv(&users);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], TABLE_CSV_HEADERS.join(","));
        assert_eq!(
            lines[1],
            "\"1\",\"Summoner One\",\"s1@example.com\",\"20\",\"Female\",\"Advanced\",\"Jinx\",\"2026-10-21\""
        );
    }

    #[test]
    fn recent_champions_unshifts_deduplicates_and_truncates_to_15() {
        let existing = vec!["Yasuo".into(), "Zed".into()];
        let new_champions = vec!["Ahri".into(), "Yasuo".into(), "Lux".into()];
        let updated = update_recent_champions(&existing, &new_champions);
        // "Ahri" unshifted -> ["Ahri", "Yasuo", "Zed"]
        // "Yasuo" already exists -> skipped
        // "Lux" unshifted -> ["Lux", "Ahri", "Yasuo", "Zed"]
        assert_eq!(updated, vec!["Lux", "Ahri", "Yasuo", "Zed"]);

        // Test truncation to 15
        let large_list: Vec<String> = (0..20).map(|i| format!("Champ{}", i)).collect();
        let updated_large = update_recent_champions(&[], &large_list);
        assert_eq!(updated_large.len(), 15);
        assert_eq!(updated_large[0], "Champ19"); // last unshifted
    }
}
