<script lang="ts">
    export let title: string | null = null;
    export let subtitle: string | null = null;
    export let padding: string = "p-4";
    export let elevated: boolean = false;
    export let bordered: boolean = true;
    export let rounded: boolean = true;
    export let animate: boolean = false;

    interface $$restProps {
        class?: string;
        [key: string]: any;
    }

    $: classes = [
        "bg-white",
        padding,
        elevated ? "shadow-elevated" : "",
        bordered ? "border border-gray-200" : "",
        rounded ? "rounded-lg" : "",
        animate ? "blur-focus" : "",
        $$restProps.class,
    ]
        .filter(Boolean)
        .join(" ");
</script>

<div class={classes} {...$$restProps}>
    {#if title}
        <div class="mb-4">
            <h3 class="text-lg font-medium m-0">
                {title}
            </h3>

            {#if subtitle}
                <p class="text-sm text-gray-500 mt-1 mb-0">
                    {subtitle}
                </p>
            {/if}
        </div>
    {/if}

    <slot></slot>
</div>

<style>
    /* Any custom styles not covered by Tailwind can go here */
</style>
