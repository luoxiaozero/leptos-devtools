mod component_node;
mod tree;

use component_node::ComponentNode;
use leptos::*;
use std::num::NonZeroU64;
pub use tree::merge_component;
use tree::with_component_store;

pub fn get_component_view(id: Option<NonZeroU64>, level: u64) -> Vec<(NonZeroU64, View)> {
    let mut views = vec![];
    let ids = with_component_store(|store| {
        if let Some(id) = id {
            vec![id]
        } else {
            store.tree_root.borrow().clone()
        }
    });

    for id in ids {
        let children = with_component_store(|store| {
            let components = store.components.borrow();
            let Some(comp) = components.get(&id) else {
                return None;
            };

            views.push((
                id,
                view! {
                    <ComponentNode id name=comp.name.clone() level/>
                }
                .into_view(),
            ));

            store.tree.borrow().get(&comp.id).cloned()
        });

        if let Some(children) = children {
            let children: Vec<(NonZeroU64, View)> = children
                .iter()
                .map(|id| get_component_view(Some(id.clone()), level + 1))
                .flatten()
                .collect();
            views.extend(children);
        }
    }
    views
}
