use leptos::*;

use crate::{NavBar, PageWrapper};

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <PageWrapper>
      <NavBar />
      <MainHero />
    </PageWrapper>
  }
}

#[component]
pub fn MainHero() -> impl IntoView {
  view! {
    <div class="flex flex-row h-dvh items-center justify-center">
      <div class="flex flex-col mt-12 md:mt-0 md:grid md:grid-rows-1 md:grid-cols-2 gap-12 p-8">
        // title side
        <div class="flex flex-col gap-4 justify-center">
          <p class="text-5xl lg:text-7xl font-[anton] uppercase tracking-wide text-balance">
            "We Are Solid Ground Farm"
          </p>
          <p class="text-xl text-balance">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
          </p>
        </div>

        // picture side
        <div class="grid grid-rows-10 grid-cols-9 aspect-[5/4] max-w-xl">
          <div class="relative col-start-3 col-end-9 row-start-5 row-end-11 overflow-hidden rounded-2xl border-2 border-black/80">
            <img src="/photo3.jpeg" class="absolute inset-0 object-cover w-full h-full"/>
          </div>
          <div class="relative col-start-4 col-end-10 row-start-1 row-end-7 overflow-hidden rounded-2xl border-2 border-black/80">
            <img src="/photo2.jpeg" class="absolute inset-0 object-cover w-full h-full"/>
          </div>
          <div class="relative col-start-1 col-end-7 row-start-3 row-end-9 overflow-hidden rounded-2xl border-2 border-black/80">
            <img src="/photo1.jpeg" class="absolute inset-0 object-cover w-full h-full"/>
          </div>
        </div>
      </div>
    </div>
  }
}
