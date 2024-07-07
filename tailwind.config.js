/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
      "./src/**/*.{rs, html, css}",
      "./dist/**/*.html"
  ],
  theme: {
    extend: {
        keyframes :{
            progressComplete: {
                '0%' : {
                    strokeDasharray : '0 1400',
                },
                "100%": {
                    strokeDasharray : '1257 1400',
                },
            }
        },
        animation: {
            'progress': "progressComplete 4s linear infinite alternate",
        }
    },
  },
  plugins: [],
}

