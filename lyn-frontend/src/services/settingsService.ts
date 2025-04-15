import { browser } from "$app/environment";
import isTauri from "$lib/tauri-helpers";

// Get API URL from environment variable
const API_URL = browser ? import.meta.env.PUBLIC_API_URL : "";

/**
 * Settings service for interacting with the backend
 */
export class SettingsService {
  /**
   * Fetch user settings from the backend
   * @returns {Promise<any>} - Resolves with the settings
   */
  async fetchSettings(): Promise<any> {
    if (!browser) return {};

    try {
      // In Tauri environment, use commands
      if (isTauri) {
        const { invoke } = window.__TAURI__;
        return invoke("get_settings");
      }

      // In web environment, use fetch
      const response = await fetch(`${API_URL}/api/settings`);
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      console.error("Failed to fetch settings:", error);
      throw error;
    }
  }

  /**
   * Update user settings
   * @param {any} settings - The settings to update
   * @returns {Promise<void>} - Resolves when settings are updated
   */
  async updateSettings(settings: any): Promise<void> {
    if (!browser) return;

    try {
      // In Tauri environment, use commands
      if (window.__TAURI__) {
        const { invoke } = window.__TAURI__;
        return invoke("update_settings", { settings });
      }

      // In web environment, use fetch
      const response = await fetch(`${API_URL}/api/settings`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(settings),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to update settings:", error);
      throw error;
    }
  }

  /**
   * Reset settings to default
   * @returns {Promise<void>} - Resolves when settings are reset
   */
  async resetSettings(): Promise<void> {
    if (!browser) return;

    try {
      // In Tauri environment, use commands
      if (window.__TAURI__) {
        const { invoke } = window.__TAURI__;
        return invoke("reset_settings");
      }

      // In web environment, use fetch
      const response = await fetch(`${API_URL}/api/settings/reset`, {
        method: "POST",
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to reset settings:", error);
      throw error;
    }
  }

  /**
   * Export settings
   * @returns {Promise<any>} - Resolves with the settings data
   */
  async exportSettings(): Promise<any> {
    if (!browser) return {};

    try {
      // In Tauri environment, use commands
      if (window.__TAURI__) {
        const { invoke, dialog } = window.__TAURI__;
        const settings = await invoke("get_settings");

        // Prompt user for save location
        const filePath = await dialog.save({
          filters: [
            {
              name: "JSON",
              extensions: ["json"],
            },
          ],
        });
        if (filePath) {
          // Write to file
          const { fs } = window.__TAURI__;
          await fs.writeTextFile(filePath, JSON.stringify(settings, null, 2));
        }

        return settings;
      }

      // In web environment, use fetch and download
      const settings = await this.fetchSettings();
      const blob = new Blob([JSON.stringify(settings, null, 2)], {
        type: "application/json",
      });
      const url = URL.createObjectURL(blob);

      const a = document.createElement("a");
      a.href = url;
      a.download = "lyn-settings.json";
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      return settings;
    } catch (error) {
      console.error("Failed to export settings:", error);
      throw error;
    }
  }

  /**
   * Import settings
   * @param {any | string | null} settings - Settings object or file path
   * @returns {Promise<void>} - Resolves when settings are imported
   */
  async importSettings(settings: any | string | null): Promise<void> {
    if (!browser) return;

    try {
      // In Tauri environment
      if (window.__TAURI__) {
        const { invoke, dialog, fs } = window.__TAURI__;
        // If settings is a string, assume it's a file path
        if (typeof settings === "string") {
          const fileContent = await fs.readTextFile(settings);
          settings = JSON.parse(fileContent);
        } else if (!settings) {
          // Prompt user for file
          const filePath = await dialog.open({
            filters: [
              {
                name: "JSON",
                extensions: ["json"],
              },
            ],
          });

          if (filePath) {
            if (typeof filePath === "string") {
              try {
                const fileContent = await fs.readTextFile(filePath);
                settings = JSON.parse(fileContent);
              } catch (error) {
                console.error("Error reading file:", error);
              }
            }
          } else {
            return; // User cancelled
          }
        }

        // Update settings
        return invoke("update_settings", { settings });
      }

      // In web environment
      if (!settings) {
        // Create file input
        const input = document.createElement("input");
        input.type = "file";
        input.accept = ".json";

        // Wait for file selection
        const file = await new Promise<File | null>((resolve) => {
          input.onchange = (e) => {
            const target = e.target as HTMLInputElement;
            try {
              resolve((target.files as FileList)[0]);
            } catch (error) {
              console.error("Error reading file:", error);
            }
          };
          input.click();
        });
        if (!file) return; // User cancelled

        // Read the file
        const fileContent = await file.text();
        settings = JSON.parse(fileContent);
      }

      // Update settings
      await this.updateSettings(settings);
    } catch (error) {
      console.error("Failed to import settings:", error);
      throw error;
    }
  }
}

// Create and export a singleton instance
export const settingsService = new SettingsService();
