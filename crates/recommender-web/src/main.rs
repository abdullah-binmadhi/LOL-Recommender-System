#[cfg(target_arch = "wasm32")]
mod analytics_view;
#[cfg(target_arch = "wasm32")]
mod player_analytics_view;
#[cfg(target_arch = "wasm32")]
mod components;
#[cfg(any(target_arch = "wasm32", test))]
mod presenter;
#[cfg(any(target_arch = "wasm32", test))]
mod state;
#[cfg(target_arch = "wasm32")]
mod storage;
#[cfg(target_arch = "wasm32")]
mod views;

#[cfg(target_arch = "wasm32")]
use recommender_core::contracts::EvaluationResult;
#[cfg(target_arch = "wasm32")]
use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use state::Metric;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Deserialize)]
struct CounterRecord {
    #[serde(default)]
    strong_against: Vec<String>,
    #[serde(default)]
    weak_against: Vec<String>,
    #[serde(default)]
    synergizes_with: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
fn metric_class(value: f64) -> &'static str {
    if value >= 0.7 {
        "excellent"
    } else if value >= 0.4 {
        "good"
    } else {
        "poor"
    }
}

#[cfg(target_arch = "wasm32")]
fn metric_value(metric: Metric, evaluation: &EvaluationResult) -> f64 {
    match metric {
        Metric::PrecisionAt1 => evaluation.precision_at1,
        Metric::PrecisionAt3 => evaluation.precision_at3,
        Metric::PrecisionAt10 => evaluation.precision_at10,
        Metric::RecallAt10 => evaluation.recall_at10,
        Metric::F1At10 => evaluation.f1_at10,
        Metric::MeanReciprocalRank => evaluation.mrr,
    }
}

#[cfg(target_arch = "wasm32")]
fn metric_explanation(metric: Metric) -> &'static str {
    match metric {
        Metric::PrecisionAt1 => "Precision@1 checks whether the top champion belongs to the relevant set produced by the current preference heuristic.",
        Metric::PrecisionAt3 => "Precision@3 measures the share of the first three recommendations included in that relevant set.",
        Metric::PrecisionAt10 => "Precision@10 measures how many of the first ten recommendations are relevant to the supplied answers.",
        Metric::RecallAt10 => "Recall@10 measures the share of all champions in the heuristic relevant set that appear in the top ten.",
        Metric::F1At10 => "F1-Score@10 is the harmonic mean of Precision@10 and Recall@10.",
        Metric::MeanReciprocalRank => "Mean reciprocal rank uses the position of the first relevant recommendation; the existing UI labels this Confidence & Diversity.",
    }
}

#[cfg(target_arch = "wasm32")]
fn champion_image_url(
    name: &str,
    champion: Option<&recommender_core::contracts::Champion>,
) -> String {
    if let Some(image) = champion
        .and_then(|champion| champion.additional_fields.get("image"))
        .and_then(serde_json::Value::as_str)
    {
        return image.to_owned();
    }

    let api_name = match name {
        "Aurelion Sol" => "AurelionSol",
        "Bel'Veth" => "Belveth",
        "Cho'Gath" => "Chogath",
        "Dr. Mundo" => "DrMundo",
        "Jarvan IV" => "JarvanIV",
        "Kai'Sa" => "Kaisa",
        "Kha'Zix" => "Khazix",
        "Kog'Maw" => "KogMaw",
        "Lee Sin" => "LeeSin",
        "Master Yi" => "MasterYi",
        "Miss Fortune" => "MissFortune",
        "Nunu & Willump" => "Nunu",
        "Rek'Sai" => "RekSai",
        "Tahm Kench" => "TahmKench",
        "Twisted Fate" => "TwistedFate",
        "Vel'Koz" => "Velkoz",
        "Xin Zhao" => "XinZhao",
        _ => {
            return format!(
                "https://ddragon.leagueoflegends.com/cdn/15.16.1/img/champion/{}.png",
                name.chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .collect::<String>()
            )
        }
    };

    format!("https://ddragon.leagueoflegends.com/cdn/15.16.1/img/champion/{api_name}.png")
}

