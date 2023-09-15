use crate::{
    utils::{merge_component, remove_all, remove_component_children},
    SelectedComponentId,
};
use chrome_wasm_bindgen::*;
use leptos::*;
use leptos_devtools_extension_api::{ComponentChildrenRemove, Event, Message};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub(crate) fn chrome() -> Option<Chrome> {
    let obj = window().get("chrome")?;
    Some(obj.unchecked_into())
}

const LEPTOS_DEVTOOLS_PANEL: &str = "LEPTOS_DEVTOOLS_PANEL";

pub(crate) fn on_message(
    message_component_update: RwSignal<bool>,
    selected_component_id: RwSignal<Option<SelectedComponentId>>,
) {
    let port = chrome().unwrap().runtime().connect_with_connect_info(
        ConnectInfo {
            name: Some(LEPTOS_DEVTOOLS_PANEL),
        }
        .into(),
    );

    let on_message = Closure::<dyn FnMut(JsValue)>::new(move |message: JsValue| {
        let Message { payload, .. } = Message::from(message);
        let mut component_update = false;
        for event in payload {
            match event {
                Event::Component(comp) => {
                    merge_component(comp);
                    component_update = true;
                }
                Event::ComponentChildrenRemove(ComponentChildrenRemove { id, deep }) => {
                    remove_component_children(&id, deep);
                    component_update = true;
                }
                Event::OpenDevtoolsPanel => {}
                Event::PageUnload => {
                    remove_all();
                    selected_component_id.set(None);
                    component_update = true;
                }
            }
        }
        if component_update {
            message_component_update.set(true);
        }
    })
    .into_js_value();
    port.on_message().add_listener(&on_message.unchecked_ref());
}
