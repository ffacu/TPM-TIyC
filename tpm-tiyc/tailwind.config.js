/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "var(--color-primary)",
          hover: "var(--color-primary-hover)",
        },
        secondary: {
          DEFAULT: "var(--color-secondary)",
          hover: "var(--color-secondary-hover)",
        },
        background: "var(--color-background)",
        card: "var(--color-card)",
        text: {
          main: "var(--color-text-main)",
          muted: "var(--color-text-muted)",
        }
      }
    },
  },
  plugins: [],
}
