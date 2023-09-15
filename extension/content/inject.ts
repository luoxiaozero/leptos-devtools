import { createMessage } from "../utils/message"

if (Reflect.get(window, "__LEPTOS_DEVTOOLS__") === true) {
    if (document.visibilityState === "hidden" && document.readyState === "complete") {
        document.addEventListener(
            "visibilitychange",
            () => {
                if (document.visibilityState === "visible") {
                    register_leptos()
                }
            },
            {
                once: true,
            }
        )
    } else {
        register_leptos()
    }
}

function register_leptos() {
    console.log(
        "🚧%c%s%c%s",
        "background-color: #ff7f00; color: #fff; border-radius: 3px; padding: 1px 4px",
        "leptos-devtools",
        "",
        " is in early development! Please report any bugs to https://github.com/luoxiaozero/leptos-devtools/issues"
    )
    window.postMessage(createMessage([]))
}
