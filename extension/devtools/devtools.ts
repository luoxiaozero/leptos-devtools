import { createPortMessanger } from "../utils/bridge"
import { ConnectionName } from "../utils/constant"

const port = chrome.runtime.connect({ name: ConnectionName.Developer })
const { postPortMessage: toBackground, onPortMessage: fromBackground } = createPortMessanger(port)
toBackground({
    payload: [
        {
            TabId: chrome.devtools.inspectedWindow.tabId,
        },
    ],
})

let panel: chrome.devtools.panels.ExtensionPanel | null = null

fromBackground(message => {
    if (message.payload.length === 1 && message.payload[0] === "OpenDevtoolsPanel") {
        if (panel) {
            return
        }
        chrome.devtools.panels.create("Leptos", "", "index.html", newPanel => {
            panel = newPanel
        })
    }
})
