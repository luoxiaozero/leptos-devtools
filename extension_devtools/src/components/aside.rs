use super::aside_component::AsideComponentInfo;
use super::aside_props::AsideProps;
use crate::SelectedComponentId;
use leptos::*;

#[component]
pub fn Aside() -> impl IntoView {
    let mouse_move_value = create_rw_signal::<Option<i32>>(None);
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
                mouse_move_value.set(Some(client_width - 320));
            } else if ev_x >= client_width - 320 {
                mouse_move_value.set(Some(320));
            } else {
                mouse_move_value.set(Some(client_width - ev_x));
            }
        }
    });
    on_cleanup(move || on_mouse_move.remove());
    let selected_comp_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
    let style = create_memo(move |_| {
        if selected_comp_id.get().is_none() {
            String::from("display: none;")
        } else if let Some(value) = mouse_move_value.get() {
            format!("width: {value}px")
        } else {
            String::new()
        }
    });
    view! {
        <aside class="w-320px flex font-size-14px" style=move || style.get()>
            <div class="relative w-6px left--3px cursor-ew-resize" on:mousedown=on_mouse_down></div>
            <div class="p-8px overflow-auto">
                <AsideProps/>
                <AsideComponentInfo />
            </div>
        </aside>
    }
}
