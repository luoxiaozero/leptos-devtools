use crate::Event;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    pub parent_id: Option<NonZeroU64>,
    pub id: NonZeroU64,
    pub name: String,
    pub children: Vec<Component>,
}

impl Component {
    pub fn into_event(self) -> Event {
        Event::Component(self)
    }
}

impl Component {
    pub fn post_message(self) -> Result<(), JsValue> {
        self.into_event().into_message().post_message()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentChildrenRemove {
    pub id: NonZeroU64,
    pub deep: bool
}

impl ComponentChildrenRemove {
    pub fn into_event(self) -> Event {
        Event::ComponentChildrenRemove(self)
    }
}

impl ComponentChildrenRemove {
    pub fn post_message(self) -> Result<(), JsValue> {
        self.into_event().into_message().post_message()
    }
}