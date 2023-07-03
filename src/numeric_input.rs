use leptos::*;

#[component]
pub fn NumericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <h1>"Number: "</h1>
        <label>
          "type a number (or not): "
            <input type="text" on:input=on_input />
            <ErrorBoundary
              fallback=|cx, errors| view! { cx,
                <div class="error">
                  <p>"Not a number! Errors: "</p>
                  <ul>
                    { move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { cx, <li>{ e.to_string() }</li>})
                        .collect_view(cx)
                    }
                  </ul>
                </div>
              }
           >
               <p>"You typed: "<strong>{ value }</strong></p>
           </ErrorBoundary>
        </label>
    }
}
