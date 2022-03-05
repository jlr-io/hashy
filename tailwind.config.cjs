// module.exports = {
//   content: ['./src/**/*.{html,js,svelte,ts}'],
//   theme: {
//     extend: {},
//   },
//   plugins: [require("daisyui")],
//   // daisyUI config (optional)
//   daisyui: {
//     styled: true,
//     themes: true,
//     base: true,
//     utils: true,
//     logs: true,
//     rtl: false,
//     prefix: "",
//     darkTheme: "false",
//   },
// }
module.exports = {
  mode: "jit",
  purge: {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    options: {
      safelist: [/data-theme$/],
    },
  },
  plugins: [require("daisyui")],
  // daisyUI config (optional)
  daisyui: {
    styled: true,
    themes: true,
    base: true,
    utils: true,
    logs: true,
    rtl: false,
    prefix: "",
    darkTheme: "dark",
  },
};