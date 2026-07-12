/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  theme: {
    extend: {
      colors: {
        "on-secondary-fixed-variant": "#4f5c70",
        "surface-container-lowest": "#ffffff",
        "on-error-container": "#752121",
        "error-container": "#fe8983",
        "error": "#9f403d",
        "on-error": "#fff7f6",
        "tertiary-container": "#215bf7",
        "tertiary": "#004de9",
        "primary": "#565e71",
        "primary-container": "#dbe2f9",
        "on-primary": "#f7f7ff",
        "on-primary-container": "#4a5264",
        "on-secondary-container": "#465365",
        "secondary-container": "#d6e3fb",
        "on-surface": "#323235",
        "on-background": "#323235",
        "on-surface-variant": "#5f5f61",
        "surface": "#fcf8f9",
        "surface-container": "#f0edef",
        "surface-container-high": "#eae7ea",
        "surface-container-low": "#f6f3f4",
        "surface-variant": "#e4e2e5",
        "outline": "#7b7a7d",
        "outline-variant": "#b3b1b4",
        "background": "#fcf8f9",
        "inverse-primary": "#e0e8ff"
      },
      fontFamily: {
        "headline": ["system-ui", "-apple-system", "Segoe UI", "Microsoft YaHei", "sans-serif"],
        "body": ["system-ui", "-apple-system", "Segoe UI", "Microsoft YaHei", "sans-serif"],
        "label": ["system-ui", "-apple-system", "Segoe UI", "Microsoft YaHei", "sans-serif"]
      }
    }
  }
}
