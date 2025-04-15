<script>
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let isOpen = false;

    function toggle() {
        isOpen = !isOpen;
        dispatch("toggle", isOpen);
    }
</script>

<button
    class="w-10 h-10 relative focus:outline-none bg-white hover:bg-gray-100 rounded-full p-2"
    on:click={toggle}
    aria-label={isOpen ? "Close menu" : "Open menu"}
>
    <div
        class="block w-6 absolute left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2"
    >
        <span
            class="block absolute h-0.5 w-6 bg-current transform transition duration-300 ease-in-out"
            class:rotate-45={isOpen}
            class:translate-y-1.5={!isOpen}
        ></span>
        <span
            class="block absolute h-0.5 w-6 bg-current transform transition duration-300 ease-in-out"
            class:opacity-0={isOpen}
        ></span>
        <span
            class="block absolute h-0.5 w-6 bg-current transform transition duration-300 ease-in-out"
            class:-rotate-45={isOpen}
            class:-translate-y-1.5={!isOpen}
        ></span>
    </div>
</button>
