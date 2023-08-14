import type { Event, OnEvent, Message } from "../types/message"
import { LEPTOS_DEVTOOLS_MESSAGE, LEPTOS_DEVTOOLS_ON_MESSAGE } from "./constant"

export function createMessage(payload: Event | Array<Event> | undefined): Message {
    if (typeof payload === "undefined") {
        payload = []
    } else if (!Array.isArray(payload)) {
        payload = [payload]
    }
    return {
        id: LEPTOS_DEVTOOLS_MESSAGE,
        payload,
    }
}

export function createOnMessage(payload: OnEvent | Array<OnEvent> | undefined): Message {
    if (typeof payload === "undefined") {
        payload = []
    } else if (!Array.isArray(payload)) {
        payload = [payload]
    }
    return {
        id: LEPTOS_DEVTOOLS_ON_MESSAGE,
        payload,
    }
}
