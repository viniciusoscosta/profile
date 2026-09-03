use components::{hero::HeroSection, nav::Nav};
use leptos::prelude::*;
// Declara as sub-sessões como privadas para está feature
mod components;

// Expõe publicamente apenas a visualização consolidada da Landing Page.
#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <main class="bg-black">
            <div class="min-h-screen bg-blue-950 flex flex-col">
                <Nav />
                <HeroSection />
            </div>
        </main>
    }
}
