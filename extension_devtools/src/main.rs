mod component;
mod message;

use std::num::NonZeroU64;
use crate::{component::get_component_view, message::on_message};
use leptos::*;

fn main() {
    mount_to_body(App);
}

#[derive(Clone, PartialEq)]
pub struct SelectedComponentId(NonZeroU64);

#[component]
fn App() -> impl IntoView {
    let message_component_update = create_rw_signal::<bool>(false);
    let selected_component_id = create_rw_signal::<Option<SelectedComponentId>>(None);
    provide_context(selected_component_id);
    on_message(message_component_update);

    let component_views = create_memo(move |_| {
        if message_component_update.get() {
            message_component_update.set_untracked(false);
        }
        get_component_view(None, 0)
    });
    view! {
        <section class="flex h-screen">
            <main class="flex-1 p-8px">
                <For
                    each=move || component_views.get()
                    key=|view| view.0.clone()
                    view=|view| {
                        view.1
                    }
                />
            </main>
        </section>
    }
}
