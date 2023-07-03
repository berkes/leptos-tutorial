use leptos::{html::Input, ev::SubmitEvent, *};

/// Shows progress towards a goal.
#[component]
pub fn NameForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Bob".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exists").value();

        set_name(value);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Change Name" />
        </form>

        <pre>{name}</pre>
    }
}
