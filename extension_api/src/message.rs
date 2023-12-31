use crate::{Event, OnEvent};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const LEPTOS_DEVTOOLS_MESSAGE: &str = "LEPTOS_DEVTOOLS_MESSAGE";
const LEPTOS_DEVTOOLS_ON_MESSAGE: &str = "LEPTOS_DEVTOOLS_ON_MESSAGE";

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap_throw();
}

pub fn register_leptos() {
    _ = WINDOW.with(|window| {
        js_sys::Reflect::set(
            window,
            &wasm_bindgen::JsValue::from("__LEPTOS_DEVTOOLS__"),
            &wasm_bindgen::JsValue::TRUE,
        )
        .unwrap()
    });
}

fn post_message(message: &wasm_bindgen::JsValue) -> Result<(), JsValue> {
    WINDOW.with(|window| window.post_message(message, "*"))
}

fn on_message(cb: impl Fn(OnMessage) + 'static) -> Result<(), JsValue> {
    fn wel(cb: Box<dyn FnMut(web_sys::MessageEvent)>) -> Result<(), JsValue> {
        let cb = Closure::wrap(cb).into_js_value();
        WINDOW.with(|window| window.add_event_listener_with_callback("message", cb.unchecked_ref()))
    }

    wel(Box::new(move |ev| {
        if let Some(message) = OnMessage::from_js_value(ev.data()) {
            cb(message)
        }
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: String,
    pub payload: Vec<Event>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: LEPTOS_DEVTOOLS_MESSAGE.to_string(),
            payload: vec![],
        }
    }
}

impl Message {
    pub fn new(payload: Vec<Event>) -> Self {
        Self {
            id: LEPTOS_DEVTOOLS_MESSAGE.to_string(),
            payload,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.payload.push(event);
    }

    pub fn post_message(self) -> Result<(), JsValue> {
        let value = serde_wasm_bindgen::to_value(&self).unwrap();
        post_message(&value)
    }

    pub fn into_js_value(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

impl From<JsValue> for Message {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OnMessage {
    id: String,
    pub payload: Vec<OnEvent>,
}

impl OnMessage {
    pub fn on_message(cb: impl Fn(OnMessage) + 'static) -> Result<(), JsValue> {
        on_message(cb)
    }

    pub fn from_js_value(value: JsValue) -> Option<Self> {
        let message: OnMessage = serde_wasm_bindgen::from_value(value).ok()?;
        if message.id == LEPTOS_DEVTOOLS_ON_MESSAGE {
            Some(message)
        } else {
            None
        }
    }
}
