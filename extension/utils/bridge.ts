export function createPortMessanger(port: chrome.runtime.Port) {
    const listeners: Array<(message: any) => void> = []

    let connected = true
    port.onDisconnect.addListener(() => {
        console.log(`${port.name} port disconnected.`)
        connected = false
        listeners.splice(0, listeners.length)
        port.onMessage.removeListener(onMessage)
    })

    function onMessage(message: any) {
        listeners.forEach(fn => fn(message))
    }
    port.onMessage.addListener(onMessage)

    return {
        postPortMessage: (message: any) => {
            if (!connected) return
            port.postMessage(message)
        },
        onPortMessage: (handler: (message: any) => void) => {
            if (!connected) return

            listeners.push(handler)
        },
    }
}
