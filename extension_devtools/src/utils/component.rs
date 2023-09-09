use leptos_devtools_extension_api::Component;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, num::NonZeroU64};

thread_local! {
    pub(crate) static COMPONENT_STORE: ComponentStore = Default::default();
}

#[derive(Default, Clone, Deserialize, PartialEq)]
pub(crate) struct Prop {
    pub name: String,
    pub value: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Default)]
pub(crate) struct ComponentStore {
    pub components: RefCell<HashMap<NonZeroU64, Component>>,
    pub props: RefCell<HashMap<NonZeroU64, Vec<Prop>>>,
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

        if let Some(props) = comp.props.take() {
            if let Ok(props) = serde_json::from_str::<Vec<Prop>>(&props) {
                store.props.borrow_mut().insert(comp.id, props);
            }
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

pub(crate) fn get_component_props(comp_id: &NonZeroU64) -> Vec<Prop> {
    with_component_store(|store| {
        store
            .props
            .borrow()
            .get(comp_id)
            .map_or(vec![], |p| p.to_vec())
    })
}

#[derive(PartialEq, Clone)]
pub(crate) struct ComponentInfo {
    pub location: Option<String>,
}

pub(crate) fn get_component_info(comp_id: &NonZeroU64) -> Option<ComponentInfo> {
    with_component_store(|store| {
        store
            .components
            .borrow()
            .get(comp_id)
            .map(|comp| ComponentInfo {
                location: comp.location.clone(),
            })
    })
}
