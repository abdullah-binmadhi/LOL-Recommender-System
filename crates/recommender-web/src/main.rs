#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use recommender_core::{
    contracts::{Answers, QuestionnaireDefinition},
    Engine,
};

#[cfg(target_arch = "wasm32")]
#[component]
fn App() -> impl IntoView {
    let questions = QuestionnaireDefinition::current()
        .expect("bundled production questionnaire must remain valid")
        .effective_questions();
    let question_count = questions.len();
    let answers = RwSignal::new(Answers::default());
    let result_text = RwSignal::new(String::new());

    let question_views = questions
        .into_iter()
        .map(|question| {
            let question_id = question.id.to_string();
            let options = question
                .options
                .into_iter()
                .map(|option| {
                    let answer_id = question_id.clone();
                    let answer_value = option.value.clone();
                    view! {
                        <label class="option">
                            <input
                                type="radio"
                                name=question_id.clone()
                                value=option.value
                                on:change=move |_| {
                                    answers.update(|current| {
                                        current.insert(answer_id.clone(), answer_value.clone());
                                    });
                                }
                            />
                            <span>{option.text}</span>
                        </label>
                    }
                })
                .collect_view();

            view! {
                <fieldset class="question-card">
                    <legend>{format!("{}. {}", question.id, question.text)}</legend>
                    <div class="options">{options}</div>
                </fieldset>
            }
        })
        .collect_view();

    view! {
        <main class="page-shell">
            <header class="hero">
                <span class="eyebrow">"Leptos + WebAssembly · Rust preview"</span>
                <h1>"League of Legends Champion Recommender"</h1>
                <p>
                    "This isolated preview reads the existing questionnaire and effective champion catalog through Rust contracts. The current website remains the default while the migration is in progress."
                </p>
                <p class="contract-summary">
                    {format!("{} production questions · {} effective champions · all scoring runs in Rust", question_count, Engine::reference().map(|engine| engine.champion_count()).unwrap_or_default())}
                </p>
            </header>

            <section class="questionnaire" aria-label="Rust questionnaire preview">
                <h2>"Questionnaire contract preview"</h2>
                <p>"Selections are held in Rust UI state in this browser session and are not persisted."</p>
                {question_views}
                <div class="toolbar">
                    <span>
                        "Answered: "
                        {move || answers.with(|value| value.0.len())}
                        " / "
                        {question_count}
                    </span>
                    <button
                        class="primary-button"
                        disabled=move || answers.with(|value| value.0.len() != question_count)
                        on:click=move |_| {
                            let result = answers.with_untracked(|answers| {
                                Engine::reference()?.recommend_typed(answers)
                            });
                            let text = match result {
                                Ok(recommendation) => {
                                    let top10 = recommendation
                                        .top10
                                        .iter()
                                        .enumerate()
                                        .map(|(index, champion)| {
                                            format!("{}. {} — {:.1}%", index + 1, champion.champion_name, champion.average)
                                        })
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    format!(
                                        "Rust-converted features\nRole: {} · Position: {} · Difficulty: {:.1} · Playstyle: {}\n\nRust recommendations\n{}",
                                        recommendation.features.role,
                                        recommendation.features.position,
                                        recommendation.features.difficulty,
                                        recommendation.features.playstyle,
                                        top10,
                                    )
                                }
                                Err(error) => format!("Rust recommendation failed: {error}"),
                            };
                            result_text.set(text);
                        }
                    >
                        "Run Rust recommendations"
                    </button>
                </div>
            </section>

            <section class="results" aria-live="polite">
                <h2>"Rust result preview"</h2>
                <pre>{move || result_text.get()}</pre>
            </section>
        </main>
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