#[cfg(target_arch = "wasm32")]
fn normalize_name(name: &str) -> String {
    name.chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn role_tips(role: &str) -> &'static [(&'static str, &'static str)] {
    match role {
        "Tank" => &[
            (
                "🛡️",
                "Engage when your team is ready and enemies are grouped together",
            ),
            (
                "🎯",
                "Focus on peeling for your carries rather than diving their backline",
            ),
            (
                "👥",
                "Use your crowd control abilities to set up kills for your team",
            ),
            (
                "⚡",
                "Build according to enemy team composition - armor vs AD, MR vs AP",
            ),
            (
                "🛡️",
                "Use your tankiness to soak up damage and protect your carries",
            ),
            (
                "👥",
                "Look for opportunities to initiate fights and create space for your team",
            ),
        ],
        "Fighter" => &[
            (
                "⚔️",
                "Look for extended trades where you can use your full combo",
            ),
            (
                "🏃",
                "Use teleport to join teamfights or split push when ahead",
            ),
            (
                "💪",
                "Build a mix of damage and survivability to stay relevant",
            ),
            (
                "🎯",
                "Target the enemy backline but don't dive too deep alone",
            ),
        ],
        "Assassin" => &[
            (
                "💀",
                "Wait for enemy cooldowns before going in for the kill",
            ),
            (
                "👤",
                "Use fog of war and flanking to catch enemies off guard",
            ),
            (
                "⚡",
                "Burst down high-priority targets quickly then get out",
            ),
            (
                "🌙",
                "Roam frequently to get kills and help other lanes snowball",
            ),
            (
                "👤",
                "Use your mobility to dodge key enemy abilities and reposition",
            ),
            (
                "💀",
                "Look for enemies low on health to use your ultimate effectively",
            ),
        ],
        "Marksman" => &[
            (
                "🏹",
                "Position safely behind your team and focus nearest target",
            ),
            ("💰", "Farm consistently and avoid risky trades early game"),
            (
                "🎯",
                "Use attack-move to maintain DPS while kiting backwards",
            ),
            ("⚡", "Build lifesteal to sustain through teamfights"),
            (
                "🎯",
                "Use your long range to control minion waves and deny farm to enemy ADC",
            ),
            (
                "🏹",
                "Position for teamfights where you can maximize your damage output",
            ),
        ],
        "Mage" => &[
            (
                "🔮",
                "Manage your mana carefully and look for ability combos",
            ),
            ("📏", "Use your range advantage to poke before teamfights"),
            (
                "🛡️",
                "Position safely and use crowd control to protect yourself",
            ),
            ("💥", "Save your ultimate for maximum impact in teamfights"),
            (
                "🔮",
                "Use your abilities to zone enemies from minion waves and control CS",
            ),
            (
                "📏",
                "Position for teamfights where you can land abilities on multiple enemies",
            ),
        ],
        "Support" => &[
            (
                "👁️",
                "Ward key areas and deny enemy vision with control wards",
            ),
            ("🤝", "Roam to help other lanes when your ADC is safe"),
            ("🛡️", "Protect your carries and peel for them in teamfights"),
            (
                "💡",
                "Build according to team needs - tank items or enchanter items",
            ),
            (
                "👁️",
                "Use your vision control to set up ganks and avoid enemy ganks",
            ),
            (
                "🤝",
                "Look for opportunities to create picks with your crowd control",
            ),
        ],
        _ => &[
            (
                "⚔️",
                "Adapt your playstyle based on the enemy team composition",
            ),
            (
                "🛡️",
                "Focus on farming safely and scaling into the late game",
            ),
        ],
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    leptos::mount::mount_to_body(views::App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("recommender-web is a browser application; build it with Trunk.");
}
