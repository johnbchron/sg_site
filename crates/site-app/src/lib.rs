mod about;
mod home;

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
        <Route path="/" view=home::HomePage />
        <Route path="/about" view=about::AboutPage />
        <Route path="/values" view=BlankPage />
        <Route path="/vision" view=BlankPage />
        <Route path="/projects" view=BlankPage />
        <Route path="/donate" view=BlankPage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn PageWrapper(
  #[prop(default = false)] fullscreen: bool,
  children: Children,
) -> impl IntoView {
  view! {
    <div class="mx-auto container min-h-dvh flex flex-col gap-4">
      <NavBar />
      { (!fullscreen).then_some(view! { <div class="mt-12" /> }) }
      {children()}
    </div>
  }
}

#[component]
pub fn BlankPage() -> impl IntoView {
  view! {
    <PageWrapper>
      <div />
    </PageWrapper>
  }
}

#[component]
pub fn Content(children: Children) -> impl IntoView {
  view! {
    <div class="prose px-8">
      {children()}
    </div>
  }
}

#[component]
pub fn NavBarLink(title: &'static str, href: &'static str) -> impl IntoView {
  let route = leptos_router::use_route().path();

  view! {
    <a class={format!(
      "text-sm md:text-base text-black {} transition hover:text-black/70",
      if route == href { "underline" } else { "" }
    )} href={href}>
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
    <a href="/donate" class="group relative inline-block text-sm text-black">
      <span class="absolute inset-0 translate-x-0.5 translate-y-0.5 bg-black transition-transform group-hover:translate-x-0 group-hover:translate-y-0"
      ></span>
      <span class="relative block border border-current bg-white px-4 py-1.5">"Donate"</span>
    </a>
  }
}
