use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::features::landing::LandingPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router base="/profile">
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=LandingPage/>
            </Routes>
        </Router>
    }
}
