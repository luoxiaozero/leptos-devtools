import { createMessage } from "../utils/message"

if (Reflect.get(window, "__LEPTOS_DEVTOOLS__") === true) {
    if (document.visibilityState === "hidden" && document.readyState === "complete") {
        document.addEventListener(
            "visibilitychange",
            () => {
                if (document.visibilityState === "visible") {
                    window.postMessage(createMessage([]))
                }
            },
            {
                once: true,
            }
        )
    } else {
        window.postMessage(createMessage([]))
    }
}
