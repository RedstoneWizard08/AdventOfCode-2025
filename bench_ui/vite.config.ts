import { svelte, vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import UnoCSS from "unocss/vite";

export default defineConfig({
    plugins: [
        UnoCSS(),
        svelte({
            preprocess: vitePreprocess(),
        }),
    ],

    base: "CI" in process.env ? "/AdventOfCode-2025/" : "/",
});
