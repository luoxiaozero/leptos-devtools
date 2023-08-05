mod devtools;
mod runtime;
mod extension;
mod component;

pub use devtools::Devtools;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub fn devtools() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .with_ansi(false)
        .pretty()
        .finish()
        .with(Devtools::default())
        .init();
}