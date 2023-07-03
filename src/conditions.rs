use leptos::*;

#[component]
fn Big(cx: Scope) -> impl IntoView {
    view! { cx,
        <span style="font-size: 2em">
            "BIG"
        </span>
    }
}

#[component]
fn Small(cx: Scope) -> impl IntoView {
    view! { cx,
        <span style="font-size: 0.5em">
            "small"
        </span>
    }
}

#[component]
pub fn Conditions(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;

    let message = move || {
        if is_odd() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };

    let spoken = move || match value() {
        0 => "Zero",
        1 => "One",
        42 => "Fourtytwo",
        _n if is_odd() => "Odd",
        _ => "Even",
    };

    let size = move || {
        if value() > 5 {
            log!("{}: rendering big", value());
            "big"
        } else {
            log!("{}: rendering small", value());
            "small"
        }
    };

    view! { cx,
         <input type="number" value=value on:input=move |ev| set_value(event_target_value(&ev).parse().unwrap()) />
         <p>
         { spoken }
         " - "
         { message }
         " - "
         { size }
         </p>
         <Show
           when={move || value() > 5}
           fallback=|cx| view! { cx, <Small/> }
         >
             <Big/>
         </Show>

    }
}
