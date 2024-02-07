use super::view::Homepage;
use leptos::{component, view, IntoView};
use leptos_router::Route;

#[component(transparent)]
pub fn HomepageRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=Homepage />
    }
}
