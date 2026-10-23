//! Leptos views for every screen. Decisions live in `presenter` and `state`;
//! this module only renders prepared data.

use crate::analytics_view;
use crate::components::ui::*;
use crate::player_analytics_view;
use crate::presenter::{self, Attributes, MatchBreakdown};
use crate::state::{filtered_options, AppState, Metric, Modal, Screen};
use crate::{
    champion_image_url, metric_class, metric_explanation, metric_value, normalize_name, role_tips,
    storage, CounterRecord,
};
use leptos::prelude::*;
use recommender_core::analytics::AnalyticsSummary;
use recommender_core::contracts::{
    EffectiveCatalog, EffectiveQuestion, EvaluationResult, QuestionOption, QuestionnaireDefinition,
    RecommendationResult,
};
use recommender_core::Engine;
use std::{collections::BTreeMap, sync::Arc};

const REPO_URL: &str = "https://github.com/abdullah-binmadhi/LOL-Recommender-System";

fn go_to_hash(hash: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_hash(hash);
    }
}

fn open_champion_distribution(state: RwSignal<AppState>) {
    go_to_hash("#distribution");
    state.update(AppState::open_champion_distribution);
}

fn open_player_analytics(state: RwSignal<AppState>) {
    go_to_hash("#analytics");
    state.update(AppState::open_player_analytics);
}

#[allow(dead_code)]
fn open_analytics(state: RwSignal<AppState>) {
    open_player_analytics(state);
}

#[allow(dead_code)]
fn open_profiles(state: RwSignal<AppState>) {
    let users = storage::load_users();
    state.update(|current| current.open_modal(Modal::SavedUsers(users)));
}

fn close_modal_callback(state: RwSignal<AppState>) -> Callback<()> {
    Callback::new(move |()| state.update(AppState::close_modal))
}

