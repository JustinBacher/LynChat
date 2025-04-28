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

// Creates a Svelte store that manages a WebSocket connection.
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

    // Helper function to parse message data
    const parseMessageData = (data: any): any => {
        if (typeof data === 'string') {
            try {
                return JSON.parse(data);
            } catch (e) {
                // Not JSON, keep as string
                return data;
            }
        }
        return data;
    };

    // Helper function to handle reconnection logic
    const handleReconnection = (wasClean: boolean) => {
        // Reset reconnect attempts if the connection was previously established successfully
        if (reconnectAttempts > 0 && wasClean) {
            console.log('Connection was cleanly closed, resetting reconnect attempts');
            reconnectAttempts = 0;
            return;
        }

        // Attempt to reconnect if we haven't exceeded the maximum attempts
        if (reconnectAttempts < maxReconnectAttempts) {
            reconnectAttempts++;
            // Use a longer base delay (5 seconds) with exponential backoff
            const delay = reconnectAttempts === 1 ? 5000 : 5000 * Math.min(reconnectAttempts, 3);
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

    // Handler for WebSocket open event
    const handleOpen = () => {
        state.update(s => ({ ...s, connected: true, error: null }));
        reconnectAttempts = 0;
    };

    // Handler for WebSocket message event
    const handleMessage = (event: MessageEvent) => {
        try {
            // Try to parse JSON if the message is a string
            const messageData = parseMessageData(event.data);

            // Check if the message contains an error
            if (messageData && typeof messageData === 'object' && messageData.error) {
                console.error('Error from server:', messageData.error);
                state.update(s => ({
                    ...s,
                    error: messageData.error
                }));
            } else {
                // Reset any previous errors when we get a successful message
                state.update(s => ({
                    ...s,
                    error: null
                }));

                // Reset reconnect attempts on successful message
                reconnectAttempts = 0;
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
            console.error('Raw message data:', event.data);
        }
    };

    // Handler for WebSocket close event
    const handleClose = (event: CloseEvent) => {
        state.update(s => ({ ...s, connected: false }));
        console.log(`WebSocket connection closed with code: ${event.code}, reason: ${event.reason || 'No reason provided'}`);

        handleReconnection(event.wasClean);
    };

    // Handler for WebSocket error event
    const handleError = (error: Event) => {
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

    // Connect to WebSocket
    const connect = () => {
        // Clean up any existing connection first
        disconnect();

        try {
            ws = new WebSocket(url);

            // Set up event handlers
            ws.onopen = handleOpen;
            ws.onmessage = handleMessage;
            ws.onclose = handleClose;
            ws.onerror = handleError;
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

    // Helper function to prepare payload
    const preparePayload = (data: any): string => {
        return typeof data === 'string' ? data : JSON.stringify(data);
    };

    // Helper function to send payload when connection is ready
    const sendPayloadWhenReady = (data: any, delay: number, afterAction: string): boolean => {
        setTimeout(() => {
            if (ws && ws.readyState === WebSocket.OPEN) {
                const payload = preparePayload(data);
                ws.send(payload);

                if (isDevelopment) {
                    console.log(`WebSocket message sent after ${afterAction}:`, data);
                }
            } else {
                if (isDevelopment) {
                    console.warn(`Failed to send WebSocket message after ${afterAction} - connection still not open`);
                }
            }
        }, delay);

        return true; // Return true to indicate we're handling it
    };

    // Send data through WebSocket
    const send = (data: any): boolean => {
        // Case 1: Connection is open - send immediately
        if (ws && ws.readyState === WebSocket.OPEN) {
            const payload = preparePayload(data);
            ws.send(payload);

            if (isDevelopment) {
                console.log('WebSocket message sent:', data);
            }
            return true;
        }

        // Case 2: Connection is still being established
        else if (ws && ws.readyState === WebSocket.CONNECTING) {
            if (isDevelopment) {
                console.warn('WebSocket is still connecting, will retry sending message shortly');
            }
            return sendPayloadWhenReady(data, 1000, 'connection established');
        }

        // Log warning for closed connection
        if (isDevelopment) {
            console.warn('Failed to send WebSocket message - connection not open');
        }

        // Case 3: Connection is closed or closing - reconnect first
        if (!ws || ws.readyState === WebSocket.CLOSED || ws.readyState === WebSocket.CLOSING) {
            if (isDevelopment) {
                console.log('Connection is closed or closing, attempting to reconnect...');
            }

            // Reconnect and then try to send again after a delay
            connect();
            return sendPayloadWhenReady(data, 2000, 'reconnection');
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

// Create a dummy WebSocket store for non-browser environments
const createDummyWebSocketStore = (): WebSocketStore => ({
    subscribe: () => { return () => {}; },
    connect: () => {},
    disconnect: () => {},
    send: () => false,
    isConnected: () => false
});

// Create the WebSocket store
export const websocketStore = isBrowser
    ? createWebSocketStore(`${WS_BASE_URL}/ws/chat`)
    : createDummyWebSocketStore();

// Setup development mode features
if (isBrowser && isDevelopment) {
    console.log(`Development mode detected. WebSocket will auto-connect to ${WS_BASE_URL}/ws/chat`);

    // Setup error handlers in development mode
    const setupDevErrorHandlers = () => {
        console.log(`Using hostname: ${window.location.hostname}`);
        console.log(`Full window.location: ${window.location.href}`);

        // Add a global error handler for unhandled promise rejections
        window.addEventListener('unhandledrejection', (event) => {
            console.error('Unhandled promise rejection:', event.reason);
        });

        // Add a global error handler
        window.onerror = function(message, source, lineno, colno, error) {
            console.error('Global error:', message, source, lineno, colno, error);
            return false;
        };
    };

    // Safe access to window properties
    if (typeof window !== 'undefined') {
        setupDevErrorHandlers();
    }

    // Auto-connect after a longer delay to ensure the backend is ready
    // This helps prevent rapid reconnection attempts
    setTimeout(() => {
        websocketStore.connect();
        console.log('Auto-connecting to WebSocket...');
    }, 5000);
}

// Export a function to create additional WebSocket connections if needed
export function createChatWebSocketStore(chatId: string): WebSocketStore {
    if (!isBrowser) {
        return createDummyWebSocketStore();
    }
    return createWebSocketStore(`${WS_BASE_URL}/ws/chat/${chatId}`);
}
