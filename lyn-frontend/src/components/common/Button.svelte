<script lang="ts">
    // Define prop types
    export let type: "button" | "submit" | "reset" = "button";
    export let variant: "primary" | "secondary" | "accent" = "primary";
    export let size: "small" | "medium" | "large" = "medium";
    export let disabled: boolean = false;
    export let fullWidth: boolean = false;
    export let icon: string | null = null;
    export let loading: boolean = false;

    // Define restProps type
    interface $$restProps {
        "class"?: string;
        "on:click"?: (event: MouseEvent) => void;
        "on:mouseover"?: (event: MouseEvent) => void;
        "on:focus"?: (event: FocusEvent) => void;
        [key: string]: any;
    }

    // Generate class based on props
    $: classes = [
        "btn",
        `btn-${variant}`,
        size === "small" ? "text-xs py-1 px-2" : "",
        size === "large" ? "text-base py-3 px-6" : "",
        fullWidth ? "w-full" : "",
        loading ? "opacity-70 cursor-not-allowed" : "",
        $$restProps.class,
    ]
        .filter(Boolean)
        .join(" ");
</script>

<button
    {type}
    class={classes}
    disabled={disabled || loading}
    on:click
    on:mouseover
    on:focus
    {...$$restProps}
>
    {#if loading}
        <span
            class="h-4 w-4 rounded-full border-2 border-t-transparent border-white animate-spin mr-2"
        ></span>
    {/if}

    {#if icon && !loading}
        <span class="mr-2">{icon}</span>
    {/if}

    <slot></slot>
</button>

<style>
    /* Any custom styles not covered by Tailwind can go here */
</style>
