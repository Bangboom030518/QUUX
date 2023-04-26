const colors = require("tailwindcss/colors")

/** @type {import('tailwindcss').Config} */

module.exports = {
    content: ["./src/**/*.{html,css,rs}"],
    theme: {
        extend: {
            colors: {
                terrible: {
                    DEFAULT: colors.red["500"],
                    accent: colors.red["600"],
                },
                bad: {
                    DEFAULT: colors.orange["500"],
                    accent: colors.orange["600"]
                },
                ok: {
                    DEFAULT: colors.yellow["500"],
                    accent: colors.yellow["600"]
                },
                good: {
                    DEFAULT: colors.lime["500"],
                    accent: colors.lime["600"]
                },
                perfect: {
                    DEFAULT: colors.green["500"],
                    accent: colors.green["600"]
                }
            }
        },
    },
    daisyui: {
        themes: ["light", "dark"],
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
