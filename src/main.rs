use leptos::*;
mod app;
use app::App;

mod progress_bar;
mod name;
mod conditions;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}
