import { defineManifest } from "@crxjs/vite-plugin"
import { version } from "./package.json"
import icons from "./utils/icon"

export default defineManifest({
    manifest_version: 3,
    name: "Leptos Devtools",
    version,
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
        extension_pages: "script-src 'self' 'wasm-unsafe-eval'; default-src 'self' style-src 'self' 'unsafe-inline';",
    },
    action: {
        default_icon: icons.gray,
        default_title: "Leptos Devtools",
        default_popup: "popup/popup.html",
    },
    icons: icons.normal,
})