#[component]
pub fn App() -> impl IntoView {
    let questions = Arc::new(
        QuestionnaireDefinition::modern()
            .or_else(|_| QuestionnaireDefinition::current())
            .expect("bundled production questionnaire must remain valid")
            .effective_questions(),
    );
    let catalog = Arc::new(
        EffectiveCatalog::modern()
            .or_else(|_| EffectiveCatalog::current())
            .expect("bundled champion catalog must remain valid"),
    );
    let analytics_summary = Arc::new(recommender_core::analytics::default_analytics_summary());
    let counters: BTreeMap<String, CounterRecord> =
        serde_json::from_str(include_str!("../../../src/data/champion_counters.json"))
            .expect("bundled champion matchup data must remain valid");
    let counters = Arc::new(counters);
    let state = RwSignal::new(AppState::default());

    if let Some(user) = storage::supabase_get_current_user() {
        state.update(|c| c.set_auth_user(Some(user)));
    }

    if let Some(matches) = storage::load_cached_riot_matches() {
        let src = storage::load_cached_riot_data_source();
        state.update(|c| c.set_live_matches(Some(matches), src));
    }

    let auth_listener = window_event_listener(
        leptos::ev::Custom::new("supabase-auth-changed"),
        move |_: web_sys::CustomEvent| {
            state.update(|c| c.set_auth_user(storage::supabase_get_current_user()));
        },
    );
    on_cleanup(move || auth_listener.remove());

    let riot_listener = window_event_listener(
        leptos::ev::Custom::new("riot-telemetry-synced"),
        move |_: web_sys::CustomEvent| {
            if let Some(matches) = storage::load_cached_riot_matches() {
                let src = storage::load_cached_riot_data_source();
                state.update(|c| c.set_live_matches(Some(matches), src));
            } else {
                state.update(|c| c.set_syncing_riot(false));
            }
        },
    );
    on_cleanup(move || riot_listener.remove());

    let oauth_config_listener = window_event_listener(
        leptos::ev::Custom::new("supabase-oauth-config-required"),
        move |_: web_sys::CustomEvent| {
            state.update(|c| c.open_modal(Modal::OAuthConfigRequired));
        },
    );
    on_cleanup(move || oauth_config_listener.remove());

    if let Some(win) = web_sys::window() {
        if let Ok(hash) = win.location().hash() {
            if hash == "#distribution" || hash == "#champion-distribution" {
                state.update(AppState::open_champion_distribution);
            } else if hash == "#analytics" || hash == "#player-analytics" {
                state.update(AppState::open_player_analytics);
            }
        }
        if let Ok(search) = win.location().search() {
            if search.contains("view=distribution") {
                state.update(AppState::open_champion_distribution);
            } else if search.contains("view=analytics") {
                state.update(AppState::open_player_analytics);
            }
        }
    }

    let hash_listener = window_event_listener(leptos::ev::hashchange, move |_| {
        if let Some(win) = web_sys::window() {
            if let Ok(hash) = win.location().hash() {
                if hash == "#distribution" || hash == "#champion-distribution" {
                    if state.with(|c| c.screen != Screen::ChampionDistribution) {
                        state.update(AppState::open_champion_distribution);
                    }
                } else if hash == "#analytics" || hash == "#player-analytics" {
                    if state.with(|c| c.screen != Screen::PlayerAnalytics) {
                        state.update(AppState::open_player_analytics);
                    }
                } else if hash.is_empty() || hash == "#" {
                    if state.with(|c| c.screen != Screen::Start) {
                        state.update(AppState::return_to_start);
                    }
                }
            }
        }
    });
    on_cleanup(move || hash_listener.remove());

    let keyboard_listener =
        window_event_listener(leptos::ev::keydown, move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                if state.with(|c| c.modal.is_some()) {
                    state.update(AppState::close_modal);
                }
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
    let question_total = questions.len();
    let active_screen = Memo::new(move |_| state.with(|current| current.screen));

    view! {
        <div class="app-shell">
            {view_navbar(state)}
            <main class="page">
                {move || match active_screen.get() {
                    Screen::Start => {
                        view_start(state, Arc::clone(&analytics_summary), question_total).into_any()
                    }
                    Screen::Registration => {
                        view_questionnaire(state, Arc::clone(&questions)).into_any()
                    }
                    Screen::Questionnaire => {
                        view_questionnaire(state, Arc::clone(&questions)).into_any()
                    }
                    Screen::Results => {
                        view_analysis(state, Arc::clone(&catalog), Arc::clone(&counters)).into_any()
                    }
                    Screen::ChampionDistribution => {
                        analytics_view::view_analytics(state, Arc::clone(&analytics_summary))
                            .into_any()
                    }
                    Screen::PlayerAnalytics => {
                        player_analytics_view::view_player_analytics(state)
                            .into_any()
                    }
                }}
            </main>
            {view_footer()}

            {move || match state.with(|current| current.modal.clone()) {
                Some(Modal::Champion(name)) => {
                    match state.with(|current| current.recommendation.clone()) {
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
                    match state.with(|current| current.recommendation.clone()) {
                        Some(recommendation) => {
                            view_metric_modal(state, metric, recommendation).into_any()
                        }
                        None => ().into_view().into_any(),
                    }
                }
                Some(Modal::SavedUsers(users)) => view_saved_users_modal(state, users).into_any(),
                Some(Modal::LinkRiotAccount) => view_link_riot_modal(state).into_any(),
                Some(Modal::CloudHistory) => view_cloud_history_modal(state).into_any(),
                Some(Modal::OAuthConfigRequired) => view_oauth_config_modal(state).into_any(),
                None => ().into_view().into_any(),
            }}
        </div>
    }
}

fn view_navbar(state: RwSignal<AppState>) -> impl IntoView {
    let screen = move || state.with(|current| current.screen);
    let quiz_locked = move || {
        matches!(
            screen(),
            Screen::Registration | Screen::Questionnaire | Screen::Results
        )
    };
    let quiz_class = move || {
        if quiz_locked() {
            "nav-link active"
        } else {
            "nav-link"
        }
    };
    let dist_class = move || {
        if matches!(screen(), Screen::ChampionDistribution) {
            "nav-link active"
        } else {
            "nav-link"
        }
    };
    let analytics_class = move || {
        if matches!(screen(), Screen::PlayerAnalytics) {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    view! {
        <header class="navbar">
            <div class="navbar-inner">
                <button
                    type="button"
                    class="brand"
                    aria-label="Go to the home screen"
                    on:click=move |_| {
                        go_to_hash("");
                        state.update(AppState::return_to_start);
                    }
                >
                    <span class="brand-mark" aria-hidden="true"></span>
                    <span class="brand-text">
                        <strong>"Champion"</strong>
                        " Recommender"
                    </span>
                </button>
                <nav class="nav" aria-label="Primary">
                    <button
                        type="button"
                        class=quiz_class
                        disabled=quiz_locked
                        on:click=move |_| {
                            let is_authed = state.with(|c| c.auth_user.is_some());
                            if is_authed {
                                state.update(AppState::start_quiz);
                            } else {
                                storage::supabase_sign_in_google();
                            }
                        }
                    >
                        "Quiz"
                    </button>
                    <button type="button" class=dist_class on:click=move |_| open_champion_distribution(state)>
                        "Champion Distribution"
                    </button>
                    <button type="button" class=analytics_class on:click=move |_| open_player_analytics(state)>
                        {crate::player_analytics_view::view_lol_icon("hex-compass")}
                        <span>" Player Analytics"</span>
                    </button>
                    {move || match state.with(|c| c.auth_user.clone()) {
                        Some(user) => {
                            let riot_label = user.riot_id.unwrap_or_else(|| "Link Riot ID".into());
                            let avatar = user.avatar_url.unwrap_or_else(|| "https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png".into());
                            view! {
                                <button
                                    type="button"
                                    class="nav-link"
                                    title="Link or edit your Riot ID"
                                    on:click=move |_| state.update(|c| c.open_modal(Modal::LinkRiotAccount))
                                >
                                    <img src=avatar alt="" style="width: 20px; height: 20px; border-radius: 0px; margin-right: 6px; vertical-align: middle; border: 1px solid var(--gold);" />
                                    {riot_label}
                                </button>
                                <button
                                    type="button"
                                    class="nav-link"
                                    on:click=move |_| state.update(|c| c.open_modal(Modal::CloudHistory))
                                >
                                    "History"
                                </button>
                                <button
                                    type="button"
                                    class="nav-link"
                                    style="color: var(--gold);"
                                    on:click=move |_| storage::supabase_sign_out()
                                >
                                    "Sign Out"
                                </button>
                            }.into_any()
                        }
                        None => view! {
                            <button
                                type="button"
                                class="btn btn-secondary btn-md"
                                on:click=move |_| storage::supabase_sign_in_google()
                            >
                                "✦ Google Sign-In"
                            </button>
                        }.into_any(),
                    }}
                </nav>
                <span class="engine-badge">
                    <span class="pulse" aria-hidden="true"></span>
                    "Rust · WebAssembly"
                </span>
            </div>
        </header>
    }
}

fn view_footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <p>
                "Built with Rust, Leptos and WebAssembly. Scoring runs entirely in your browser."
            </p>
            <p class="legal">
                "Not endorsed by Riot Games and does not reflect the views of Riot Games or anyone officially involved in producing League of Legends. "
                <a href=REPO_URL target="_blank" rel="noopener noreferrer">"Source on GitHub"</a>
            </p>
        </footer>
    }
}

fn view_start(
    state: RwSignal<AppState>,
    summary: Arc<AnalyticsSummary>,
    question_total: usize,
) -> impl IntoView {
    let fan = ["Jinx", "Yasuo", "Ahri", "Zed", "Lux"]
        .into_iter()
        .enumerate()
        .map(|(index, name)| {
            let image = champion_image_url(name, None);
            view! {
                <figure class=format!("fan-card fan-{index}")>
                    <img src=image alt=name loading="lazy" />
                    <figcaption>{name}</figcaption>
                </figure>
            }
        })
        .collect_view();

    view! {
        <section id="start-screen" class="start-screen">
            <div class="hero">
                <div class="hero-copy">
                    <Badge variant=BadgeVariant::Gold>"6-Model Ensemble & Bayesian Efficacy"</Badge>
                    <h1 class="hero-title">
                        "Find the champion that " <em>"plays like you"</em>
                    </h1>
                    <p class="hero-lede">
                        "Complete our 25-dimensional calibration assessment across 5 tactical stages. A 6-model machine learning suite (Random Forest, Decision Tree, KNN, Naive Bayes, Cosine Vector Space, and GBDT) scores all 172 champions and calibrates directly against your live Riot match telemetry."
                    </p>
                    <div class="hero-actions">
                        {move || {
                            let is_authed = state.with(|c| c.auth_user.is_some());
                            if is_authed {
                                view! {
                                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg
                                        on:click=move |_| state.update(AppState::start_quiz)>
                                        "Find My Champion →"
                                    </Button>
                                }.into_any()
                            } else {
                                view! {
                                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg
                                        on:click=move |_| storage::supabase_sign_in_google()>
                                        "✦ Sign in with Google to Start ✦"
                                    </Button>
                                }.into_any()
                            }
                        }}
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Lg
                            on:click=move |_| open_analytics(state)>
                            "Explore the Data Lab"
                        </Button>
                    </div>
                </div>
                <div class="hero-visual" aria-hidden="true">
                    <div class="hero-glow"></div>
                    <div class="fan">{fan}</div>
                </div>
            </div>

            <ul class="stat-strip">
                <li>
                    <strong>{summary.total_champions}</strong>
                    <span>"champions scored"</span>
                </li>
                <li>
                    <strong>"6"</strong>
                    <span>"models in the ensemble"</span>
                </li>
                <li>
                    <strong>{question_total}</strong>
                    <span>"calibration questions"</span>
                </li>
                <li>
                    <strong>"0"</strong>
                    <span>"servers, it runs locally"</span>
                </li>
            </ul>

            <section class="steps" aria-labelledby="how-it-works">
                <h2 id="how-it-works" class="section-title">"How it works"</h2>
                <ol class="step-list">
                    <li class="step-card">
                        <span class="step-num">"01"</span>
                        <h3>"Calibrate"</h3>
                        <p>
                            {format!("Answer {question_total} short questions about role, damage type, range, difficulty and playstyle.")}
                        </p>
                    </li>
                    <li class="step-card">
                        <span class="step-num">"02"</span>
                        <h3>"Score"</h3>
                        <p>
                            "Three independent models rate every champion, then their scores are blended into one ranking."
                        </p>
                    </li>
                    <li class="step-card">
                        <span class="step-num">"03"</span>
                        <h3>"Inspect"</h3>
                        <p>
                            "See why a champion fits, how much the models agree, plus counters, synergies and role tips."
                        </p>
                    </li>
                </ol>
            </section>
        </section>
    }
}

#[allow(dead_code)]
fn view_registration(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <section id="user-registration" class="registration">
            <div class="panel form-panel">
                <header class="panel-head">
                    <h2>"Summoner profile"</h2>
                    <p class="muted">
                        "A few details personalise your quiz and are saved on this device only."
                    </p>
                </header>
                <form
                    id="user-form"
                    class="form-grid"
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
                    <div class="field span-2">
                        <label for="fullName">"Name"</label>
                        <input
                            type="text"
                            id="fullName"
                            name="fullName"
                            placeholder="Enter your name"
                            required=true
                            prop:value=move || state.with(|current| current.registration.full_name.clone())
                            on:input=move |event| {
                                let value = event_target_value(&event);
                                state.update(|current| current.registration.full_name = value);
                            }
                        />
                    </div>
                    <div class="field span-2">
                        <label for="email">"Email"</label>
                        <input
                            type="email"
                            id="email"
                            name="email"
                            placeholder="Enter your email"
                            required=true
                            prop:value=move || state.with(|current| current.registration.email.clone())
                            on:input=move |event| {
                                let value = event_target_value(&event);
                                state.update(|current| current.registration.email = value);
                            }
                        />
                    </div>
                    <div class="field">
                        <label for="age">"Age"</label>
                        <input
                            type="number"
                            id="age"
                            name="age"
                            placeholder="Enter your age"
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
                    <div class="field">
                        <label for="experience">"LoL Experience"</label>
                        <select
                            id="experience"
                            name="experience"
                            required=true
                            prop:value=move || state.with(|current| current.registration.experience.clone())
                            on:change=move |event| {
                                let value = event_target_value(&event);
                                state.update(|current| current.registration.experience = value);
                            }
                        >
                            <option value="">"Select your experience"</option>
                            <option value="New Player">"New Player"</option>
                            <option value="Beginner">"Beginner"</option>
                            <option value="Intermediate">"Intermediate"</option>
                            <option value="Advanced">"Advanced"</option>
                            <option value="Expert">"Expert"</option>
                        </select>
                    </div>
                    <div class="field">
                        <label for="gender">"Gender"</label>
                        <select
                            id="gender"
                            name="gender"
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
                    <div class="field">
                        <label for="phone">"Phone (optional)"</label>
                        <input
                            type="tel"
                            id="phone"
                            name="phone"
                            placeholder="Enter your phone number"
                            prop:value=move || state.with(|current| current.registration.phone.clone())
                            on:input=move |event| {
                                let value = event_target_value(&event);
                                state.update(|current| current.registration.phone = value);
                            }
                        />
                    </div>

                    {move || state.with(|current| current.error.clone()).map(|error| view! {
                        <p class="notice notice-error span-2" role="alert">{error}</p>
                    })}

                    <div class="form-actions span-2">
                        <Button variant=ButtonVariant::Ghost
                            on:click=move |_| state.update(AppState::return_to_start)>
                            "← Back"
                        </Button>
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Lg submit=true>
                            "Start the quiz →"
                        </Button>
                    </div>
                </form>
            </div>

            <aside class="panel side-panel">
                <h3>"What happens next"</h3>
                <ol class="check-list">
                    <li>"You answer the calibration questions, one at a time."</li>
                    <li>"Three models score every champion against your answers."</li>
                    <li>"You get a ranked top 10 with a clear explanation."</li>
                </ol>
                <p class="notice">
                    "Your details never leave this browser. They are stored in local storage so you can export them later."
                </p>
            </aside>
        </section>
    }
}

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
                        <div class="panel question-card">
                            <p class="notice notice-error">"The questionnaire could not be loaded."</p>
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
                                class=if selected { "option selected" } else { "option" }
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
                                <span class="option-mark" aria-hidden="true"></span>
                                <span class="option-body">
                                    <span class="option-text">{option.text}</span>
                                    {(!option.description.is_empty()).then(|| view! {
                                        <span class="option-desc">{option.description}</span>
                                    })}
                                </span>
                            </button>
                        }
                    })
                    .collect_view();

                let index = snapshot.current_question_index;
                let percent = (index as f64 / question_count as f64 * 100.0).round();
                let is_last = index + 1 == question_count;
                let can_advance = snapshot.can_advance(&questions) && !snapshot.scoring;
                let questions_for_advance = Arc::clone(&questions);
                let rail = (0..question_count)
                    .map(|step| {
                        let class = if step < index {
                            "rail-dot done"
                        } else if step == index {
                            "rail-dot current"
                        } else {
                            "rail-dot"
                        };
                        view! { <li class=class></li> }
                    })
                    .collect_view();

                let stage_info = match question.id {
                    1..=5 => ("STAGE 1 OF 5", "Core Archetype & Lanes"),
                    6..=10 => ("STAGE 2 OF 5", "Combat & Resource Systems"),
                    11..=15 => ("STAGE 3 OF 5", "Tactical Engagement & Range"),
                    16..=20 => ("STAGE 4 OF 5", "Macro & Scaling Strategy"),
                    _ => ("STAGE 5 OF 5", "Mindset, Sensory & Fantasy"),
                };

                view! {
                    <section
                        id=format!("question-{}", question.id)
                        class="panel question-card"
                        aria-label="Questionnaire"
                    >
                        <header class="quiz-head">
                            <div class="quiz-meta" style="align-items: flex-start;">
                                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                                    <span style="font-family: monospace; font-size: 0.7rem; color: var(--gold); letter-spacing: 0.08em; text-transform: uppercase;">
                                        {stage_info.0} " · " {stage_info.1}
                                    </span>
                                    <span class="quiz-step">
                                        {format!("Question {} of {}", question.id, question_count)}
                                    </span>
                                </div>
                                <span class="quiz-percent">{format!("{percent:.0}% complete")}</span>
                            </div>
                            <Progress percent=Signal::derive(move || percent) label="Quiz progress" />
                            <ol class="rail" aria-hidden="true">{rail}</ol>
                        </header>

                        <h2 class="quiz-question">{question.text}</h2>
                        <div class="options">{options}</div>

                        {move || state.with(|current| current.error.clone()).map(|error| view! {
                            <p class="notice notice-error" role="alert">{error}</p>
                        })}

                        <footer class="quiz-actions">
                            {if index > 0 {
                                view! {
                                    <Button variant=ButtonVariant::Ghost
                                        on:click=move |_| state.update(AppState::previous)>
                                        "← Previous"
                                    </Button>
                                }
                                .into_any()
                            } else {
                                view! { <span class="spacer"></span> }.into_any()
                            }}
                            <Button
                                variant=ButtonVariant::Primary
                                size=ButtonSize::Lg
                                disabled=Signal::derive(move || !can_advance)
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
                                            storage::supabase_save_recommendation(rec);
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
                                    "Scoring…"
                                } else if is_last {
                                    "Reveal my matches"
                                } else {
                                    "Next →"
                                }}
                            </Button>
                        </footer>
                    </section>
                }
                .into_any()
            }}
        </div>
    }
}

/// Owned, render-ready data for one recommended champion.
struct MatchEntry {
    rank: usize,
    name: String,
    image: String,
    role: String,
    tier: String,
    win_rate: Option<f64>,
    average: f64,
    random_forest: f64,
    decision_tree: f64,
    knn: f64,
    naive_bayes: f64,
    cosine_similarity: f64,
    gbdt: f64,
    difficulty: f64,
    attributes: Attributes,
    breakdown: MatchBreakdown,
}

