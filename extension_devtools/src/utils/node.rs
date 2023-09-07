use super::component::with_component_store;
use std::num::NonZeroU64;

#[derive(PartialEq, Clone)]
pub(crate) struct Node {
    pub id: NonZeroU64,
    pub level: u64,
    pub name: String,
}

pub(crate) fn gen_nodes(id: Option<NonZeroU64>, level: u64) -> Vec<Node> {
    let mut nodes = vec![];
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

            nodes.push(Node {
                id,
                level,
                name: comp.name.clone(),
            });

            store.tree.borrow().get(&comp.id).cloned()
        });

        if let Some(children) = children {
            let children: Vec<Node> = children
                .iter()
                .map(|id| gen_nodes(Some(id.clone()), level + 1))
                .flatten()
                .collect();
            nodes.extend(children);
        }
    }
    nodes
}
