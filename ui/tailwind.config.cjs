/** @type {import('tailwindcss').Config}*/
const config = {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Quicksand', ...tailwind_theme.fontFamily.sans],
                mono: ['Victor Mono', ...tailwind_theme.fontFamily.mono],
                // or name them
                // 'victor-mono': ['Victor Mono'],
                // poppins: ['Poppins'],
            },
        },
    },

    plugins: [typography, daisyui],
}

module.exports = config
