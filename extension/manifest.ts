import { defineManifest } from "@crxjs/vite-plugin"
import { version } from "./package.json"

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
    icons: {
        "48": "assets/icons/leptos-logo-48.png",
    },
})
