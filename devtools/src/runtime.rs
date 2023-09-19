use crate::component::Component;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};
use tracing::span;

thread_local! {
    pub(crate) static RUNTIME: Runtime = Default::default();
}

#[derive(Default)]
pub(crate) struct Runtime {
    pub cargo_manifest_dir: RefCell<Option<String>>,

    pub is_memo_view: RefCell<Option<MemoView>>,
    pub store_id: RefCell<HashMap<String, Vec<span::Id>>>,
    // user_id -> store_id
    pub use_id: RefCell<HashMap<String, String>>,

    pub ancestors: RefCell<Vec<AncestorId>>,
    pub owner: RefCell<Option<Owner>>,

    pub components: RefCell<HashMap<span::Id, Component>>,
    pub component_tree_root: RefCell<Vec<span::Id>>,
    pub component_tree: RefCell<HashMap<span::Id, Vec<span::Id>>>,
    pub component_tree_set: RefCell<HashSet<span::Id>>,

    // extension
    pub devtools_panel_open_status: RefCell<bool>,
}

pub(crate) fn with_runtime<T>(f: impl FnOnce(&Runtime) -> T) -> T {
    RUNTIME.with(|runtime| f(runtime))
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum AncestorId {
    SpanId(span::Id),
    StoreId(String, span::Id),
}

#[derive(Debug)]
pub(crate) struct Owner {
    pub id: span::Id,
    pub parent_id: Option<span::Id>,
}

impl Owner {
    pub fn new(id: span::Id, parent_id: Option<span::Id>) -> Self {
        Self { id, parent_id }
    }
}

pub(crate) fn remove_component_children(id: &span::Id) {
    let children = with_runtime(|runtime| {
        let Some(children) = runtime.component_tree.borrow_mut().remove(id) else {
            return None;
        };
        children.iter().for_each(|id| {
            runtime.component_tree_set.borrow_mut().remove(id);
            runtime.components.borrow_mut().remove(id);
        });

        Some(children)
    });

    if let Some(children) = children {
        children.iter().for_each(|id| {
            remove_component_children(id);
        })
    }
}

#[derive(Debug)]
pub(crate) struct MemoView {
    comp_id: span::Id,
    pub store_id: Option<String>,
    pub use_id: Option<String>,
}

impl MemoView {
    pub fn new(comp_id: span::Id) -> Self {
        Self {
            comp_id,
            store_id: None,
            use_id: None,
        }
    }

    pub fn is_clear(&self, comp_id: &span::Id) -> bool {
        if self.comp_id == *comp_id && self.store_id.is_some() && self.use_id.is_some() {
            true
        } else {
            false
        }
    }
}

pub fn set_cargo_manifest_dir(dir: String) {
    with_runtime(|runtime| {
        *runtime.cargo_manifest_dir.borrow_mut() = Some(dir);
    })
}
