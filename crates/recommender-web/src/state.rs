use recommender_core::contracts::{
    Answers, EffectiveQuestion, QuestionOption, RecommendationResult, StoredUserRecord,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Start,
    #[allow(dead_code)]
    Registration,
    Questionnaire,
    Results,
    ChampionDistribution,
    PlayerAnalytics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    PrecisionAt1,
    PrecisionAt3,
    PrecisionAt10,
    RecallAt10,
    F1At10,
    MeanReciprocalRank,
}

impl Metric {
    pub fn label(self) -> &'static str {
        match self {
            Self::PrecisionAt1 => "Precision@1",
            Self::PrecisionAt3 => "Precision@3",
            Self::PrecisionAt10 => "Precision@10",
            Self::RecallAt10 => "Recall@10",
            Self::F1At10 => "F1-Score@10",
            Self::MeanReciprocalRank => "Confidence & Diversity",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modal {
    Champion(String),
    Metric(Metric),
    #[allow(dead_code)]
    SavedUsers(Vec<StoredUserRecord>),
    #[allow(dead_code)]
    CloudHistory,
    #[allow(dead_code)]
    LinkRiotAccount,
    #[allow(dead_code)]
    OAuthConfigRequired,
    #[allow(dead_code)]
    TieBreaker,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
    pub riot_id: Option<String>,
    pub riot_region: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegistrationDraft {
    pub full_name: String,
    pub email: String,
    pub age: String,
    pub experience: String,
    pub gender: String,
    pub phone: String,
}

impl RegistrationDraft {
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        let age_is_valid = self
            .age
            .parse::<u8>()
            .is_ok_and(|age| (13..=100).contains(&age));

        !self.full_name.trim().is_empty()
            && self.email.contains('@')
            && age_is_valid
            && !self.experience.is_empty()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RosterFilter {
    pub hero_type: Option<String>,
    pub range_type: Option<String>,
    pub resource_type: Option<String>,
    pub difficulty: Option<u8>,
    pub release_year: Option<String>,
}

impl RosterFilter {
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.hero_type.is_some()
            || self.range_type.is_some()
            || self.resource_type.is_some()
            || self.difficulty.is_some()
            || self.release_year.is_some()
    }

    #[allow(dead_code)]
    pub fn active_count(&self) -> usize {
        let mut count = 0;
        if self.hero_type.is_some() {
            count += 1;
        }
        if self.range_type.is_some() {
            count += 1;
        }
        if self.resource_type.is_some() {
            count += 1;
        }
        if self.difficulty.is_some() {
            count += 1;
        }
        if self.release_year.is_some() {
            count += 1;
        }
        count
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub screen: Screen,
    #[allow(dead_code)]
    pub auth_user: Option<AuthUser>,
    pub registration: RegistrationDraft,
    pub answers: Answers,
    pub roster_filter: RosterFilter,
    pub current_question_index: usize,
    pub recommendation: Option<RecommendationResult>,
    pub modal: Option<Modal>,
    pub error: Option<String>,
    pub scoring: bool,
    #[allow(dead_code)]
    pub demo_telemetry: bool,
    pub live_player_matches: Option<Vec<recommender_core::player_analytics::AdoptedMatch>>,
    pub is_syncing_riot: bool,
    pub riot_data_source: Option<String>,
    #[allow(dead_code)]
    pub analytics_active_tab: String,
    #[allow(dead_code)]
    pub analytics_selected_match_id: Option<String>,
    #[allow(dead_code)]
    pub analytics_curve_match_id: Option<String>,
    #[allow(dead_code)]
    pub analytics_tracked_champion: String,
    #[allow(dead_code)]
    pub analytics_visible_games: usize,
    #[allow(dead_code)]
    pub model_weights: recommender_core::aggregation::ModelWeights,
    #[allow(dead_code)]
    pub tie_breaker_answers: Vec<bool>,
    pub model_trust_lab_open: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Start,
            auth_user: None,
            registration: RegistrationDraft::default(),
            answers: Answers::default(),
            roster_filter: RosterFilter::default(),
            current_question_index: 0,
            recommendation: None,
            modal: None,
            error: None,
            scoring: false,
            demo_telemetry: false,
            live_player_matches: None,
            is_syncing_riot: false,
            riot_data_source: None,
            analytics_active_tab: "overview".to_string(),
            analytics_selected_match_id: None,
            analytics_curve_match_id: None,
            analytics_tracked_champion: String::new(),
            analytics_visible_games: 10,
            model_weights: recommender_core::aggregation::ModelWeights::default(),
            tie_breaker_answers: vec![true, true, true, true, true],
            model_trust_lab_open: true,
        }
    }
}

impl AppState {
    #[allow(dead_code)]
    pub fn toggle_roster_hero_type(&mut self, val: String) {
        if self.roster_filter.hero_type.as_deref() == Some(&val) {
            self.roster_filter.hero_type = None;
        } else {
            self.roster_filter.hero_type = Some(val);
        }
    }

    #[allow(dead_code)]
    pub fn toggle_roster_range_type(&mut self, val: String) {
        if self.roster_filter.range_type.as_deref() == Some(&val) {
            self.roster_filter.range_type = None;
        } else {
            self.roster_filter.range_type = Some(val);
        }
    }

    #[allow(dead_code)]
    pub fn toggle_roster_resource_type(&mut self, val: String) {
        if self.roster_filter.resource_type.as_deref() == Some(&val) {
            self.roster_filter.resource_type = None;
        } else {
            self.roster_filter.resource_type = Some(val);
        }
    }

    #[allow(dead_code)]
    pub fn toggle_roster_difficulty(&mut self, val: u8) {
        if self.roster_filter.difficulty == Some(val) {
            self.roster_filter.difficulty = None;
        } else {
            self.roster_filter.difficulty = Some(val);
        }
    }

    #[allow(dead_code)]
    pub fn toggle_roster_release_year(&mut self, val: String) {
        if self.roster_filter.release_year.as_deref() == Some(&val) {
            self.roster_filter.release_year = None;
        } else {
            self.roster_filter.release_year = Some(val);
        }
    }

    #[allow(dead_code)]
    pub fn clear_roster_filter(&mut self) {
        self.roster_filter.clear();
    }
    #[allow(dead_code)]
    pub fn start_quiz(&mut self) {
        self.screen = Screen::Questionnaire;
        self.current_question_index = 0;
        self.error = None;
    }

    #[allow(dead_code)]
    pub fn set_auth_user(&mut self, user: Option<AuthUser>) {
        self.auth_user = user;
    }

    #[allow(dead_code)]
    pub fn update_riot_profile(&mut self, riot_id: String, riot_region: String) {
        let summoner = riot_id.split('#').next().unwrap_or(&riot_id).to_string();
        if let Some(ref mut user) = self.auth_user {
            user.riot_id = Some(riot_id.clone());
            user.riot_region = Some(riot_region.clone());
            if user.full_name.is_empty() || user.full_name == "Mock Summoner" {
                user.full_name = summoner;
            }
        } else {
            self.auth_user = Some(AuthUser {
                id: "local-user".into(),
                email: "".into(),
                full_name: summoner,
                avatar_url: None,
                riot_id: Some(riot_id.clone()),
                riot_region: Some(riot_region.clone()),
            });
        }
        self.demo_telemetry = false;
    }

    #[allow(dead_code)]
    pub fn set_live_matches(
        &mut self,
        matches: Option<Vec<recommender_core::player_analytics::AdoptedMatch>>,
        data_source: Option<String>,
    ) {
        self.live_player_matches = matches;
        self.riot_data_source = data_source;
        self.demo_telemetry = false;
        self.is_syncing_riot = false;
    }

    #[allow(dead_code)]
    pub fn set_syncing_riot(&mut self, syncing: bool) {
        self.is_syncing_riot = syncing;
    }

    #[allow(dead_code)]
    pub fn begin_registration(&mut self) {
        self.screen = Screen::Registration;
        self.error = None;
    }

    #[allow(dead_code)]
    pub fn open_champion_distribution(&mut self) {
        self.screen = Screen::ChampionDistribution;
        self.error = None;
    }

    #[allow(dead_code)]
    pub fn open_player_analytics(&mut self) {
        self.screen = Screen::PlayerAnalytics;
        self.error = None;
    }

    #[allow(dead_code)]
    pub fn open_analytics(&mut self) {
        self.open_player_analytics();
    }

    #[allow(dead_code)]
    pub fn toggle_demo_telemetry(&mut self) {
        self.demo_telemetry = !self.demo_telemetry;
    }

    pub fn return_to_start(&mut self) {
        self.screen = Screen::Start;
        self.error = None;
    }

    #[allow(dead_code)]
    pub fn submit_registration(&mut self) -> bool {
        if !self.registration.is_valid() {
            self.error = Some("Please complete the required fields with valid values.".into());
            return false;
        }

        self.screen = Screen::Questionnaire;
        self.current_question_index = 0;
        self.error = None;
        true
    }

    pub fn can_advance(&self, questions: &[EffectiveQuestion]) -> bool {
        questions
            .get(self.current_question_index)
            .and_then(|question| self.answers.0.get(&question.id.to_string()))
            .is_some_and(Option::is_some)
    }

    /// Advances one question. Returns true when the final answer is ready for scoring.
    pub fn advance(&mut self, questions: &[EffectiveQuestion]) -> bool {
        if !self.can_advance(questions) {
            return false;
        }

        if self.current_question_index + 1 == questions.len() {
            true
        } else {
            self.current_question_index += 1;
            false
        }
    }

    pub fn previous(&mut self) {
        self.current_question_index = self.current_question_index.saturating_sub(1);
    }

    pub fn select_answer(
        &mut self,
        question_id: u32,
        answer: String,
        questions: &[EffectiveQuestion],
    ) {
        if questions
            .get(self.current_question_index)
            .is_none_or(|question| question.id != question_id)
        {
            return;
        }

        self.answers.insert(question_id.to_string(), answer);
        self.error = None;

        if let Some(next_question) = questions
            .iter()
            .find(|question| question.id == question_id + 1)
        {
            let next_id = next_question.id.to_string();
            let allowed = filtered_options(next_question, &self.answers);
            if self
                .answers
                .0
                .get(&next_id)
                .and_then(Option::as_ref)
                .is_some_and(|selected| !allowed.iter().any(|option| &option.value == selected))
            {
                self.answers.0.remove(&next_id);
            }
        }
    }

    /// Checks if a question ID supports multi-selection (Tier 1 questions)
    pub fn is_multi_select_question(question_id: u32) -> bool {
        matches!(question_id, 2 | 3 | 7 | 12 | 13 | 14 | 16 | 23)
    }

    /// Maximum number of selections allowed for a multi-select question
    pub fn max_selections_for_question(question_id: u32) -> usize {
        match question_id {
            2 | 3 | 7 | 12 | 13 | 14 | 16 | 23 => 2,
            _ => 1,
        }
    }

    /// Toggles an option in a multi-select question, maintaining a sliding window up to max_selections
    pub fn toggle_answer_option(
        &mut self,
        question_id: u32,
        option_value: &str,
        questions: &[EffectiveQuestion],
    ) {
        if questions
            .get(self.current_question_index)
            .is_none_or(|question| question.id != question_id)
        {
            return;
        }

        if !Self::is_multi_select_question(question_id) {
            self.select_answer(question_id, option_value.to_string(), questions);
            return;
        }

        let q_key = question_id.to_string();
        let max_sel = Self::max_selections_for_question(question_id);
        let cur = self
            .answers
            .0
            .get(&q_key)
            .and_then(Option::as_deref)
            .unwrap_or("");

        let mut items: Vec<String> = if cur.is_empty() {
            Vec::new()
        } else {
            cur.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        if let Some(pos) = items.iter().position(|x| x == option_value) {
            items.remove(pos);
        } else {
            if items.len() >= max_sel {
                items.remove(0);
            }
            items.push(option_value.to_string());
        }

        if items.is_empty() {
            self.answers.0.remove(&q_key);
        } else {
            self.answers.insert(&q_key, items.join(","));
        }
        self.error = None;
    }

    /// Checks if an option is currently selected for a question
    pub fn is_option_selected(&self, question_id: u32, option_value: &str) -> bool {
        self.answers
            .0
            .get(&question_id.to_string())
            .and_then(Option::as_deref)
            .is_some_and(|cur| {
                cur.split(',').any(|item| item.trim() == option_value)
            })
    }

    pub fn begin_scoring(&mut self) -> Answers {
        self.scoring = true;
        self.error = None;
        self.answers
            .0
            .insert("quiz_version".to_string(), Some("2".to_string()));
        self.answers.clone()
    }

    pub fn finish_scoring(&mut self, result: Result<RecommendationResult, String>) {
        self.scoring = false;
        match result {
            Ok(recommendation) => {
                self.recommendation = Some(recommendation);
                self.screen = Screen::Results;
                self.error = None;
            }
            Err(error) => {
                self.screen = Screen::Questionnaire;
                self.error = Some(error);
            }
        }
    }

    pub fn open_modal(&mut self, modal: Modal) {
        self.modal = Some(modal);
    }

    pub fn close_modal(&mut self) {
        self.modal = None;
    }

    pub fn restart(&mut self) {
        let registration = self.registration.clone();
        let auth_user = self.auth_user.clone();
        *self = Self {
            registration,
            auth_user,
            ..Self::default()
        };
    }
}

/// Applies the exact answer-driven option filters used by the legacy quiz.
pub fn filtered_options(question: &EffectiveQuestion, answers: &Answers) -> Vec<QuestionOption> {
    let answer = |id: u32| answers.0.get(&id.to_string()).and_then(Option::as_deref);
    let role = answer(2);

    let allowed: Option<&[&str]> = match question.id {
        3 => match role {
            Some("Tank") => Some(&["Top", "Jungle", "Support"]),
            Some("Fighter") => Some(&["Top", "Jungle", "Mid"]),
            Some("Assassin") => Some(&["Mid", "Jungle"]),
            Some("Marksman") => Some(&["Bot"]),
            Some("Mage") => Some(&["Mid", "Support"]),
            Some("Support") => Some(&["Support"]),
            _ => None,
        },
        4 => match role {
            Some("Tank") => Some(&["Tanky and Durable", "Support Team", "Balanced/Hybrid"]),
            Some("Fighter") => {
                Some(&["High Damage Output", "Tanky and Durable", "Balanced/Hybrid"])
            }
            Some("Assassin" | "Marksman") => Some(&["High Damage Output"]),
            Some("Mage") => Some(&["High Damage Output", "Support Team", "Balanced/Hybrid"]),
            Some("Support") => Some(&["Support Team", "Balanced/Hybrid"]),
            _ => None,
        },
        // Range (Q5) and Damage Profile (Q7) are universally unlocked for all classes/roles
        5 => None,
        7 => None,
        1 => {
            let challenge_answer = answer(12).or_else(|| answer(10));
            match challenge_answer {
                Some("Focus on long-term improvement") => {
                    Some(&["Medium (4-6)", "Hard (7-8)", "Very Hard (9-10)"])
                }
                Some("Play cautiously to avoid mistakes") => Some(&["Easy (1-3)", "Medium (4-6)"]),
                _ => None,
            }
        }
        _ => None,
    };

    question
        .options
        .iter()
        .filter(|option| allowed.is_none_or(|values| values.contains(&option.value.as_str())))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use recommender_core::contracts::QuestionnaireDefinition;

    fn questions() -> Vec<EffectiveQuestion> {
        QuestionnaireDefinition::modern()
            .or_else(|_| QuestionnaireDefinition::current())
            .unwrap()
            .effective_questions()
    }

    fn question(id: u32, questions: &[EffectiveQuestion]) -> &EffectiveQuestion {
        questions.iter().find(|question| question.id == id).unwrap()
    }

    #[test]
    fn start_registration_returns_and_registration_requires_existing_fields() {
        let mut state = AppState::default();
        state.begin_registration();
        assert_eq!(state.screen, Screen::Registration);
        assert!(!state.submit_registration());
        assert!(state.error.is_some());

        state.registration = RegistrationDraft {
            full_name: "Test Summoner".into(),
            email: "test@example.invalid".into(),
            age: "25".into(),
            experience: "Intermediate".into(),
            gender: String::new(),
            phone: String::new(),
        };
        assert!(state.submit_registration());
        assert_eq!(state.screen, Screen::Questionnaire);
        assert_eq!(state.current_question_index, 0);

        state.return_to_start();
        assert_eq!(state.screen, Screen::Start);
    }

    #[test]
    fn registration_validates_age_and_required_fields_but_not_optional_fields() {
        let mut registration = RegistrationDraft {
            full_name: "Test Summoner".into(),
            email: "test@example.invalid".into(),
            age: "12".into(),
            experience: "Intermediate".into(),
            gender: String::new(),
            phone: String::new(),
        };
        assert!(!registration.is_valid());
        registration.age = "13".into();
        assert!(registration.is_valid());
        registration.full_name.clear();
        assert!(!registration.is_valid());
    }

    #[test]
    fn test_multi_select_toggling_and_limits() {
        let qs = questions();
        let mut state = AppState::default();
        state.current_question_index = 1; // Question 2

        assert!(AppState::is_multi_select_question(2));
        assert!(AppState::is_multi_select_question(7));
        assert!(!AppState::is_multi_select_question(1));

        // Toggle Tank on Q2
        state.toggle_answer_option(2, "Tank", &qs);
        assert!(state.is_option_selected(2, "Tank"));
        assert_eq!(state.answers.0.get("2").unwrap(), &Some("Tank".to_string()));

        // Toggle Fighter on Q2
        state.toggle_answer_option(2, "Fighter", &qs);
        assert!(state.is_option_selected(2, "Tank"));
        assert!(state.is_option_selected(2, "Fighter"));
        assert_eq!(state.answers.0.get("2").unwrap(), &Some("Tank,Fighter".to_string()));

        // Toggle Tank again to remove it
        state.toggle_answer_option(2, "Tank", &qs);
        assert!(!state.is_option_selected(2, "Tank"));
        assert!(state.is_option_selected(2, "Fighter"));
        assert_eq!(state.answers.0.get("2").unwrap(), &Some("Fighter".to_string()));
    }

    #[test]
    fn legacy_role_filters_match_the_questionnaire_contract() {
        type FilterCase<'a> = (
            &'a str,
            &'a [&'a str],
            &'a [&'a str],
            &'a [&'a str],
            &'a [&'a str],
        );

        let questions = questions();
        let all_ranges = &["Ranged", "Melee", "No Preference"];
        let all_damages = &["Physical", "Magic", "Hybrid (Physical & Magic)", "No Preference"];

        let cases: [FilterCase<'_>; 6] = [
            (
                "Tank",
                &["Top", "Jungle", "Support"],
                &["Tanky and Durable", "Support Team", "Balanced/Hybrid"],
                all_ranges,
                all_damages,
            ),
            (
                "Fighter",
                &["Top", "Jungle", "Mid"],
                &["High Damage Output", "Tanky and Durable", "Balanced/Hybrid"],
                all_ranges,
                all_damages,
            ),
            (
                "Assassin",
                &["Jungle", "Mid"],
                &["High Damage Output"],
                all_ranges,
                all_damages,
            ),
            (
                "Marksman",
                &["Bot"],
                &["High Damage Output"],
                all_ranges,
                all_damages,
            ),
            (
                "Mage",
                &["Mid", "Support"],
                &["High Damage Output", "Support Team", "Balanced/Hybrid"],
                all_ranges,
                all_damages,
            ),
            (
                "Support",
                &["Support"],
                &["Support Team", "Balanced/Hybrid"],
                all_ranges,
                all_damages,
            ),
        ];

        for (role, positions, playstyles, ranges, damage_types) in cases {
            let mut answers = Answers::default();
            answers.insert("2", role);
            let values = |question_id| {
                filtered_options(question(question_id, &questions), &answers)
                    .into_iter()
                    .map(|option| option.value)
                    .collect::<Vec<_>>()
            };

            assert_eq!(
                values(3),
                positions
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>(),
                "position filter for {role}"
            );
            assert_eq!(
                values(4),
                playstyles
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>(),
                "playstyle filter for {role}"
            );
            assert_eq!(
                values(5),
                ranges
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>(),
                "range filter for {role}"
            );
            assert_eq!(
                values(7),
                damage_types
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>(),
                "damage-type filter for {role}"
            );
        }
    }

    #[test]
    fn changing_role_clears_only_an_invalid_immediate_next_answer() {
        let questions = questions();
        let mut state = AppState {
            current_question_index: 1,
            ..AppState::default()
        };
        state.answers.insert("3", "Top");
        state.answers.insert("4", "High Damage Output");

        state.select_answer(2, "Mage".into(), &questions);
        assert!(!state.answers.0.contains_key("3"));
        assert!(state.answers.0.contains_key("4"));
    }

    #[test]
    fn psychological_filter_for_question_one_uses_legacy_answer_ten() {
        let questions = questions();
        let mut answers = Answers::default();
        answers.insert("10", "Play cautiously to avoid mistakes");
        let options: Vec<_> = filtered_options(question(1, &questions), &answers)
            .into_iter()
            .map(|option| option.value)
            .collect();
        assert_eq!(options, ["Easy (1-3)", "Medium (4-6)"]);
    }

    #[test]
    fn navigation_requires_an_answer_and_keeps_the_final_question_for_scoring() {
        let questions = questions();
        let mut state = AppState {
            screen: Screen::Questionnaire,
            ..AppState::default()
        };
        assert!(!state.can_advance(&questions));
        assert!(!state.advance(&questions));
        assert_eq!(state.current_question_index, 0);

        state.answers.insert("1", "Medium (4-6)");
        assert!(!state.advance(&questions));
        assert_eq!(state.current_question_index, 1);
        state.previous();
        assert_eq!(state.current_question_index, 0);
        state.previous();
        assert_eq!(state.current_question_index, 0);

        state.current_question_index = questions.len() - 1;
        let last_q = &questions[questions.len() - 1];
        state
            .answers
            .insert(&last_q.id.to_string(), &last_q.options[0].value);
        assert!(state.advance(&questions));
        assert_eq!(state.current_question_index, questions.len() - 1);
    }

    #[test]
    fn scoring_success_error_and_modal_transitions_are_explicit() {
        use recommender_core::Engine;

        let labels = [
            Metric::PrecisionAt1,
            Metric::PrecisionAt3,
            Metric::PrecisionAt10,
            Metric::RecallAt10,
            Metric::F1At10,
            Metric::MeanReciprocalRank,
        ]
        .map(Metric::label);
        assert_eq!(labels[0], "Precision@1");
        assert_eq!(labels[5], "Confidence & Diversity");

        let mut state = AppState {
            screen: Screen::Questionnaire,
            ..AppState::default()
        };
        let answers = state.begin_scoring();
        assert!(state.scoring);
        let result = Engine::reference()
            .unwrap()
            .recommend_typed(&answers)
            .map_err(|error| error.to_string());
        state.finish_scoring(result);
        assert_eq!(state.screen, Screen::Results);
        assert!(state.recommendation.is_some());
        assert!(!state.scoring);

        state.open_modal(Modal::Champion("Ahri".into()));
        assert_eq!(state.modal, Some(Modal::Champion("Ahri".into())));
        state.close_modal();
        assert!(state.modal.is_none());

        state.open_modal(Modal::SavedUsers(vec![StoredUserRecord::default()]));
        assert!(matches!(state.modal, Some(Modal::SavedUsers(_))));
        state.close_modal();
        assert!(state.modal.is_none());

        state.screen = Screen::Questionnaire;
        state.begin_scoring();
        state.finish_scoring(Err("scoring failed".into()));
        assert_eq!(state.screen, Screen::Questionnaire);
        assert_eq!(state.error.as_deref(), Some("scoring failed"));
        assert!(!state.scoring);
    }

    #[test]
    fn restart_clears_quiz_state_and_modal_but_retains_registration_values() {
        let mut state = AppState::default();
        state.begin_registration();
        state.registration.full_name = "Synthetic".into();
        state.answers.insert("1", "Easy (1-3)");
        state.open_modal(Modal::Metric(Metric::PrecisionAt1));
        state.error = Some("test error".into());

        state.restart();
        assert_eq!(state.screen, Screen::Start);
        assert!(state.answers.0.is_empty());
        assert_eq!(state.registration.full_name, "Synthetic");
        assert!(state.modal.is_none());
        assert!(state.error.is_none());

        state.open_analytics();
        assert_eq!(state.screen, Screen::PlayerAnalytics);
        state.open_champion_distribution();
        assert_eq!(state.screen, Screen::ChampionDistribution);
        state.return_to_start();
        assert_eq!(state.screen, Screen::Start);
    }

    #[test]
    fn test_model_trust_lab_and_tie_breaker_defaults() {
        let mut state = AppState::default();
        assert!(state.model_trust_lab_open);
        assert_eq!(state.tie_breaker_answers.len(), 5);

        state.model_trust_lab_open = false;
        assert!(!state.model_trust_lab_open);
    }
}
