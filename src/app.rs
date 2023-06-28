use leptos::*;

use crate::progress_bar::ProgressBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! { cx,
        <button
            class:odd=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|x| *x += 1);
            }
        >
            "Click me: "
            { move || count.get() }
        </button>
        <br/>

        <ProgressBar progress=count/>
        <br/>
        <ProgressBar progress=Signal::derive(cx, double_count) />
       <p>
            <strong>"Count: "</strong>
            {count}
        </p>
       <p>
            <strong>"Double count: "</strong>
            {double_count}
        </p>
    }
}
