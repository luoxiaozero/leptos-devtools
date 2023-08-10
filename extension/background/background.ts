import {
    LEPTOS_DEVTOOLS_CONNENT,
    LEPTOS_DEVTOOLS_DEVTOOLS,
    LEPTOS_DEVTOOLS_DEVELOPER_TOOLS,
    LEPTOS_DEVTOOLS_MESSAGE,
} from "../utils/constant"
interface Message {
    id: typeof LEPTOS_DEVTOOLS_MESSAGE
    payload: Array<any>
}

const devtoolsPanelPortMap = new Map<number, chrome.runtime.Port>()
const isLepotsSet = new Set<number>()
const developerToolsPortMap = new Map<number, chrome.runtime.Port>()
const contentPortMap = new Map<number, chrome.runtime.Port>()

chrome.runtime.onConnect.addListener(port => {
    if (port.name === LEPTOS_DEVTOOLS_CONNENT) {
        port.onMessage.addListener((message: Message, port) => {
            const tabId = port.sender!.tab!.id!
            if (!contentPortMap.has(tabId)) {
                contentPortMap.set(tabId, port)
            }
            if (message.payload.length === 1 && message.payload[0] === "DevtoolsPanelOpenStatus") {
                port.postMessage({
                    id: LEPTOS_DEVTOOLS_MESSAGE,
                    payload: [
                        {
                            DevtoolsPanelOpenStatus: devtoolsPanelPortMap.has(tabId),
                        },
                    ],
                })
                return
            }
            const devtoolsPanelPort = devtoolsPanelPortMap.get(tabId)
            if (devtoolsPanelPort) {
                devtoolsPanelPort.postMessage(message)
            } else {
                const developerToolsPort = developerToolsPortMap.get(tabId)
                if (developerToolsPort) {
                    developerToolsPort.postMessage({ payload: ["OpenDevtoolsPanel"] })
                } else {
                    isLepotsSet.add(tabId)
                }
            }
        })
        port.onDisconnect.addListener(port => {
            for (const [key, value] of contentPortMap.entries()) {
                if (port === value) {
                    contentPortMap.delete(key)
                    break
                }
            }
        })
    } else if (port.name === LEPTOS_DEVTOOLS_DEVELOPER_TOOLS) {
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
                developerToolsPortMap.set(tabId, port)
                if (isLepotsSet.has(tabId)) {
                    port.postMessage({
                        payload: ["OpenDevtoolsPanel"],
                    })
                }
            }
        })
        port.onDisconnect.addListener(port => {
            for (const [key, value] of developerToolsPortMap.entries()) {
                if (port === value) {
                    developerToolsPortMap.delete(key)
                    break
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
                devtoolsPanelPortMap.set(tabId, port)
                contentPortMap.get(tabId)?.postMessage({
                    id: LEPTOS_DEVTOOLS_MESSAGE,
                    payload: [
                        {
                            DevtoolsPanelOpenStatus: true,
                        },
                    ],
                })
            }
        })
        port.onDisconnect.addListener(port => {
            for (const [key, value] of devtoolsPanelPortMap.entries()) {
                if (port === value) {
                    devtoolsPanelPortMap.delete(key)
                    break
                }
            }
        })
    }
})
