use leptos::*;

use crate::conditions::Conditions;
use crate::name::NameForm;
use crate::numeric_input::NumericInput;
use crate::progress_bar::ProgressBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // create a list of N signals
    let counters = (1..=40).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n -= 1)
                    >
                        "-"
                    </button>
                    <input type="range" min=0 max=40 value=count on:input=move |ev| set_count(event_target_value(&ev).parse().unwrap()) />
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        "+"
                    </button>
                    <br/>
                    <ProgressBar progress=count max=40 />
                    {count}
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
        <h1>"Counter Buttons"</h1>

        <NameForm/>
        <hr/>

        <NumericInput/>
        <hr/>

        <Conditions/>
        <hr/>

        <ul>{counter_buttons}</ul>
    }
}
