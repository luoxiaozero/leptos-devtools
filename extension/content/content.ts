import {
    LEPTOS_DEVTOOLS_CONNENT,
    LEPTOS_DEVTOOLS_MESSAGE,
    LEPTOS_DEVTOOLS_ON_MESSAGE,
} from "../utils/constant"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_CONNENT })
window.addEventListener("message", ev => {
    if (ev.source !== window) {
        return
    }

    if (ev.data.id && ev.data.id === LEPTOS_DEVTOOLS_MESSAGE) {
        port.postMessage(ev.data)
    }
})

port.postMessage({
    id: LEPTOS_DEVTOOLS_MESSAGE,
    payload: ["DevtoolsPanelOpenStatus"],
})
port.onMessage.addListener((message, _port) => {
    window.postMessage({
        id: LEPTOS_DEVTOOLS_ON_MESSAGE,
        payload: message.payload,
    })
})
