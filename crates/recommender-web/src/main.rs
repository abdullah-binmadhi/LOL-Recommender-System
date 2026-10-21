#[cfg(target_arch = "wasm32")]
mod analytics_view;
#[cfg(target_arch = "wasm32")]
mod components;
#[cfg(any(target_arch = "wasm32", test))]
mod state;
#[cfg(target_arch = "wasm32")]
mod storage;

#[cfg(target_arch = "wasm32")]
use components::ui::*;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use recommender_core::{
    contracts::{
        EffectiveCatalog, EffectiveQuestion, EvaluationResult, QuestionOption,
        QuestionnaireDefinition, RecommendationResult,
    },
    Engine,
};
#[cfg(target_arch = "wasm32")]
use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use state::{filtered_options, AppState, Metric, Modal, Screen};
#[cfg(target_arch = "wasm32")]
use std::{collections::BTreeMap, sync::Arc};

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
#[component]
fn App() -> impl IntoView {
    let questions = Arc::new(
        QuestionnaireDefinition::current()
            .expect("bundled production questionnaire must remain valid")
            .effective_questions(),
    );
    let catalog = Arc::new(
        EffectiveCatalog::current().expect("bundled effective champion catalog must remain valid"),
    );
    let analytics_summary = Arc::new(recommender_core::analytics::default_analytics_summary());
    let counters: BTreeMap<String, CounterRecord> =
        serde_json::from_str(include_str!("../../../src/data/champion_counters.json"))
            .expect("bundled champion matchup data must remain valid");
    let counters = Arc::new(counters);
    let state = RwSignal::new(AppState::default());

    if let Some(win) = web_sys::window() {
        if let Ok(hash) = win.location().hash() {
            if hash == "#analytics" {
                state.update(AppState::open_analytics);
            }
        }
        if let Ok(search) = win.location().search() {
            if search.contains("view=analytics") {
                state.update(AppState::open_analytics);
            }
        }
    }

    let hash_listener = window_event_listener(leptos::ev::hashchange, move |_| {
        if let Some(win) = web_sys::window() {
            if let Ok(hash) = win.location().hash() {
                if hash == "#analytics" {
                    state.update(AppState::open_analytics);
                } else if hash.is_empty() || hash == "#" {
                    state.update(AppState::return_to_start);
                }
            }
        }
    });
    on_cleanup(move || hash_listener.remove());

    let keyboard_listener =
        window_event_listener(leptos::ev::keydown, move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                state.update(AppState::close_modal);
            } else if event.ctrl_key()
                && event.shift_key()
                && (event.key() == "E" || event.key() == "e")
            {
                event.prevent_default();
                let _ = storage::export_users_csv();
            } else if event.ctrl_key()
                && event.shift_key()
                && (event.key() == "A" || event.key() == "a")
            {
                event.prevent_default();
                storage::show_admin_panel();
            }
        });
    on_cleanup(move || keyboard_listener.remove());
    let modal_catalog = Arc::clone(&catalog);
    let view_analytics_summary = Arc::clone(&analytics_summary);

    view! {
        <main class="container">
            <header class="header">
                <h1><i class="league-icon"></i> " League of Legends Champion Recommender"</h1>
                <p>"Discover your perfect champion match with AI-powered recommendations"</p>
            </header>

            {move || match state.with(|current| current.screen) {
                Screen::Start => view_start(state).into_any(),
                Screen::Registration => view_registration(state).into_any(),
                Screen::Questionnaire => {
                    view_questionnaire(state, Arc::clone(&questions)).into_any()
                }
                Screen::Results => view_analysis(
                    state,
                    Arc::clone(&catalog),
                    Arc::clone(&counters),
                )
                .into_any(),
                Screen::Analytics => {
                    analytics_view::view_analytics(state, Arc::clone(&view_analytics_summary)).into_any()
                }
            }}

            {move || match state.with(|current| current.modal.clone()) {
                Some(Modal::Champion(name)) => {
                    let recommendation = state.with(|current| current.recommendation.clone());
                    match recommendation {
                        Some(recommendation) => view_champion_modal(
                            state,
                            name,
                            Arc::clone(&modal_catalog),
                            recommendation,
                        )
                        .into_any(),
                        None => ().into_view().into_any(),
                    }
                }
                Some(Modal::Metric(metric)) => {
                    let recommendation = state.with(|current| current.recommendation.clone());
                    match recommendation {
                        Some(recommendation) => {
                            view_metric_modal(state, metric, recommendation).into_any()
                        }
                        None => ().into_view().into_any(),
                    }
                }
                Some(Modal::SavedUsers(users)) => {
                    view_saved_users_modal(state, users).into_any()
                }
                None => ().into_view().into_any(),
            }}
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_start(state: RwSignal<AppState>) -> impl IntoView {
    let featured_champions = ["Jinx", "Yasuo", "Ahri", "Zed", "Lux"]
        .into_iter()
        .map(|champion| {
            let image = champion_image_url(champion, None);
            view! {
                <div class="showcase-avatar-wrapper">
                    <img src=image alt=champion class="showcase-avatar-img champion-avatar" loading="lazy" />
                </div>
            }
        })
        .collect_view();

    view! {
        <section id="start-screen" class="card start-screen hero-card ui-card active">
            <div class="hero-section">
                <div class="hero-pill">
                    <Badge variant=BadgeVariant::Default class="ui-badge-gold">
                        "✦ Powered by Rust & WebAssembly"
                    </Badge>
                </div>

                <div class="hero-content">
                    <h2 class="hero-title">
                        "Find Your " <span>"Signature Champion"</span>
                    </h2>
                    <p class="hero-description">
                        "Discover the optimal League of Legends champion tailored to your psychological profile, mechanical comfort, and role preferences using a 3-model machine learning ensemble."
                    </p>
                </div>

                <div class="champion-showcase">
                    <div class="champion-showcase-grid champion-images">{featured_champions}</div>
                </div>

                <div class="features feature-cards-grid">
                    <div class="feature feature-box">
                        <div class="feature-icon feature-icon-badge">"🧠"</div>
                        <h4>"3-Model ML Ensemble"</h4>
                        <p>"KNN, Decision Tree, and Random Forest models evaluate 150+ champions simultaneously."</p>
                    </div>
                    <div class="feature feature-box">
                        <div class="feature-icon feature-icon-badge">"⚡"</div>
                        <h4>"Sub-Millisecond Speed"</h4>
                        <p>"Native Rust core computes exact ranking, tie-breaking, and evaluation metrics in 0.5ms."</p>
                    </div>
                    <div class="feature feature-box">
                        <div class="feature-icon feature-icon-badge">"🎯"</div>
                        <h4>"Precision Compatibility"</h4>
                        <p>"Deep playstyle matching with detailed math breakdown and situational counters."</p>
                    </div>
                </div>

                <div class="cta-section hero-cta-group">
                    <button
                        type="button"
                        class="btn btn-hero ui-btn ui-btn-primary ui-btn-lg"
                        on:click=move |_| state.update(AppState::begin_registration)
                    >
                        <span class="btn-text">"Find My Champion"</span>
                        <span class="btn-icon">" →"</span>
                    </button>
                    <button
                        type="button"
                        class="btn btn-hero ui-btn ui-btn-outline ui-btn-lg"
                        on:click=move |_| {
                            if let Some(win) = web_sys::window() {
                                let _ = win.location().set_hash("#analytics");
                            }
                            state.update(AppState::open_analytics);
                        }
                    >
                        <span class="btn-text">"Global Analytics"</span>
                        <span class="btn-icon">" 📊"</span>
                    </button>
                    <button
                        type="button"
                        class="ui-btn ui-btn-ghost ui-btn-lg"
                        on:click=move |_| {
                            let users = storage::load_users();
                            state.update(|current| current.open_modal(Modal::SavedUsers(users)));
                        }
                    >
                        "📁 Saved Profiles"
                    </button>
                </div>
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_registration(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <section id="user-registration" class="card user-registration ui-card registration-container active">
            <div class="registration-header ui-card-header">
                <div class="ui-card-title">
                    <i class="summoner-scroll"></i> " Summoner Profile"
                </div>
                <p class="ui-card-description">
                    "Personalize your questionnaire so the machine learning ensemble calibrates its recommendation."
                </p>
            </div>

            <form
                id="user-form"
                class="registration-form ui-card-content"
                on:submit=move |event| {
                    event.prevent_default();
                    let is_valid = state.with(|current| current.registration.is_valid());
                    if is_valid {
                        let draft = state.with(|current| current.registration.clone());
                        let _ = storage::append_user(
                            &draft.full_name,
                            &draft.email,
                            &draft.phone,
                            &draft.age,
                            &draft.gender,
                            &draft.experience,
                        );
                    }
                    state.update(|current| {
                        current.submit_registration();
                    });
                }
            >
                <div class="form-group form-field-group">
                    <label for="fullName" class="form-label">"Summoner Name / Full Name"</label>
                    <input
                        type="text"
                        id="fullName"
                        name="fullName"
                        class="form-input"
                        placeholder="e.g. Faker"
                        required=true
                        prop:value=move || state.with(|current| current.registration.full_name.clone())
                        on:input=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.full_name = value);
                        }
                    />
                </div>

                <div class="form-group form-field-group">
                    <label for="email" class="form-label">"Email Address"</label>
                    <input
                        type="email"
                        id="email"
                        name="email"
                        class="form-input"
                        placeholder="summoner@example.com"
                        required=true
                        prop:value=move || state.with(|current| current.registration.email.clone())
                        on:input=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.email = value);
                        }
                    />
                </div>

                <div class="form-group form-field-group">
                    <label for="age" class="form-label">"Age"</label>
                    <input
                        type="number"
                        id="age"
                        name="age"
                        class="form-input"
                        placeholder="e.g. 21"
                        min="13"
                        max="100"
                        required=true
                        prop:value=move || state.with(|current| current.registration.age.clone())
                        on:input=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.age = value);
                        }
                    />
                </div>

                <div class="form-group form-field-group">
                    <label for="experience" class="form-label">"League Experience Level"</label>
                    <select
                        id="experience"
                        name="experience"
                        class="form-input"
                        required=true
                        prop:value=move || state.with(|current| current.registration.experience.clone())
                        on:change=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.experience = value);
                        }
                    >
                        <option value="">"Select experience level"</option>
                        <option value="New Player">"New Player (Under 30 Levels)"</option>
                        <option value="Beginner">"Beginner (Iron / Bronze)"</option>
                        <option value="Intermediate">"Intermediate (Silver / Gold / Platinum)"</option>
                        <option value="Advanced">"Advanced (Emerald / Diamond)"</option>
                        <option value="Expert">"Expert (Master / Grandmaster / Challenger)"</option>
                    </select>
                </div>

                <div class="form-group form-field-group">
                    <label for="gender" class="form-label">"Gender (Optional)"</label>
                    <select
                        id="gender"
                        name="gender"
                        class="form-input"
                        prop:value=move || state.with(|current| current.registration.gender.clone())
                        on:change=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.gender = value);
                        }
                    >
                        <option value="">"Prefer not to say"</option>
                        <option value="Male">"Male"</option>
                        <option value="Female">"Female"</option>
                        <option value="Non-binary">"Non-binary"</option>
                        <option value="Other">"Other"</option>
                    </select>
                </div>

                <div class="form-group form-field-group">
                    <label for="phone" class="form-label">"Phone (Optional)"</label>
                    <input
                        type="tel"
                        id="phone"
                        name="phone"
                        class="form-input"
                        placeholder="Optional contact"
                        prop:value=move || state.with(|current| current.registration.phone.clone())
                        on:input=move |event| {
                            let value = event_target_value(&event);
                            state.update(|current| current.registration.phone = value);
                        }
                    />
                </div>

                <div class="form-buttons ui-card-footer" style="padding: 24px 0 0; border-top: 1px solid var(--border-subtle); display: flex; justify-content: space-between;">
                    <button
                        type="button"
                        class="btn btn-secondary ui-btn ui-btn-secondary"
                        on:click=move |_| state.update(AppState::return_to_start)
                    >
                        "← Back"
                    </button>
                    <button type="submit" class="btn btn-primary ui-btn ui-btn-primary ui-btn-lg">
                        "Begin Questionnaire →"
                    </button>
                </div>
            </form>

            {move || state.with(|current| current.error.clone()).map(|error| view! {
                <p class="application-error" role="alert">{error}</p>
            })}
            <p class="registration-note" style="color: var(--text-muted); font-size: 0.85rem; text-align: center; margin-top: 16px;">
                "✦ Client-Side Secure: Your inputs are processed locally in WebAssembly."
            </p>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_questionnaire(
    state: RwSignal<AppState>,
    questions: Arc<Vec<EffectiveQuestion>>,
) -> impl IntoView {
    let question_count = questions.len();

    view! {
        <div id="questions-container">
            {move || {
                let snapshot = state.get();
                let Some(question) = questions.get(snapshot.current_question_index).cloned() else {
                    return view! {
                        <div class="card question-card active">
                            <p class="question-empty">"The questionnaire could not be loaded."</p>
                        </div>
                    }
                    .into_any();
                };

                let current_answer = snapshot
                    .answers
                    .0
                    .get(&question.id.to_string())
                    .and_then(Option::as_deref)
                    .unwrap_or_default()
                    .to_owned();
                let options = filtered_options(&question, &snapshot.answers)
                    .into_iter()
                    .map(|option: QuestionOption| {
                        let option_value = option.value.clone();
                        let selected = current_answer == option.value;
                        let question_id = question.id;
                        let questions_for_selection = Arc::clone(&questions);
                        view! {
                            <button
                                type="button"
                                class=if selected { "ui-option-card option selected active" } else { "ui-option-card option" }
                                aria-pressed=if selected { "true" } else { "false" }
                                on:click=move |_| {
                                    let selected_value = option_value.clone();
                                    state.update(|current| {
                                        current.select_answer(
                                            question_id,
                                            selected_value,
                                            &questions_for_selection,
                                        );
                                    });
                                }
                            >
                                <div class="option-radio-ring">
                                    <div class="option-radio-dot" />
                                </div>
                                <div class="option-text-label">
                                    <div style="font-weight: 600; font-size: 1.05rem; color: var(--text-main);">{option.text}</div>
                                    {(!option.description.is_empty()).then(|| view! {
                                        <p class="option-description" style="margin: 4px 0 0; font-size: 0.9rem; color: var(--text-muted);">{option.description}</p>
                                    })}
                                </div>
                            </button>
                        }
                    })
                    .collect_view();

                let progress = ((snapshot.current_question_index as f64 / question_count as f64)
                    * 100.0)
                    .round() as u32;
                let is_last = snapshot.current_question_index + 1 == question_count;
                let can_advance = snapshot.can_advance(&questions) && !snapshot.scoring;
                let questions_for_advance = Arc::clone(&questions);

                view! {
                    <section
                        id=format!("question-{}", question.id)
                        class="card question-card ui-card active"
                        aria-label="Questionnaire"
                        style="max-width: 760px; margin: 0 auto;"
                    >
                        <div class="progress-container" style="margin-bottom: 24px;">
                            <div class="progress-info quiz-header-meta">
                                <span class="progress-text quiz-step-title">
                                    {format!("Question {} of {}", question.id, question_count)}
                                </span>
                                <span class="progress-text ui-badge ui-badge-cyan">
                                    {format!("{}% Complete", progress)}
                                </span>
                            </div>
                            <div class="progress-bar ui-progress-track" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow=progress>
                                <div class="progress-fill ui-progress-indicator" style=format!("width: {progress}%")></div>
                            </div>
                        </div>

                        <div class="question" style="margin-bottom: 28px;">
                            <h2 style="font-size: 1.5rem; font-weight: 700; color: var(--text-main); margin-bottom: 16px; line-height: 1.3;">
                                {question.text}
                            </h2>
                            <div class="options quiz-options-list">{options}</div>
                        </div>

                        {move || state.with(|current| current.error.clone()).map(|error| view! {
                            <p class="application-error" role="alert">{error}</p>
                        })}

                        <div class="buttons" style="border-top: 1px solid var(--border-subtle); padding-top: 20px;">
                            <div class="button-group" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                                {if snapshot.current_question_index > 0 {
                                    view! {
                                        <button
                                            type="button"
                                            class="btn btn-secondary ui-btn ui-btn-secondary"
                                            on:click=move |_| state.update(AppState::previous)
                                        >
                                            "Previous"
                                        </button>
                                    }
                                    .into_any()
                                } else {
                                    view! { <div class="btn-spacer"></div> }.into_any()
                                }}
                                <button
                                    type="button"
                                    class="btn btn-primary ui-btn ui-btn-primary ui-btn-lg"
                                    disabled=!can_advance
                                    on:click=move |_| {
                                        if !state.with_untracked(|current| current.can_advance(&questions_for_advance)) {
                                            return;
                                        }

                                        if is_last {
                                            let answers = state.with_untracked(|current| current.answers.clone());
                                            state.update(|current| {
                                                current.begin_scoring();
                                            });
                                            let result = Engine::reference()
                                                .and_then(|engine| engine.recommend_typed(&answers))
                                                .map_err(|error| error.to_string());
                                            if let Ok(ref rec) = result {
                                                let top_champs: Vec<String> = rec
                                                    .top10
                                                    .iter()
                                                    .map(|c| c.champion_name.clone())
                                                    .collect();
                                                let _ = storage::update_recent_champions(&top_champs);
                                            }
                                            state.update(|current| current.finish_scoring(result));
                                        } else {
                                            state.update(|current| {
                                                current.advance(&questions_for_advance);
                                            });
                                        }
                                    }
                                >
                                    {if snapshot.scoring {
                                        "Loading Rust scoring engine..."
                                    } else if is_last {
                                        "Run All ML Algorithms"
                                    } else {
                                        "Next"
                                    }}
                                </button>
                            </div>
                        </div>
                    </section>
                }
                .into_any()
            }}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_analysis(
    state: RwSignal<AppState>,
    catalog: Arc<EffectiveCatalog>,
    counters: Arc<BTreeMap<String, CounterRecord>>,
) -> impl IntoView {
    view! {
        <section id="detailed-analysis" class="card detailed-analysis active">
            <div id="analysis-content">
                {move || match state.with(|current| current.recommendation.clone()) {
                    Some(recommendation) => view_analysis_content(
                        state,
                        Arc::clone(&catalog),
                        Arc::clone(&counters),
                        recommendation,
                    )
                    .into_any(),
                    None => view! {
                        <p class="application-error" role="alert">
                            "Recommendations are unavailable. Please restart and try again."
                        </p>
                    }
                    .into_any(),
                }}
            </div>
            <div class="buttons results-action-bar" style="margin-top: 36px; display: flex; justify-content: center; gap: 16px; flex-wrap: wrap;">
                <button
                    type="button"
                    class="btn btn-primary ui-btn ui-btn-primary ui-btn-lg"
                    on:click=move |_| {
                        let users = storage::load_users();
                        if users.is_empty() {
                            if let Some(w) = web_sys::window() {
                                let _ = w.alert_with_message("No user data available to display.");
                            }
                        } else {
                            state.update(|current| current.open_modal(Modal::SavedUsers(users)));
                            if let Some(w) = web_sys::window() {
                                let _ = w.alert_with_message("Results displayed on web page!");
                            }
                        }
                    }
                >
                    "Save"
                </button>
                <button
                    type="button"
                    class="btn btn-primary ui-btn ui-btn-ghost ui-btn-lg"
                    on:click=move |_| state.update(AppState::restart)
                >
                    "Try Again"
                </button>
                <button
                    type="button"
                    class="btn btn-secondary ui-btn ui-btn-secondary ui-btn-lg"
                    on:click=move |_| {
                        let _ = storage::export_users_csv();
                    }
                >
                    "Export CSV"
                </button>
                <button
                    type="button"
                    class="btn btn-hero ui-btn ui-btn-outline ui-btn-lg"
                    on:click=move |_| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_hash("#analytics");
                        }
                        state.update(AppState::open_analytics);
                    }
                >
                    "Analytics"
                </button>
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_analysis_content(
    state: RwSignal<AppState>,
    catalog: Arc<EffectiveCatalog>,
    counters: Arc<BTreeMap<String, CounterRecord>>,
    recommendation: RecommendationResult,
) -> impl IntoView {
    let top10 = recommendation.top10.clone();
    let top_count = top10.len();
    let champions = top10
        .iter()
        .enumerate()
        .filter_map(|(index, score)| {
            catalog
                .0
                .iter()
                .find(|record| record.name == score.champion_name)
                .map(|record| (index, score.clone(), record.clone()))
        })
        .map(|(index, score, record)| {
            let name = score.champion_name.clone();
            let champion = record.data;
            let image = champion_image_url(&name, Some(&champion));
            let tier = champion
                .additional_fields
                .get("stats")
                .and_then(|stats| stats.get("tier"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("B")
                .to_owned();
            let win_rate = champion
                .additional_fields
                .get("stats")
                .and_then(|stats| stats.get("winRate"))
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(50.0);
            let role_match = if recommendation.features.role == "No Preference"
                || champion.role == recommendation.features.role
            {
                100.0
            } else {
                70.0
            };
            let difficulty_match = (100.0
                - (champion.difficulty - recommendation.features.difficulty).abs() * 10.0)
                .max(0.0);
            let toughness_match = (100.0 - (champion.toughness - 5.0).abs() * 8.0).max(0.0);
            let is_top_pick = index == 0;
            let state_for_card = state;
            let modal_name = name.clone();
            let card_class = if is_top_pick {
                "match-card winner"
            } else {
                "match-card"
            };

            view! {
                <button
                    type="button"
                    class=card_class
                    aria-label=format!("Open {} recommendation details", name)
                    on:click=move |_| {
                        state_for_card.update(|current| {
                            current.open_modal(Modal::Champion(modal_name.clone()));
                        });
                    }
                >
                    <div class="match-header">
                        <img src=image alt=name.clone() class="match-champion-img" loading="lazy" />
                        <div class="match-info">
                            <h4>
                                {name.clone()}
                                {if is_top_pick {
                                    view! { <span class="winner-badge">"TOP PICK"</span> }.into_any()
                                } else {
                                    ().into_view().into_any()
                                }}
                            </h4>
                            <div class="match-tier">{format!("{} • {:.1}% WR", tier, win_rate)}</div>
                            <div class="algorithm-info">
                                {format!("Unified ML Score • {:.1}% match", score.average)}
                            </div>
                        </div>
                    </div>

                    <div class="compatibility-score">
                        <div class="score-label">"Compatibility Score"</div>
                        <div class="score-value">{format!("{:.0}%", score.average)}</div>
                    </div>

                    <div class="match-breakdown">
                        <div class="breakdown-item">
                            <span class="breakdown-label">"Random Forest"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {}%", score.random_forest.clamp(0.0, 100.0))></span></span>
                            <span class="breakdown-value">{format!("{:.0}%", score.random_forest)}</span>
                        </div>
                        <div class="breakdown-item">
                            <span class="breakdown-label">"Decision Tree"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {}%", score.decision_tree.clamp(0.0, 100.0))></span></span>
                            <span class="breakdown-value">{format!("{:.0}%", score.decision_tree)}</span>
                        </div>
                        <div class="breakdown-item">
                            <span class="breakdown-label">"KNN"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {}%", score.knn.clamp(0.0, 100.0))></span></span>
                            <span class="breakdown-value">{format!("{:.0}%", score.knn)}</span>
                        </div>
                        <div class="breakdown-item">
                            <span class="breakdown-label">"Role Match"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {role_match}%")></span></span>
                            <span class="breakdown-value">{format!("{role_match:.0}%")}</span>
                        </div>
                        <div class="breakdown-item">
                            <span class="breakdown-label">"Difficulty"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {difficulty_match}%")></span></span>
                            <span class="breakdown-value">{format!("{difficulty_match:.0}%")}</span>
                        </div>
                        <div class="breakdown-item">
                            <span class="breakdown-label">"Survivability"</span>
                            <span class="breakdown-bar"><span class="breakdown-fill" style=format!("width: {toughness_match}%")></span></span>
                            <span class="breakdown-value">{format!("{toughness_match:.0}%")}</span>
                        </div>
                    </div>

                    <div class="champion-stats-grid">
                        <div class="stat-item"><span class="stat-label">"Damage"</span><span class="stat-value">{format!("{}/10", champion.damage)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Toughness"</span><span class="stat-value">{format!("{}/10", champion.toughness)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Mobility"</span><span class="stat-value">{format!("{}/10", champion.mobility)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Control"</span><span class="stat-value">{format!("{}/10", champion.control)}</span></div>
                    </div>

                    <div class="champion-highlights">
                        <div class="highlight-title">"Key Strengths:"</div>
                        <div class="highlight-tags">
                            {highlight_tag(champion.damage >= 8.0, "damage", "High Damage")}
                            {highlight_tag(champion.toughness >= 7.0, "tank", "Tanky")}
                            {highlight_tag(champion.mobility >= 7.0, "mobile", "Mobile")}
                            {highlight_tag(champion.control >= 7.0, "control", "Crowd Control")}
                            {highlight_tag(champion.utility >= 8.0, "utility", "High Utility")}
                            {highlight_tag(champion.difficulty <= 3.0, "easy", "Beginner Friendly")}
                        </div>
                    </div>
                </button>
            }
        })
        .collect_view();

    let top = top10.first().cloned();
    let alternatives = top10.iter().skip(1).take(2).filter_map(|score| {
        catalog
            .0
            .iter()
            .find(|record| record.name == score.champion_name)
            .map(|record| (score.clone(), record.clone()))
    });
    let alternatives = alternatives
        .map(|(score, record)| {
            let name = score.champion_name.clone();
            let image = champion_image_url(&name, Some(&record.data));
            view! {
                <div class="alternative-item">
                    <img src=image alt=name.clone() class="alt-champion-img" loading="lazy" />
                    <div class="alt-info">
                        <div class="alt-name">{name}</div>
                        <div class="alt-reason">
                            {format!("{:.0}% match • {}", score.average, record.data.role)}
                        </div>
                    </div>
                </div>
            }
        })
        .collect_view();

    let average_win_rate = if top10.is_empty() {
        50.0
    } else {
        top10
            .iter()
            .filter_map(|score| {
                catalog
                    .0
                    .iter()
                    .find(|record| record.name == score.champion_name)
                    .and_then(|record| record.data.additional_fields.get("stats"))
                    .and_then(|stats| stats.get("winRate"))
                    .and_then(serde_json::Value::as_f64)
            })
            .sum::<f64>()
            / top10.len() as f64
    };
    let average_difficulty = if top10.is_empty() {
        5.0
    } else {
        top10
            .iter()
            .filter_map(|score| {
                catalog
                    .0
                    .iter()
                    .find(|record| record.name == score.champion_name)
                    .map(|record| record.data.difficulty)
            })
            .sum::<f64>()
            / top10.len() as f64
    };
    let all_positive_win_rates = top10.iter().all(|score| {
        catalog
            .0
            .iter()
            .find(|record| record.name == score.champion_name)
            .and_then(|record| record.data.additional_fields.get("stats"))
            .and_then(|stats| stats.get("winRate"))
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(50.0)
            >= 50.0
    });

    let top_name = top
        .as_ref()
        .map(|score| score.champion_name.clone())
        .unwrap_or_default();
    let guide_url = format!(
        "https://www.mobafire.com/league-of-legends/{}-guide",
        top_name.to_lowercase()
    );
    let build_url = format!(
        "https://u.gg/lol/champions/{}/build",
        top_name.to_lowercase()
    );
    let matchup_key = normalize_name(&top_name);
    let matchup = counters.get(&matchup_key);
    let strong_against = matchup
        .map(|record| record.strong_against.clone())
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["Immobile champions".into(), "Squishy targets".into()]);
    let weak_against = matchup
        .map(|record| record.weak_against.clone())
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["Mobile champions".into(), "Tanks".into()]);
    let synergies = matchup
        .map(|record| record.synergizes_with.clone())
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["Engage supports".into(), "CC-heavy junglers".into()]);

    let metrics = recommendation
        .evaluation
        .as_ref()
        .map(|evaluation| view_evaluation_metrics(state, evaluation.clone(), top_count).into_any());
    let tips = role_tips(&recommendation.features.role)
        .iter()
        .take(6)
        .map(|(icon, tip)| {
            view! {
                <div class="tip-item">
                    <span class="tip-icon">{*icon}</span>
                    <div class="tip-text">{*tip}</div>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="analysis-header">
            <h2><i class="analysis-icon"></i> " Detailed Champion Analysis"</h2>
            <p>"Personalized champion recommendations based on your preferences"</p>
        </div>

        <section class="champion-analysis-section">
            <h3><i class="champion-finder-icon"></i> " Champion Match Analysis"</h3>
            <div class="analysis-explanation">
                <p>{format!("Based on your preferences, here are your top {top_count} champion recommendations:")}</p>
            </div>
            <div class="match-analysis-grid">{champions}</div>
        </section>

        {metrics}

        <section class="recommendations-section">
            <h3>"🎯 Personalized Recommendations & Next Steps"</h3>
            <div class="recommendation-cards">
                <div class="recommendation-card primary-rec">
                    <div class="rec-icon"><i class="league-icon"></i></div>
                    <h4>"Primary Recommendation"</h4>
                    <div class="primary-champion">{top_name.clone()}</div>
                    {top.as_ref().map(|score| view! {
                        <p style="color: #333; text-align: center; margin-bottom: 15px; font-size: 0.95rem;">
                            {format!("Top recommendation based on unified ML analysis with {:.1}% average match score.", score.average)}
                        </p>
                    })}
                    <div class="action-buttons">
                        <a class="action-btn" href=guide_url target="_blank" rel="noopener noreferrer">"📚 View Guide"</a>
                        <a class="action-btn" href=build_url target="_blank" rel="noopener noreferrer">"⚙️ Build Guide"</a>
                    </div>
                </div>

                <div class="recommendation-card counter-synergies">
                    <div class="rec-icon"><i class="champion-finder-icon"></i></div>
                    <h4>"Counter Matchups & Synergies"</h4>
                    <div class="matchup-list">
                        <div class="matchup-category"><span class="matchup-icon">"✓"</span> "Strong Against:"</div>
                        {matchup_items(&strong_against, "strong-against")}
                        <div class="matchup-category"><span class="matchup-icon">"✕"</span> "Weak Against:"</div>
                        {matchup_items(&weak_against, "weak-against")}
                        <div class="matchup-category"><span class="matchup-icon">"🤝"</span> "Synergizes With:"</div>
                        {matchup_items(&synergies, "synergizes-with")}
                    </div>
                </div>

                <div class="recommendation-card alternative-picks">
                    <div class="rec-icon"><i class="target-icon"></i></div>
                    <h4>"Alternative Champions"</h4>
                    <div class="alternatives-list">{alternatives}</div>
                </div>

                <div class="recommendation-card meta-insights">
                    <div class="rec-icon"><i class="analysis-icon"></i></div>
                    <h4>"Current Meta Insights"</h4>
                    <div class="meta-info">
                        <div class="meta-stat">
                            <span class="meta-label">"Average Win Rate:"</span>
                            <span class="meta-value">{format!("{average_win_rate:.1}%")}</span>
                        </div>
                        <div class="meta-stat">
                            <span class="meta-label">"Avg. Difficulty:"</span>
                            <span class="meta-value">{format!("{average_difficulty:.1}/10")}</span>
                        </div>
                        <div class="meta-recommendation">
                            {if all_positive_win_rates {
                                "✅ All recommendations are currently meta-viable with positive win rates!"
                            } else {
                                "⚠️ Some recommendations may require extra practice due to current meta shifts."
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="final-tips">
            <h4><i class="sparkle-icon-sm"></i> " Pro Tips"</h4>
            <div class="tips-grid">{tips}</div>
            <div class="champion-tip-source">
                <small>{format!("🎯 Role-based tips for {top_name} - Master these fundamentals to excel with your recommended champion!")}</small>
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_evaluation_metrics(
    state: RwSignal<AppState>,
    evaluation: EvaluationResult,
    recommended_count: usize,
) -> impl IntoView {
    let values = [
        (
            Metric::PrecisionAt1,
            evaluation.precision_at1,
            "Top recommendation relevance",
            "metric-card-p1",
        ),
        (
            Metric::PrecisionAt3,
            evaluation.precision_at3,
            "Top 3 recommendations relevance",
            "metric-card-p3",
        ),
        (
            Metric::PrecisionAt10,
            evaluation.precision_at10,
            "Top 10 recommendations relevance",
            "metric-card-p5",
        ),
        (
            Metric::RecallAt10,
            evaluation.recall_at10,
            "Coverage of relevant champions",
            "metric-card-r5",
        ),
        (
            Metric::F1At10,
            evaluation.f1_at10,
            "Balanced accuracy & coverage",
            "metric-card-f1",
        ),
        (
            Metric::MeanReciprocalRank,
            evaluation.mrr,
            "Algorithm confidence & variety balance",
            "metric-card-mrr",
        ),
    ];
    let metric_cards = values
        .into_iter()
        .map(|(metric, value, description, class)| {
            let modal = metric;
            view! {
                <button
                    type="button"
                    class=format!("metric-card {class}")
                    on:click=move |_| state.update(|current| current.open_modal(Modal::Metric(modal)))
                >
                    <div class="metric-label">{metric.label()}</div>
                    <div class=format!("metric-value {}", metric_class(value))>
                        {format!("{:.1}%", value * 100.0)}
                    </div>
                    <div class="metric-description">{description}</div>
                </button>
            }
        })
        .collect_view();

    let relevant_count = evaluation.relevant_champions.len();
    let performance_level = if evaluation.precision_at10 >= 0.6 && evaluation.mrr >= 0.5 {
        "Excellent"
    } else if evaluation.precision_at10 >= 0.4 || evaluation.mrr >= 0.3 {
        "Good"
    } else {
        "Needs Improvement"
    };
    let performance_message = match performance_level {
        "Excellent" => "The recommendations are highly aligned with your preferences! The top suggestions should be great matches for your playstyle.",
        "Good" => "The recommendations show good alignment with your preferences. You should find several suitable champions in the top suggestions.",
        _ => "The recommendations show room for improvement. Consider adjusting your preferences for better matches.",
    };

    view! {
        <section class="evaluation-metrics-section">
            <h3><span class="metrics-icon">"📊"</span> " Recommendation Quality Metrics"</h3>
            <div class="metrics-grid">{metric_cards}</div>
            <div class="performance-interpretation">
                <h4>{format!("Overall Performance: {performance_level}")}</h4>
                <p>{performance_message}</p>
                <div class="performance-description">
                    "These scores use the existing preference-relevance heuristic; they are not independent measures of model accuracy."
                </div>
                <div class="metrics-summary">
                    <div>
                        <div class="metric-label">"Relevant Champions"</div>
                        <div class="performance-metric">{relevant_count}</div>
                    </div>
                    <div>
                        <div class="metric-label">"Captured in Top 10"</div>
                        <div class="performance-metric">
                            {(evaluation.recall_at10 * relevant_count as f64).round() as usize}
                        </div>
                    </div>
                    <div>
                        <div class="metric-label">"Recommended"</div>
                        <div class="performance-metric">{recommended_count}</div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn view_champion_modal(
    state: RwSignal<AppState>,
    name: String,
    catalog: Arc<EffectiveCatalog>,
    recommendation: RecommendationResult,
) -> impl IntoView {
    let Some(champion) = catalog.0.iter().find(|record| record.name == name) else {
        return view! { <div class="modal active"><p class="application-error">"Champion details are unavailable."</p></div> }.into_any();
    };
    let champion = champion.clone();
    let image = champion_image_url(&name, Some(&champion.data));
    let algorithm_details = [
        (
            "Random Forest",
            recommendation.scores.random_forest.get(&name),
        ),
        (
            "Decision Tree",
            recommendation.scores.decision_tree.get(&name),
        ),
        ("K-Nearest Neighbors", recommendation.scores.knn.get(&name)),
    ]
    .into_iter()
    .map(|(label, score)| {
        let details = score
            .map(|score| {
                serde_json::to_string_pretty(&score.details)
                    .unwrap_or_else(|_| score.details.to_string())
            })
            .unwrap_or_else(|| "No explanation details available.".into());
        let score_value = score.map(|score| score.score).unwrap_or_default();
        view! {
            <details class="algorithm-detail">
                <summary>{format!("{label} — {score_value:.1}%")}</summary>
                <pre>{details}</pre>
            </details>
        }
    })
    .collect_view();

    view! {
        <div
            class="modal ui-dialog-overlay active"
            role="presentation"
            on:click=move |_| state.update(AppState::close_modal)
        >
            <section
                class="modal-content champion-modal-content ui-dialog-content"
                role="dialog"
                aria-modal="true"
                aria-label=format!("{name} champion details")
                on:click=move |event| event.stop_propagation()
            >
                <header class="champion-modal-header" style="display: flex; gap: 20px; align-items: center; margin-bottom: 24px; position: relative;">
                    <img src=image alt=name.clone() class="champion-modal-img" style="width: 80px; height: 80px; border-radius: 16px; border: 2px solid var(--gold-primary);" />
                    <div class="champion-modal-info" style="flex: 1;">
                        <h2 style="margin: 0 0 6px; color: var(--text-main); font-size: 1.8rem;">{name.clone()}</h2>
                        <div class="champion-modal-subtitle">
                            <span class="ui-badge ui-badge-gold">{champion.data.role.clone()}</span>
                            " "
                            <span class="ui-badge ui-badge-cyan">{format!("Difficulty {:.0}/10", champion.data.difficulty)}</span>
                        </div>
                    </div>
                    <button
                        type="button"
                        class="modal-close ui-dialog-close"
                        aria-label="Close champion details"
                        on:click=move |_| state.update(AppState::close_modal)
                    >
                        "×"
                    </button>
                </header>
                <div class="modal-body">
                    <h3 style="color: var(--text-main); margin-bottom: 12px;">"Combat Attributes"</h3>
                    <div class="champion-stats-grid" style="margin-bottom: 24px;">
                        <div class="stat-item"><span class="stat-label">"Damage"</span><span class="stat-value">{format!("{}/10", champion.data.damage)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Toughness"</span><span class="stat-value">{format!("{}/10", champion.data.toughness)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Mobility"</span><span class="stat-value">{format!("{}/10", champion.data.mobility)}</span></div>
                        <div class="stat-item"><span class="stat-label">"Utility"</span><span class="stat-value">{format!("{}/10", champion.data.utility)}</span></div>
                    </div>
                    <h3 style="color: var(--text-main); margin-bottom: 8px;">"Ensemble Scoring Breakdown"</h3>
                    <p style="color: var(--text-muted); font-size: 0.9rem; margin-bottom: 16px;">
                        "Deterministic scoring contributions computed directly from the Rust scoring engine."
                    </p>
                    {algorithm_details}
                </div>
            </section>
        </div>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
fn view_metric_modal(
    state: RwSignal<AppState>,
    metric: Metric,
    recommendation: RecommendationResult,
) -> impl IntoView {
    let Some(evaluation) = recommendation.evaluation else {
        return ().into_view().into_any();
    };
    let value = metric_value(metric, &evaluation);
    let recommended: Vec<_> = recommendation
        .top10
        .iter()
        .map(|champion| champion.champion_name.clone())
        .collect();
    let relevant = evaluation.relevant_champions.clone();
    let list = recommended
        .iter()
        .take(match metric {
            Metric::PrecisionAt1 => 1,
            Metric::PrecisionAt3 => 3,
            _ => recommended.len(),
        })
        .enumerate()
        .map(|(index, name)| {
            let is_relevant = relevant.iter().any(|champion| champion == name);
            view! {
                <div class="champion-list-item">
                    <span class="champion-list-icon">{format!("#{}", index + 1)}</span>
                    <span class="champion-list-name">{name.clone()}</span>
                    <span class=if is_relevant {
                        "champion-list-badge relevant"
                    } else {
                        "champion-list-badge not-relevant"
                    }>
                        {if is_relevant { "✓ Relevant" } else { "✗ Not Optimal" }}
                    </span>
                </div>
            }
        })
        .collect_view();

    view! {
        <div
            class="modal ui-dialog-overlay active"
            role="presentation"
            on:click=move |_| state.update(AppState::close_modal)
        >
            <section
                class="modal-content ui-dialog-content"
                role="dialog"
                aria-modal="true"
                aria-label=format!("{} metric details", metric.label())
                on:click=move |event| event.stop_propagation()
            >
                <header class="modal-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <h2 style="margin: 0; color: var(--text-main); font-size: 1.4rem;">{metric.label()}</h2>
                    <button
                        type="button"
                        class="modal-close ui-dialog-close"
                        aria-label="Close metric details"
                        on:click=move |_| state.update(AppState::close_modal)
                    >
                        "×"
                    </button>
                </header>
                <div class="modal-body">
                    <div class=format!("modal-score {}", metric_class(value))>
                        <div class="modal-score-value">{format!("{:.1}%", value * 100.0)}</div>
                        <div class="modal-score-label">{metric.label()}</div>
                    </div>
                    <h3 style="color: var(--text-main); margin-top: 20px;">"What this metric means"</h3>
                    <p style="color: var(--text-muted); line-height: 1.5;">{metric_explanation(metric)}</p>
                    <h3 style="color: var(--text-main); margin-top: 20px;">"Recommendations evaluated"</h3>
                    <div class="champion-list">{list}</div>
                    <p class="metric-footnote" style="color: var(--text-subtle); font-size: 0.85rem; margin-top: 16px;">
                        {format!(
                            "The current heuristic identifies {} relevant champions from your answers. This is a recommendation evaluation aid, not an independently validated accuracy score.",
                            relevant.len()
                        )}
                    </p>
                </div>
            </section>
        </div>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
fn highlight_tag(enabled: bool, class: &'static str, label: &'static str) -> impl IntoView {
    if enabled {
        view! { <span class=format!("highlight-tag {class}")>{label}</span> }.into_any()
    } else {
        ().into_view().into_any()
    }
}

#[cfg(target_arch = "wasm32")]
fn matchup_items(items: &[String], class: &'static str) -> impl IntoView {
    items
        .iter()
        .map(|item| view! { <div class=format!("matchup-item {class}")>{item.clone()}</div> })
        .collect_view()
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
fn view_saved_users_modal(
    state: RwSignal<AppState>,
    users: Vec<recommender_core::contracts::StoredUserRecord>,
) -> impl IntoView {
    let rows = users
        .iter()
        .enumerate()
        .map(|(index, user)| {
            let id = index + 1;
            let name = user.full_name.clone().unwrap_or_default();
            let email = user.email.clone().unwrap_or_default();
            let age = match &user.age {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Number(n)) => n.to_string(),
                _ => String::new(),
            };
            let gender = user.gender.clone().unwrap_or_default();
            let experience = user.experience.clone().unwrap_or_default();
            let champion = user.recommended_champion.clone().unwrap_or_default();
            let reg_date = user.registration_date.clone().unwrap_or_default();
            view! {
                <tr style=if id % 2 == 1 { "background-color: rgba(255, 255, 255, 0.1);" } else { "background-color: rgba(255, 255, 255, 0.05);" }>
                    <td style="border: 1px solid #444; padding: 8px;">{id}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{name}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{email}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{age}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{gender}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{experience}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{champion}</td>
                    <td style="border: 1px solid #444; padding: 8px;">{reg_date}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <div
            class="modal ui-dialog-overlay active"
            role="dialog"
            aria-modal="true"
            on:click=move |_| state.update(AppState::close_modal)
        >
            <section
                class="modal-content modal-content-large ui-dialog-content"
                style="max-width: 960px; max-height: 85vh; display: flex; flex-direction: column;"
                on:click=move |event| event.stop_propagation()
            >
                <header class="modal-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <h2 style="margin: 0; color: var(--text-main); font-size: 1.4rem;">
                        <i class="analysis-icon"></i> " Saved Summoner Profiles & Results"
                    </h2>
                    <button
                        type="button"
                        class="modal-close ui-dialog-close"
                        aria-label="Close modal"
                        on:click=move |_| state.update(AppState::close_modal)
                    >
                        "×"
                    </button>
                </header>
                <div class="modal-body" style="overflow: auto; flex: 1;">
                    <table style="width: 100%; border-collapse: collapse; margin-top: 8px; font-size: 0.9rem;">
                        <thead>
                            <tr style="background: rgba(200, 170, 110, 0.15); color: var(--gold-light); border-bottom: 2px solid var(--gold-primary);">
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"ID"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Name"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Email"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Age"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Gender"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Experience"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Recommended Champion"</th>
                                <th style="border: 1px solid var(--border-subtle); padding: 10px; text-align: left;">"Registration Date"</th>
                            </tr>
                        </thead>
                        <tbody>{rows}</tbody>
                    </table>
                </div>
                <div class="modal-footer" style="padding: 20px 0 0; display: flex; justify-content: flex-end; gap: 12px; border-top: 1px solid var(--border-subtle); margin-top: 16px;">
                    <button
                        type="button"
                        class="btn btn-secondary ui-btn ui-btn-secondary"
                        on:click=move |_| {
                            let _ = storage::export_table_csv();
                        }
                    >
                        "📥 Download CSV"
                    </button>
                    <button
                        type="button"
                        class="btn btn-primary ui-btn ui-btn-primary"
                        on:click=move |_| state.update(AppState::close_modal)
                    >
                        "Close"
                    </button>
                </div>
            </section>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("recommender-web is a browser application; build it with Trunk.");
}
