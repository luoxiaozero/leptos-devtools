use crate::runtime::with_runtime;
use leptos_devtools_extension_api::{Component, Event, Message, OnEvent, OnMessage, PostMessage};
use tracing::span;

pub(crate) fn generate_extension_component(
    id: &span::Id,
    parent_id: Option<span::Id>,
) -> Component {
    let mut comp = with_runtime(|runtime| {
        let components = runtime.components.borrow();
        let comp = components.get(id).unwrap();

        Component {
            parent_id: parent_id.map(|v| v.into_non_zero_u64()),
            id: id.into_non_zero_u64(),
            name: comp.name().clone(),
            props: comp.props().clone(),
            children: vec![],
            location: comp.location().clone(),
        }
    });

    let parent_id = id;
    with_runtime(|runtime| {
        let children = { runtime.component_tree.borrow().get(id).cloned() };
        if let Some(children) = children {
            comp.children = children
                .iter()
                .map(|id| generate_extension_component(id, Some(parent_id.clone())))
                .collect();
        }
    });

    comp
}

pub(crate) fn on_message() {
    OnMessage::on_message(|OnMessage { payload, .. }| {
        for event in payload {
            match event {
                OnEvent::DevtoolsPanelOpenStatus(status) => {
                    let roots = with_runtime(|runtime| {
                        *runtime.devtools_panel_open_status.borrow_mut() = status;
                        if status {
                            Some(runtime.component_tree_root.borrow().clone())
                        } else {
                            Event::OpenDevtoolsPanel
                                .into_message()
                                .post_message()
                                .unwrap();
                            None
                        }
                    });

                    let Some(roots) = roots else {
                        return;
                    };
                    if roots.is_empty() {
                        return;
                    }

                    let payload = roots
                        .into_iter()
                        .map(|root| generate_extension_component(&root, None).into_event())
                        .collect();
                    Message::new(payload).post_message().unwrap()
                }
            }
        }
    })
    .unwrap();
}

pub(crate) fn post_message<T>(f: impl FnOnce() -> T)
where
    T: PostMessage,
{
    with_runtime(|runtime| {
        if *runtime.devtools_panel_open_status.borrow() {
            f().post_message().unwrap()
        }
    });
}
