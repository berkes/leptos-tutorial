use leptos::*;

/// Shows progress towards a goal.
#[component]
pub fn ProgressBar(
    cx: Scope,
    #[prop(default = 30)]
    /// The goal to reach.
    max: u16,
    #[prop(into)]
    /// The current progress towards the goal.
    progress: Signal<i32>,
) -> impl IntoView {
    let low = max as f32 * 0.2;
    let high = max as f32 * 0.8;
    let optimum = max as f32 * 0.6;

    view! { cx,
        <meter max=max value=progress min=0 high=high low=low optimum=optimum />
    }
}
