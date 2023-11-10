/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./lib/templates/**/*.html'],
  theme: {
    extend: {
      colors: {
        bg: '#201e29',
        nav: '#271d2c'
      },
      spacing: {
        main: 'calc(100vh - 3.625rem)',
      },
      minHeight: {
        main: 'calc(100vh - 3.625rem)',
      }
    },
  },
  plugins: [],
}

