import { LEPTOS_DEVTOOLS_MESSAGE, LEPTOS_DEVTOOLS_ON_MESSAGE } from "../utils/constant"

export interface Message {
    id: typeof LEPTOS_DEVTOOLS_MESSAGE | typeof LEPTOS_DEVTOOLS_ON_MESSAGE
    payload: Array<Event | OnEvent>
}

type Event = "DevtoolsPanelOpenStatus"


interface DevtoolsPanelOpenStatus {
    DevtoolsPanelOpenStatus: boolean
}

type OnEvent = DevtoolsPanelOpenStatus
