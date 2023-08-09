use crate::{component::Component, extension};
use leptos_devtools_extension_api::{Message, OnEvent, OnMessage, PostMessage};
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
    pub ancestors: RefCell<Vec<span::Id>>,
    pub owner: RefCell<Option<Owner>>,

    pub components: RefCell<HashMap<span::Id, Component>>,
    pub component_tree_root: RefCell<Vec<span::Id>>,
    pub component_tree: RefCell<HashMap<span::Id, Vec<span::Id>>>,
    pub component_tree_set: RefCell<HashSet<span::Id>>,

    // extension
    pub show_devtools: RefCell<bool>,
}

pub(crate) fn with_runtime<T>(f: impl FnOnce(&Runtime) -> T) -> T {
    RUNTIME.with(|runtime| f(runtime))
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

pub fn remove_component_children(id: &span::Id) {
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

pub fn on_message() {
    OnMessage::on_message(|OnMessage { payload, .. }| {
        for event in payload {
            match event {
                OnEvent::ShowDevtools(status) => {
                    let roots = with_runtime(|runtime| {
                        *runtime.show_devtools.borrow_mut() = status;
                        if status {
                            Some(runtime.component_tree_root.borrow().clone())
                        } else {
                            None
                        }
                    });

                    let Some(roots) = roots else {
                        return;
                    };
                    if roots.is_empty() {
                        return;
                    }

                    let mut message = Message::new();
                    for root in roots {
                        message.add_event(
                            extension::generate_extension_component(&root, None).into_event(),
                        )
                    }
                    message.post_message().unwrap()
                }
            }
        }
    })
    .unwrap();
}

pub fn post_message<T>(f: impl FnOnce() -> T)
where
    T: PostMessage,
{
    with_runtime(|runtime| {
        if *runtime.show_devtools.borrow() {
            f().post_message().unwrap()
        }
    });
}
