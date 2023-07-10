use wasm_bindgen_test::*;
use leptos::*;
use wasm_bindgen::JsCast;
use leptos_tutorial::*;

// tell the test runner to run tests in the browser
wasm_bindgen_test_configure!(run_in_browser);

// like marking a regular test with #[test]
#[wasm_bindgen_test]
fn clear() {
    let document = leptos::document();
    let test_wrapper = document.create_element("section").unwrap();
    document.body().unwrap().append_child(&test_wrapper).unwrap();

    // start by rendering our counter and mounting it to the DOM
    // note that we start at the initial value of 10
    mount_to(
        test_wrapper.clone().unchecked_into(),
        |cx| view! { cx, <SimpleCounter initial_value=10 step=1/> },
    );

    let div = test_wrapper.query_selector("div").unwrap().unwrap();
    let clear = test_wrapper
        .query_selector("button")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();


    clear.click();

    assert_eq!(
        div.outer_html(),
        // here we spawn a mini reactive system to render the test case
        run_scope(create_runtime(), |cx| {
            // it's as if we're creating it with a value of 0, right?
            let (value, set_value) = create_signal(cx, 0);

            // we can remove the event listeners because they're not rendered to HTML
            view! { cx,
            <div>
                <button>"Clear"</button>
                <button>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button>"+1"</button>
                </div>
            }
            // the view returned an HtmlElement<Div>, which is a smart pointer for
            // a DOM element. So we can still just call .outer_html()
            .outer_html()
        })
        );
}
