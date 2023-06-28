use leptos::*;

#[component]
pub fn ProgressBar<F>(
    cx: Scope,
    #[prop(default = 30)]
    max: u16,
    progress: F) -> impl IntoView 
    where
        F: Fn() -> i32 + 'static,
{

    view! { cx,
        <progress max=max value=progress />
    }
}
