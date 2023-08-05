use crate::{Component, ComponentChildrenRemove, Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    Component(Component),
    ComponentChildrenRemove(ComponentChildrenRemove),
}

impl Event {
    pub fn into_message(self) -> Message {
        let mut message = Message::new();
        message.add_event(self);
        message
    }
}
