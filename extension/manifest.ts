import { defineManifest } from "@crxjs/vite-plugin"
import { version } from "./package.json"
import icons from "./utils/icon"
import popups from "./popup/popup"

export default defineManifest({
    manifest_version: 3,
    name: "Leptos Devtools",
    version,
    description: "Browser DevTools extension for debugging Leptos applications.",
    homepage_url: "https://github.com/luoxiaozero/leptos-devtools",
    content_scripts: [
        {
            js: ["content/content.ts"],
            matches: ["*://*/*"],
            run_at: "document_end",
        },
    ],
    background: {
        service_worker: "background/background.ts",
        type: "module",
    },
    devtools_page: "devtools/devtools.html",
    content_security_policy: {
        extension_pages:
            "script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; default-src 'self';",
    },
    action: {
        default_icon: icons.gray,
        default_title: "Leptos Devtools",
        default_popup: popups.disabled,
    },
    icons: icons.normal,
})
