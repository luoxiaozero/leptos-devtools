import { LEPTOS_DEVTOOLS_DEVTOOLS } from "../utils/constant"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_DEVTOOLS })

console.log(chrome.devtools, chrome.devtools.inspectedWindow, chrome.devtools.inspectedWindow.tabId)

port.postMessage({
    payload: [
        {
            TabId: chrome.devtools.inspectedWindow.tabId,
        },
    ],
})

port.onMessage.addListener(message => {
    if (
        message.payload.length === 1 &&
        typeof message.payload[0] === "object" &&
        "ShowDevtools" in message.payload[0] &&
        message.payload[0]["ShowDevtools"]
    ) {
        chrome.devtools.panels.create("Leptos", "", "index.html", _panel => {})
    }
})
