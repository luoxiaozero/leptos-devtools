use crate::{Component, ComponentChildrenRemove, Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    Component(Component),
    ComponentChildrenRemove(ComponentChildrenRemove),
    TabId(u32),
    OpenDevtoolsPanel,
    PageUnload,
}

impl Event {
    pub fn into_message(self) -> Message {
        let mut message = Message::new();
        message.add_event(self);
        message
    }
}

#[derive(Serialize, Deserialize)]
pub enum OnEvent {
    DevtoolsPanelOpenStatus(bool),
}

#[test]
fn serde() {
    assert_eq!(
        "{\"id\":\"LEPTOS_DEVTOOLS_MESSAGE\",\"payload\":[\"PageUnload\"]}",
        serde_json::to_string(&Event::PageUnload.into_message()).unwrap()
    )
}
