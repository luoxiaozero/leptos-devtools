use crate::{
    component::Component,
    extension,
    runtime::{post_message, remove_component_children, with_runtime, Owner},
};
use regex::Regex;
use tracing::{span, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

#[derive(Default)]
pub struct Devtools;

impl<S> Layer<S> for Devtools
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, _ctx: Context<'_, S>) {
        let metadata = attrs.metadata();
        if metadata.target() == "leptos_dom::components" && metadata.name() == "<Component />" {
            return;
        }

        let re = Regex::new(r"^<(.*?) />$").unwrap();
        if let Some(name) = re
            .captures(metadata.name())
            .map(|cap| cap.get(1).map(|v| v.as_str()))
            .flatten()
        {
            with_runtime(|runtime| {
                runtime.components.borrow_mut().insert(
                    id.clone(),
                    Component::new(name.to_string(), metadata.target().to_string()),
                );

                let mut owner = runtime.owner.borrow_mut();
                if owner.is_none() {
                    let ancestors = runtime.ancestors.borrow_mut();
                    *owner = Some(Owner::new(id.clone(), ancestors.first().cloned()));
                }
            });
        }
    }

    fn on_enter(&self, id: &span::Id, _ctx: Context<'_, S>) {
        with_runtime(|runtime| {
            let is_dyn_child = {
                let components = runtime.components.borrow();
                let Some(comp) = components.get(id) else {
                    return;
                };
                if comp.name() == "DynChild" && comp.target() == "leptos_dom::components::dyn_child"
                {
                    true
                } else {
                    false
                }
            };

            if is_dyn_child {
                let mut owner = runtime.owner.borrow_mut();
                if owner.is_none() {
                    remove_component_children(id);
                    post_message(|| leptos_devtools_extension_api::ComponentChildrenRemove {
                        id: id.into_non_zero_u64(),
                        deep: true,
                    });

                    let ancestors = runtime.ancestors.borrow_mut();
                    *owner = Some(Owner {
                        id: id.clone(),
                        parent_id: ancestors.first().cloned(),
                    });
                }
            }

            runtime.ancestors.borrow_mut().push(id.clone());
        });
    }

    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        with_runtime(|runtime| {
            let is_dyn_child = {
                let components = runtime.components.borrow();
                let Some(comp) = components.get(id) else {
                    return;
                };
                if comp.name() == "DynChild" && comp.target() == "leptos_dom::components::dyn_child"
                {
                    true
                } else {
                    false
                }
            };

            let mut ancestors = runtime.ancestors.borrow_mut();
            let mut component_tree_set = runtime.component_tree_set.borrow_mut();

            ancestors.pop().expect("ancestors is empty");
            if ancestors.is_empty() {
                if ctx.span(id).unwrap().parent().is_none() {
                    if !component_tree_set.contains(id) {
                        runtime.component_tree_root.borrow_mut().push(id.clone());
                        component_tree_set.insert(id.clone());
                    }
                }

                let mut owner = runtime.owner.borrow_mut();
                if let Some(Owner { id, parent_id }) = owner.take() {
                    post_message(|| extension::generate_extension_component(&id, parent_id));
                }
                return;
            }

            if ancestors.contains(id) {
                return;
            }

            if is_dyn_child {
                let mut owner = runtime.owner.borrow_mut();
                if owner.as_ref().map_or(false, |o| &o.id == id) {
                    if let Some(Owner { id, parent_id }) = owner.take() {
                        post_message(|| extension::generate_extension_component(&id, parent_id));
                    }
                }
                
            }
            if !component_tree_set.contains(id) {
                let parent_id = ancestors.last().expect("ancestors is empty");
                let mut component_tree = runtime.component_tree.borrow_mut();
                if let Some(children) = component_tree.get_mut(parent_id) {
                    children.push(id.clone());
                } else {
                    component_tree.insert(parent_id.clone(), vec![id.clone()]);
                }
                component_tree_set.insert(id.clone());
            }
        });
    }
}
