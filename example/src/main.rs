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
                        <ShowRead read />
                    }.into()
                } else {
                    None
                }
            }
        }
        <ShowRead on:click=move |_| set_read.set(!read.get()) read/>
    }
}

#[component]
fn ShowRead(read: ReadSignal<bool>) -> impl IntoView {
    view! {
        <span>{move || read.get() }</span>
    }
}