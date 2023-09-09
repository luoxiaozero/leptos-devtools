use crate::{
    utils::{get_component_crumbs, ComponentCrumb},
    SelectedComponentId,
};
use leptos::*;

#[component]
pub fn Crumb() -> impl IntoView {
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let crumbs = create_memo(move |_| {
        selected_comp_id.with(|comp| {
            if let Some(comp) = comp {
                get_component_crumbs(&comp.0)
            } else {
                vec![]
            }
        })
    });
    // TODO overflow-x-auto
    view! {
        <div class="h-22px line-height-22px b-t b-t-solid b-t-#ddd pl-8px overflow-x-auto">
            <For
                each=move || crumbs.get()
                key=|crumb| crumb.id.clone()
                view=move |crumb| {
                    let ComponentCrumb { id, name } = crumb;
                    let on_click = move |_| {
                        selected_comp_id.set(Some(SelectedComponentId(id)));
                    };
                    view! {
                        <span class="inline-block px-6px color-#2080f0 hover-bg-#f3f3f5 cursor-pointer" on:click=on_click>
                            { name }
                        </span>
                    }
                }
            />
        </div>
    }
}
