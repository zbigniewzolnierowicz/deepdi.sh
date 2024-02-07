use crate::modules::homepage::router::HomepageRoutes;
use leptos::{component, view, IntoView};
use leptos_router::{Router, Routes};

#[component]
pub fn InternalRouter() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
          // all our routes will appear inside <main>
          <Routes>
              <HomepageRoutes root_path="/" />
          </Routes>
        </main>
      </Router>
    }
}
