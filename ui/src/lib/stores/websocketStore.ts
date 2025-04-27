import { writable, type Readable, type Writable } from 'svelte/store';

// Define the WebSocket store interface
export interface WebSocketStore extends Readable<WebSocketState> {
    connect: () => void;
    disconnect: () => void;
    send: (data: any) => void;
    isConnected: () => boolean;
}

// Define the WebSocket state interface
export interface WebSocketState {
    connected: boolean;
    error: string | null;
}

/**
 * Creates a Svelte store that manages a WebSocket connection.
 * @param {string} url - The WebSocket URL to connect to.
 * @param {number} [maxReconnectAttempts=5] - Maximum number of reconnection attempts.
 * @returns {WebSocketStore} A readable store with WebSocket management methods.
 */
export function createWebSocketStore(
    url: string,
    maxReconnectAttempts: number = 5
): WebSocketStore {
    // Create the internal state store
    const state: Writable<WebSocketState> = writable({
        connected: false,
        error: null
    });

    // WebSocket instance
    let ws: WebSocket | null = null;
    let reconnectAttempts = 0;
    let reconnectTimeout: ReturnType<typeof setTimeout> | null = null;

    // Connect to WebSocket
    const connect = () => {
        // Clean up any existing connection first
        disconnect();

        try {
            ws = new WebSocket(url);

            ws.onopen = () => {
                state.update(s => ({ ...s, connected: true, error: null }));
                reconnectAttempts = 0;
            };

            ws.onmessage = (event) => {
                try {
                    // Try to parse JSON if the message is a string
                    let messageData = event.data;
                    if (typeof messageData === 'string') {
                        try {
                            messageData = JSON.parse(messageData);
                        } catch (e) {
                            // Not JSON, keep as string
                        }
                    }

                    // The store itself doesn't process messages
                    // Subscribers will handle this via the custom event
                    const customEvent = new CustomEvent('ws-message', {
                        detail: messageData
                    });
                    window.dispatchEvent(customEvent);

                    // Log in development mode
                    if (isDevelopment) {
                        console.log('WebSocket message received:', messageData);
                    }
                } catch (error) {
                    console.error('Error handling WebSocket message:', error);
                }
            };

            ws.onclose = () => {
                state.update(s => ({ ...s, connected: false }));
                console.log('WebSocket connection closed');

                // Attempt to reconnect if we haven't exceeded the maximum attempts
                // Use a longer delay to prevent rapid reconnection attempts
                if (reconnectAttempts < maxReconnectAttempts) {
                    reconnectAttempts++;
                    // Use a longer base delay (5 seconds) with exponential backoff
                    const delay = 5000 * reconnectAttempts;
                    console.log(`Will attempt to reconnect in ${delay/1000} seconds (attempt ${reconnectAttempts}/${maxReconnectAttempts})`);
                    reconnectTimeout = setTimeout(connect, delay);
                } else {
                    console.log(`Failed to connect after ${maxReconnectAttempts} attempts`);
                    state.update(s => ({
                        ...s,
                        error: `Failed to connect after ${maxReconnectAttempts} attempts`
                    }));
                }
            };

            ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                console.error('WebSocket URL:', url);
                console.error('WebSocket readyState:', ws?.readyState);

                // Try to provide more detailed error information
                let errorMessage = 'Connection error occurred';
                if (error && (error as any).message) {
                    errorMessage += `: ${(error as any).message}`;
                }

                state.update(s => ({
                    ...s,
                    error: errorMessage
                }));
            };
        } catch (error) {
            console.error('Error creating WebSocket:', error);
            state.update(s => ({
                ...s,
                error: `Failed to create WebSocket: ${error}`
            }));
        }
    };

    // Disconnect from WebSocket
    const disconnect = () => {
        if (reconnectTimeout) {
            clearTimeout(reconnectTimeout);
            reconnectTimeout = null;
        }

        if (ws) {
            // Only attempt to close if the socket exists and isn't already closed
            if (ws.readyState === WebSocket.OPEN || ws.readyState === WebSocket.CONNECTING) {
                ws.close();
            }
            ws = null;
        }

        state.update(s => ({ ...s, connected: false }));
    };

    // Send data through WebSocket
    const send = (data: any) => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            const payload = typeof data === 'string' ? data : JSON.stringify(data);
            ws.send(payload);

            // Log in development mode
            if (isDevelopment) {
                console.log('WebSocket message sent:', data);
            }
            return true;
        }

        if (isDevelopment) {
            console.warn('Failed to send WebSocket message - connection not open');
        }
        return false;
    };

    // Check if WebSocket is connected
    const isConnected = () => {
        return ws !== null && ws.readyState === WebSocket.OPEN;
    };

    // Return the store with additional methods
    return {
        subscribe: state.subscribe,
        connect,
        disconnect,
        send,
        isConnected
    };
}

// Development mode flag
export const isDevelopment = import.meta.env.DEV;

// Check if we're in a browser environment before accessing window
const isBrowser = typeof window !== 'undefined' && typeof WebSocket !== 'undefined';

// Create a singleton instance for the default WebSocket URL
// This can be imported and used throughout the application
// When running in Docker, we need to use the window.location.hostname to ensure proper connection
const getWebSocketUrl = () => {
    if (isDevelopment) {
        // In development, use the current hostname with the correct port
        // Only access window if in browser environment
        if (isBrowser) {
            const hostname = window.location.hostname;
            return `ws://${hostname}:8083`;
        }
        // Default for SSR in development
        return 'ws://localhost:8083';
    }
    return 'wss://api.lyn.ai'; // Production URL
};

const WS_BASE_URL = getWebSocketUrl();

// Create the WebSocket store
export const websocketStore = isBrowser
    ? createWebSocketStore(`${WS_BASE_URL}/ws/chat`)
    : {
        subscribe: () => { return () => {}; },
        connect: () => {},
        disconnect: () => {},
        send: () => false,
        isConnected: () => false
      } as WebSocketStore;

// Auto-connect in development mode for easier testing
if (isBrowser && isDevelopment) {
    console.log(`Development mode detected. WebSocket will auto-connect to ${WS_BASE_URL}/ws/chat`);

    // Safe access to window properties
    if (typeof window !== 'undefined') {
        console.log(`Using hostname: ${window.location.hostname}`);
        console.log(`Full window.location: ${window.location.href}`);
    }

    // Auto-connect after a longer delay to ensure the backend is ready
    // This helps prevent rapid reconnection attempts
    setTimeout(() => {
        websocketStore.connect();
        console.log('Auto-connecting to WebSocket...');
    }, 3000);
}

// Export a function to create additional WebSocket connections if needed
export function createChatWebSocketStore(chatId: string): WebSocketStore {
    if (!isBrowser) {
        return {
            subscribe: () => { return () => {}; },
            connect: () => {},
            disconnect: () => {},
            send: () => false,
            isConnected: () => false
        } as WebSocketStore;
    }
    return createWebSocketStore(`${WS_BASE_URL}/ws/chat/${chatId}`);
}
