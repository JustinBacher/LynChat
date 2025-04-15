import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
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
};

export default config;
