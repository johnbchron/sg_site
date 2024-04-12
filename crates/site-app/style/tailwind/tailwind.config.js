/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        "anton": [ 'inter', 'ui-sans-serif', 'system-ui', 'sans-serif', ],
  			'sans': [
          'inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji",
          "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
        ],
      },
    },
    screens: {
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
    }
  },
  plugins: [],
}
