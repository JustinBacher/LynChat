/** @type {import('tailwindcss').Config}*/
const config = {
    content: [
        './src/**/*.{html,js,svelte,ts}',
        './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
    ],
    darkMode: 'class',
    theme: {
        extend: {
            fontFamily: {
                sans: ['Fredoka', 'ui-sans-serif', 'system-ui', 'sans-serif'],
                mono: ['Victor Mono', 'ui-monospace', 'monospace'],
            },
            colors: {
                // Custom colors from PRD
                prussian: {
                    50: '#eef1f7',
                    100: '#d4dbe8',
                    200: '#b9c5d9',
                    300: '#9eafca',
                    400: '#8399bb',
                    500: '#6883ac',
                    600: '#4d6d9d',
                    700: '#345995', // Primary Prussian blue
                    800: '#294776',
                    900: '#1e3557',
                },
                autumn: {
                    50: '#fef2ee',
                    100: '#fde5dd',
                    200: '#fbcbbb',
                    300: '#f9b199',
                    400: '#f89777',
                    500: '#f87d55',
                    600: '#F76F3B', // Accent autumn orange
                    700: '#c55930',
                    800: '#934325',
                    900: '#622d19',
                },
                mustard: {
                    50: '#fdf9ed',
                    100: '#faf3db',
                    200: '#f6e7b7',
                    300: '#f2db93',
                    400: '#EFC94C', // Accent mustard yellow
                    500: '#e9b71e',
                    600: '#ba9218',
                    700: '#8c6e12',
                    800: '#5d490c',
                    900: '#2f2506',
                },
                // Override primary colors to use Prussian blue
                primary: {
                    50: '#eef1f7',
                    100: '#d4dbe8',
                    200: '#b9c5d9',
                    300: '#9eafca',
                    400: '#8399bb',
                    500: '#6883ac',
                    600: '#4d6d9d',
                    700: '#345995', // Primary Prussian blue
                    800: '#294776',
                    900: '#1e3557',
                }
            }
        },
    },
    plugins: [require('@tailwindcss/typography'), require('flowbite/plugin')],
}

module.exports = config
