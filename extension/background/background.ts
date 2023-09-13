import { ConnectionName } from "../utils/constant"
import type { Message } from "../types/message"
import { createMessage, createOnMessage } from "../utils/message"
import icons from "../utils/icon"
import { createPortMessanger } from "../utils/bridge"
import popups from "../popup/popup"

const panelPortMap = new Map<number, chrome.runtime.Port>()
const developerToolsPortMap = new Map<number, chrome.runtime.Port>()
const contentPortMap = new Map<number, chrome.runtime.Port>()

let activeTabId: number = -1
chrome.tabs.onActivated.addListener(({ tabId }) => (activeTabId = tabId))

function handleContent(port: chrome.runtime.Port) {
    const { postPortMessage: toContent, onPortMessage: fromContent } = createPortMessanger(port)
    const tabId = port.sender!.tab!.id!
    toContent(
        createOnMessage({
            DevtoolsPanelOpenStatus: panelPortMap.has(tabId),
        })
    )
    chrome.action.setIcon({ tabId, path: icons.normal })
    chrome.action.setPopup({ tabId, popup: popups.enabled })
    contentPortMap.set(tabId, port)

    fromContent((message: Message, port) => {
        const tabId = port.sender!.tab!.id!

        const devtoolsPanelPort = panelPortMap.get(tabId)
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
                const devtoolsPanelPort = panelPortMap.get(key)
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
            if (activeTabId === -1) {
                return
            }
            const { postPortMessage: toDeveloper, onDisconnect } = createPortMessanger(port)

            developerToolsPortMap.set(activeTabId, port)
            if (contentPortMap.has(activeTabId)) {
                toDeveloper(createMessage("OpenDevtoolsPanel"))
            }
            onDisconnect(() => {
                developerToolsPortMap.delete(activeTabId)
            })
            break
        }
        case ConnectionName.Panel: {
            if (activeTabId === -1) {
                return
            }
            const { onDisconnect } = createPortMessanger(port)

            panelPortMap.set(activeTabId, port)
            contentPortMap.get(activeTabId)?.postMessage(
                createOnMessage({
                    DevtoolsPanelOpenStatus: true,
                })
            )
            onDisconnect(() => {
                panelPortMap.delete(activeTabId)
            })
            break
        }
    }
})
