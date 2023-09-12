import { createMessage } from "../utils/message"

if (Reflect.get(window, "__LEPTOS_DEVTOOLS__") === true) {
    window.postMessage(createMessage([]))
}
