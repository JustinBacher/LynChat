import { writable, type Readable, type Writable } from 'svelte/store';

// Create a store to hold the initial message
export const initialMessageStore = writable<string>('');

// Define the Chat store interface
export interface ChatStore extends Readable<ChatState> {
    sendMessage: (message: string, options?: ChatOptions) => Promise<string>;
    cancelRequest: () => void;
}

// Define the Chat state interface
export interface ChatState {
    isStreaming: boolean;
    error: string | null;
}

// Define chat options
export interface ChatOptions {
    model?: string;
    reasoningEnabled?: boolean;
}

// Define message event types
interface StreamStartEvent {
    event: string;
    data: {
        type_: string;
        model: string;
        request_id: string;
    };
}

interface StreamChunkEvent {
    event: string;
    data: {
        type_: string;
        content: string;
        is_first: boolean;
        request_id: string;
    };
}

interface StreamEndEvent {
    event: string;
    data: {
        type_: string;
        content: string;
        request_id: string;
    };
}

interface StreamErrorEvent {
    event: string;
    data: {
        type_: string;
        error: string;
        request_id: string;
    };
}

/**
 * Creates a Svelte store that manages chat interactions using HTTP streaming.
 * @returns {ChatStore} A readable store with chat management methods.
 */
export function createChatStore(): ChatStore {
    // Create the internal state store
    const state: Writable<ChatState> = writable({
        isStreaming: false,
        error: null
    });

    // Track the current request
    let currentController: AbortController | null = null;

    // Get the API base URL
    const getApiBaseUrl = () => {
        if (isDevelopment) {
            // In development, use the current hostname with the correct port
            if (isBrowser) {
                const hostname = window.location.hostname;
                return `http://${hostname}:8083`;
            }
            // Default for SSR in development
            return 'http://localhost:8083';
        }
        return 'https://api.lyn.ai'; // Production URL
    };

    const API_BASE_URL = getApiBaseUrl();

    // Send a message and get streaming response
    const sendMessage = async (message: string, options: ChatOptions = {}): Promise<string> => {
        console.log('chatStore.sendMessage called with message:', message);

        // Cancel any existing request
        cancelRequest();

        // Create a new abort controller
        currentController = new AbortController();
        const signal = currentController.signal;

        // Update state to indicate streaming
        state.update(s => ({ ...s, isStreaming: true, error: null }));

        // Create a promise that will resolve with the final response
        return new Promise((resolve, reject) => {
            // Create the request payload
            const payload = {
                message,
                model: options.model,
                reasoning_enabled: options.reasoningEnabled
            };

            const apiUrl = `${API_BASE_URL}/stream/chat`;
            console.log('API URL:', apiUrl);
            console.log('Sending message to streaming API:', payload);

            // Make the fetch request
            fetch(apiUrl, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'text/event-stream'
                },
                body: JSON.stringify(payload),
                signal
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                if (!response.body) {
                    throw new Error('Response body is null');
                }

                // Get a reader from the response body
                const reader = response.body.getReader();
                const decoder = new TextDecoder();
                let accumulatedContent = '';
                let buffer = '';
                let receivedAnyEvent = false;

                // Function to process the stream
                const processStream = async () => {
                    try {
                        while (true) {
                            const { done, value } = await reader.read();

                            if (done) {
                                // End of stream
                                break;
                            }

                            // Decode the chunk and add to buffer
                            const chunk = decoder.decode(value, { stream: true });
                            buffer += chunk;

                            // Process complete JSON objects in the buffer
                            let newlineIndex;
                            while ((newlineIndex = buffer.indexOf('\n')) >= 0) {
                                const line = buffer.slice(0, newlineIndex).trim();
                                buffer = buffer.slice(newlineIndex + 1);

                                if (line) {
                                    try {
                                        const event = JSON.parse(line);

                                        if (isDevelopment) {
                                            console.log('Received event:', event);
                                        }

                                        // Mark that we received an event
                                        receivedAnyEvent = true;

                                        // Process different event types
                                        if (event.event === 'start') {
                                            // Stream start event
                                            const startEvent = event as StreamStartEvent;
                                            if (isDevelopment) {
                                                console.log('Stream started with request ID:', startEvent.data.request_id);
                                            }

                                            // Dispatch a custom event for the start
                                            dispatchCustomEvent('chat-stream-start', startEvent.data);
                                        }
                                        else if (event.event === 'chunk') {
                                            // Stream chunk event
                                            const chunkEvent = event as StreamChunkEvent;
                                            accumulatedContent += chunkEvent.data.content;

                                            // Dispatch a custom event for the chunk
                                            dispatchCustomEvent('chat-stream-chunk', {
                                                ...chunkEvent.data,
                                                accumulated_content: accumulatedContent
                                            });
                                        }
                                        else if (event.event === 'end') {
                                            // Stream end event
                                            const endEvent = event as StreamEndEvent;

                                            // Dispatch a custom event for the end
                                            dispatchCustomEvent('chat-stream-end', endEvent.data);

                                            // Update state to indicate streaming is done
                                            state.update(s => ({ ...s, isStreaming: false }));

                                            // Resolve the promise with the final content
                                            resolve(endEvent.data.content);
                                        }
                                        else if (event.event === 'error') {
                                            // Stream error event
                                            const errorEvent = event as StreamErrorEvent;

                                            // Update state with the error
                                            state.update(s => ({
                                                ...s,
                                                isStreaming: false,
                                                error: errorEvent.data.error
                                            }));

                                            // Dispatch a custom event for the error
                                            dispatchCustomEvent('chat-stream-error', errorEvent.data);

                                            // Reject the promise with the error
                                            reject(new Error(errorEvent.data.error));
                                        }
                                    } catch (e) {
                                        console.error('Error parsing event JSON:', e);
                                        console.error('Raw line:', line);
                                    }
                                }
                            }
                        }

                        // If we get here without resolving, the stream ended without a proper end event
                        if (accumulatedContent) {
                            // If we have accumulated content, resolve with that
                            console.info("Stream ended without proper end event, but we have content");
                            state.update(s => ({ ...s, isStreaming: false }));
                            resolve(accumulatedContent);
                        } else {
                            // Check if we received any events at all
                            if (receivedAnyEvent) {
                                console.warn("Stream ended without content but we received some events");
                                state.update(s => ({ ...s, isStreaming: false }));
                                resolve("The model didn't generate any content. This might be due to an issue with the model or the prompt.");
                            } else {
                                // Otherwise, reject with an error
                                console.error("Stream ended unexpectedly without any content or events");
                                state.update(s => ({
                                    ...s,
                                    isStreaming: false,
                                    error: 'Stream ended without generating any content'
                                }));
                                reject(new Error('Stream ended without generating any content'));
                            }
                        }
                    } catch (err) {
                        // Handle errors during stream processing
                        const error = err as Error;
                        if (error.name === 'AbortError') {
                            // Request was aborted
                            state.update(s => ({ ...s, isStreaming: false }));
                            reject(new Error('Request was cancelled'));
                        } else {
                            // Other error
                            console.error('Error processing stream:', error);
                            state.update(s => ({
                                ...s,
                                isStreaming: false,
                                error: error.message || 'Error processing stream'
                            }));
                            reject(error);
                        }
                    }
                };

                // Start processing the stream
                processStream();
            })
            .catch(error => {
                // Handle fetch errors
                console.error('Fetch error:', error);
                console.error('Error details:', {
                    name: error.name,
                    message: error.message,
                    stack: error.stack,
                    cause: error.cause
                });

                // Check for network errors
                if (error.name === 'TypeError' && error.message.includes('Failed to fetch')) {
                    console.error('Network error: Check if the llm-proxy service is running on port 8083');
                }

                state.update(s => ({
                    ...s,
                    isStreaming: false,
                    error: error.message || 'Error fetching response'
                }));
                reject(error);
            });
        });
    };

    // Cancel the current request
    const cancelRequest = () => {
        if (currentController) {
            currentController.abort();
            currentController = null;
            state.update(s => ({ ...s, isStreaming: false }));
        }
    };

    // Helper function to dispatch custom events
    const dispatchCustomEvent = (eventName: string, detail: any) => {
        if (isBrowser) {
            const event = new CustomEvent(eventName, { detail });
            window.dispatchEvent(event);
        }
    };

    // Return the store with additional methods
    return {
        subscribe: state.subscribe,
        sendMessage,
        cancelRequest
    };
}

