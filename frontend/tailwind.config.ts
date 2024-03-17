import plugin from 'tailwindcss/plugin';

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  plugins: [
    plugin(({ matchUtilities, theme }) => {
      matchUtilities({
        "sidewinder-left": (value) => {
          return {
            "left": `calc(50% + ${value} / 2)`
          }
        },
        "sidewinder-right": (value) => {
          return {
            "right": `calc(50% + ${value} / 2)`
          }
        }
      }, {
        type: "position",
        values: { 
          ...Object.fromEntries(
            Object.entries(
              theme("screens") ?? {})
              .map(([k, v]) => [`screen-${k}`, v])
          ),
          ...theme("width")
        }
      })
    })
  ]
};
