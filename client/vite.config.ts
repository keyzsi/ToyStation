import { defineConfig } from "vite";

export default defineConfig({
    build: {
        target: "ES2022"
    },
    resolve: {
        alias: {
            "@": new URL("./src", import.meta.url).pathname
        }
    }
});