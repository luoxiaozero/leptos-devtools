import {
    LEPTOS_DEVTOOLS_CONNENT,
    LEPTOS_DEVTOOLS_DEVTOOLS,
    LEPTOS_DEVTOOLS_DEVTOOLS_HTML,
    LEPTOS_DEVTOOLS_MESSAGE,
} from "../utils/constant"
interface Message {
    id: typeof LEPTOS_DEVTOOLS_MESSAGE
    payload: Array<any>
}

const devtoolsPortMap = new Map<number, chrome.runtime.Port>()
const devtoolsHtmlPortMap = new Map<number, chrome.runtime.Port>()
const contentPortMap = new Map<number, chrome.runtime.Port>()
chrome.runtime.onConnect.addListener(port => {
    if (port.name === LEPTOS_DEVTOOLS_CONNENT) {
        port.onMessage.addListener((message: Message, port) => {
            const tabId = port.sender!.tab!.id!
            if (!contentPortMap.has(tabId)) {
                contentPortMap.set(tabId, port)
            }
            if (message.payload.length === 1 && message.payload[0] === "ShowDevtools") {
                port.postMessage({
                    id: LEPTOS_DEVTOOLS_MESSAGE,
                    payload: [
                        {
                            ShowDevtools: devtoolsPortMap.has(tabId),
                        },
                    ],
                })
                return
            }
            const devtoolsPort = devtoolsPortMap.get(tabId)

            if (devtoolsPort) {
                devtoolsPort.postMessage(message)
            }
        })
    } else if (port.name === LEPTOS_DEVTOOLS_DEVTOOLS_HTML) {
        port.onMessage.addListener((message, port) => {
            if (
                message.payload.length === 1 &&
                typeof message.payload[0] === "object" &&
                "TabId" in message.payload[0]
            ) {
                const tabId: number | null = message.payload[0]["TabId"]
                if (!tabId) {
                    return
                }
                devtoolsHtmlPortMap.set(tabId, port)
                if (contentPortMap.has(tabId)) {
                    port.postMessage({
                        payload: [
                            {
                                ShowDevtools: true,
                            },
                        ],
                    })
                }
            }
        })
    } else if (port.name === LEPTOS_DEVTOOLS_DEVTOOLS) {
        port.onMessage.addListener((message, port) => {
            if (
                message.payload.length === 1 &&
                typeof message.payload[0] === "object" &&
                "TabId" in message.payload[0]
            ) {
                const tabId: number | null = message.payload[0]["TabId"]
                if (!tabId) {
                    return
                }
                devtoolsPortMap.set(tabId, port)
            }
        })
    }
})
