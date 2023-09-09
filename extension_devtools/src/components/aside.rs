use super::aside_component::AsideComponentInfo;
use super::aside_props::AsideProps;
use crate::SelectedComponentId;
use leptos::*;

#[component]
pub fn Aside(aside_width: RwSignal<i32>) -> impl IntoView {
    let is_mouse_move = create_rw_signal(false);
    let on_mouse_down = move |_| {
        is_mouse_move.set(true);
    };
    let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
        is_mouse_move.set(false);
    });
    on_cleanup(move || on_mouse_up.remove());
    let on_mouse_move = window_event_listener(ev::mousemove, move |ev| {
        if is_mouse_move.get() {
            let ev_x = ev.x();
            let client_width = document().body().unwrap().client_width();
            if ev_x <= 320 {
                aside_width.set(client_width - 320);
            } else if ev_x >= client_width - 320 {
                aside_width.set(320);
            } else {
                aside_width.set(client_width - ev_x);
            }
        }
    });
    on_cleanup(move || on_mouse_move.remove());
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let style = create_memo(move |_| {
        if selected_comp_id.get().is_none() {
            String::from("display: none;")
        } else {
            format!("width: {}px", aside_width.get())
        }
    });
    view! {
        <aside class="flex font-size-14px box-border" style=move || style.get()>
            <div class="relative w-6px left--3px cursor-ew-resize" on:mousedown=on_mouse_down></div>
            <div class="p-8px overflow-auto">
                <AsideProps/>
                <AsideComponentInfo />
            </div>
        </aside>
    }
}
