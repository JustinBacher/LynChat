import { browser } from "$app/environment";

// Get API URL from environment variable
const API_URL: string = browser ? import.meta.env.PUBLIC_API_URL : "";
const WS_URL: string = browser ? import.meta.env.PUBLIC_WS_URL : "";

/**
 * Chat service for interacting with the backend
 */
export class ChatService {
  private ws: WebSocket | null = null;
  private messageCallbacks: Map<string, (data: any) => void> = new Map();
  public isConnected: boolean = false;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 5;

  constructor() {}

  /**
   * Connect to the WebSocket server
   * @param onMessageCallback - Callback for receiving messages
   * @param onStatusCallback - Callback for connection status updates
   * @returns Promise - Resolves when connected
   */
  async connect(
    onMessageCallback?: (data: any) => void,
    onStatusCallback?: (status: string) => void,
  ): Promise<void> {
    if (!browser) return; // Server-side rendering guard

    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(`${WS_URL}/chat`);

        this.ws.onopen = () => {
          this.isConnected = true;
          this.reconnectAttempts = 0;
          onStatusCallback?.("connected");
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            onMessageCallback?.(data);

            // Check if this is a response to a specific message
            if (data.requestId && this.messageCallbacks.has(data.requestId)) {
              const callback = this.messageCallbacks.get(data.requestId);
              callback(data);

              // If this is a final response, remove the callback
              if (data.isComplete) {
                this.messageCallbacks.delete(data.requestId);
              }
            }
          } catch (error) {
            console.error("Error parsing WebSocket message:", error);
          }
        };

        this.ws.onclose = () => {
          this.isConnected = false;
          onStatusCallback?.("disconnected");

          // Try to reconnect
          if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            setTimeout(() => {
              this.connect(onMessageCallback, onStatusCallback).catch(
                console.error,
              );
            }, 1000 * this.reconnectAttempts); // Exponential backoff
          }
        };

        this.ws.onerror = (error) => {
          console.error("WebSocket error:", error);
          onStatusCallback?.("error");
          reject(error);
        };
      } catch (error) {
        console.error("Failed to connect to WebSocket:", error);
        reject(error);
      }
    });
  }

  /**
   * Disconnect from the WebSocket server
   */
  disconnect(): void {
    if (this.ws && this.isConnected) {
      this.ws.close();
      this.isConnected = false;
    }
  }

  /**
   * Send a message to the backend
   * @param message - The message to send
   * @param onChunkCallback - Callback for receiving streaming chunks
   * @returns Promise - Resolves with the complete response
   */
  async sendMessage(
    message: string,
    onChunkCallback?: (chunk: string) => void,
  ): Promise<any> {
    // Check if we're in a Tauri environment
    const isTauri: boolean = browser && window.__TAURI__ !== undefined;
    if (isTauri) {
      // Use Tauri command in desktop environment
      return this.sendMessageTauri(message, onChunkCallback);
    } else if (browser) {
      // Use WebSocket in browser environment
      return this.sendMessageWebSocket(message, onChunkCallback);
    }

    throw new Error("Cannot send message: not in browser or Tauri environment");
  }

  /**
   * Send a message via WebSocket
   * @private
   */
  private async sendMessageWebSocket(
    message: string,
    onChunkCallback?: (chunk: string) => void,
  ): Promise<any> {
    if (!this.isConnected) {
      throw new Error("WebSocket not connected");
    }

    // Generate a unique ID for this request
    const requestId: string = `msg_${Date.now()}_${Math.random()
      .toString(36)
      .substr(2, 9)}`;
    return new Promise((resolve, reject) => {
      // Set up a timeout for the response
      const timeout = setTimeout(() => {
        this.messageCallbacks.delete(requestId);
        reject(new Error("Request timed out"));
      }, 30000); // 30 seconds timeout

      // Set up callback for the response
      this.messageCallbacks.set(requestId, (data: any) => {
        if (data.type === "chunk") {
          // Call the chunk callback for streaming
          onChunkCallback?.(data.content);
        } else if (data.type === "complete") {
          // Clear the timeout and resolve the promise
          clearTimeout(timeout);
          resolve(data);
        } else if (data.type === "error") {
          // Clear the timeout and reject the promise
          clearTimeout(timeout);
          reject(new Error(data.error));
        }
      });

      // Send the message
      this.ws!.send(
        JSON.stringify({
          type: "message",
          requestId,
          content: message,
        }),
      );
    });
  }

  /**
   * Send a message via Tauri command
   * @private
   */
  private async sendMessageTauri(
    message: string,
    onChunkCallback?: (chunk: string) => void,
  ): Promise<any> {
    const { invoke, event } = window.__TAURI__;
    try {
      // Register for chat events
      const unlisten = await event.listen("chat:chunk", (event: any) => {
        onChunkCallback?.(event.payload.content);
      });
      // Send the message
      const messageId: string = await invoke("send_message", { message });
      // Wait for the completion event
      return new Promise((resolve, reject) => {
        event.once(`chat:complete:${messageId}`, (event: any) => {
          unlisten(); // Stop listening for chunks
          resolve(event.payload);
        });

        event.once(`chat:error:${messageId}`, (event: any) => {
          unlisten(); // Stop listening for chunks
          reject(new Error(event.payload.error));
        });
      });
    } catch (error) {
      console.error("Tauri command error:", error);
      throw error;
    }
  }

  /**
   * Fetch chat history from the backend
   * @param limit - Maximum number of messages to fetch
   * @param offset - Number of messages to skip
   * @returns Promise - Resolves with the history
   */
  async fetchHistory(limit: number = 50, offset: number = 0): Promise<any[]> {
    if (!browser) return []; // Server-side rendering guard

    try {
      // In Tauri environment, use commands
      if (window.__TAURI__) {
        const { invoke } = window.__TAURI__;
        return invoke("get_chat_history", { limit, offset });
      }

      // In web environment, use fetch
      const response: Response = await fetch(
        `${API_URL}/api/chat/history?limit=${limit}&offset=${offset}`,
      );
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      console.error("Failed to fetch chat history:", error);
      throw error;
    }
  }

  /**
   * Clear chat history
   * @returns Promise - Resolves when history is cleared
   */
  async clearHistory(): Promise<void> {
    if (!browser) return; // Server-side rendering guard

    try {
      // In Tauri environment, use commands
      if (window.__TAURI__) {
        const { invoke } = window.__TAURI__;
        return invoke("clear_chat_history");
      }

      // In web environment, use fetch
      const response: Response = await fetch(`${API_URL}/api/chat/history`, {
        method: "DELETE",
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to clear chat history:", error);
      throw error;
    }
  }
}

// Create and export a singleton instance
export const chatService: ChatService = new ChatService();