// Development mode flag
export const isDevelopment = import.meta.env.DEV;

// Check if we're in a browser environment
const isBrowser = typeof window !== 'undefined' && typeof fetch !== 'undefined';

// Function to check if the llm-proxy service is running
export async function checkLLMProxyHealth(): Promise<boolean> {
    try {
        const apiBaseUrl = (() => {
            if (isDevelopment) {
                if (isBrowser) {
                    const hostname = window.location.hostname;
                    return `http://${hostname}:8083`;
                }
                return 'http://localhost:8083';
            }
            return 'https://api.lyn.ai';
        })();

        console.log('Checking LLM proxy health at:', `${apiBaseUrl}/health`);
        const response = await fetch(`${apiBaseUrl}/health`, {
            method: 'GET',
            headers: {
                'Accept': 'text/plain'
            }
        });

        if (response.ok) {
            const text = await response.text();
            console.log('LLM proxy health check response:', text);
            return true;
        } else {
            console.error('LLM proxy health check failed with status:', response.status);
            return false;
        }
    } catch (error) {
        console.error('LLM proxy health check error:', error);
        return false;
    }
}

// Create a singleton instance of the chat store
export const chatStore = createChatStore();

// Log in development mode and check service health
if (isDevelopment && isBrowser) {
    console.log('Development mode detected. Chat store initialized.');

    // Add global error handlers
    window.addEventListener('unhandledrejection', (event) => {
        console.error('Unhandled promise rejection:', event.reason);
    });

    window.onerror = function(message, source, lineno, colno, error) {
        console.error('Global error:', message, source, lineno, colno, error);
        return false;
    };

    // Check if the llm-proxy service is running
    checkLLMProxyHealth().then(isHealthy => {
        if (isHealthy) {
            console.log('LLM proxy service is running and healthy');
        } else {
            console.error('LLM proxy service is not running or not healthy');
            console.error('Please make sure the llm-proxy service is running on port 8083');
        }
    });
}
