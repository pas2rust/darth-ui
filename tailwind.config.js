/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
    ],
    theme: {
      extend: {
        fontFamily: {
          'bebas-neue': ['Bebas Neue', 'sans-serif'],
          'fira-code': ['Fira Code', 'monospace'],
          'lato': ['Lato', 'sans-serif'],
          'league-spartan': ['League Spartan', 'sans-serif'],
          'poppins': ['Poppins', 'sans-serif'],
        },
        keyframes: {
          elastic: {
            "0%": { transform: "scale(1)" },
            "50%": { transform: "scale(1.25)" },
            "100%": { transform: "scale(1)" }
          },
          fadein: {
            "0%": { opacity: 0 },
            "100%": { opacity: 1 }
          },
          slideInRight: {
            "0%": {
              transform: "translateX(100%)"
            },
            "100%": {
              transform: "translateX(0)"
            }
          },
          slideInTop: {
            "0%": {
              transform: "translateY(-100%)"
            },
            "100%": {
              transform: "translateY(0)"
            }
          },
          exitInTop: {
            "0%": {
              transform: "translateY(0%)"
            },
            "100%": {
              transform: "translateY(-100%)"
            }
          },
          toastin: {
            "0%": { opacity: 0, transform: "scale(0)" },
            "100%": { opacity: 1, transform: "scale(1)" },
          },
          shake: {
            '0%':  { transform: 'translate(1px, 1px) rotate(0deg)' },
            '10%': { transform: 'translate(-1px, -2px) rotate(-1deg)' },
            '20%': { transform: 'translate(-3px, 0px) rotate(1deg)' },
            '30%': { transform: 'translate(3px, 2px) rotate(0deg)' },
            '40%': { transform: 'translate(1px, -1px) rotate(1deg)' },
            '50%': { transform: 'translate(-1px, 2px) rotate(-1deg)' },
            '60%': { transform: 'translate(-3px, 1px) rotate(0deg)' },
            '70%': { transform: 'translate(3px, 1px) rotate(-1deg)' },
            '80%': { transform: 'translate(-1px, -1px) rotate(1deg)' },
            '90%': { transform: 'translate(1px, 2px) rotate(0deg)' },
            '100%':{ transform: 'translate(1px, -2px) rotate(-1deg)' },
          },
        },
        animation: {
          shake: 'shake 1s ease-in-out',
          toast_in: "toast_in 1s ease-in-out",
          exitInTop: "exitInTop 1s ease-in-out",
          slideInRight: "slideInRight 1s",
          slideInTop: "slideInTop 1s",
          fadein: "fadein 1s ease-in-out",
          elastic: "elastic 1s ease-in-out",
          loop_fadein: "fadein 1s ease-in-out infinite",
          loop_shake: 'shake 1s ease-in-out infinite',
          loop_elastic: "elastic 1s ease-in-out infinite",
        }
      }
    },
    variants: {
      extend: {
        fontWeight: ['responsive', 'hover', 'focus'],
      },
    },
    plugins: [

    ],
};