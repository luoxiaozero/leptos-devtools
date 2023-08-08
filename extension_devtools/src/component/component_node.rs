use std::num::NonZeroU64;
use leptos::*;
use crate::SelectedComponentId;

#[component]
pub fn ComponentNode(id: NonZeroU64, name: String, level: u64) -> impl IntoView {
    let selected_comp_id = use_context::<RwSignal<Option<SelectedComponentId>>>().expect("not found SelectedComponentId");
    view! {
        <div class="node"
            class:node-selected=move || selected_comp_id.get() == Some(SelectedComponentId(id))
            on:click=move |_| selected_comp_id.set(Some(SelectedComponentId(id)))
        >
            <Indent level>
            </Indent>
            <span class="node-component__name">
                "<"{ name }">"
            </span>
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

// #[component]
// pub fn Arrow(cx: Scope, show: bool) -> impl IntoView {
//     if show {
//         view! {cx,
//             <span class="inline-block w-4 h-4">
//                 <span class="arrow-right"></span>
//             </span>
//         }
//     } else {
//         view! {cx,
//             <span class="inline-block w-4 h-4">
//             </span>
//         }
//     }
// }