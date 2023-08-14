import { LEPTOS_DEVTOOLS_CONNENT, LEPTOS_DEVTOOLS_MESSAGE } from "../utils/constant"
import { createMessage, createOnMessage } from "../utils/message"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_CONNENT })
port.postMessage(createMessage("DevtoolsPanelOpenStatus"))
port.onMessage.addListener((message, _port) => {
    window.postMessage(createOnMessage(message.payload))
})

window.addEventListener("message", ev => {
    if (ev.source !== window) {
        return
    }

    if (ev.data.id && ev.data.id === LEPTOS_DEVTOOLS_MESSAGE) {
        port.postMessage(ev.data)
    }
})
window.addEventListener("unload", () => {
    port.postMessage(createMessage("PageUnload"))
})
