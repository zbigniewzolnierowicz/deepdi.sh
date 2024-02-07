use super::view::Homepage;
use leptos::{component, view, IntoView};
use leptos_router::Route;

#[component(transparent)]
pub fn HomepageRoutes(root_path: &'static str) -> impl IntoView {
    view! {
        <Route path={root_path} view=Homepage />
    }
}
