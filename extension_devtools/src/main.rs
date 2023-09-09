mod components;
mod message;
mod utils;

use crate::{
    message::on_message,
    utils::{gen_nodes, Node},
};
use components::{Aside, ComponentNode, Crumb};
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
        gen_nodes(None, 0)
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
            <main class="flex-1 flex flex-col">
                <div class="flex-1 p-8px overflow-auto">
                    <For
                        each=move || nodes_filter.get()
                        key=|node| node.id.clone()
                        view=|node| {
                            let Node { id, name, level } = node;
                            view! {
                                <ComponentNode id name level/>
                            }
                        }
                    />
                </div>
                <Crumb />
            </main>
            <Aside />
        </section>
    }
}
