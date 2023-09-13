import { createPortMessanger } from "../utils/bridge"
import { LEPTOS_DEVTOOLS_MESSAGE, ConnectionName } from "../utils/constant"
import { createOnMessage } from "../utils/message"
// @ts-expect-error ?script&module query ensures output in ES module format and only import the script path
import injectPath from "./inject?script&module"

let port: chrome.runtime.Port | null = null
let toBackground: ((message: any) => void) | null = null

window.addEventListener("message", ev => {
    if (ev.source !== window) {
        return
    }

    if (ev.data.id && ev.data.id === LEPTOS_DEVTOOLS_MESSAGE) {
        if (port) {
            toBackground && toBackground(ev.data)
            return
        }

        port = chrome.runtime.connect({ name: ConnectionName.Content })
        const { postPortMessage, onPortMessage: fromBackground } = createPortMessanger(port)
        toBackground = postPortMessage
        fromBackground(message => {
            window.postMessage(createOnMessage(message.payload))
        })
    }
})

function inject(src: string) {
    return new Promise<void>((resolve, reject) => {
        const script = document.createElement("script")
        script.src = chrome.runtime.getURL(src)
        script.type = "module"
        script.addEventListener("error", err => reject(err))

        document.head.append(script)
        script.addEventListener("load", () => resolve())
    })
}
inject(injectPath).catch(err => console.error("Detector script failed to load.", err))
