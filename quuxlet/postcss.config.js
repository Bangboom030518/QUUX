const postcss = require("postcss")

module.exports = {
  plugins: [
    {
      postcssPlugin: "grouped",
      Once(root, { result }) {
        return postcss([
          require("postcss-nested"),
          require("postcss-mixins"),
          require("postcss-simple-vars"),
        ]).process(root, result.opts)
      },
    },
    require("tailwindcss"),
    // require("autoprefixer"),
    // require("cssnano")
  ],
}
