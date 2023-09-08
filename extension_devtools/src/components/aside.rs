use super::aside_props::AsideProps;
use leptos::*;

#[component]
pub fn Aside() -> impl IntoView {
    view! {
        <aside class="w-320px flex font-size-14px">
            <div class="p-8px overflow-auto">
                <AsideProps/>
            </div>
        </aside>
    }
}
