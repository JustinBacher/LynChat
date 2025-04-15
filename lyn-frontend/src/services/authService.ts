import { browser } from "$app/environment";
import { writable, type Writable } from "svelte/store";

// Get API URL from environment variable
const API_URL: string = browser ? import.meta.env.PUBLIC_API_URL : "";

// User store
export const user: Writable<any | null> = writable(null);

/**
 * Authentication service for interacting with the backend
 */
export class AuthService {
  constructor() {
    // Initialize auth state from localStorage if available
    if (browser) {
      const storedUser: string | null = localStorage.getItem("lynUser");
      if (storedUser) {
        try {
          user.set(JSON.parse(storedUser));
        } catch (error) {
          console.error("Failed to parse stored user:", error);
          localStorage.removeItem("lynUser");
        }
      }
    }
  }

  /**
   * Check if user is authenticated
   * @returns boolean - True if authenticated
   */
  isAuthenticated(): boolean {
    // In Tauri, we assume the user is always authenticated
    if (browser && window.__TAURI__) {
      return true;
    }

    // Get user from store
    let authenticated: boolean = false;
    user.subscribe((value) => {
      authenticated = !!value;
    })();

    return authenticated;
  }

  /**
   * Login with email and password
   * @param email - User email
   * @param password - User password
   * @returns Promise - Resolves with user data
   */
  async login(email: string, password: string): Promise<any> {
    if (!browser) return; // Server-side rendering guard

    try {
      // In Tauri environment, we don't need authentication
      if (window.__TAURI__) {
        const mockUser = {
          id: "1",
          email: email,
          name: "Local User",
          isLocal: true,
        };
        user.set(mockUser);
        return mockUser;
      }

      // In web environment, use fetch
      const response: Response = await fetch(`${API_URL}/api/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
        credentials: "include", // Include cookies for session
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      const userData: any = await response.json();
      // Update user store
      user.set(userData);
      // Store in localStorage
      localStorage.setItem("lynUser", JSON.stringify(userData));

      return userData;
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  }

  /**
   * Register a new user
   * @param email - User email
   * @param password - User password
   * @param name - User name
   * @returns Promise - Resolves with user data
   */
  async register(email: string, password: string, name: string): Promise<any> {
    if (!browser) return; // Server-side rendering guard

    try {
      // In Tauri environment, we don't need registration
      if (window.__TAURI__) {
        const mockUser = {
          id: "1",
          email: email,
          name: name,
          isLocal: true,
        };
        user.set(mockUser);
        return mockUser;
      }

      // In web environment, use fetch
      const response: Response = await fetch(`${API_URL}/api/auth/register`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password, name }),
        credentials: "include", // Include cookies for session
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      const userData: any = await response.json();
      // Update user store
      user.set(userData);
      // Store in localStorage
      localStorage.setItem("lynUser", JSON.stringify(userData));

      return userData;
    } catch (error) {
      console.error("Registration failed:", error);
      throw error;
    }
  }

  /**
   * Logout the current user
   * @returns Promise - Resolves when logged out
   */
  async logout(): Promise<void> {
    if (!browser) return; // Server-side rendering guard

    try {
      // In Tauri environment, just clear local state
      if (window.__TAURI__) {
        user.set(null);
        return;
      }

      // In web environment, use fetch
      const response: Response = await fetch(`${API_URL}/api/auth/logout`, {
        method: "POST",
        credentials: "include", // Include cookies for session
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      // Update user store
      user.set(null);
      // Remove from localStorage
      localStorage.removeItem("lynUser");
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }

  /**
   * Get the current user's data
   * @returns Promise - Resolves with user data
   */
  async getCurrentUser(): Promise<any | null> {
    if (!browser) return null; // Server-side rendering guard

    try {
      // In Tauri environment, return from store
      if (window.__TAURI__) {
        let currentUser: any = null;
        user.subscribe((value) => {
          currentUser = value;
        })();
        return (
          currentUser || {
            id: "1",
            name: "Local User",
            isLocal: true,
          }
        );
      }

      // In web environment, use fetch
      const response: Response = await fetch(`${API_URL}/api/auth/me`, {
        credentials: "include", // Include cookies for session
      });
      if (!response.ok) {
        if (response.status === 401) {
          // Not authenticated
          user.set(null);
          localStorage.removeItem("lynUser");
          return null;
        }

        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      const userData: any = await response.json();
      // Update user store
      user.set(userData);
      // Store in localStorage
      localStorage.setItem("lynUser", JSON.stringify(userData));

      return userData;
    } catch (error) {
      console.error("Failed to get current user:", error);
      throw error;
    }
  }
}

// Create and export a singleton instance
export const authService: AuthService = new AuthService();
