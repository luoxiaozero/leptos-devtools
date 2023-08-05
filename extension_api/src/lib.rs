mod component;
mod event;
mod message;

pub use component::{Component, ComponentChildrenRemove};
pub use event::Event;
pub use message::Message;
pub use web_sys::console::log_1;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::log_1(&format!($($t)*).into()))
}