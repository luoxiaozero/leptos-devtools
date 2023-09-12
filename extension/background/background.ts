import { ConnectionName } from "../utils/constant"
import type { Message } from "../types/message"
import { createMessage, createOnMessage } from "../utils/message"
import icons from "../utils/icon"
import { createPortMessanger } from "../utils/bridge"
import popups from "../popup/popup"

const devtoolsPanelPortMap = new Map<number, chrome.runtime.Port>()
const developerToolsPortMap = new Map<number, chrome.runtime.Port>()
const contentPortMap = new Map<number, chrome.runtime.Port>()

// let activeTabId: number = -1
// chrome.tabs.onActivated.addListener(({ tabId }) => (activeTabId = tabId))

function handleContent(port: chrome.runtime.Port) {
    const { postPortMessage: toContent, onPortMessage: fromContent } = createPortMessanger(port)
    const tabId = port.sender!.tab!.id!
    toContent(
        createOnMessage({
            DevtoolsPanelOpenStatus: devtoolsPanelPortMap.has(tabId),
        })
    )
    chrome.action.setIcon({ tabId, path: icons.normal })
    chrome.action.setPopup({ tabId, popup: popups.enabled })
    contentPortMap.set(tabId, port)

    fromContent((message: Message, port) => {
        const tabId = port.sender!.tab!.id!

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
                const devtoolsPanelPort = devtoolsPanelPortMap.get(key)
                if (devtoolsPanelPort) {
                    devtoolsPanelPort.postMessage(createMessage("PageUnload"))
                }
                break
            }
        }
    })
}

chrome.runtime.onConnect.addListener(port => {
    switch (port.name) {
        case ConnectionName.Content: {
            handleContent(port)
            break
        }
        case ConnectionName.Developer: {
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
                    if (contentPortMap.has(tabId)) {
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
            break
        }
        case ConnectionName.Devtools: {
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
            break
        }
    }
})
