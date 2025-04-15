<script>
    import { fade, fly } from "svelte/transition";
    import { quintOut } from "svelte/easing";
    import { onMount } from "svelte";

    export let isOpen = false;

    // Detect if we're running in Tauri (desktop) vs web
    let isTauri = false;

    onMount(() => {
        // Check if we're in a Tauri environment
        isTauri = window.__TAURI__ !== undefined;
    });

    // Handle click outside to close menu
    function handleClickOutside(event) {
        if (isOpen && !event.target.closest(".settings-menu")) {
            isOpen = false;
        }
    }

    // Mock settings sections and options
    const sections = [
        {
            title: "Appearance",
            icon: "🎨",
            options: [
                {
                    id: "theme",
                    label: "Theme",
                    type: "select",
                    value: "light",
                    choices: ["light", "dark", "system"],
                },
                {
                    id: "fontSize",
                    label: "Font Size",
                    type: "select",
                    value: "medium",
                    choices: ["small", "medium", "large"],
                },
            ],
        },
        {
            title: "Privacy",
            icon: "🔒",
            options: [
                {
                    id: "storeConversations",
                    label: "Store Conversations",
                    type: "toggle",
                    value: true,
                },
                {
                    id: "storePreferences",
                    label: "Store Preferences",
                    type: "toggle",
                    value: true,
                },
                {
                    id: "allowAnonymizedData",
                    label: "Allow Anonymized Data",
                    type: "toggle",
                    value: false,
                },
            ],
        },
        {
            title: "LLM Settings",
            icon: "🤖",
            options: [
                {
                    id: "llmProvider",
                    label: "LLM Provider",
                    type: "select",
                    value: "local",
                    choices: ["local", "openai", "anthropic"],
                },
                {
                    id: "modelName",
                    label: "Model Name",
                    type: "text",
                    value: "llama3.2:1b",
                },
            ],
        },
    ];

    // Add desktop-specific settings if in Tauri
    $: allSections = isTauri
        ? [
              ...sections,
              {
                  title: "Desktop Settings",
                  icon: "💻",
                  options: [
                      {
                          id: "startOnBoot",
                          label: "Start on Boot",
                          type: "toggle",
                          value: false,
                      },
                      {
                          id: "minimizeToTray",
                          label: "Minimize to Tray",
                          type: "toggle",
                          value: true,
                      },
                  ],
              },
          ]
        : sections;

    // Handle setting change
    function updateSetting(sectionIndex, optionIndex, newValue) {
        allSections[sectionIndex].options[optionIndex].value = newValue;
        // In a real application, this would be persisted to a store or backend
    }
</script>

<svelte:window on:click={handleClickOutside} />

{#if isOpen}
    <div
        class="settings-menu fixed right-0 top-0 h-full w-full sm:w-96 max-w-full bg-white shadow-lg z-40 overflow-y-auto"
        in:fly={{ x: 300, duration: 200, easing: quintOut }}
        out:fly={{ x: 300, duration: 200 }}
    >
        <!-- Header -->
        <div
            class="sticky top-0 bg-white z-10 border-b px-4 py-3 flex justify-between items-center"
        >
            <h2 class="text-xl font-medium m-0">Settings</h2>
            <button
                class="p-2 rounded-full hover:bg-gray-100"
                on:click={() => (isOpen = false)}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        </div>

        <!-- Settings Sections -->
        <div class="p-4">
            {#each allSections as section, sectionIndex}
                <div class="mb-6">
                    <div class="flex items-center mb-2">
                        <span class="mr-2 text-xl">{section.icon}</span>
                        <h3 class="text-lg font-medium m-0">{section.title}</h3>
                    </div>

                    <div class="ml-8">
                        {#each section.options as option, optionIndex}
                            <div class="mb-4">
                                <label class="block mb-2 text-sm font-medium"
                                    >{option.label}</label
                                >

                                {#if option.type === "toggle"}
                                    <div
                                        class="relative inline-block w-10 mr-2 align-middle select-none"
                                    >
                                        <input
                                            type="checkbox"
                                            id={option.id}
                                            class="sr-only peer"
                                            checked={option.value}
                                            on:change={(e) =>
                                                updateSetting(
                                                    sectionIndex,
                                                    optionIndex,
                                                    e.target.checked,
                                                )}
                                        />
                                        <label
                                            for={option.id}
                                            class="block h-6 rounded-full bg-gray-300 cursor-pointer peer-checked:bg-primary"
                                        ></label>
                                        <span
                                            class="absolute left-1 top-1 bg-white h-4 w-4 rounded-full transition peer-checked:translate-x-4"
                                        ></span>
                                    </div>
                                    <span class="text-sm"
                                        >{option.value
                                            ? "Enabled"
                                            : "Disabled"}</span
                                    >
                                {/if}

                                {#if option.type === "select"}
                                    <select
                                        class="input"
                                        value={option.value}
                                        on:change={(e) =>
                                            updateSetting(
                                                sectionIndex,
                                                optionIndex,
                                                e.target.value,
                                            )}
                                    >
                                        {#each option.choices as choice}
                                            <option value={choice}
                                                >{choice
                                                    .charAt(0)
                                                    .toUpperCase() +
                                                    choice.slice(1)}</option
                                            >
                                        {/each}
                                    </select>
                                {/if}

                                {#if option.type === "text"}
                                    <input
                                        type="text"
                                        class="input"
                                        value={option.value}
                                        on:input={(e) =>
                                            updateSetting(
                                                sectionIndex,
                                                optionIndex,
                                                e.target.value,
                                            )}
                                    />
                                {/if}
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}

            <!-- Version information -->
            <div class="text-xs text-gray-500 mt-8 pt-4 border-t">
                <p class="m-0">Lyn AI Assistant v0.1.0</p>
                <p class="m-0">
                    Running on {isTauri ? "Desktop (Tauri)" : "Web"}
                </p>
            </div>
        </div>
    </div>

    <!-- Backdrop for mobile -->
    <div
        class="fixed inset-0 bg-black bg-opacity-50 z-30 sm:hidden"
        in:fade={{ duration: 200 }}
        out:fade={{ duration: 200 }}
        on:click={() => (isOpen = false)}
    ></div>
{/if}
