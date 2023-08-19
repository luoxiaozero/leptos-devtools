mod component;
mod components;
mod message;

use crate::{component::get_component_view, message::on_message};
use components::Aside;
use leptos::*;
use std::{collections::HashSet, num::NonZeroU64};

fn main() {
    mount_to_body(App);
}

#[derive(Clone, PartialEq)]
pub struct SelectedComponentId(NonZeroU64);

#[component]
fn App() -> impl IntoView {
    let message_component_update = create_rw_signal::<bool>(false);
    let selected_component_id = create_rw_signal::<Option<SelectedComponentId>>(None);
    let expand_component = create_rw_signal(HashSet::<NonZeroU64>::new());
    provide_context(selected_component_id);
    provide_context(expand_component);
    on_message(message_component_update);

    let nodes = create_memo(move |_| {
        if message_component_update.get() {
            message_component_update.set_untracked(false);
        }
        get_component_view(None, 0)
    });

    let nodes_filter = create_memo(move |_| {
        nodes.with(|nodes| {
            let mut nodes_filter = vec![];
            let mut level_filter = None;
            for node in nodes {
                if let Some(level) = level_filter {
                    if level < node.level {
                        continue;
                    } else {
                        level_filter = None;
                    }
                }
                if !expand_component.with(|ec| ec.contains(&node.id)) {
                    level_filter = Some(node.level);
                }
                nodes_filter.push(node.clone());
            }
            nodes_filter
        })
    });
    view! {
        <section class="flex h-screen">
            <main class="flex-1 p-8px">
                <For
                    each=move || nodes_filter.get()
                    key=|node| node.id.clone()
                    view=|node| {
                        node.view
                    }
                />
            </main>
            {
                move || if selected_component_id.get().is_some() {
                    view! {
                        <Aside />
                    }.into()
                } else {
                    None
                }
            }
        </section>
    }
}
