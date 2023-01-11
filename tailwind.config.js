/** @type {import('tailwindcss').Config} */

module.exports = {
    content: ["./src/**/*.{html,css,rs}"],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
