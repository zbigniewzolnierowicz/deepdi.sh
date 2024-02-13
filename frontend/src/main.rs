use frontend::routes::InternalRouter;
use leptos::{create_node_ref, mount_to_body, view};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(module = "/dist/.stage/index.js")]
extern "C" {
    fn trial(thing: String);
    fn boop(element: Element);
}

fn main() {
    let element = create_node_ref::<leptos::html::P>();
    let on_click = move |_| {
        let el = element.get().unwrap();
        let element: Element = (*el).clone().into();
        boop(element);
    };

    mount_to_body(move || {
        view! {
            <p on:click={on_click} _ref=element>
                <span>"THIS IS A TEST TEXT"</span>
            </p>
            <InternalRouter />
        }
    })
}
