mod component;
mod event;
mod message;

pub use component::{Component, ComponentChildrenRemove, PostMessage};
pub use event::{Event, OnEvent};
pub use message::{register_leptos, Message, OnMessage};
pub use web_sys::console::log_1;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::log_1(&format!($($t)*).into()))
}
