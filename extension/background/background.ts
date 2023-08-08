import { LEPTOS_DEVTOOLS_CONNENT, LEPTOS_DEVTOOLS_MESSAGE } from "../utils/constant"
interface Message {
    id: typeof LEPTOS_DEVTOOLS_MESSAGE
    payload: Array<any>
}

const devtoolsPortMap = new Map<number, chrome.runtime.Port>()
chrome.runtime.onConnect.addListener(port => {
    if (port.name === LEPTOS_DEVTOOLS_CONNENT) {
        port.onMessage.addListener((message: Message, port) => {
            if (message.payload.length === 1 && message.payload[0] === "ShowDevtools") {
                port.postMessage({
                    id: LEPTOS_DEVTOOLS_MESSAGE,
                    payload: [
                        {
                            ShowDevtools: devtoolsPortMap.has(port.sender!.tab!.id!),
                        },
                    ],
                })
                return
            }
            const devtoolsPort = devtoolsPortMap.get(port.sender!.tab!.id!)
            if (devtoolsPort) {
                devtoolsPort.postMessage(message)
            }
        })
    }
})
