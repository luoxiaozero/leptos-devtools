use crate::{
    component::Component,
    extension::{generate_extension_component, post_message},
    runtime::{remove_component_children, with_runtime, Owner},
};
use std::fmt::Debug;
use tracing::{
    field::{Field, Visit},
    span, Subscriber,
};
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
        let name = metadata.name();

        if name == "leptos_dom::tracing_props" {
            let mut visitor = PropsVisitor(None);
            attrs.record(&mut visitor);

            with_runtime(|runtime| {
                if let Some(comp_id) = runtime.ancestors.borrow().last() {
                    if let Some(comp) = runtime.components.borrow_mut().get_mut(comp_id) {
                        comp.set_props(visitor.0);
                    }
                }
            });
            return;
        }

        // whether is component
        if !name.starts_with("<") || !name.ends_with(" />") {
            return;
        }

        if metadata.target() == "leptos_dom::components" && name == "<Component />" {
            return;
        }
        if metadata.target() == "leptos_dom::html" && name == "<HtmlElement />" {
            return;
        }

        let Some(name) = name.get(1..(name.len() - 3)) else {
            return;
        };

        with_runtime(|runtime| {
            runtime.components.borrow_mut().insert(
                id.clone(),
                Component::new(
                    name.to_string(),
                    location(
                        runtime.cargo_manifest_dir.borrow().clone(),
                        metadata.module_path(),
                        metadata.file(),
                        metadata.line(),
                    ),
                    metadata.target().to_string(),
                ),
            );

            let mut owner = runtime.owner.borrow_mut();
            if owner.is_none() {
                let ancestors = runtime.ancestors.borrow_mut();
                *owner = Some(Owner::new(id.clone(), ancestors.first().cloned()));
            }
        });
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
                    post_message(|| generate_extension_component(&id, parent_id));
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
                        post_message(|| {
                            let comp = generate_extension_component(&id, parent_id);
                            comp.children
                        });
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

struct PropsVisitor(Option<String>);
impl Visit for PropsVisitor {
    fn record_debug(&mut self, _field: &Field, _value: &dyn Debug) {}
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "props" {
            self.0 = Some(value.to_string());
        }
    }
}

fn location<'a>(
    cargo_manifest_dir: Option<String>,
    module_path: Option<&'a str>,
    file: Option<&'a str>,
    line: Option<u32>,
) -> Option<String> {
    let full_path = match (cargo_manifest_dir, module_path, file) {
        (None, None, None)
        | (None, Some(_), None)
        | (Some(_), None, None)
        | (Some(_), Some(_), None) => None,
        (None, None, Some(file)) | (None, Some(_), Some(file)) | (Some(_), None, Some(file)) => {
            Some(file.to_string())
        }
        (Some(dir), Some(module_path), Some(file)) => {
            if file.starts_with(module_path) {
                Some(format!("{dir}{}", &file[module_path.len()..file.len()]))
            } else {
                Some(file.to_string())
            }
        }
    };

    match (full_path, line) {
        (None, None) => None,
        (None, Some(line)) => Some(format!(":{}", line)),
        (Some(path), None) => Some(path),
        (Some(path), Some(line)) => Some(format!("{}:{}", path, line)),
    }
}
