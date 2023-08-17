import { Message } from "../types/message"
import { LEPTOS_DEVTOOLS_POPUP } from "../utils/constant"
import { createMessage } from "../utils/message"

window.onload = () => {
    setLeptos(false)
    const port = chrome.runtime.connect({ name: LEPTOS_DEVTOOLS_POPUP })
    port.postMessage(createMessage("Detected"))
    port.onMessage.addListener((message: Message, _port) => {
        if (
            message.payload.length === 1 &&
            typeof message.payload[0] === "object" &&
            "Detected" in message.payload[0]
        ) {
            setLeptos(message.payload[0].Detected.Lepots)
        }
    })
}

function setLeptos(state: boolean) {
    if (state) {
        document.body.innerText = "Lepots is detected on this page."
    } else {
        document.body.innerText = "Lepots not detected."
    }
}
