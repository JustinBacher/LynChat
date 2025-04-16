import type { Config } from "tailwindcss"

const config: Config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    backgroundImage: {
      "ai-gradient": "linear-gradient(135deg, #345995 0%, #4A6BA8 100%)",
      "chat-pattern":
        "url(\"data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M54.627 0l.83.828-1.415 1.415L51.8 0h2.827zM5.373 0l-.83.828L5.96 2.243 8.2 0H5.374zM48.97 0l3.657 3.657-1.414 1.414L46.143 0h2.828zM11.03 0L7.372 3.657 8.787 5.07 13.857 0H11.03zm32.284 0L49.8 6.485 48.384 7.9l-7.9-7.9h2.83zM16.686 0L10.2 6.485 11.616 7.9l7.9-7.9h-2.83zM22.344 0L13.858 8.485 15.272 9.9l7.9-7.9h-.828zm5.656 0L17.515 10.485 18.93 11.9l7.9-7.9h-2.83zm5.656 0L23.172 12.485 24.586 13.9l7.9-7.9h-2.83zm5.656 0L28.828 14.485 30.242 15.9l7.9-7.9h-2.83zM32 0l-9.9 9.9 1.415 1.415 7.9-7.9h-2.83L32 0zm5.657 0L30.142 7.515 28.728 6.1l7.9-7.9h2.83L37.657 0zm5.657 0L35.8 5.485 34.385 4.07l7.9-7.9h2.83L43.314 0zm5.657 0L41.456 3.485 40.042 2.07l7.9-7.9h2.83L48.97 0zm5.657 0L47.113 1.485 45.7.07l7.9-7.9h2.83L54.627 0zm5.657 0L52.77-.485 51.355-1.9l7.9-7.9h2.83L60.284 0z' fill='%23345995' fill-opacity='0.03' fill-rule='evenodd'/%3E%3C/svg%3E\")",
    },
    extend: {
      colors: {
        // Primary colors based on Prussian blue
        primary: "#345995",
        "primary-light": "#4A6BA8",
        "primary-dark": "#264980",

        // Accent colors
        "accent-orange": "#F76F3B",
        "accent-mustard": "#EFC94C",

        // UI colors
        background: "#F7F5F2", // Dirty light mode (slightly darker than Claude's white)
        "text-dark": "#333333", // Dark text for light backgrounds
        "text-light": "#F5F5F5", // Light text for dark backgrounds

        // Functional colors
        success: "#4CAF50",
        warning: "#FFC107",
        error: "#F44336",
        info: "#2196F3",

        // Message colors
        "user-bubble": "#E3F2FD",
        "ai-bubble": "#F5F5F5",
        "thought-bubble": "#EFEBE9",
        "tool-bubble": "#E8F5E9",
      },
      boxShadow: {
        card: "0 2px 8px rgba(0, 0, 0, 0.1)",
        elevated: "0 4px 12px rgba(0, 0, 0, 0.15)",
      },
      animation: {
        "fade-in": "fadeIn 0.2s ease-in-out",
        "blur-focus": "blurFocus 0.2s ease-in-out",
        "zoom-in": "zoomIn 0.2s ease-in-out",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        blurFocus: {
          "0%": { filter: "blur(10px)", opacity: "0" },
          "100%": { filter: "blur(0)", opacity: "1" },
        },
        zoomIn: {
          "0%": { transform: "scale(0.95)", opacity: "0" },
          "100%": { transform: "scale(1)", opacity: "1" },
        },
      },
    },
  },
  plugins: [],
}

export default config
