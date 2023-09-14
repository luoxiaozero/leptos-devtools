mod component;
mod devtools;
mod extension;
mod runtime;

use devtools::Devtools;
use extension::on_message;
use leptos_devtools_extension_api::register_leptos;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub use runtime::set_cargo_manifest_dir;

pub fn devtools() {
    register_leptos();
    on_message();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
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

#[macro_export]
macro_rules! devtools {
    () => {
        leptos_devtools::set_cargo_manifest_dir(env!("CARGO_MANIFEST_DIR").to_string());
        leptos_devtools::devtools();
    };
}
