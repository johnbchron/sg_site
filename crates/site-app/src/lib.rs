use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Style>{include_str!("../style/fonts.css")}</Style>
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <leptos_meta::Link rel="preload" href="/fonts/inter.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />
    <leptos_meta::Link rel="preload" href="/fonts/anton.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />

    <Title text="Solid Ground Farm"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=HomePage />
        <Route path="/about" view=AboutPage />
        <Route path="/values" view=AboutPage />
        <Route path="/vision" view=AboutPage />
        <Route path="/projects" view=AboutPage />
        <Route path="/donate" view=AboutPage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="mx-auto container min-h-dvh flex flex-col gap-4">
      {children()}
    </div>
  }
}

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
pub fn AboutPage() -> impl IntoView {
  view! {
    <PageWrapper>
      <NavBar />
    </PageWrapper>
  }
}

#[component]
pub fn NavBarLink(title: &'static str, href: &'static str) -> impl IntoView {
  let route = leptos_router::use_route().path();

  view! {
    <a class={format!("text-sm md:text-base text-black {} transition hover:text-black/70", if route == href { "underline" } else { "" })} href={href}>
      {title}
    </a>
  }
}

#[component]
pub fn EllipsisVertical(
  #[prop(default = "")] class: &'static str,
) -> impl IntoView {
  view! {
    <svg class={class} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
    </svg>
  }
}

#[component]
pub fn NavBar() -> impl IntoView {
  view! {
    <div class="fixed top-0 left-0 right-0 z-50 bg-white/60 font-medium backdrop-blur-sm">
      <div class="mx-auto container px-8 h-12 flex flex-row items-center gap-4 md:gap-8 border-black/20 border-b">
        <a href="/" class="text-3xl font-[anton] font-normal link">"SGF"</a>
        <div>
          <div class="hidden sm:flex flex-row gap-2 items-center">
            <NavBarLink title="About" href="/about" />
            <NavBarLink title="Values" href="/values" />
            <NavBarLink title="Vision" href="/vision" />
            <NavBarLink title="Projects" href="/projects" />
          </div>
          <EllipsisVertical class="sm:hidden" />
        </div>
        <div class="flex-1" />
        <FunkyButton />
      </div>
    </div>
  }
}

#[component]
pub fn FunkyButton() -> impl IntoView {
  view! {
    <a href="/" class="group relative inline-block text-sm text-black">
      <span class="absolute inset-0 translate-x-0.5 translate-y-0.5 bg-black transition-transform group-hover:translate-x-0 group-hover:translate-y-0"
      ></span>
      <span class="relative block border border-current bg-white px-4 py-2">"Donate"</span>
    </a>
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
