use crate::Event;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const LEPTOS_DEVTOOLS_MESSAGE: &str = "LEPTOS_DEVTOOLS_MESSAGE";

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap_throw();
}

fn post_message(message: &wasm_bindgen::JsValue) -> Result<(), JsValue> {
    WINDOW.with(|window| window.post_message(message, "*"))
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: String,
    pub payload: Vec<Event>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            id: LEPTOS_DEVTOOLS_MESSAGE.to_string(),
            payload: vec![],
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.payload.push(event);
    }

    pub fn post_message(self) -> Result<(), JsValue> {
        let value = serde_wasm_bindgen::to_value(&self).unwrap();
        post_message(&value)
    }
}

impl From<JsValue> for Message {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}
