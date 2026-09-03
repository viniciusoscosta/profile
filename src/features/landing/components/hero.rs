use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="flex-1 w-full flex justify-center">
            <div class="w-full flex flex-col px-4 md:px-8 lg:px-12 xl:px-16">
                <div class="flex-1 w-full bg-white"></div>
                <div class="h-10 w-full bg-black"></div>
            </div>
        </section>
    }
}
