use leptos::*;

use crate::conditions::Conditions;
use crate::name::NameForm;
use crate::numeric_input::NumericInput;
use crate::progress_bar::ProgressBar;

#[component]
fn Wrapper(cx: Scope) -> impl IntoView {
    view! {cx, <div style="border: 1px solid black; padding: 1em;"><Toggler/></div>}
}

#[component]
fn Toggler(cx: Scope) -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>(cx).expect("to have found the setter provided");
    let getter = use_context::<ReadSignal<bool>>(cx).expect("to have found the getter provided");

    let color = create_memo(cx, move |_| if getter() { "green" } else { "red" });

    view! { cx,
        <button style:color=color on:click=move |_| setter.update(|toggled| *toggled = !*toggled) >
            "Make it " { getter }
        </button>
    }
}

#[component]
fn TakesChildren<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn(Scope) -> IV,
    IV: IntoView,
{
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! {cx, <li>{child}</li>})
        .collect_view(cx);

    view! { cx,
        <div>
            <h2>"Render Prop"</h2>
            {render_prop(cx)}
            <h2>"Children"</h2>
            <ul>{children}</ul>
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // create a list of N signals
    let counters = (1..=40).map(|idx| create_signal(cx, idx));

    let (toggled, set_toggled) = create_signal(cx, false);
    provide_context(cx, set_toggled);
    provide_context(cx, toggled);

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

        <TakesChildren render_prop=|_| view! { cx, <p>"Hi from render"</p> }>
          "I am a child"
          <strong>" mee too"</strong>
        </TakesChildren>
        <hr/>

        <NameForm/>
        <hr/>

        <NumericInput/>
        <hr/>

        <Conditions/>
        <hr/>

        <p>"Toggled? " {toggled}</p>
        <Wrapper/>
        <hr/>

        <ul>{counter_buttons}</ul>
    }
}
