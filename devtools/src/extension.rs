use crate::runtime::with_runtime;
use leptos_devtools_extension_api::Component;
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
