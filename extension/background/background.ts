import { LEPTOS_DEVTOOLS_CONNENT } from "../utils/constant"

chrome.runtime.onConnect.addListener(port => {
    if (port.name === LEPTOS_DEVTOOLS_CONNENT) {
        port.onMessage.addListener((_message, _port) => {})
    }
})
