import { LEPTOS_DEVTOOLS_CONNENT, LEPTOS_DEVTOOLS_MESSAGE } from "../utils/constant"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_CONNENT })
window.addEventListener("message", ev => {
    if (ev.source !== window) {
        return
    }

    if (ev.data.id && ev.data.id === LEPTOS_DEVTOOLS_MESSAGE) {
        port.postMessage(ev)
    }
})
