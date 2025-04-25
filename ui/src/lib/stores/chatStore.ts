import { writable } from 'svelte/store';

// Create a store to hold the initial message
export const initialMessageStore = writable<string>('');
