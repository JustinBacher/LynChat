import { browser } from "$app/environment"; // For SvelteKit

declare global {
  interface Window {
    __TAURI__: {
      invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
      event: {
        listen<T>(
          event: string,
          handler: (event: { payload: T }) => void,
        ): Promise<() => void>;
        once<T>(
          event: string,
          handler: (event: { payload: T }) => void,
        ): Promise<() => void>;
        unlisten(event: string, handler?: (event: any) => void): Promise<void>;
      };
      dialog: {
        // Added dialog
        open(options?: OpenDialogOptions): Promise<string | string[] | null>;
        save(options?: SaveDialogOptions): Promise<string | null>;
        message(
          message: string,
          options?: MessageDialogOptions,
        ): Promise<boolean>;
        ask(message: string, options?: MessageDialogOptions): Promise<boolean>;
        confirm(
          message: string,
          options?: MessageDialogOptions,
        ): Promise<boolean>;
      };
      fs: {
        // Added fs
        readFile(
          filePath: string,
          options?: { encoding?: string },
        ): Promise<Uint8Array>;
        writeFile(
          filePath: string,
          contents: string | Uint8Array,
          options?: { encoding?: string },
        ): Promise<void>;
        readTextFile(
          filePath: string,
          options?: { encoding?: string },
        ): Promise<string>;
        writeTextFile(
          filePath: string,
          contents: string,
          options?: { encoding?: string },
        ): Promise<void>;
        removeFile(filePath: string): Promise<void>;
        createDir(
          dirPath: string,
          options?: { recursive?: boolean },
        ): Promise<void>;
        readDir(
          dirPath: string,
          options?: { recursive?: boolean },
        ): Promise<Dirent[]>;
        removeDir(
          dirPath: string,
          options?: { recursive?: boolean },
        ): Promise<void>;
        copyFile(source: string, destination: string): Promise<void>;
        renameFile(oldPath: string, newPath: string): Promise<void>;
      };
    };
  }
  interface OpenDialogOptions {
    filters?: { name: string; extensions: string[] }[];
    directory?: boolean;
    multiple?: boolean;
    title?: string;
    defaultPath?: string;
  }

  interface SaveDialogOptions {
    filters?: { name: string; extensions: string[] }[];
    title?: string;
    defaultPath?: string;
  }

  interface MessageDialogOptions {
    title?: string;
    type?: "info" | "warning" | "error";
  }

  interface Dirent {
    path: string;
    // The name of the entry.
    name: string;
    // Is the entry a directory?
    is_dir: boolean;
    // The children of this entry if it is a directory. `null` otherwise.
    children: Dirent[] | null;
  }
}

const isTauriCheck = () => {
  return browser && import.meta.env.MODE === "tauri" && "__TAURI__" in window;
};

const isTauri: boolean = isTauriCheck();

export default isTauri;

export const isWeb: boolean = browser && !isTauri;
export const isMobile: boolean = browser && navigator.maxTouchPoints > 0;
export const isDesktop: boolean = browser && !isMobile;

export const isTauriMobile: boolean = isTauri && isMobile;
export const isTauriDesktop: boolean = isTauri && isDesktop;

export const isWebMobile: boolean = isWeb && isMobile;
export const isWebDesktop: boolean = isWeb && isDesktop;

// Enhanced Tauri check
export const hasTauriApi = (api: string): boolean => {
  return isTauri && window.__TAURI__ && api in window.__TAURI__;
};