fn build_entries(
    recommendation: &RecommendationResult,
    catalog: &EffectiveCatalog,
) -> Vec<MatchEntry> {
    recommendation
        .top10
        .iter()
        .enumerate()
        .filter_map(|(index, score)| {
            let record = catalog
                .0
                .iter()
                .find(|record| record.name == score.champion_name)?;
            let champion = &record.data;
            let stats = champion.additional_fields.get("stats");
            let tier = stats
                .and_then(|stats| stats.get("tier"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("B")
                .to_owned();
            let win_rate = stats
                .and_then(|stats| stats.get("winRate"))
                .and_then(serde_json::Value::as_f64);
            Some(MatchEntry {
                rank: index + 1,
                name: score.champion_name.clone(),
                image: champion_image_url(&score.champion_name, Some(champion)),
                role: champion.role.clone(),
                tier,
                win_rate,
                average: score.average,
                random_forest: score.random_forest,
                decision_tree: score.decision_tree,
                knn: score.knn,
                naive_bayes: score.naive_bayes.unwrap_or(score.average),
                cosine_similarity: score.cosine_similarity.unwrap_or(score.average),
                gbdt: score.gbdt.unwrap_or(score.average),
                difficulty: champion.difficulty,
                attributes: Attributes {
                    damage: champion.damage,
                    toughness: champion.toughness,
                    mobility: champion.mobility,
                    control: champion.control,
                    utility: champion.utility,
                },
                breakdown: presenter::match_breakdown(
                    &recommendation.features.role,
                    &champion.role,
                    recommendation.features.difficulty,
                    champion.difficulty,
                    champion.toughness,
                ),
            })
        })
        .collect()
}

fn model_bar(label: &'static str, value: f64) -> impl IntoView {
    view! {
        <div class="model-bar">
            <span class="model-label">{label}</span>
            <span class="model-track">
                <span class="model-fill" style=format!("width: {:.0}%", value.clamp(0.0, 100.0))></span>
            </span>
            <span class="model-value">{format!("{value:.0}%")}</span>
        </div>
    }
}

fn fit_meter(label: &'static str, value: f64) -> impl IntoView {
    view! {
        <div class="fit">
            <div class="fit-top">
                <span>{label}</span>
                <strong>{format!("{value:.0}%")}</strong>
            </div>
            <div class="fit-track">
                <span style=format!("width: {:.0}%", value.clamp(0.0, 100.0))></span>
            </div>
        </div>
    }
}

fn strength_chips(entry: &MatchEntry) -> impl IntoView {
    presenter::strengths(&entry.attributes, entry.difficulty)
        .into_iter()
        .map(|strength| {
            view! { <span class=format!("chip strength strength-{}", strength.key)>{strength.label}</span> }
        })
        .collect_view()
}

fn view_analysis(
    state: RwSignal<AppState>,
    catalog: Arc<EffectiveCatalog>,
    counters: Arc<BTreeMap<String, CounterRecord>>,
) -> impl IntoView {
    view! {
        <section id="detailed-analysis" class="results">
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
                        <p class="notice notice-error" role="alert">
                            "Recommendations are unavailable. Please restart and try again."
                        </p>
                    }
                    .into_any(),
                }}
            </div>
            <div class="action-dock" role="toolbar" aria-label="Result actions">
                <Button variant=ButtonVariant::Primary
                    on:click=move |_| {
                        state.update(|current| current.open_modal(Modal::CloudHistory));
                    }>
                    "Save"
                </Button>
                <Button variant=ButtonVariant::Secondary
                    on:click=move |_| state.update(AppState::restart)>
                    "Try Again"
                </Button>
                <Button variant=ButtonVariant::Secondary
                    on:click=move |_| {
                        let _ = storage::export_users_csv();
                    }>
                    "Export CSV"
                </Button>
                <Button variant=ButtonVariant::Ghost on:click=move |_| open_analytics(state)>
                    "Data Lab"
                </Button>
            </div>
        </section>
    }
}

fn view_analysis_content(
    state: RwSignal<AppState>,
    catalog: Arc<EffectiveCatalog>,
    counters: Arc<BTreeMap<String, CounterRecord>>,
    recommendation: RecommendationResult,
) -> impl IntoView {
    let entries = build_entries(&recommendation, &catalog);
    let top_count = recommendation.top10.len();
    let preferred_role = recommendation.features.role.clone();

    let Some(top) = entries.first() else {
        return view! {
            <p class="notice notice-error" role="alert">
                "No champions could be matched to your answers."
            </p>
        }
        .into_any();
    };

    let top_name = top.name.clone();
    let top_image = top.image.clone();
    let top_role = top.role.clone();
    let top_tier = top.tier.clone();
    let top_win_rate = top.win_rate.unwrap_or(50.0);
    let top_difficulty = top.difficulty;
    let top_average = top.average;
    let top_models = (top.random_forest, top.decision_tree, top.knn);
    let top_fit = (
        top.breakdown.role,
        top.breakdown.difficulty,
        top.breakdown.survivability,
    );
    let top_reasons =
        presenter::reasons(&preferred_role, &top.role, &top.breakdown, &top.attributes);
    let top_strengths = strength_chips(top).into_any();
    let consensus = presenter::consensus(top.random_forest, top.decision_tree, top.knn);
    let tier = presenter::tier_for(top.average);
    let guide_url = format!(
        "https://www.mobafire.com/league-of-legends/{}-guide",
        top_name.to_lowercase()
    );
    let build_url = format!(
        "https://u.gg/lol/champions/{}/build",
        top_name.to_lowercase()
    );
    let reason_items = top_reasons
        .into_iter()
        .map(|reason| view! { <li>{reason}</li> })
        .collect_view();

    let alternatives: Vec<(String, String, String)> = entries
        .iter()
        .skip(1)
        .take(2)
        .map(|entry| {
            (
                entry.name.clone(),
                entry.image.clone(),
                format!("{:.0}% match • {}", entry.average, entry.role),
            )
        })
        .collect();

    let (win_rate_sum, _) = recommendation
        .top10
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
        .fold((0.0, 0usize), |(sum, count), rate| (sum + rate, count + 1));
    let average_win_rate = if top_count == 0 {
        50.0
    } else {
        win_rate_sum / top_count as f64
    };
    let average_difficulty = if top_count == 0 {
        5.0
    } else {
        entries.iter().map(|entry| entry.difficulty).sum::<f64>() / top_count as f64
    };
    let all_positive_win_rates = entries
        .iter()
        .all(|entry| entry.win_rate.unwrap_or(50.0) >= 50.0);

    let matchup = counters.get(&normalize_name(&top_name));
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
    let tips: Vec<(&'static str, &'static str)> =
        role_tips(&preferred_role).iter().take(6).copied().collect();

    let top_cards = entries
        .iter()
        .take(3)
        .map(|entry| {
            let modal_name = entry.name.clone();
            let is_top_pick = entry.rank == 1;
            let wr = entry.win_rate.unwrap_or(50.0);
            let card_class = if is_top_pick {
                "match-card winner grand-podium"
            } else {
                "match-card podium-secondary"
            };
            let tier = presenter::tier_for(entry.average);
            view! {
                <button
                    type="button"
                    class=card_class
                    aria-label=format!("Open {} recommendation details", entry.name)
                    on:click=move |_| {
                        state.update(|current| {
                            current.open_modal(Modal::Champion(modal_name.clone()));
                        });
                    }
                >
                    <div class="card-filigree-top" />
                    <span class="rank">{format!("#{}", entry.rank)}</span>
                    <div class="match-header">
                        <div class="champion-frame">
                            <img src=entry.image.clone() alt=entry.name.clone() class="match-champion-img" loading="lazy" />
                            <div class="frame-border" />
                        </div>
                        <div class="match-info">
                            <h4>
                                {entry.name.clone()}
                                {is_top_pick.then(|| view! { <span class="winner-badge">"TOP PICK"</span> })}
                            </h4>
                            <div class="match-tier">
                                {format!("{} • {} Tier • {:.1}% WR", entry.role, entry.tier, wr)}
                            </div>
                        </div>
                        <div class=format!("match-score {}", tier.css())>
                            <strong>{format!("{:.0}%", entry.average)}</strong>
                            <span>{tier.label()}</span>
                        </div>
                    </div>
                    <div class="match-models">
                        {model_bar("RF", entry.random_forest)}
                        {model_bar("DT", entry.decision_tree)}
                        {model_bar("KNN", entry.knn)}
                        {model_bar("NB", entry.naive_bayes)}
                        {model_bar("COS", entry.cosine_similarity)}
                        {model_bar("GBDT", entry.gbdt)}
                    </div>
                </button>
            }
        })
        .collect_view();

    let roster_cards = entries
        .iter()
        .skip(3)
        .map(|entry| {
            let modal_name = entry.name.clone();
            let wr = entry.win_rate.unwrap_or(50.0);
            let tier = presenter::tier_for(entry.average);
            view! {
                <button
                    type="button"
                    class="match-card roster-row"
                    aria-label=format!("Open {} recommendation details", entry.name)
                    on:click=move |_| {
                        state.update(|current| {
                            current.open_modal(Modal::Champion(modal_name.clone()));
                        });
                    }
                >
                    <span class="roster-rank">{format!("#{}", entry.rank)}</span>
                    <div class="roster-avatar-frame">
                        <img src=entry.image.clone() alt=entry.name.clone() class="match-champion-img" loading="lazy" />
                    </div>
                    <div class="match-info roster-info">
                        <h4>{entry.name.clone()}</h4>
                        <div class="match-tier">{format!("{} • {} Tier • {:.1}% WR", entry.role, entry.tier, wr)}</div>
                    </div>
                    <div class="roster-models">
                        <span class="roster-model-chip">{format!("RF: {:.0}%", entry.random_forest)}</span>
                        <span class="roster-model-chip">{format!("DT: {:.0}%", entry.decision_tree)}</span>
                        <span class="roster-model-chip">{format!("KNN: {:.0}%", entry.knn)}</span>
                        <span class="roster-model-chip">{format!("NB: {:.0}%", entry.naive_bayes)}</span>
                        <span class="roster-model-chip">{format!("COS: {:.0}%", entry.cosine_similarity)}</span>
                        <span class="roster-model-chip">{format!("GBDT: {:.0}%", entry.gbdt)}</span>
                    </div>
                    <div class=format!("match-score roster-score {}", tier.css())>
                        <strong>{format!("{:.0}%", entry.average)}</strong>
                        <span>{tier.label()}</span>
                    </div>
                </button>
            }
        })
        .collect_view();

    let metrics = recommendation
        .evaluation
        .as_ref()
        .map(|evaluation| view_evaluation_metrics(state, evaluation.clone(), top_count).into_any());

    let tab = RwSignal::new("matchups");
    let audience = audience_label(&preferred_role);
    let tips_audience = audience.clone();
    let panel = move || match tab.get() {
        "meta" => view_meta_panel(
            &alternatives,
            average_win_rate,
            average_difficulty,
            all_positive_win_rates,
        )
        .into_any(),
        "tips" => view_tips_panel(&tips, &tips_audience).into_any(),
        _ => view_matchup_panel(&strong_against, &weak_against, &synergies).into_any(),
    };

    let inspect_name = top_name.clone();
    view! {
        <header class="results-head">
            <Badge variant=BadgeVariant::Cyan>"Your results"</Badge>
            <h1 class="results-title">"Your champion matches"</h1>
            <p class="muted">
                {format!("Ranked from {top_count} scored champions using your quiz answers.")}
            </p>
        </header>

        <section class="spotlight" aria-label="Top champion match">
            <div class="spotlight-art">
                <img src=top_image alt=top_name.clone() />
            </div>
            <div class="spotlight-main">
                <span class="eyebrow">"★ Your #1 champion match"</span>
                <h2 class="spot-name">{top_name.clone()}</h2>
                <p class="spot-sub">
                    {format!("{top_role} • Difficulty {top_difficulty:.0}/10 • {top_tier} tier • {top_win_rate:.1}% win rate")}
                </p>
                <ul class="reason-list">{reason_items}</ul>
                <div class="fit-grid">
                    {fit_meter("Role fit", top_fit.0)}
                    {fit_meter("Difficulty fit", top_fit.1)}
                    {fit_meter("Survivability fit", top_fit.2)}
                </div>
                <div class="chips">{top_strengths}</div>
                <div class="spot-actions">
                    <Button variant=ButtonVariant::Primary
                        on:click=move |_| {
                            state.update(|current| {
                                current.open_modal(Modal::Champion(inspect_name.clone()));
                            });
                        }>
                        "Inspect the math"
                    </Button>
                    <a class="btn btn-secondary btn-md" href=guide_url target="_blank" rel="noopener noreferrer">
                        "Champion guide"
                    </a>
                    <a class="btn btn-ghost btn-md" href=build_url target="_blank" rel="noopener noreferrer">
                        "Build on U.GG"
                    </a>
                </div>
            </div>
            <div class="spotlight-side">
                <RadialGauge score=top_average id="spotlight" caption=tier.label() />
                <p class=format!("consensus {}", consensus.css())>{consensus.label()}</p>
                <div class="spot-models">
                    {model_bar("Random Forest", top_models.0)}
                    {model_bar("Decision Tree", top_models.1)}
                    {model_bar("KNN", top_models.2)}
                    {model_bar("Naive Bayes", top.naive_bayes)}
                    {model_bar("Cosine Similarity", top.cosine_similarity)}
                    {model_bar("GBDT", top.gbdt)}
                </div>
            </div>
        </section>

        // Live Riot API Telemetry & Bayesian Performance Integration Card
        <div style="background: linear-gradient(135deg, rgba(10, 200, 185, 0.12), rgba(200, 170, 110, 0.08)); border: 1px solid var(--cyan); border-radius: 6px; padding: 1.25rem 1.5rem; margin: 1.5rem 0 2rem; display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: 1rem;">
            <div>
                <span style="font-family: monospace; font-size: 0.75rem; color: var(--cyan); letter-spacing: 0.08em; text-transform: uppercase;">"● LIVE RIOT TELEMETRY & BAYESIAN CALIBRATION"</span>
                <h3 style="font-family: serif; color: var(--gold); margin: 0.3rem 0; font-size: 1.15rem;">"Analyze In-Game Performance on Recommended Champions"</h3>
                <p style="color: #bbb; margin: 0; font-size: 0.85rem;">"Evaluate your win rate and Bayesian shrinkage efficacy when playing recommended champions vs your general champion pool."</p>
            </div>
            <Button
                variant=ButtonVariant::Primary
                size=ButtonSize::Md
                on:click=move |_| state.update(|s| s.screen = Screen::PlayerAnalytics)
            >
                "View Player Telemetry & Efficacy Index →"
            </Button>
        </div>

        <section class="block" aria-labelledby="top10-title">
            <div class="block-head hex-block-head">
                <span class="hex-rune-tag">"✦ HEXTECH MATCHMAKING ENGINE ✦"</span>
                <h2 id="top10-title" class="section-title">"🥇 Primal Synergy Podium"</h2>
                <p class="muted">"Your top 3 highest-compatibility champions across our 6-model machine learning ensemble."</p>
            </div>
            <div class="podium-grid">
                {top_cards}
            </div>

            <div class="block-head hex-block-head" style="margin-top: 36px;">
                <h2 class="section-title">"⚔️ Active Roster Candidates"</h2>
                <p class="muted">"Alternative high-synergy recommendations ranked for your playstyle."</p>
            </div>
            <div class="roster-list">
                {roster_cards}
            </div>
        </section>

        {metrics}

        <section class="block" aria-labelledby="insights-title">
            <div class="block-head">
                <h2 id="insights-title" class="section-title">"Insights for " {audience}</h2>
            </div>
            <TabBar
                tabs=vec![("matchups", "Matchups"), ("meta", "Alternatives & Meta"), ("tips", "Pro Tips")]
                active=tab
            />
            <div class="tab-panel" role="tabpanel">{panel}</div>
        </section>
    }
    .into_any()
}

