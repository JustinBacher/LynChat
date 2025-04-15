import { writable, Writable } from "svelte/store";

// Define the message type
export interface Message {
  id: string;
  type: "user" | "ai";
  text: string;
  timestamp: Date;
  thoughts?: string | null;
  tools?: any[];
}

// Initial chat state
interface ChatState {
  messages: Message[];
  isLoading: boolean;
  error: string | null;
  streamingMessage: Message | null;
}

const initialState: ChatState = {
  messages: [],
  isLoading: false,
  error: null,
  streamingMessage: null,
};

// Create the store
const createChatStore = () => {
  const { subscribe, set, update } = writable(initialState);

  return {
    subscribe,

    // Add a new message from the user
    addUserMessage: (text: string): string => {
      const message: Message = {
        id: Date.now().toString(),
        type: "user",
        text,
        timestamp: new Date(),
      };
      update((state) => ({
        ...state,
        messages: [...state.messages, message],
        isLoading: true,
      }));
      return message.id;
    },

    // Add a complete AI message
    addAIMessage: (
      text: string,
      thoughts: string | null = null,
      tools: any[] = [],
    ): string => {
      const message: Message = {
        id: Date.now().toString(),
        type: "ai",
        text,
        thoughts,
        tools,
        timestamp: new Date(),
      };
      update((state) => ({
        ...state,
        messages: [...state.messages, message],
        isLoading: false,
        streamingMessage: null,
      }));
      return message.id;
    },

    // Update a streaming message
    updateStreamingMessage: (text: string): void => {
      update((state) => ({
        ...state,
        streamingMessage: {
          id: "streaming",
          type: "ai",
          text,
          timestamp: new Date(),
        },
      }));
    },

    // Complete a streaming message
    completeStreamingMessage: (
      messageId: string,
      text: string,
      thoughts: string | null = null,
      tools: any[] = [],
    ): void => {
      update((state) => {
        // Find if we need to update an existing message
        const messageExists: boolean = state.messages.some(
          (m) => m.id === messageId,
        );

        if (messageExists) {
          // Update existing message
          return {
            ...state,
            messages: state.messages.map((m) =>
              m.id === messageId ? { ...m, text, thoughts, tools } : m,
            ),
            isLoading: false,
            streamingMessage: null,
          };
        } else {
          // Add new message
          return {
            ...state,
            messages: [
              ...state.messages,
              {
                id: messageId,
                type: "ai",
                text,
                thoughts,
                tools,
                timestamp: new Date(),
              },
            ],
            isLoading: false,
            streamingMessage: null,
          };
        }
      });
    },

    // Set an error
    setError: (error: string | null): void => {
      update((state) => ({
        ...state,
        error,
        isLoading: false,
      }));
    },

    // Clear all messages
    clearMessages: (): void => {
      update((state) => ({
        ...state,
        messages: [],
        error: null,
        streamingMessage: null,
      }));
    },
  };
};

// Export the store
export const chatStore = createChatStore();
