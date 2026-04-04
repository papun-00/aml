/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"IBM Plex Sans"', 'sans-serif'],
        mono: ['"IBM Plex Mono"', 'monospace'],
      },
      colors: {
        'carbon-blue-60': '#0f62fe', // Primary interaction
        'carbon-blue-70': '#0043ce', // Hover primary
        'carbon-gray-100': '#161616', // Dark background
        'carbon-gray-90': '#262626',
        'carbon-gray-80': '#393939',
        'carbon-gray-10': '#f4f4f4', // Light background element
        'ocean-blue': '#001d6c', // Adapted deep blue for Marine (Darker blue)
        'marine-teal': '#005d5d', // Adapted teal (Carbon Teal 70)
        'grid-line': 'rgba(15, 98, 254, 0.1)', // Marine blue grid 
      },
      animation: {
        'float': 'floating 3s ease-in-out infinite',
        'shrimp-pop': 'shrimpPop 0.8s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards',
        'spin-reverse': 'spin-reverse 25s linear infinite',
      },
      keyframes: {
        floating: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-15px)' },
        },
        shrimpPop: {
          '0%': { transform: 'scale(0) rotate(-15deg)', opacity: '0' },
          '100%': { transform: 'scale(1) rotate(0deg)', opacity: '1' },
        },
        'spin-reverse': {
          from: { transform: 'rotate(360deg)' },
          to: { transform: 'rotate(0deg)' },
        },
      },
    },
  },
  plugins: [],
}
