const colors = require("@tailwindcss/colors")

/** @type {import('tailwindcss').Config} */

module.exports = {
    content: ["./src/**/*.{html,css,rs}"],
    theme: {
        extend: {
            colors: {
                terrible: {
                    DEFAULT: colors.red["500"],
                    accent: colors.red["600"],
                }
            }
        },
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
