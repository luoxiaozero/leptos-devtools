import { Message } from "../types/message"
import { createPortMessanger } from "../utils/bridge"
import { ConnectionName } from "../utils/constant"
import { createMessage } from "../utils/message"

window.onload = () => {
    setLeptos(false)
    const port = chrome.runtime.connect({ name: ConnectionName.Popup })
    const { postPortMessage: toBackground, onPortMessage: fromBackground } =
        createPortMessanger(port)
    toBackground(createMessage("Detected"))
    fromBackground((message: Message) => {
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
