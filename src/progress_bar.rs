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
    progress: Signal<i32>) -> impl IntoView 
{

    view! { cx,
        <progress max=max value=progress />
    }
}
