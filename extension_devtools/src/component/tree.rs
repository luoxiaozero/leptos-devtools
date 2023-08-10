use leptos_devtools_extension_api::Component;
use std::{cell::RefCell, collections::HashMap, num::NonZeroU64};

thread_local! {
    pub(crate) static COMPONENT_STORE: ComponentStore = Default::default();
}

#[derive(Default)]
pub(crate) struct ComponentStore {
    pub components: RefCell<HashMap<NonZeroU64, Component>>,
    pub tree_root: RefCell<Vec<NonZeroU64>>,
    pub tree: RefCell<HashMap<NonZeroU64, Vec<NonZeroU64>>>,
}

pub(crate) fn with_component_store<T>(f: impl FnOnce(&ComponentStore) -> T) -> T {
    COMPONENT_STORE.with(|store| f(store))
}

pub fn merge_component(mut comp: Component) {
    with_component_store(|store| {
        if let Some(parent_id) = comp.parent_id {
            let mut tree = store.tree.borrow_mut();
            if let Some(children) = tree.get_mut(&parent_id) {
                children.push(comp.id);
            } else {
                tree.insert(parent_id, vec![comp.id]);
            }
        } else {
            store.tree_root.borrow_mut().push(comp.id);
        }

        let children: Vec<Component> = comp.children.drain(0..comp.children.len()).collect();
        for child in children {
            merge_component(child);
        }

        store.components.borrow_mut().insert(comp.id, comp);
    })
}
pub fn remove_component_children(id: &NonZeroU64, deep: bool) {
    let children = with_component_store(|store| {
        let Some(ids) = store.tree.borrow_mut().remove(id) else {
            return None;
        };

        ids.iter().for_each(|id| {
            store.components.borrow_mut().remove(id);
        });

        Some(ids)
    });

    if let Some(children) = children {
        children.iter().for_each(|id| {
            remove_component_children(id, deep);
        })
    }
}

pub fn remove_all() {
    with_component_store(|store| {
        store.components.borrow_mut().clear();
        store.tree_root.borrow_mut().clear();
        store.tree.borrow_mut().clear();
    });
}
