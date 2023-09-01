use leptos::*;
use leptos_devtools::devtools;

fn main() {
    devtools();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (read, set_read) = create_signal(false);
    view! {
        {
            move || {
                if read.get() {
                    view! {
                        <ShowRead read count=3/>
                    }.into()
                } else {
                    None
                }
            }
        }
        <ShowRead on:click=move |_| set_read.set(!read.get()) read count=2/>
    }
}

#[component]
fn ShowRead(read: ReadSignal<bool>, count: u32) -> impl IntoView {
    view! {
        <span>{move || read.get() } { count }</span>
    }
}
