use crate::SelectedComponentId;
use leptos::*;
use std::{collections::HashSet, num::NonZeroU64};

#[component]
pub fn ComponentNode(id: NonZeroU64, name: String, level: u64) -> impl IntoView {
    let selected_comp_id = use_context::<RwSignal<Option<SelectedComponentId>>>()
        .expect("not found SelectedComponentId");
    let expand_component =
        use_context::<RwSignal<HashSet<NonZeroU64>>>().expect("not found expand_component");
    expand_component.with_untracked(|ec| ec.contains(&id));

    let arrow_click = move |_| {
        expand_component.update(|ec| {
            if ec.contains(&id) {
                ec.remove(&id);
            } else {
                ec.insert(id);
            }
        });
    };
    let selected = create_memo(move |_| selected_comp_id.get() == Some(SelectedComponentId(id)));
    view! {
        <div class="node"
            class:node-selected=move || selected.get()
            on:click=move |_| selected_comp_id.set(Some(SelectedComponentId(id)))
        >
            <Indent level />
            <span class="arrow" on:click=arrow_click>
                <span class="arrow-right" class:arrow-bottom=move || expand_component.with(|ec| ec.contains(&id))>
                </span>
            </span>
            <span class="node-component__name">
                "<"{ name }">"
            </span>
            {
                #[cfg(feature = "development")]
                view! {
                    <span class="pl-12px color-#aaa">
                        "id="{ id }
                    </span>
                }
            }
        </div>
    }
}

#[component]
fn Indent(level: u64) -> impl IntoView {
    view! {
        <span>
            <For
                each=move || 0..level
                key=|num| num.clone()
                view=|_num| {
                    view! {
                        <span class="inline-block w-16px"></span>
                    }
                }
            />
        </span>
    }
}
