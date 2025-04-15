<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";

    export let darkMode: boolean = false;

    const theme = writable<"light" | "dark">(darkMode ? "dark" : "light");

    export function toggleTheme(): void {
        theme.update((current) => {
            const newTheme: "light" | "dark" = current === "light" ? "dark" : "light";
            document.documentElement.classList.toggle("dark", newTheme === "dark");
            localStorage.setItem("theme", newTheme);
            return newTheme;
        });
    }

    onMount(() => {
        // Check if user has a saved theme preference
        const savedTheme = localStorage.getItem("theme") as "light" | "dark" | null;
        if (savedTheme) {
            theme.set(savedTheme);
            document.documentElement.classList.toggle("dark", savedTheme === "dark");
        } else if (
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: dark)").matches
        ) {
            theme.set("dark");
            document.documentElement.classList.add("dark");
        }

        window
            .matchMedia("(prefers-color-scheme: dark)")
            .addEventListener("change", (event: MediaQueryListEvent) => {
                if (!localStorage.getItem("theme")) {
                    const newTheme: "light" | "dark" = event.matches ? "dark" : "light";
                    theme.set(newTheme);
                    document.documentElement.classList.toggle("dark", newTheme === "dark");
                }
            });

        return () => {
            window
                .matchMedia("(prefers-color-scheme: dark)")
                .removeEventListener("change", () => {});
        };
    });
</script>

<slot></slot>
