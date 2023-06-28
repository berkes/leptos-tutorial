use leptos::*;

use crate::progress_bar::ProgressBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let values = vec![1, 2, 3, 4, 5];
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    // create a list of N signals
    let counters = (1..=40).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
        <ul>{counter_buttons}</ul>
        <hr/>
        <p>{values.clone()}</p>
        <ul>
            {values.into_iter().map(|x| view! { cx, <li>{x}</li> }).collect_view(cx)}
        </ul>
        <hr/>
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
