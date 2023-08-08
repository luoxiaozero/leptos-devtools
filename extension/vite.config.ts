import { defineConfig } from "vite"
import { crx } from "@crxjs/vite-plugin"
import { viteStaticCopy as copy } from "vite-plugin-static-copy"
import manifest from "./manifest"

export default defineConfig({
    plugins: [
        crx({ manifest }),
        copy({
            targets: [
                {
                    src: "../extension_devtools/dist/**/*",
                    dest: ".",
                },
                {
                    src: "./assets/devtools-web/**/*",
                    dest: ".",
                },
            ],
        }),
    ],
})
