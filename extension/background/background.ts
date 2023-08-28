import {
    LEPTOS_DEVTOOLS_CONNENT,
    LEPTOS_DEVTOOLS_DEVTOOLS,
    LEPTOS_DEVTOOLS_DEVELOPER_TOOLS,
    LEPTOS_DEVTOOLS_POPUP,
} from "../utils/constant"
import type { Message } from "../types/message"
import { createMessage, createOnMessage } from "../utils/message"
import icons from "../utils/icon"

const devtoolsPanelPortMap = new Map<number, chrome.runtime.Port>()
const isLepotsSet = new Set<number>()
const developerToolsPortMap = new Map<number, chrome.runtime.Port>()
const contentPortMap = new Map<number, chrome.runtime.Port>()
const popupPortMap = new Map<number, chrome.runtime.Port>()
let activeTabId: number = -1
chrome.tabs.onActivated.addListener(({ tabId }) => (activeTabId = tabId))

chrome.runtime.onConnect.addListener(port => {
    if (port.name === LEPTOS_DEVTOOLS_CONNENT) {
        port.onMessage.addListener((message: Message, port) => {
            const tabId = port.sender!.tab!.id!
            if (!contentPortMap.has(tabId)) {
                contentPortMap.set(tabId, port)
            }
            
            if (message.payload.length === 1) {
                if (message.payload[0] === "DevtoolsPanelOpenStatus") {
                    port.postMessage(
                        createOnMessage({
                            DevtoolsPanelOpenStatus: devtoolsPanelPortMap.has(tabId),
                        })
                    )
                    return
                } else if (message.payload[0] === "OpenDevtoolsPanel") {
                    popupPortMap.get(tabId)?.postMessage(createOnMessage({ Detected: { Lepots: true } }))
                    chrome.action.setIcon({ tabId, path: icons.normal })
                    if (!isLepotsSet.has(tabId)) {
                        isLepotsSet.add(tabId)
                    }
                } else if (message.payload[0] === "PageUnload") {
                    isLepotsSet.delete(tabId)
                    popupPortMap.get(tabId)?.postMessage(createOnMessage({ Detected: { Lepots: false } }))
                }
            }
            const devtoolsPanelPort = devtoolsPanelPortMap.get(tabId)
            if (devtoolsPanelPort) {
                devtoolsPanelPort.postMessage(message)
            } else {
                const developerToolsPort = developerToolsPortMap.get(tabId)
                if (developerToolsPort) {
                    developerToolsPort.postMessage(createMessage("OpenDevtoolsPanel"))
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
                    port.postMessage(createMessage("OpenDevtoolsPanel"))
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
                contentPortMap.get(tabId)?.postMessage(
                    createOnMessage({
                        DevtoolsPanelOpenStatus: true,
                    })
                )
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
    } else if (port.name === LEPTOS_DEVTOOLS_POPUP) {
        popupPortMap.set(activeTabId, port)
        port.onMessage.addListener((message: Message, port) => {
            if (message.payload.length === 1 && message.payload[0] === "Detected") {
                if (activeTabId !== -1 && isLepotsSet.has(activeTabId)) {
                    chrome.action.setIcon({ tabId: activeTabId, path: icons.normal })
                }
                port.postMessage(
                    createOnMessage({ Detected: { Lepots: isLepotsSet.has(activeTabId) } })
                )
            }
        })
        port.onDisconnect.addListener(port => {
            for (const [key, value] of popupPortMap.entries()) {
                if (port === value) {
                    popupPortMap.delete(key)
                    break
                }
            }
        })
    }
})
