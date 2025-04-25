import { writable, type Readable, type Writable } from 'svelte/store';

// Helper function for delays
const delay = (ms: number): Promise<void> => new Promise(resolve => setTimeout(resolve, ms));

/**
 * Creates a Svelte store that cycles through strings with a typing effect.
 * Provides a `stop` method to clean up resources.
 * @param {string[]} strings - The array of strings to cycle through.
 * @param {number} [typingSpeed=70] - Speed of typing in milliseconds per character.
 * @param {number} [stallDuration=2000] - Duration to wait after typing is complete (in ms).
 * @returns {Readable<string> & { stop: () => void }} A readable store with an added `stop` method.
 */
function createTypingPlaceholderStore(
    strings: string[],
    typingSpeed: number = 70,
    stallDuration: number = 2000
): Readable<string> & { stop: () => void } { // Type the return object

    const placeholderStore: Writable<string> = writable('');

    let currentStringIndex: number = 0;
    let typingTimeout: ReturnType<typeof setTimeout> | undefined;
    let stallTimeout: ReturnType<typeof setTimeout> | undefined;
    let isRunning: boolean = true; // Flag to control the main loop

    // Function to type out a single string, updating the store
    const typeString = (text: string): Promise<void> => {
        // Clear any existing typing timeout before starting a new string
        if (typingTimeout) clearTimeout(typingTimeout);

        // If not running, stop immediately
        if (!isRunning) {
            placeholderStore.set(''); // Optionally clear placeholder on stop
            return Promise.resolve();
        }


        return new Promise(resolve => {
            let i = 0;
            // Only reset if we are starting a new typing sequence while running
            if (isRunning) {
                placeholderStore.set('');
            }


            function typeChar() {
                // Important: Check isRunning *before* each step
                if (!isRunning) {
                    clearTimeout(typingTimeout); // Ensure timeout is cleared
                    typingTimeout = undefined;
                    return; // Stop the recursive calls
                }

                if (i < text.length) {
                    placeholderStore.set(text.substring(0, i + 1));
                    i++;
                    typingTimeout = setTimeout(typeChar, typingSpeed);
                } else {
                    // Typing finished for this string
                    typingTimeout = undefined; // Clear timeout reference
                    resolve();
                }
            }

            typeChar(); // Start the typing process
        });
    };

    // Function to cycle through the strings
    const cycleStrings = async (): Promise<void> => {
        while (isRunning) { // Loop based on the flag
            const currentString: string = strings[currentStringIndex];

            // 1. Type the current placeholder string and wait
            //    The typeString function checks isRunning internally as well
            await typeString(currentString);

            // If isRunning became false while typing, break the loop
            if (!isRunning) break;

            // 2. Wait for the stall duration after typing is complete
            //    Use a cancellable delay if needed, but standard await delay
            //    is fine as the while(isRunning) check happens after the delay.
            await delay(stallDuration);

            // If isRunning became false while waiting, break the loop
            if (!isRunning) break;

            // 3. Move to the next placeholder, loop back if at the end
            let randomIndex;
            do {
                randomIndex = Math.floor(Math.random() * strings.length);
            } while (randomIndex === currentStringIndex && strings.length > 1); // Ensure a different index
            currentStringIndex = randomIndex;
        }
        // Cleanup timers when the loop finishes (only happens if isRunning becomes false)
        if (typingTimeout) clearTimeout(typingTimeout);
        if (stallTimeout) clearTimeout(stallTimeout); // If you were using stallTimeout separately
        typingTimeout = undefined;
        stallTimeout = undefined;
        placeholderStore.set(''); // Optionally clear the placeholder when stopping
    };

    // Start the cycling process
    cycleStrings();

    // Method to stop the loop and clean up timers
    const stop = () => {
        isRunning = false; // Set the flag to false to stop the loop
        if (typingTimeout) clearTimeout(typingTimeout); // Clear any active typing timeout
        if (stallTimeout) clearTimeout(stallTimeout); // Clear any active stall timeout
        typingTimeout = undefined;
        stallTimeout = undefined;
        placeholderStore.set(''); // Optionally clear placeholder on stop
    };

    // Return the store's subscribe method PLUS the new stop method
    return {
        subscribe: placeholderStore.subscribe,
        stop // Expose the stop method
    };
}

export default createTypingPlaceholderStore;
