import { createPortMessanger } from "../utils/bridge"
import { LEPTOS_DEVTOOLS_MESSAGE, ConnectionName } from "../utils/constant"
import { createMessage, createOnMessage } from "../utils/message"

const port = chrome.runtime.connect({ name: ConnectionName.Content })
const { postPortMessage: toBackground, onPortMessage: fromBackground } = createPortMessanger(port)

toBackground(createMessage("DevtoolsPanelOpenStatus"))
fromBackground(message => {
    window.postMessage(createOnMessage(message.payload))
})

window.addEventListener("message", ev => {
    if (ev.source !== window) {
        return
    }

    if (ev.data.id && ev.data.id === LEPTOS_DEVTOOLS_MESSAGE) {
        toBackground(ev.data)
    }
})
window.addEventListener("unload", () => {
    toBackground(createMessage("PageUnload"))
})
