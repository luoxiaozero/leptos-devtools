use crate::{Event, Message};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use wasm_bindgen::JsValue;

pub trait PostMessage {
    fn post_message(self) -> Result<(), JsValue>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub parent_id: Option<NonZeroU64>,
    pub id: NonZeroU64,
    pub name: String,
    pub props: Option<String>,
    pub children: Vec<Component>,
}

impl Component {
    pub fn into_event(self) -> Event {
        Event::Component(self)
    }
}

impl PostMessage for Component {
    fn post_message(self) -> Result<(), JsValue> {
        self.into_event().into_message().post_message()
    }
}

impl PostMessage for Vec<Component> {
    fn post_message(self) -> Result<(), JsValue> {
        let payload = self.into_iter().map(|v| v.into_event()).collect();
        Message::new(payload).post_message()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentChildrenRemove {
    pub id: NonZeroU64,
    pub deep: bool,
}

impl ComponentChildrenRemove {
    pub fn into_event(self) -> Event {
        Event::ComponentChildrenRemove(self)
    }
}

impl PostMessage for ComponentChildrenRemove {
    fn post_message(self) -> Result<(), JsValue> {
        self.into_event().into_message().post_message()
    }
}
