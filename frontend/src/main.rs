use leptos::{mount_to_body, view};
use wasm_bindgen::prelude::*;
use frontend::routes::InternalRouter;

#[wasm_bindgen(module = "/thing.js")]
extern "C" {
    pub fn jsfunc();
}

fn main() {
    mount_to_body(|| view! { <InternalRouter></InternalRouter> })
}
