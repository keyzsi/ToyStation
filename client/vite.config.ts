import { defineConfig } from "vite";
import mkcert from "vite-plugin-mkcert";

export default defineConfig({
    plugins: [
        mkcert()
    ],
    build: {
        target: "ES2022"
    },
    resolve: {
        alias: {
            "@": new URL("./src", import.meta.url).pathname
        }
    },
    server: {
        https: true
    }
});