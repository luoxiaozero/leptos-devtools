use leptos::*;
use router::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos_devtools::devtools!();
    mount_to_body(|| view! { <RouterExample/> })
}