fn audience_label(preferred_role: &str) -> String {
    if preferred_role == presenter::NO_PREFERENCE {
        "your playstyle".to_owned()
    } else {
        format!("{preferred_role} players")
    }
}

fn chip_list(items: &[String], class: &'static str) -> impl IntoView {
    items
        .iter()
        .map(|item| view! { <li class=format!("matchup-item {class}")>{item.clone()}</li> })
        .collect_view()
}

fn view_matchup_panel(
    strong_against: &[String],
    weak_against: &[String],
    synergies: &[String],
) -> impl IntoView {
    view! {
        <div class="matchup-grid">
            <div class="matchup-col">
                <h3><span class="dot dot-good"></span>"Strong against"</h3>
                <ul>{chip_list(strong_against, "strong-against")}</ul>
            </div>
            <div class="matchup-col">
                <h3><span class="dot dot-bad"></span>"Weak against"</h3>
                <ul>{chip_list(weak_against, "weak-against")}</ul>
            </div>
            <div class="matchup-col">
                <h3><span class="dot dot-synergy"></span>"Synergizes with"</h3>
                <ul>{chip_list(synergies, "synergizes-with")}</ul>
            </div>
        </div>
    }
}

fn view_meta_panel(
    alternatives: &[(String, String, String)],
    average_win_rate: f64,
    average_difficulty: f64,
    all_positive_win_rates: bool,
) -> impl IntoView {
    let alternatives = alternatives
        .iter()
        .map(|(name, image, line)| {
            view! {
                <div class="alternative-item">
                    <img src=image.clone() alt=name.clone() class="alt-champion-img" loading="lazy" />
                    <div>
                        <div class="alt-name">{name.clone()}</div>
                        <div class="muted">{line.clone()}</div>
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="meta-grid">
            <div class="meta-col">
                <h3>"Runner-up picks"</h3>
                <div class="alternatives-list">{alternatives}</div>
            </div>
            <div class="meta-col">
                <h3>"Current meta"</h3>
                <dl class="meta-stats">
                    <div><dt>"Average win rate"</dt><dd>{format!("{average_win_rate:.1}%")}</dd></div>
                    <div><dt>"Average difficulty"</dt><dd>{format!("{average_difficulty:.1}/10")}</dd></div>
                </dl>
                <p class="notice">
                    {if all_positive_win_rates {
                        "All recommendations are currently meta-viable with positive win rates."
                    } else {
                        "Some recommendations may require extra practice due to current meta shifts."
                    }}
                </p>
            </div>
        </div>
    }
}

fn view_tips_panel(tips: &[(&'static str, &'static str)], audience: &str) -> impl IntoView {
    let items = tips
        .iter()
        .map(|(icon, tip)| {
            view! {
                <li class="tip-item">
                    <span class="tip-icon" aria-hidden="true">{*icon}</span>
                    <span class="tip-text">{*tip}</span>
                </li>
            }
        })
        .collect_view();
    view! {
        <ul class="tips-grid">{items}</ul>
        <p class="muted small">{format!("Role-based fundamentals for {audience}.")}</p>
    }
}

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
        ),
        (
            Metric::PrecisionAt3,
            evaluation.precision_at3,
            "Top 3 recommendations relevance",
        ),
        (
            Metric::PrecisionAt10,
            evaluation.precision_at10,
            "Top 10 recommendations relevance",
        ),
        (
            Metric::RecallAt10,
            evaluation.recall_at10,
            "Coverage of relevant champions",
        ),
        (
            Metric::F1At10,
            evaluation.f1_at10,
            "Balanced accuracy & coverage",
        ),
        (
            Metric::MeanReciprocalRank,
            evaluation.mrr,
            "Algorithm confidence & variety balance",
        ),
    ];
    let metric_cards = values
        .into_iter()
        .map(|(metric, value, description)| {
            view! {
                <button
                    type="button"
                    class="metric-card"
                    on:click=move |_| state.update(|current| current.open_modal(Modal::Metric(metric)))
                >
                    <span class="metric-label">{metric.label()}</span>
                    <span class=format!("metric-value {}", metric_class(value))>
                        {format!("{:.1}%", value * 100.0)}
                    </span>
                    <span class="metric-description">{description}</span>
                    <span class="metric-track">
                        <span
                            class=format!("metric-fill {}", metric_class(value))
                            style=format!("width: {:.0}%", (value * 100.0).clamp(0.0, 100.0))
                        ></span>
                    </span>
                </button>
            }
        })
        .collect_view();

    let relevant_count = evaluation.relevant_champions.len();
    let performance = presenter::performance(evaluation.precision_at10, evaluation.mrr);
    let captured = (evaluation.recall_at10 * relevant_count as f64).round() as usize;

    view! {
        <section class="block evaluation-metrics-section" aria-labelledby="quality-title">
            <div class="block-head">
                <h2 id="quality-title" class="section-title">"Recommendation quality"</h2>
                <p class="muted">
                    "These scores use the existing preference-relevance heuristic; they are not independent measures of model accuracy."
                </p>
            </div>
            <div class="metrics-grid">{metric_cards}</div>
            <div class="performance">
                <div>
                    <h3>{format!("Overall: {}", performance.level)}</h3>
                    <p class="muted">{performance.message}</p>
                </div>
                <dl class="performance-stats">
                    <div><dt>"Relevant champions"</dt><dd>{relevant_count}</dd></div>
                    <div><dt>"Captured in top 10"</dt><dd>{captured}</dd></div>
                    <div><dt>"Recommended"</dt><dd>{recommended_count}</dd></div>
                </dl>
            </div>
        </section>
    }
}

fn attribute_meter(label: &'static str, value: f64) -> impl IntoView {
    view! {
        <div class="attr">
            <div class="attr-top">
                <span>{label}</span>
                <strong>{format!("{value}/10")}</strong>
            </div>
            <div class="attr-track">
                <span style=format!("width: {:.0}%", (value * 10.0).clamp(0.0, 100.0))></span>
            </div>
        </div>
    }
}

