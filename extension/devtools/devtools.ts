import { LEPTOS_DEVTOOLS_DEVELOPER_TOOLS } from "../utils/constant"

const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_DEVELOPER_TOOLS })

port.postMessage({
    payload: [
        {
            TabId: chrome.devtools.inspectedWindow.tabId,
        },
    ],
})

let panel: chrome.devtools.panels.ExtensionPanel | null = null

port.onMessage.addListener(message => {
    if (message.payload.length === 1 && message.payload[0] === "OpenDevtoolsPanel") {
        if (panel) {
            return
        }
        chrome.devtools.panels.create("Leptos", "", "index.html", newPanel => {
            panel = newPanel
        })
    }
})
