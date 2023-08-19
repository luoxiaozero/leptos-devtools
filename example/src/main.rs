use leptos::*;
use leptos_devtools::devtools;

fn main() {
    devtools();
    mount_to_body(App);
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (read, set_read) = create_signal(cx, false);
    view! {cx,
        {
            move || {
                if read.get() {
                    view! {cx,
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
fn ShowRead(cx: Scope, read: ReadSignal<bool>) -> impl IntoView {
    view! {cx,
        <span>{move || read.get() }</span>
    }
}
