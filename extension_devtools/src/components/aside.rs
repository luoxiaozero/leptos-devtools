use super::aside_props::AsideProps;
use leptos::*;

#[component]
pub fn Aside() -> impl IntoView {
    view! {
        <aside class="w-320px p-8px font-size-14px">
            <AsideProps/>
        </aside>
    }
}
