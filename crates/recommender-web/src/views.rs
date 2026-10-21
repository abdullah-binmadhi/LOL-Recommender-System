//! Leptos views for every screen. Decisions live in `presenter` and `state`;
//! this module only renders prepared data.

use crate::analytics_view;
use crate::player_analytics_view;
use crate::components::ui::*;
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
        QuestionnaireDefinition::current()
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

    let auth_listener = window_event_listener(
        leptos::ev::Custom::new("supabase-auth-changed"),
        move |_: web_sys::CustomEvent| {
            state.update(|c| c.set_auth_user(storage::supabase_get_current_user()));
        },
    );
    on_cleanup(move || auth_listener.remove());

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
                    state.update(AppState::open_champion_distribution);
                } else if hash == "#analytics" || hash == "#player-analytics" {
                    state.update(AppState::open_player_analytics);
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
    let question_total = questions.len();

    view! {
        <div class="app-shell">
            {view_navbar(state)}
            <main class="page">
                {move || match state.with(|current| current.screen) {
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
                        "✦ Player Analytics ✦"
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
                    <Badge variant=BadgeVariant::Gold>"3-model ML ensemble"</Badge>
                    <h1 class="hero-title">
                        "Find the champion that " <em>"plays like you"</em>
                    </h1>
                    <p class="hero-lede">
                        "Answer a short calibration quiz. A K-Nearest Neighbors model, a Decision Tree and a Random Forest score every champion, then show you exactly why the winner fits."
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
                    <strong>"3"</strong>
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

                view! {
                    <section
                        id=format!("question-{}", question.id)
                        class="panel question-card"
                        aria-label="Questionnaire"
                    >
                        <header class="quiz-head">
                            <div class="quiz-meta">
                                <span class="quiz-step">
                                    {format!("Question {} of {}", question.id, question_count)}
                                </span>
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
                </div>
            </div>
        </section>

        <section class="block" aria-labelledby="top10-title">
            <div class="block-head hex-block-head">
                <span class="hex-rune-tag">"✦ HEXTECH MATCHMAKING ENGINE ✦"</span>
                <h2 id="top10-title" class="section-title">"🥇 Primal Synergy Podium"</h2>
                <p class="muted">"Your top 3 highest-compatibility champions across all 3 AI models."</p>
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

    let rf_val = rf_score.map(|s| s.score).unwrap_or_default();
    let dt_val = dt_score.map(|s| s.score).unwrap_or_default();
    let knn_val = knn_score.map(|s| s.score).unwrap_or_default();

    let mut contributing_factors = Vec::new();
    let mut penalties = Vec::new();
    let mut matched_criteria = Vec::new();

    for score in [dt_score, rf_score, knn_score].into_iter().flatten() {
        if let Some(factors) = score.details.get("contributingFactors").and_then(|v| v.as_array()) {
            for f in factors {
                let factor = f.get("factor").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                let contribution = f.get("contribution").and_then(|v| v.as_f64()).unwrap_or_else(|| {
                    f.get("contribution").and_then(|v| v.as_str()).and_then(|s| s.parse().ok()).unwrap_or(0.0)
                });
                let weight = f.get("weight").and_then(|v| v.as_f64()).unwrap_or_else(|| {
                    f.get("weight").and_then(|v| v.as_str()).and_then(|s| s.parse().ok()).unwrap_or(0.0)
                });
                if !factor.is_empty() && !contributing_factors.iter().any(|(existing, _, _)| existing == &factor) {
                    contributing_factors.push((factor, contribution, weight));
                }
            }
        }
        if let Some(pens) = score.details.get("penalties").and_then(|v| v.as_array()) {
            for p in pens {
                let reason = p.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                let amount = p.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                if !reason.is_empty() && !penalties.iter().any(|(existing, _)| existing == &reason) {
                    penalties.push((reason, amount));
                }
            }
        }
        if let Some(crit) = score.details.get("matchedCriteria").and_then(|v| v.as_array()) {
            for c in crit {
                if let Some(s) = c.as_str() {
                    if !matched_criteria.iter().any(|existing: &String| existing == s) {
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

    let raw_details = [
        ("Random Forest", rf_score),
        ("Decision Tree", dt_score),
        ("K-Nearest Neighbors", knn_score),
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
                <summary>
                    <span>{label}</span>
                    <strong>{format!("{score_value:.1}%")}</strong>
                </summary>
                <pre>{details}</pre>
            </details>
        }
    })
    .collect_view();

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
                        <h3>"3-Lens Machine Learning Consensus"</h3>
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

                <details class="raw-math-toggle">
                    <summary>"Inspect Raw Model Matrices & Vectors"</summary>
                    <div class="raw-details-wrap">
                        {raw_details}
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
                        <option value="NA">"North America (NA)"</option>
                        <option value="EUW">"Europe West (EUW)"</option>
                        <option value="EUNE">"Europe Nordic & East (EUNE)"</option>
                        <option value="KR">"Korea (KR)"</option>
                        <option value="BR">"Brazil (BR)"</option>
                        <option value="LAN">"Latin America North (LAN)"</option>
                        <option value="LAS">"Latin America South (LAS)"</option>
                        <option value="OCE">"Oceania (OCE)"</option>
                        <option value="JP">"Japan (JP)"</option>
                        <option value="TR">"Turkey (TR)"</option>
                        <option value="RU">"Russia (RU)"</option>
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
                        state.update(|c| {
                            c.update_riot_profile(id, reg);
                            c.close_modal();
                        });
                    }
                >
                    "Save Riot Profile"
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
