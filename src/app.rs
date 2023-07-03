use leptos::{ev::SubmitEvent, *};
use leptos::html::Input;

use crate::progress_bar::ProgressBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // create a list of N signals
    let counters = (1..=40).map(|idx| create_signal(cx, idx));

    // create a signal for the name
    let (name, set_name) = create_signal(cx, "Bob".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exists").value();

        set_name(value);
    };

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
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Change Name" />
        </form>

        <pre>{name}</pre>

        <hr/>
        <ul>{counter_buttons}</ul>
    }
}
