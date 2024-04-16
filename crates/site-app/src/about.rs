use leptos::*;

use super::{Content, PageWrapper};

#[component]
pub fn AboutPage() -> impl IntoView {
  view! {
    <PageWrapper>
      <Content>
        <h1>"About Us"</h1>
      </Content>
    </PageWrapper>
  }
}
