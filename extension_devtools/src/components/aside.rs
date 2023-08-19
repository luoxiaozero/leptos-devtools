use crate::{component::get_component_props, SelectedComponentId};
use leptos::*;

#[component]
pub fn Aside() -> impl IntoView {
    let selected_comp_id = use_context::<RwSignal<Option<SelectedComponentId>>>()
        .expect("not found SelectedComponentId");
    let props = create_memo(move |_| {
        if let Some(comp_id) = selected_comp_id.get() {
            get_component_props(&comp_id.0)
        } else {
            vec![]
        }
    });
    view! {
        <aside class="w-360px p-8">
            <div>
                "Props"
            </div>
            <For
                each=move || props.get()
                key=|prop| prop.name.clone()
                view=|prop| {
                    view! {
                        <div>
                            <span>{ prop.name }</span>
                            {
                                if let Some(err) = prop.error {
                                    Some(view! {
                                        ":"
                                        <span title=err>"Error"</span>
                                    })
                                } else if let Some(value) = prop.value {
                                    Some(view! {
                                        ":"
                                        <span>{ value.to_string() }</span>
                                    })
                                } else {
                                    None
                                }
                            }
                        </div>
                    }
                }
            />
        </aside>
    }
}
