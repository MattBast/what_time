/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  safelist: ["hidden"],
  theme: {
    extend: {},
  },
  plugins: [],
};
