import { LEPTOS_DEVTOOLS_MESSAGE, LEPTOS_DEVTOOLS_ON_MESSAGE } from "../utils/constant"

export interface Message {
    id: typeof LEPTOS_DEVTOOLS_MESSAGE | typeof LEPTOS_DEVTOOLS_ON_MESSAGE
    payload: Array<Event | OnEvent>
}

export type Event = "DevtoolsPanelOpenStatus" | "OpenDevtoolsPanel" | "PageUnload" | "Detected"

interface DevtoolsPanelOpenStatus {
    DevtoolsPanelOpenStatus: boolean
}

interface Detected {
    Detected: {
        Lepots: boolean
    }
}

export type OnEvent = DevtoolsPanelOpenStatus | Detected