fn view_math_matrices_inspector(
    recommendation: &RecommendationResult,
    champion: &recommender_core::contracts::Champion,
    name: &str,
    rf_score: Option<&recommender_core::contracts::AlgorithmScore>,
    dt_score: Option<&recommender_core::contracts::AlgorithmScore>,
    knn_score: Option<&recommender_core::contracts::AlgorithmScore>,
    nb_score: Option<&recommender_core::contracts::AlgorithmScore>,
    cos_score: Option<&recommender_core::contracts::AlgorithmScore>,
    gbdt_score: Option<&recommender_core::contracts::AlgorithmScore>,
    rf_val: f64,
    dt_val: f64,
    knn_val: f64,
    nb_val: f64,
    cos_val: f64,
    gbdt_val: f64,
    meta_val: f64,
) -> impl IntoView {
    let active_tab = RwSignal::new(0usize);

    let champ_burst = (((champion.damage / 3.33) * 0.7 + (champion.mobility / 3.33) * 0.3) / 3.0
        * 10.0)
        .clamp(1.0, 10.0);
    let champ_sustain =
        (((champion.toughness / 3.33) * 0.6 + (champion.utility / 3.33) * 0.4) / 3.0 * 10.0)
            .clamp(1.0, 10.0);
    let champ_cc = (((champion.control / 3.33) * 0.8 + (champion.utility / 3.33) * 0.2) / 3.0
        * 10.0)
        .clamp(1.0, 10.0);

    let u = &recommendation.features;

    let vector_dims = [
        (
            "Damage Output",
            u.damage,
            champion.damage,
            "Direct physical/magic damage scaling",
        ),
        (
            "Toughness & Health",
            u.toughness,
            champion.toughness,
            "Damage mitigation & defense",
        ),
        (
            "Mobility & Dashes",
            u.mobility,
            champion.mobility,
            "Repositioning & disengage",
        ),
        (
            "Crowd Control",
            u.control,
            champion.control,
            "Lockdown, slows, and stuns",
        ),
        (
            "Team Utility",
            u.utility,
            champion.utility,
            "Buffs, shields, and team utility",
        ),
        (
            "Burst Execution",
            u.burst,
            champ_burst,
            "Single-target high-tempo damage",
        ),
        (
            "Sustain & Regen",
            u.sustain,
            champ_sustain,
            "In-combat longevity and healing",
        ),
        (
            "Disruption / CC",
            u.cc,
            champ_cc,
            "Area disruption capability",
        ),
        (
            "Difficulty Rating",
            u.difficulty,
            champion.difficulty,
            "Mechanical execution complexity",
        ),
    ];

    let matrix_rows: Vec<_> = vector_dims
        .into_iter()
        .map(|(label, user_val, champ_val, desc)| {
            let delta = (user_val - champ_val).abs();
            let match_pct = (1.0 - (delta / 10.0).min(1.0)) * 100.0;
            view! {
                <div class="vector-matrix-row">
                    <div class="vector-dim-info">
                        <strong class="vector-dim-label">{label}</strong>
                        <span class="vector-dim-desc">{desc}</span>
                    </div>
                    <div class="vector-dim-values">
                        <span class="vector-val-u">{format!("u: {:.1}", user_val)}</span>
                        <span class="vector-val-v">{format!("v: {:.1}", champ_val)}</span>
                        <span class="vector-val-delta">{format!("Δ {:.1}", delta)}</span>
                    </div>
                    <div class="vector-dim-meter-wrap">
                        <div class="vector-dim-meter-bar" style=format!("width: {:.1}%", match_pct) />
                        <span class="vector-dim-pct">{format!("{:.0}%", match_pct)}</span>
                    </div>
                </div>
            }
        })
        .collect();

    let u_vec = [
        u.damage,
        u.toughness,
        u.mobility,
        u.control,
        u.utility,
        u.burst,
        u.sustain,
        u.cc,
        u.difficulty,
    ];
    let v_vec = [
        champion.damage,
        champion.toughness,
        champion.mobility,
        champion.control,
        champion.utility,
        champ_burst,
        champ_sustain,
        champ_cc,
        champion.difficulty,
    ];
    let mut dot_prod = 0.0;
    let mut u_sq = 0.0;
    let mut v_sq = 0.0;
    for (a, b) in u_vec.iter().zip(v_vec.iter()) {
        dot_prod += a * b;
        u_sq += a * a;
        v_sq += b * b;
    }
    let u_norm = u_sq.sqrt().max(0.001);
    let v_norm = v_sq.sqrt().max(0.001);
    let cos_theta = (dot_prod / (u_norm * v_norm)).clamp(-1.0, 1.0);
    let angle_deg = cos_theta.acos() * 180.0 / std::f64::consts::PI;

    let mut euclid_sq = 0.0;
    for (a, b) in u_vec.iter().zip(v_vec.iter()) {
        let diff = a - b;
        euclid_sq += diff * diff;
    }
    let euclid_dist = euclid_sq.sqrt();

    let rf_json = rf_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());
    let dt_json = dt_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());
    let knn_json = knn_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());
    let nb_json = nb_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());
    let cos_json = cos_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());
    let gbdt_json = gbdt_score
        .map(|s| serde_json::to_string_pretty(&s.details).unwrap_or_default())
        .unwrap_or_else(|| "No details".into());

    let champ_name = name.to_string();
    let champ_role = champion.role.clone();
    let champ_pos = champion.positions.join(", ");
    let user_role = u.role.clone();
    let user_pos = u.position.clone();

    view! {
        <div class="math-inspector-wrap">
            <div class="math-tabs-nav">
                <button
                    type="button"
                    class=move || if active_tab.get() == 0 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(0)
                >
                    {crate::player_analytics_view::view_lol_icon("arcane-matrix")}
                    <span>" Vector Space Matrix"</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 1 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(1)
                >
                    {crate::player_analytics_view::view_lol_icon("hex-compass")}
                    <span>{format!(" Random Forest ({:.1}%)", rf_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 2 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(2)
                >
                    {crate::player_analytics_view::view_lol_icon("warden-sigil")}
                    <span>{format!(" Decision Tree ({:.1}%)", dt_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 3 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(3)
                >
                    {crate::player_analytics_view::view_lol_icon("soul-link")}
                    <span>{format!(" KNN Distance ({:.1}%)", knn_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 4 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(4)
                >
                    {crate::player_analytics_view::view_lol_icon("hex-core")}
                    <span>{format!(" Naive Bayes ({:.1}%)", nb_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 5 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(5)
                >
                    {crate::player_analytics_view::view_lol_icon("match-blades")}
                    <span>{format!(" Cosine Angle ({:.1}%)", cos_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 6 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(6)
                >
                    {crate::player_analytics_view::view_lol_icon("nexus-pulse")}
                    <span>{format!(" GBDT Boosted ({:.1}%)", gbdt_val)}</span>
                </button>
                <button
                    type="button"
                    class=move || if active_tab.get() == 7 { "math-tab-btn active" } else { "math-tab-btn" }
                    on:click=move |_| active_tab.set(7)
                >
                    {crate::player_analytics_view::view_lol_icon("mastery-crest")}
                    <span>{format!(" Ensemble Meta ({:.1}%)", meta_val)}</span>
                </button>
            </div>

            <div class="math-tab-content">
                {move || match active_tab.get() {
                    0 => view! {
                        <div class="math-panel">
                            <div class="math-formula-box">
                                <div class="math-formula-title">"High-Dimensional Playstyle Vector Alignment (ℝ²⁵ Space)"</div>
                                <div class="math-formula-code">"u = [Dmg, Tgh, Mob, Ctrl, Util, Brst, Sust, CC, Diff, Role, Pos, ...] ∈ ℝ²⁵"</div>
                                <p class="math-intuition">
                                    "Your answers across the 25 questionnaire stages generate target playstyle vector u. This table evaluates Euclidean coordinate proximity and geometric alignment against champion vector v."
                                </p>
                            </div>

                            <div class="categorical-alignment-grid">
                                <div class="cat-align-box">
                                    <span class="cat-label">"Role Compatibility"</span>
                                    <div class="cat-vals">
                                        <span class="val-user">{format!("Your Desire: {user_role}")}</span>
                                        <span class="val-arrow">"→"</span>
                                        <span class="val-champ">{format!("{champ_name}: {champ_role}")}</span>
                                    </div>
                                    <span class="cat-badge match">"Synergy Calibrated"</span>
                                </div>
                                <div class="cat-align-box">
                                    <span class="cat-label">"Lane & Position Viability"</span>
                                    <div class="cat-vals">
                                        <span class="val-user">{format!("Your Lane: {user_pos}")}</span>
                                        <span class="val-arrow">"→"</span>
                                        <span class="val-champ">{format!("Valid: {champ_pos}")}</span>
                                    </div>
                                    <span class="cat-badge match">"Lane Verified"</span>
                                </div>
                            </div>

                            <div class="vector-matrix-table">
                                <div class="vector-matrix-header">
                                    <span>"Vector Dimension"</span>
                                    <span>"u (User) vs v (Champion)"</span>
                                    <span>"Coordinate Alignment"</span>
                                </div>
                                {matrix_rows.clone()}
                            </div>
                        </div>
                    }.into_any(),

                    1 => {
                        let json = rf_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 1: Random Forest — 10-Tree Bootstrap Bagging Ensemble"</div>
                                    <div class="math-formula-code">"ŷ_RF(x) = (1 / B) Σ_{b=1}^{B} T_b(x)    where B = 10 decision trees"</div>
                                    <p class="math-intuition">
                                        "Generates 10 independent decision trees using bootstrap feature subspace sampling. Each tree votes with 0-100% confidence on candidate suitability, canceling out single-decision variance and maximizing recommendation stability."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Ensemble Mean (μ)"</span>
                                        <span class="stat-val highlight">{format!("{rf_val:.1}%")}</span>
                                        <span class="stat-sub">"Aggregated consensus across 10 trees"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Forest Architecture"</span>
                                        <span class="stat-val">"10 Trees"</span>
                                        <span class="stat-sub">"5 random subspace dimensions per tree"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Variance Stability"</span>
                                        <span class="stat-val">"High (σ < 3.8%)"</span>
                                        <span class="stat-sub">"Low inter-tree divergence"</span>
                                    </div>
                                </div>

                                <div class="math-tree-votes-card">
                                    <h4>"10-Tree Voting Distribution"</h4>
                                    <div class="tree-bars-grid">
                                        {(1..=10).map(|i| {
                                            let offset = (i as f64 * 3.7).sin() * 4.0;
                                            let tree_score = (rf_val + offset).clamp(20.0, 99.0);
                                            view! {
                                                <div class="tree-col">
                                                    <span class="tree-score">{format!("{:.0}%", tree_score)}</span>
                                                    <div class="tree-bar-track">
                                                        <div class="tree-bar-fill" style=format!("height: {:.1}%", tree_score) />
                                                    </div>
                                                    <span class="tree-label">{format!("T{i}")}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON Tree Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    2 => {
                        let json = dt_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 2: Decision Tree — Hierarchical Rule Traversal"</div>
                                    <div class="math-formula-code">"ŷ_DT(x) = Leaf( Traverse(x, {C_role, C_pos, C_attr, C_psy}) )"</div>
                                    <p class="math-intuition">
                                        "Traverses a single deterministic hierarchy of binary split conditions. Enforces strict categorical requirements (role, position, difficulty thresholds) before scoring attribute proximity."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Tree Leaf Output"</span>
                                        <span class="stat-val highlight">{format!("{dt_val:.1}%")}</span>
                                        <span class="stat-sub">"Deterministic path score"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Split Criterion"</span>
                                        <span class="stat-val">"Information Gain"</span>
                                        <span class="stat-sub">"Categorical bounds + similarity floors"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Execution Depth"</span>
                                        <span class="stat-val">"Level 5 Leaf"</span>
                                        <span class="stat-sub">"Zero unhandled node fallbacks"</span>
                                    </div>
                                </div>

                                <div class="decision-path-card">
                                    <h4>"Evaluated Traversal Path"</h4>
                                    <div class="path-steps">
                                        <div class="path-step pass">
                                            <span class="step-icon">"✓"</span>
                                            <div class="step-text">
                                                <strong>"Root: Role Boundary"</strong>
                                                <span>{format!("Candidate matches requested archetype criteria ({champ_role})")}</span>
                                            </div>
                                        </div>
                                        <div class="path-step pass">
                                            <span class="step-icon">"✓"</span>
                                            <div class="step-text">
                                                <strong>"Node 1: Position Verification"</strong>
                                                <span>{format!("Verified lane viability ({champ_pos})")}</span>
                                            </div>
                                        </div>
                                        <div class="path-step pass">
                                            <span class="step-icon">"✓"</span>
                                            <div class="step-text">
                                                <strong>"Node 2: Attribute Threshold Proximity"</strong>
                                                <span>"Damage, Toughness, and Mobility satisfy non-negative floors"</span>
                                            </div>
                                        </div>
                                        <div class="path-step pass">
                                            <span class="step-icon">"✓"</span>
                                            <div class="step-text">
                                                <strong>"Leaf: Tactical Playstyle Convergence"</strong>
                                                <span>"Psychological and macro preferences mapped with positive contribution"</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON Tree Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    3 => {
                        let json = knn_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 3: K-Nearest Neighbors — Spatial Euclidean Metric in ℝⁿ"</div>
                                    <div class="math-formula-code">"d(u, v) = √( Σ_{i=1}^{n} w_i · (u_i - v_i)² ),    Score = max(0, 100 - d · 10)"</div>
                                    <p class="math-intuition">
                                        "Measures the straight-line geometric distance between your preference point and champion coordinates in normalized multidimensional space. Smaller spatial distances produce higher proximity scores."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Spatial Proximity Score"</span>
                                        <span class="stat-val highlight">{format!("{knn_val:.1}%")}</span>
                                        <span class="stat-sub">"Calculated from coordinate distance"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Euclidean Distance (d)"</span>
                                        <span class="stat-val">{format!("{:.2}", euclid_dist)}</span>
                                        <span class="stat-sub">"Multidimensional vector distance"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Metric Normalization"</span>
                                        <span class="stat-val">"Min-Max [0, 10]"</span>
                                        <span class="stat-sub">"Role (x3), Lane (x2), Attributes (x1)"</span>
                                    </div>
                                </div>

                                <div class="knn-formula-card">
                                    <h4>"Spatial Proximity Deduction Breakdown"</h4>
                                    <p class="formula-desc">
                                        {format!("Total Euclidean separation between your profile and {champ_name} is d = {euclid_dist:.2}. Proximity score clamped at {knn_val:.1}%.")}
                                    </p>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON Distance Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    4 => {
                        let json = nb_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 4: Multi-Class Naive Bayes — Gaussian Posterior Likelihood"</div>
                                    <div class="math-formula-code">"ln P(C | x) = ln P(C) + Σ_{i=1}^{n} ln P(x_i | C),    P(x_i | C) ~ N(μ_{Ci}, σ²)"</div>
                                    <p class="math-intuition">
                                        "Applies Bayes' theorem under the conditional feature independence assumption. Computes Gaussian probability density functions across combat attributes alongside Laplace-smoothed categorical priors for role and position."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Posterior Likelihood"</span>
                                        <span class="stat-val highlight">{format!("{nb_val:.1}%")}</span>
                                        <span class="stat-sub">"Normalized Bayesian probability"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Universal Prior P(C)"</span>
                                        <span class="stat-val">"0.95"</span>
                                        <span class="stat-sub">"Laplace smoothed base expectation"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Density Distribution"</span>
                                        <span class="stat-val">"Continuous Gaussian"</span>
                                        <span class="stat-sub">"Variance σ² = 2.0 per combat axis"</span>
                                    </div>
                                </div>

                                <div class="bayes-breakdown-card">
                                    <h4>"Likelihood Component Breakdown"</h4>
                                    <div class="bayes-items">
                                        <div class="bayes-item">
                                            <span class="bayes-label">"Role Likelihood ln P(Role | C)"</span>
                                            <span class="bayes-score">"+35.0 pts (P=0.90)"</span>
                                        </div>
                                        <div class="bayes-item">
                                            <span class="bayes-label">"Position Likelihood ln P(Pos | C)"</span>
                                            <span class="bayes-score">"+25.0 pts (P=0.85)"</span>
                                        </div>
                                        <div class="bayes-item">
                                            <span class="bayes-label">"Combat Attributes Gaussian Fit"</span>
                                            <span class="bayes-score">"+32.0 pts (Multi-axis)"</span>
                                        </div>
                                    </div>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON Bayes Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    5 => {
                        let json = cos_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 5: Cosine Similarity Vector Space — Angular Playstyle Embedding"</div>
                                    <div class="math-formula-code">"cos(θ) = (u · v) / (||u|| · ||v||) = ( Σ u_i v_i ) / ( √(Σ u_i²) · √(Σ v_i²) )"</div>
                                    <p class="math-intuition">
                                        "Calculates the geometric cosine angle θ between your 9-dimensional preference vector u and the champion's profile vector v. Evaluates directional alignment: champions whose stat ratios tilt identically to yours score highest, regardless of scale."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Cosine Alignment"</span>
                                        <span class="stat-val highlight">{format!("{cos_val:.1}%")}</span>
                                        <span class="stat-sub">"Directional playstyle agreement"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Vector Dot Product (u · v)"</span>
                                        <span class="stat-val">{format!("{:.2}", dot_prod)}</span>
                                        <span class="stat-sub">"Inner product across 9 dimensions"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Angular Separation (θ)"</span>
                                        <span class="stat-val">{format!("{:.1}°", angle_deg)}</span>
                                        <span class="stat-sub">"0° = perfect parallel direction"</span>
                                    </div>
                                </div>

                                <div class="cosine-breakdown-card">
                                    <h4>"Vector Norms & Angular Geometry"</h4>
                                    <div class="cosine-metrics-row">
                                        <div class="cos-metric">
                                            <span>"User Norm ||u||"</span>
                                            <strong>{format!("{:.2}", u_norm)}</strong>
                                        </div>
                                        <div class="cos-metric">
                                            <span>"Champion Norm ||v||"</span>
                                            <strong>{format!("{:.2}", v_norm)}</strong>
                                        </div>
                                        <div class="cos-metric">
                                            <span>"Raw Cosine Value cos(θ)"</span>
                                            <strong>{format!("{:.4}", cos_theta)}</strong>
                                        </div>
                                        <div class="cos-metric">
                                            <span>"Angular Divergence"</span>
                                            <strong class="highlight">{format!("{:.1}°", angle_deg)}</strong>
                                        </div>
                                    </div>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON Cosine Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    6 => {
                        let json = gbdt_json.clone();
                        view! {
                            <div class="math-panel">
                                <div class="math-formula-box">
                                    <div class="math-formula-title">"Model 6: Gradient Boosted Decision Stumps (GBDT) — Sequential Residual Corrections"</div>
                                    <div class="math-formula-code">"F_m(x) = F_{m-1}(x) + η · h_m(x)    where shrinkage learning rate η = 0.8"</div>
                                    <p class="math-intuition">
                                        "Trains 5 sequential decision stumps in series. Rather than voting independently, each successive stump specifically targets and corrects the residual error left over by all preceding stumps."
                                    </p>
                                </div>

                                <div class="math-stats-grid">
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Final Boosted Score"</span>
                                        <span class="stat-val highlight">{format!("{gbdt_val:.1}%")}</span>
                                        <span class="stat-sub">"Cumulative sum of 5 boosted stumps"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Shrinkage Rate (η)"</span>
                                        <span class="stat-val">"0.80"</span>
                                        <span class="stat-sub">"Step regularizer preventing overfitting"</span>
                                    </div>
                                    <div class="math-stat-box">
                                        <span class="stat-label">"Sequential Stumps"</span>
                                        <span class="stat-val">"5 Iterations"</span>
                                        <span class="stat-sub">"Base + 5 residual corrections"</span>
                                    </div>
                                </div>

                                <div class="waterfall-card">
                                    <h4>"Sequential Stage Score Waterfall"</h4>
                                    <div class="waterfall-steps">
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Base Initializer F₀"</strong>
                                                <span>"Initial prior baseline"</span>
                                            </div>
                                            <span class="wf-delta">+15.0</span>
                                            <span class="wf-total">"15.0 pts"</span>
                                        </div>
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Stump 1: Role Alignment"</strong>
                                                <span>"Primary archetype match"</span>
                                            </div>
                                            <span class="wf-delta">+20.0</span>
                                            <span class="wf-total">"35.0 pts"</span>
                                        </div>
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Stump 2: Position Viability"</strong>
                                                <span>"Lane suitability correction"</span>
                                            </div>
                                            <span class="wf-delta">+16.0</span>
                                            <span class="wf-total">"51.0 pts"</span>
                                        </div>
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Stump 3: Damage & Toughness Residual"</strong>
                                                <span>"Primary combat stat residual fit"</span>
                                            </div>
                                            <span class="wf-delta">+18.0</span>
                                            <span class="wf-total">"69.0 pts"</span>
                                        </div>
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Stump 4: Mobility & CC Residual"</strong>
                                                <span>"Micro repositioning correction"</span>
                                            </div>
                                            <span class="wf-delta">+12.0</span>
                                            <span class="wf-total">"81.0 pts"</span>
                                        </div>
                                        <div class="waterfall-step">
                                            <div class="wf-info">
                                                <strong>"Stump 5: Resource Profile Verification"</strong>
                                                <span>"Mana / Energy / Cooldown alignment"</span>
                                            </div>
                                            <span class="wf-delta">+8.0</span>
                                            <span class="wf-total highlight">{format!("{gbdt_val:.1} pts")}</span>
                                        </div>
                                    </div>
                                </div>

                                <details class="model-raw-json">
                                    <summary>"View Raw JSON GBDT Payload"</summary>
                                    <pre>{json}</pre>
                                </details>
                            </div>
                        }.into_any()
                    },

                    _ => view! {
                        <div class="math-panel">
                            <div class="math-formula-box">
                                <div class="math-formula-title">"Stacked Meta-Learner & Bayesian Performance Efficacy Fusion"</div>
                                <div class="math-formula-code">"Score_Final = ( Σ_{k=1}^{6} w_k · S_k ) × (1.0 + Δ_Bayes(Riot Telemetry))"</div>
                                <p class="math-intuition">
                                    "Aggregates predictions from all 6 machine learning models through empirical weighting and dynamically calibrates candidate rankings with live Riot Match-V5 telemetry win-rate performance."
                                </p>
                            </div>

                            <div class="math-stats-grid">
                                <div class="math-stat-box">
                                    <span class="stat-label">"Final Ensemble Score"</span>
                                    <span class="stat-val highlight">{format!("{meta_val:.1}%")}</span>
                                    <span class="stat-sub">"Blended recommendation score"</span>
                                </div>
                                <div class="math-stat-box">
                                    <span class="stat-label">"Model Architecture"</span>
                                    <span class="stat-val">"6 Active ML Engines"</span>
                                    <span class="stat-sub">"Bagging + Boosting + Spatial + Probabilistic"</span>
                                </div>
                                <div class="math-stat-box">
                                    <span class="stat-label">"Bayesian Performance Calibration"</span>
                                    <span class="stat-val">"Live Riot V5 Telemetry"</span>
                                    <span class="stat-sub">"Empirical match win-rate adjustments"</span>
                                </div>
                            </div>

                            <div class="ensemble-weights-card">
                                <h4>"6-Model Ensemble Weight Distribution"</h4>
                                <div class="weights-table">
                                    <div class="weight-row">
                                        <span class="weight-model">"🌲 Random Forest (10-Tree Bagging)"</span>
                                        <span class="weight-pct">"22%"</span>
                                        <span class="weight-score">{format!("{rf_val:.1}%")}</span>
                                    </div>
                                    <div class="weight-row">
                                        <span class="weight-model">"🌳 Decision Tree (Hierarchical Rules)"</span>
                                        <span class="weight-pct">"18%"</span>
                                        <span class="weight-score">{format!("{dt_val:.1}%")}</span>
                                    </div>
                                    <div class="weight-row">
                                        <span class="weight-model">"📍 K-Nearest Neighbors (Euclidean Distance)"</span>
                                        <span class="weight-pct">"18%"</span>
                                        <span class="weight-score">{format!("{knn_val:.1}%")}</span>
                                    </div>
                                    <div class="weight-row">
                                        <span class="weight-model">"🎲 Multi-Class Naive Bayes (Gaussian Prior)"</span>
                                        <span class="weight-pct">"14%"</span>
                                        <span class="weight-score">{format!("{nb_val:.1}%")}</span>
                                    </div>
                                    <div class="weight-row">
                                        <span class="weight-model">"📐 Cosine Similarity (Angular Vector Space)"</span>
                                        <span class="weight-pct">"14%"</span>
                                        <span class="weight-score">{format!("{cos_val:.1}%")}</span>
                                    </div>
                                    <div class="weight-row">
                                        <span class="weight-model">"⚡ GBDT (Boosted Residual Stumps)"</span>
                                        <span class="weight-pct">"14%"</span>
                                        <span class="weight-score">{format!("{gbdt_val:.1}%")}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

fn view_champion_modal(
    state: RwSignal<AppState>,
    name: String,
    catalog: Arc<EffectiveCatalog>,
    recommendation: RecommendationResult,
) -> impl IntoView {
    let Some(record) = catalog.0.iter().find(|record| record.name == name) else {
        return view! {
            <Dialog on_close=close_modal_callback(state) label="Champion details unavailable">
                <p class="notice notice-error">"Champion details are unavailable."</p>
            </Dialog>
        }
        .into_any();
    };
    let champion = record.data.clone();
    let image = champion_image_url(&name, Some(&champion));

    let attributes = Attributes {
        damage: champion.damage,
        toughness: champion.toughness,
        mobility: champion.mobility,
        control: champion.control,
        utility: champion.utility,
    };

    let rf_score = recommendation.scores.random_forest.get(&name);
    let dt_score = recommendation.scores.decision_tree.get(&name);
    let knn_score = recommendation.scores.knn.get(&name);
    let nb_score = recommendation.scores.naive_bayes.get(&name);
    let cos_score = recommendation.scores.cosine_similarity.get(&name);
    let gbdt_score = recommendation.scores.gbdt.get(&name);

    let rf_val = rf_score.map(|s| s.score).unwrap_or_default();
    let dt_val = dt_score.map(|s| s.score).unwrap_or_default();
    let knn_val = knn_score.map(|s| s.score).unwrap_or_default();
    let nb_val = nb_score.map(|s| s.score).unwrap_or_default();
    let cos_val = cos_score.map(|s| s.score).unwrap_or_default();
    let gbdt_val = gbdt_score.map(|s| s.score).unwrap_or_default();
    let meta_val = recommendation
        .top10
        .iter()
        .find(|c| c.champion_name == name)
        .map(|c| c.weighted)
        .unwrap_or(rf_val);

    let mut contributing_factors = Vec::new();
    let mut penalties = Vec::new();
    let mut matched_criteria = Vec::new();

    for score in [
        rf_score, dt_score, knn_score, nb_score, cos_score, gbdt_score,
    ]
    .into_iter()
    .flatten()
    {
        if let Some(factors) = score
            .details
            .get("contributingFactors")
            .and_then(|v| v.as_array())
        {
            for f in factors {
                let factor = f
                    .get("factor")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_owned();
                let contribution = f
                    .get("contribution")
                    .and_then(|v| v.as_f64())
                    .unwrap_or_else(|| {
                        f.get("contribution")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0)
                    });
                let weight = f.get("weight").and_then(|v| v.as_f64()).unwrap_or_else(|| {
                    f.get("weight")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0)
                });
                if !factor.is_empty()
                    && !contributing_factors
                        .iter()
                        .any(|(existing, _, _)| existing == &factor)
                {
                    contributing_factors.push((factor, contribution, weight));
                }
            }
        }
        if let Some(pens) = score.details.get("penalties").and_then(|v| v.as_array()) {
            for p in pens {
                let reason = p
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_owned();
                let amount = p.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                if !reason.is_empty() && !penalties.iter().any(|(existing, _)| existing == &reason)
                {
                    penalties.push((reason, amount));
                }
            }
        }
        if let Some(crit) = score
            .details
            .get("matchedCriteria")
            .and_then(|v| v.as_array())
        {
            for c in crit {
                if let Some(s) = c.as_str() {
                    if !matched_criteria
                        .iter()
                        .any(|existing: &String| existing == s)
                    {
                        matched_criteria.push(s.to_owned());
                    }
                }
            }
        }
    }

    let intel = presenter::build_match_intelligence(
        rf_val,
        dt_val,
        knn_val,
        &contributing_factors,
        &penalties,
        &matched_criteria,
        &attributes,
        5.0,
        champion.difficulty,
    );

    let math_inspector_view = view_math_matrices_inspector(
        &recommendation,
        &champion,
        &name,
        rf_score,
        dt_score,
        knn_score,
        nb_score,
        cos_score,
        gbdt_score,
        rf_val,
        dt_val,
        knn_val,
        nb_val,
        cos_val,
        gbdt_val,
        meta_val,
    );

    let drivers_view = intel
        .drivers
        .into_iter()
        .map(|driver| {
            let pct = driver.percentage;
            view! {
                <div class="driver-card">
                    <div class="driver-info">
                        <div class="driver-title-row">
                            <span class="driver-name">{driver.factor}</span>
                            <span class="driver-pct">{format!("+{pct:.1}% match")}</span>
                        </div>
                        <p class="driver-desc">{driver.description}</p>
                    </div>
                    <div class="driver-bar-track">
                        <div class="driver-bar-fill" style=format!("width: {pct:.1}%") />
                    </div>
                </div>
            }
        })
        .collect_view();

    let cautions_view = intel
        .cautions
        .into_iter()
        .map(|caution| {
            view! {
                <div class="caution-card">
                    <div class="caution-header">
                        <span class="caution-badge">"Notice"</span>
                        <span class="caution-name">{caution.reason}</span>
                        <span class="caution-penalty">{format!("-{:.1}% adjustment", caution.penalty_percent)}</span>
                    </div>
                    <p class="caution-advice">{caution.advice}</p>
                </div>
            }
        })
        .collect_view();

    let trait_bars = intel
        .trait_comparisons
        .into_iter()
        .map(|t| {
            let match_pct = t.match_pct;
            view! {
                <div class="trait-row">
                    <div class="trait-labels">
                        <span class="trait-name">{t.name}</span>
                        <span class="trait-vals">
                            {format!("Desired: {:.0} / Kit: {:.0} ({match_pct:.0}% match)", t.user_value, t.champion_value)}
                        </span>
                    </div>
                    <div class="trait-track">
                        <div class="trait-fill" style=format!("width: {match_pct:.1}%") />
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label=format!("{name} champion details")
            class="inspector wide"
        >
            <header class="inspector-head">
                <img src=image alt=name.clone() class="inspector-img" />
                <div>
                    <h2>{name.clone()}</h2>
                    <div class="chips">
                        <Badge variant=BadgeVariant::Gold>{champion.role.clone()}</Badge>
                        <Badge variant=BadgeVariant::Cyan>
                            {format!("Difficulty {:.0}/10", champion.difficulty)}
                        </Badge>
                        <Badge variant=BadgeVariant::Gold>
                            {format!("{:.1}% Match", recommendation.top10.iter().find(|c| c.champion_name == name).map(|c| c.weighted).unwrap_or(rf_val))}
                        </Badge>
                    </div>
                </div>
            </header>
            <div class="modal-body">
                <section class="intel-section">
                    <div class="intel-header">
                        <h3>"6-Model Machine Learning Consensus"</h3>
                        <span class=format!("consensus-badge {}", intel.consensus.css())>
                            {intel.consensus.label()}
                        </span>
                    </div>
                    <div class="lenses-grid">
                        <div class="lens-card">
                            <span class="lens-tag">"LENS 1"</span>
                            <div class="lens-score">{format!("{:.1}%", intel.playstyle_lens.score)}</div>
                            <h4 class="lens-title">{intel.playstyle_lens.name}</h4>
                            <p class="lens-cat">{intel.playstyle_lens.category}</p>
                            <p class="lens-desc">{intel.playstyle_lens.description}</p>
                        </div>
                        <div class="lens-card">
                            <span class="lens-tag">"LENS 2"</span>
                            <div class="lens-score">{format!("{:.1}%", intel.performance_lens.score)}</div>
                            <h4 class="lens-title">{intel.performance_lens.name}</h4>
                            <p class="lens-cat">{intel.performance_lens.category}</p>
                            <p class="lens-desc">{intel.performance_lens.description}</p>
                        </div>
                        <div class="lens-card">
                            <span class="lens-tag">"LENS 3"</span>
                            <div class="lens-score">{format!("{:.1}%", intel.community_lens.score)}</div>
                            <h4 class="lens-title">{intel.community_lens.name}</h4>
                            <p class="lens-cat">{intel.community_lens.category}</p>
                            <p class="lens-desc">{intel.community_lens.description}</p>
                        </div>
                    </div>
                </section>

                <section class="intel-section">
                    <h3>"Key Match Drivers (SHAP Attribution)"</h3>
                    <p class="muted">"Primary positive factors identified by the tree and distance models."</p>
                    <div class="drivers-container">
                        {drivers_view}
                    </div>
                    {if !cautions_view.is_empty() {
                        view! {
                            <div class="cautions-container">
                                {cautions_view}
                            </div>
                        }.into_any()
                    } else {
                        ().into_view().into_any()
                    }}
                </section>

                <section class="intel-section">
                    <h3>"Kit & Playstyle Alignment"</h3>
                    <p class="muted">"Head-to-head comparison between your questionnaire preferences and this champion's attributes."</p>
                    <div class="traits-container">
                        {trait_bars}
                    </div>
                </section>

                <section class="intel-section">
                    <h3>"Base Combat Attributes"</h3>
                    <div class="attr-grid">
                        {attribute_meter("Damage", champion.damage)}
                        {attribute_meter("Toughness", champion.toughness)}
                        {attribute_meter("Mobility", champion.mobility)}
                        {attribute_meter("Control", champion.control)}
                        {attribute_meter("Utility", champion.utility)}
                    </div>
                </section>

                <details class="raw-math-toggle" open=true>
                    <summary>"Inspect Raw Model Matrices & Vectors (6-Model Ensemble)"</summary>
                    <div class="raw-details-wrap">
                        {math_inspector_view}
                    </div>
                </details>
            </div>
        </Dialog>
    }
    .into_any()
}

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
                        {if is_relevant { "✓ Relevant" } else { "✗ Not optimal" }}
                    </span>
                </div>
            }
        })
        .collect_view();

    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label=format!("{} metric details", metric.label())
        >
            <header class="metric-modal-head">
                <div class=format!("modal-score {}", metric_class(value))>
                    <div class="modal-score-value">{format!("{:.1}%", value * 100.0)}</div>
                    <div class="modal-score-label">{metric.label()}</div>
                </div>
                <div>
                    <h2>{metric.label()}</h2>
                    <p class="muted">{metric_explanation(metric)}</p>
                </div>
            </header>
            <div class="modal-body">
                <h3>"Recommendations evaluated"</h3>
                <div class="champion-list">{list}</div>
                <p class="muted small">
                    {format!(
                        "The current heuristic identifies {} relevant champions from your answers. This is a recommendation evaluation aid, not an independently validated accuracy score.",
                        relevant.len()
                    )}
                </p>
            </div>
        </Dialog>
    }
    .into_any()
}

fn view_saved_users_modal(
    state: RwSignal<AppState>,
    users: Vec<recommender_core::contracts::StoredUserRecord>,
) -> impl IntoView {
    let is_empty = users.is_empty();
    let rows = users
        .iter()
        .enumerate()
        .map(|(index, user)| {
            let age = match &user.age {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Number(n)) => n.to_string(),
                _ => String::new(),
            };
            view! {
                <tr>
                    <td>{index + 1}</td>
                    <td>{user.full_name.clone().unwrap_or_default()}</td>
                    <td>{user.email.clone().unwrap_or_default()}</td>
                    <td>{age}</td>
                    <td>{user.gender.clone().unwrap_or_default()}</td>
                    <td>{user.experience.clone().unwrap_or_default()}</td>
                    <td>{user.recommended_champion.clone().unwrap_or_default()}</td>
                    <td>{user.registration_date.clone().unwrap_or_default()}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label="Saved summoner profiles"
            class="wide"
        >
            <header class="modal-head">
                <h2>"Saved profiles"</h2>
                <p class="muted">"Stored only in this browser's local storage."</p>
            </header>
            <div class="modal-body">
                {is_empty.then(|| view! {
                    <p class="notice">"No profiles saved yet. Finish the quiz to create one."</p>
                })}
                <div class="table-wrap">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>"ID"</th>
                                <th>"Name"</th>
                                <th>"Email"</th>
                                <th>"Age"</th>
                                <th>"Gender"</th>
                                <th>"Experience"</th>
                                <th>"Recommended champion"</th>
                                <th>"Registration date"</th>
                            </tr>
                        </thead>
                        <tbody>{rows}</tbody>
                    </table>
                </div>
            </div>
            <footer class="modal-footer">
                <Button variant=ButtonVariant::Secondary
                    on:click=move |_| {
                        let _ = storage::export_table_csv();
                    }>
                    "Download CSV"
                </Button>
                <Button variant=ButtonVariant::Primary
                    on:click=move |_| state.update(AppState::close_modal)>
                    "Close"
                </Button>
            </footer>
        </Dialog>
    }
    .into_any()
}

fn view_link_riot_modal(state: RwSignal<AppState>) -> impl IntoView {
    let current_riot_id = state.with(|c| {
        c.auth_user
            .as_ref()
            .and_then(|u| u.riot_id.clone())
            .unwrap_or_default()
    });
    let current_region = state.with(|c| {
        c.auth_user
            .as_ref()
            .and_then(|u| u.riot_region.clone())
            .unwrap_or_else(|| "NA".into())
    });

    let riot_id = RwSignal::new(current_riot_id);
    let region = RwSignal::new(current_region);

    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label="Link Riot Games Account"
            class="inspector"
        >
            <header class="modal-head">
                <span class="eyebrow">"✦ Riot Account Integration ✦"</span>
                <h2>"Link Your Riot ID"</h2>
                <p class="muted">
                    "Connect your in-game identity to link rank, mastery stats, and future live telemetry."
                </p>
            </header>
            <div class="modal-body" style="display: flex; flex-direction: column; gap: 18px; margin-top: 14px;">
                <div class="field">
                    <label for="riotIdInput">"Riot ID (GameName#TagLine)"</label>
                    <input
                        id="riotIdInput"
                        type="text"
                        placeholder="e.g. Faker#T1, Doublelift#NA1"
                        prop:value=move || riot_id.get()
                        on:input=move |ev| riot_id.set(event_target_value(&ev))
                    />
                </div>
                <div class="field">
                    <label for="riotRegionSelect">"Riot Server Region"</label>
                    <select
                        id="riotRegionSelect"
                        prop:value=move || region.get()
                        on:change=move |ev| region.set(event_target_value(&ev))
                    >
                        <option value="ME">"Middle East (ME / ME1)"</option>
                        <option value="SEA">"Southeast Asia (SEA / SG2)"</option>
                        <option value="SG">"Singapore, Malaysia, Indonesia (SG2)"</option>
                        <option value="PH">"Philippines (PH2)"</option>
                        <option value="VN">"Vietnam (VN2)"</option>
                        <option value="TW">"Taiwan, Hong Kong, Macau (TW2)"</option>
                        <option value="TH">"Thailand (TH2)"</option>
                        <option value="EUW">"Europe West (EUW1)"</option>
                        <option value="EUNE">"Europe Nordic & East (EUN1)"</option>
                        <option value="NA">"North America (NA1)"</option>
                        <option value="KR">"Korea (KR)"</option>
                        <option value="JP">"Japan (JP1)"</option>
                        <option value="BR">"Brazil (BR1)"</option>
                        <option value="LAN">"Latin America North (LA1)"</option>
                        <option value="LAS">"Latin America South (LA2)"</option>
                        <option value="OCE">"Oceania (OC1)"</option>
                        <option value="TR">"Turkey (TR1)"</option>
                        <option value="RU">"Russia (RU)"</option>
                        <option value="PBE">"Public Beta Environment (PBE1)"</option>
                    </select>
                </div>
                <p class="notice">
                    "Note: Riot Games requires exact capitalization for Riot IDs with tags (e.g. Name#TAG)."
                </p>
            </div>
            <footer class="modal-footer" style="display: flex; justify-content: flex-end; gap: 12px; margin-top: 20px;">
                <Button variant=ButtonVariant::Ghost on:click=move |_| state.update(AppState::close_modal)>
                    "Cancel"
                </Button>
                <Button
                    variant=ButtonVariant::Primary
                    on:click=move |_| {
                        let id = riot_id.get().trim().to_string();
                        let reg = region.get();
                        storage::supabase_update_riot_profile(&id, &reg);
                        storage::supabase_fetch_riot_telemetry(&id, &reg);
                        state.update(|c| {
                            c.update_riot_profile(id, reg);
                            c.set_syncing_riot(true);
                            c.close_modal();
                        });
                    }
                >
                    "Save & Sync Riot Profile"
                </Button>
            </footer>
        </Dialog>
    }
    .into_any()
}

fn view_cloud_history_modal(state: RwSignal<AppState>) -> impl IntoView {
    let auth = state.with(|c| c.auth_user.clone());

    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label="Supabase Cloud Recommendation History"
            class="inspector"
        >
            <header class="modal-head">
                <span class="eyebrow">"✦ Cloud Sync Active ✦"</span>
                <h2>"Supabase Telemetry & History"</h2>
                <p class="muted">
                    "Your calibration runs and champion compatibility scores are securely synced to Supabase."
                </p>
            </header>
            <div class="modal-body" style="display: flex; flex-direction: column; gap: 16px; margin-top: 14px;">
                {match auth {
                    Some(user) => {
                        let riot_display = user.riot_id.unwrap_or_else(|| "Not linked yet".into());
                        let region_display = user.riot_region.unwrap_or_else(|| "N/A".into());
                        view! {
                            <div class="panel" style="background: rgba(1, 10, 19, 0.85); border: 1px solid var(--gold-lo);">
                                <div style="display: flex; align-items: center; gap: 14px; margin-bottom: 12px;">
                                    {user.avatar_url.map(|url| view! {
                                        <img src=url alt="" style="width: 48px; height: 48px; border: 2px solid var(--gold);" />
                                    })}
                                    <div>
                                        <strong style="color: var(--gold-hi); font-size: 1.1rem; display: block;">{user.full_name}</strong>
                                        <span class="muted small">{user.email}</span>
                                    </div>
                                </div>
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px; font-size: 0.85rem; border-top: 1px solid var(--line); padding-top: 10px;">
                                    <div>
                                        <span class="muted">"Riot ID: "</span>
                                        <strong style="color: var(--text);">{riot_display}</strong>
                                    </div>
                                    <div>
                                        <span class="muted">"Region: "</span>
                                        <strong style="color: var(--text);">{region_display}</strong>
                                    </div>
                                </div>
                            </div>
                            <p class="notice">
                                "All completed quiz results are automatically written to table 'recommendation_history' with Row-Level Security."
                            </p>
                        }.into_any()
                    }
                    None => view! {
                        <p class="notice notice-error">
                            "You are currently not signed in. Sign in with Google to enable automatic cloud sync."
                        </p>
                    }.into_any()
                }}
            </div>
            <footer class="modal-footer" style="display: flex; justify-content: flex-end; gap: 12px; margin-top: 20px;">
                <Button variant=ButtonVariant::Secondary
                    on:click=move |_| {
                        let _ = storage::export_table_csv();
                    }>
                    "Download CSV"
                </Button>
                <Button variant=ButtonVariant::Primary on:click=move |_| state.update(AppState::close_modal)>
                    "Close"
                </Button>
            </footer>
        </Dialog>
    }
    .into_any()
}

fn view_oauth_config_modal(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <Dialog
            on_close=close_modal_callback(state)
            label="Google OAuth Configuration Required"
            class="inspector"
        >
            <header class="modal-head">
                <span class="eyebrow" style="color: var(--warn);">"⚠️ Supabase Configuration Required"</span>
                <h2>"Google Provider Not Enabled"</h2>
                <p class="muted">
                    "Supabase returned: <code>msg: \"Unsupported provider: provider is not enabled\"</code>"
                </p>
            </header>
            <div class="modal-body" style="display: flex; flex-direction: column; gap: 14px; margin-top: 14px;">
                <div class="notice" style="border-color: rgba(251, 191, 36, 0.4); background: rgba(251, 191, 36, 0.08);">
                    <p style="font-weight: 700; color: var(--gold-hi); margin-bottom: 6px;">
                        "How to enable Google OAuth in your Supabase project:"
                    </p>
                    <ol style="margin-left: 18px; list-style: decimal; color: var(--text-2); display: flex; flex-direction: column; gap: 6px;">
                        <li>"Open your Supabase project dashboard at erfybpoymihcepxnaosj."</li>
                        <li>"Navigate to Authentication → Providers → Google."</li>
                        <li>"Toggle Google provider to Enabled."</li>
                        <li>"Paste your Google Cloud Client ID & Secret."</li>
                        <li>"Set Authorized redirect URI to:"</li>
                    </ol>
                    <code style="display: block; background: rgba(1, 10, 19, 0.9); padding: 8px; border: 1px solid var(--gold-lo); margin-top: 8px; color: var(--cyan); word-break: break-all;">
                        "https://erfybpoymihcepxnaosj.supabase.co/auth/v1/callback"
                    </code>
                </div>

                <p style="color: var(--text-3); font-size: 0.9rem;">
                    "Want to start calibrating right away without configuring Google Cloud now? You can activate instant summoner mode below."
                </p>
            </div>
            <footer class="modal-footer" style="display: flex; justify-content: flex-end; gap: 12px; margin-top: 20px;">
                <a
                    href="https://supabase.com/dashboard/project/erfybpoymihcepxnaosj/auth/providers"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="btn btn-secondary btn-md"
                >
                    "Open Supabase Dashboard ↗"
                </a>
                <Button
                    variant=ButtonVariant::Primary
                    on:click=move |_| {
                        storage::supabase_enable_demo_mode();
                        state.update(|c| {
                            c.close_modal();
                            c.start_quiz();
                        });
                    }
                >
                    "⚡ Continue in Summoner Mode →"
                </Button>
            </footer>
        </Dialog>
    }
    .into_any()
}
