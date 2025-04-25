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
 * @param {number} [maxReconnectAttempts=3] - Maximum number of reconnection attempts.
 * @returns {WebSocketStore} A readable store with WebSocket management methods.
 */
export function createWebSocketStore(
    url: string,
    maxReconnectAttempts: number = 3
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

                // Attempt to reconnect if we haven't exceeded the maximum attempts
                if (reconnectAttempts < maxReconnectAttempts) {
                    reconnectAttempts++;
                    reconnectTimeout = setTimeout(connect, 1000 * reconnectAttempts);
                } else {
                    state.update(s => ({
                        ...s,
                        error: `Failed to connect after ${maxReconnectAttempts} attempts`
                    }));
                }
            };

            ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                state.update(s => ({
                    ...s,
                    error: 'Connection error occurred'
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

// Create a singleton instance for the default WebSocket URL
// This can be imported and used throughout the application
const defaultWsUrl = isDevelopment ? 'ws://localhost:8080' : 'wss://api.lyn.ai';
const WS_BASE_URL = import.meta.env.VITE_WS_BASE_URL || defaultWsUrl;

// Create the WebSocket store
export const websocketStore = createWebSocketStore(`${WS_BASE_URL}/ws/chat`);

// Auto-connect in development mode for easier testing
if (isDevelopment) {
    console.log(`Development mode detected. WebSocket will auto-connect to ${WS_BASE_URL}/ws/chat`);
    // Auto-connect after a short delay to ensure the backend is ready
    setTimeout(() => {
        websocketStore.connect();
        console.log('Auto-connecting to WebSocket...');
    }, 1000);
}

// Export a function to create additional WebSocket connections if needed
export function createChatWebSocketStore(chatId: string): WebSocketStore {
    return createWebSocketStore(`${WS_BASE_URL}/ws/chat/${chatId}`);
}
