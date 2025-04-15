import { writable } from "svelte/store";
import { browser } from "$app/environment";

// Define the settings interface
export interface Settings {
  theme: "light" | "dark" | "system";
  fontSize: "small" | "medium" | "large";
  storeConversations: boolean;
  storePreferences: boolean;
  allowAnonymizedData: boolean;
  llmProvider: "local" | "openai" | "anthropic";
  modelName: string;
  startOnBoot: boolean;
  minimizeToTray: boolean;
}

// Default settings
const defaultSettings: Settings = {
  theme: "light",
  fontSize: "medium",
  storeConversations: true,
  storePreferences: true,
  allowAnonymizedData: false,
  llmProvider: "local",
  modelName: "llama3.2:1b",
  startOnBoot: false,
  minimizeToTray: true,
};

// Create the settings store
const createSettingsStore = () => {
  // Load settings from localStorage if in browser
  const initialSettings: Settings = browser
    ? JSON.parse(
        localStorage.getItem("lynSettings") || JSON.stringify(defaultSettings),
      )
    : defaultSettings;

  const { subscribe, set, update } = writable(initialSettings);

  return {
    subscribe,

    // Update a single setting
    updateSetting: <K extends keyof Settings>(
      key: K,
      value: Settings[K],
    ): void => {
      update((settings) => {
        const newSettings = { ...settings, [key]: value };

        // Save to localStorage if in browser
        if (browser) {
          localStorage.setItem("lynSettings", JSON.stringify(newSettings));
        }

        return newSettings;
      });
    },

    // Reset settings to default
    resetSettings: (): void => {
      set(defaultSettings);

      // Save to localStorage if in browser
      if (browser) {
        localStorage.setItem("lynSettings", JSON.stringify(defaultSettings));
      }
    },

    // Import settings
    importSettings: (newSettings: Partial<Settings>): void => {
      // Merge with default settings to ensure all properties exist
      const mergedSettings: Settings = { ...defaultSettings, ...newSettings };
      set(mergedSettings);

      // Save to localStorage if in browser
      if (browser) {
        localStorage.setItem("lynSettings", JSON.stringify(mergedSettings));
      }
    },
  };
};

// Export the store
export const settingsStore = createSettingsStore();
