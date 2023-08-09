import { LEPTOS_DEVTOOLS_DEVTOOLS_HTML } from "../utils/constant"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_DEVTOOLS_HTML })

port.postMessage({
    payload: [
        {
            TabId: chrome.devtools.inspectedWindow.tabId,
        },
    ],
})

let panel: chrome.devtools.panels.ExtensionPanel | null = null

port.onMessage.addListener(message => {
    if (
        message.payload.length === 1 &&
        typeof message.payload[0] === "object" &&
        "ShowDevtools" in message.payload[0] &&
        message.payload[0]["ShowDevtools"]
    ) {
        if (panel) {
            return
        }
        chrome.devtools.panels.create("Leptos", "", "index.html", newPanel => {
            panel = newPanel
        })
    }
})
