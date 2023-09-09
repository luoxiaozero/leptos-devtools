use crate::{
    utils::{get_component_info, ComponentInfo},
    SelectedComponentId,
};
use leptos::*;

#[component]
pub fn AsideComponentInfo() -> impl IntoView {
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let info = create_memo(move |_| {
        if let Some(comp_id) = selected_comp_id.get() {
            get_component_info(&comp_id.0)
        } else {
            None
        }
    });

    move || {
        if let Some(info) = info.get() {
            let ComponentInfo { location } = info;
            view! {
                <div class="my-6px">"component"</div>
                <div class="ml-14px h-20px line-height-20px">
                    <span class="color-#8128e8">"location"</span>
                    <span class="mr-0.5em">":"</span>
                    {
                        if let Some(location) = location {
                            view! {
                                <span class="white-space-nowrap">{format!(r#""{location}""#)}</span>
                            }.into()
                        } else {
                            None
                        }
                    }
                </div>
            }
            .into()
        } else {
            None
        }
    }
}
